use std::fmt;
use std::str::FromStr;
use std::convert::AsRef;
use self::Command::{Select, Load, Status, Dump, Delete, TableCreate, TableRename, ColumnCreate,
                    RequestCancel, Extension};

#[derive (Debug)]
pub enum CommandError {
    Empty,
}

#[derive (Clone, PartialEq, Eq, Debug)]
pub enum Command {
    Select,
    Load,
    Status,
    Dump,
    Delete,
    TableCreate,
    TableRename,
    ColumnCreate,
    RequestCancel,
    /// Method extensions.
    ///
    /// An example would be:
    ///
    /// ```
    /// extern crate ruroonga_command as ruroonga;
    /// let m =
    ///     ruroonga::command::Command::Extension("added-command".to_string());
    /// ```
    Extension(String),
}

impl AsRef<str> for Command {
    fn as_ref(&self) -> &str {
        match *self {
            Select => "select",
            Load => "load",
            Status => "status",
            Dump => "dump",
            Delete => "delete",
            TableCreate => "table_create",
            TableRename => "table_rename",
            ColumnCreate => "column_create",
            RequestCancel => "request_cancel",
            Extension(ref s) => s.as_ref(),
        }
    }
}

impl FromStr for Command {
    type Err = CommandError;
    fn from_str(s: &str) -> Result<Command, CommandError> {
        if s == "" {
            Err(CommandError::Empty)
        } else {
            Ok(match s {
                "select" => Select,
                "load" => Load,
                "status" => Status,
                "dump" => Dump,
                "delete" => Delete,
                "table_create" => TableCreate,
                "table_rename" => TableRename,
                "column_create" => ColumnCreate,
                "request_cancel" => RequestCancel,
                _ => Extension(s.to_owned()),
            })
        }
    }
}

impl fmt::Display for Command {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(match *self {
            Select => "select",
            Load => "load",
            Status => "status",
            Dump => "dump",
            Delete => "delete",
            TableCreate => "table_create",
            TableRename => "table_rename",
            ColumnCreate => "column_create",
            RequestCancel => "request_cancel",
            Extension(ref s) => s.as_ref(),
        })
    }
}

pub type Query = Vec<(String, String)>;


#[cfg(test)]
mod test {
    use std::str::FromStr;
    use super::CommandError;
    use super::Command;
    use super::Command::{Select, Load, Status, Dump, Delete, TableCreate, TableRename,
                         ColumnCreate, RequestCancel, Extension};

    #[test]
    fn test_from_str() {
        assert_eq!(Select, FromStr::from_str("select").unwrap());
        assert_eq!(ColumnCreate, FromStr::from_str("column_create").unwrap());
        assert_eq!(RequestCancel, FromStr::from_str("request_cancel").unwrap());
        assert_eq!(TableRename, FromStr::from_str("table_rename").unwrap());
        assert_eq!(Extension("added-command".to_owned()),
                   FromStr::from_str("added-command").unwrap());
        let x: Result<Command, _> = FromStr::from_str("");
        if let Err(CommandError::Empty) = x {
        } else {
            panic!("An empty command is invalid!")
        }
    }

    #[test]
    fn test_fmt() {
        assert_eq!("load".to_owned(), format!("{}", Load));
        assert_eq!("table_create".to_owned(), format!("{}", TableCreate));
        assert_eq!("column_create".to_owned(), format!("{}", ColumnCreate));
        assert_eq!("request_cancel".to_owned(), format!("{}", RequestCancel));
        assert_eq!("table_rename".to_owned(), format!("{}", TableRename));
        assert_eq!("added-command".to_owned(),
                   format!("{}", Extension("added-command".to_owned())));
    }

    #[test]
    fn test_as_str() {
        assert_eq!(Select.as_ref(), "select");
        assert_eq!(Load.as_ref(), "load");
        assert_eq!(Dump.as_ref(), "dump");
        assert_eq!(Delete.as_ref(), "delete");
        assert_eq!(Status.as_ref(), "status");
        assert_eq!(TableCreate.as_ref(), "table_create");
        assert_eq!(TableRename.as_ref(), "table_rename");
        assert_eq!(ColumnCreate.as_ref(), "column_create");
        assert_eq!(RequestCancel.as_ref(), "request_cancel");
        assert_eq!(Extension("added-command".to_owned()).as_ref(),
                   "added-command");
    }
}
