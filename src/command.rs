use std::fmt;
use std::str::FromStr;
use std::convert::AsRef;
use self::Command::{Select, Load, Status, Dump, Delete, TableCreate, TableRename, TableRemove,
                    TableList, ColumnCreate, ColumnList, ColumnRename, ColumnRemove,
                    RequestCancel, Normalize, NormalizerList, PluginRegister, PluginUnregister,
                    Tokenize, TokenizerList, Truncate, Schema, Extension};

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
    TableList,
    TableRename,
    TableRemove,
    ColumnCreate,
    ColumnList,
    ColumnRename,
    ColumnRemove,
    RequestCancel,
    Normalize,
    NormalizerList,
    PluginRegister,
    PluginUnregister,
    Tokenize,
    TokenizerList,
    Truncate,
    Schema,
    /// Method extensions.
    ///
    /// An example would be:
    ///
    /// ```
    /// use std::str::FromStr;
    /// use ruroonga_command::command::Command;
    /// let added = Command::Extension("added-command".to_string());
    /// assert!(added == FromStr::from_str("added-command").unwrap())
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
            TableList => "table_list",
            TableRename => "table_rename",
            TableRemove => "table_remove",
            ColumnCreate => "column_create",
            ColumnList => "column_list",
            ColumnRename => "column_rename",
            ColumnRemove => "column_remove",
            RequestCancel => "request_cancel",
            Normalize => "normalize",
            NormalizerList => "normalizer_list",
            PluginRegister => "plugin_register",
            PluginUnregister => "plugin_unregister",
            Tokenize => "tokenize",
            TokenizerList => "tokenizer_list",
            Truncate => "truncate",
            Schema => "schema",
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
                "table_list" => TableList,
                "table_rename" => TableRename,
                "table_remove" => TableRemove,
                "column_create" => ColumnCreate,
                "column_list" => ColumnList,
                "column_rename" => ColumnRename,
                "column_remove" => ColumnRemove,
                "normalize" => Normalize,
                "normalizer_list" => NormalizerList,
                "plugin_register" => PluginRegister,
                "plugin_unregister" => PluginUnregister,
                "tokenize" => Tokenize,
                "tokenizer_list" => TokenizerList,
                "truncate" => Truncate,
                "request_cancel" => RequestCancel,
                "schema" => Schema,
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
            TableList => "table_list",
            TableRename => "table_rename",
            TableRemove => "table_remove",
            ColumnCreate => "column_create",
            ColumnList => "column_list",
            ColumnRename => "column_rename",
            ColumnRemove => "column_remove",
            RequestCancel => "request_cancel",
            Normalize => "normalize",
            NormalizerList => "normalizer_list",
            PluginRegister => "plugin_register",
            PluginUnregister => "plugin_unregister",
            Tokenize => "tokenize",
            TokenizerList => "tokenizer_list",
            Truncate => "truncate",
            Schema => "schema",
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
    use super::Command::{Select, Load, Status, Dump, Delete, TableCreate, TableList, TableRemove,
                         TableRename, ColumnCreate, ColumnList, ColumnRename, ColumnRemove,
                         RequestCancel, Normalize, NormalizerList, PluginRegister,
                         PluginUnregister, Tokenize, TokenizerList, Truncate, Schema, Extension};

    #[test]
    fn test_from_str() {
        assert_eq!(Select, FromStr::from_str("select").unwrap());
        assert_eq!(ColumnCreate, FromStr::from_str("column_create").unwrap());
        assert_eq!(ColumnList, FromStr::from_str("column_list").unwrap());
        assert_eq!(ColumnRename, FromStr::from_str("column_rename").unwrap());
        assert_eq!(ColumnRemove, FromStr::from_str("column_remove").unwrap());
        assert_eq!(RequestCancel, FromStr::from_str("request_cancel").unwrap());
        assert_eq!(TableCreate, FromStr::from_str("table_create").unwrap());
        assert_eq!(TableRename, FromStr::from_str("table_rename").unwrap());
        assert_eq!(TableRemove, FromStr::from_str("table_remove").unwrap());
        assert_eq!(TableList, FromStr::from_str("table_list").unwrap());
        assert_eq!(Normalize, FromStr::from_str("normalize").unwrap());
        assert_eq!(NormalizerList,
                   FromStr::from_str("normalizer_list").unwrap());
        assert_eq!(PluginRegister,
                   FromStr::from_str("plugin_register").unwrap());
        assert_eq!(PluginUnregister,
                   FromStr::from_str("plugin_unregister").unwrap());
        assert_eq!(Tokenize, FromStr::from_str("tokenize").unwrap());
        assert_eq!(TokenizerList, FromStr::from_str("tokenizer_list").unwrap());
        assert_eq!(Truncate, FromStr::from_str("truncate").unwrap());
        assert_eq!(Schema, FromStr::from_str("schema").unwrap());
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
        assert_eq!("column_list".to_owned(), format!("{}", ColumnList));
        assert_eq!("column_rename".to_owned(), format!("{}", ColumnRename));
        assert_eq!("column_remove".to_owned(), format!("{}", ColumnRemove));
        assert_eq!("request_cancel".to_owned(), format!("{}", RequestCancel));
        assert_eq!("table_list".to_owned(), format!("{}", TableList));
        assert_eq!("table_rename".to_owned(), format!("{}", TableRename));
        assert_eq!("table_remove".to_owned(), format!("{}", TableRemove));
        assert_eq!("normalize".to_owned(), format!("{}", Normalize));
        assert_eq!("normalizer_list".to_owned(), format!("{}", NormalizerList));
        assert_eq!("plugin_register".to_owned(), format!("{}", PluginRegister));
        assert_eq!("plugin_unregister".to_owned(),
                   format!("{}", PluginUnregister));
        assert_eq!("tokenize".to_owned(), format!("{}", Tokenize));
        assert_eq!("tokenizer_list".to_owned(), format!("{}", TokenizerList));
        assert_eq!("truncate".to_owned(), format!("{}", Truncate));
        assert_eq!("schema".to_owned(), format!("{}", Schema));
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
        assert_eq!(TableList.as_ref(), "table_list");
        assert_eq!(TableRename.as_ref(), "table_rename");
        assert_eq!(TableRemove.as_ref(), "table_remove");
        assert_eq!(ColumnCreate.as_ref(), "column_create");
        assert_eq!(ColumnList.as_ref(), "column_list");
        assert_eq!(ColumnRename.as_ref(), "column_rename");
        assert_eq!(ColumnRemove.as_ref(), "column_remove");
        assert_eq!(RequestCancel.as_ref(), "request_cancel");
        assert_eq!(Normalize.as_ref(), "normalize");
        assert_eq!(NormalizerList.as_ref(), "normalizer_list");
        assert_eq!(PluginRegister.as_ref(), "plugin_register");
        assert_eq!(PluginUnregister.as_ref(), "plugin_unregister");
        assert_eq!(Tokenize.as_ref(), "tokenize");
        assert_eq!(TokenizerList.as_ref(), "tokenizer_list");
        assert_eq!(Truncate.as_ref(), "truncate");
        assert_eq!(Schema.as_ref(), "schema");
        assert_eq!(Extension("added-command".to_owned()).as_ref(),
                   "added-command");
    }
}
