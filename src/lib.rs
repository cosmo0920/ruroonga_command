extern crate url;

use std::fmt;
use url::form_urlencoded;
use std::str::FromStr;
use std::convert::AsRef;
use self::Command::{Select, Load, Status, Dump, Delete, Extension};

#[derive (Debug)]
pub enum CommandError { Empty }

#[derive (Clone, PartialEq, Eq, Debug)]
pub enum Command {
    Select,
    Load,
    Status,
    Dump,
    Delete,
    /// Method extensions.
    ///
    /// An example would be:
    ///
    /// ```
    /// extern crate ruroonga_command as command;
    /// let m = command::Command::Extension("added-command".to_string());
    /// ```
    Extension(String)
}

impl AsRef<str> for Command {
    fn as_ref(&self) -> &str {
        match *self {
            Select => "select",
            Load => "load",
            Status => "status",
            Dump => "dump",
            Delete => "delete",
            Extension(ref s) => s.as_ref()
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
                _ => Extension(s.to_owned())
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
            Extension(ref s) => s.as_ref()
        })
    }
}

pub type Query<'a> = Vec<(&'a str, &'a str)>;

#[derive(Clone, Debug)]
pub struct CommandQuery<'a> {
    command: Command,
    arguments: Query<'a>,
    prefix: String,
}

impl<'a> Default for CommandQuery<'a> {
    fn default() -> CommandQuery<'a> {
        CommandQuery {
            command: Dump,
            arguments: vec![],
            prefix: "/d".to_string(),
        }
    }
}

impl<'a> CommandQuery<'a> {
    pub fn new(command: Command) -> CommandQuery<'a> {
        let default: CommandQuery = Default::default();
        CommandQuery {
            command: command,
            arguments: default.arguments,
            prefix: default.prefix,
        }
    }

    /// Get vectorize `("key", "value")` pairs to construct url encoded query.
    pub fn get_command(&mut self) -> Command {
        self.command.clone()
    }

    /// Set vectorize `("key", "value")` pairs to construct url encoded query.
    pub fn set_argument(&mut self, arguments: Query<'a>) {
        self.arguments = arguments
    }

    #[doc(hidden)]
    // get HTTP URI prefix. default: /d
    // This function is mainly provided for internal usage.
    pub fn get_prefix(&mut self) -> String {
        self.prefix.clone()
    }

    #[doc(hidden)]
    // set HTTP URI prefix. This function is provided for advanced user.
    pub fn set_prefix(&mut self, prefix: String) {
        self.prefix = prefix
    }

    /// Create url encoded command query.
    ///
    /// `vec![("key","value")]` interprets to `"key=value"`.
    /// And two or more value pair are concatinate with `&`.
    pub fn make_query(&mut self) -> String {
        form_urlencoded::serialize(self.arguments.clone().into_iter())
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;
    use super::CommandError;
    use super::Command;
    use super::Command::{Select, Load, Status, Dump, Delete, Extension};

    #[test]
    fn test_from_str() {
        assert_eq!(Select, FromStr::from_str("select").unwrap());
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
        assert_eq!(Extension("added-command".to_owned()).as_ref(),
                   "added-command");
    }
}
