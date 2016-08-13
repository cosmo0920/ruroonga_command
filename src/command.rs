use std::fmt;
use std::str::FromStr;
use std::convert::AsRef;
use self::Command::{Select, Load, Status, CacheLimit, Dump, DatabaseUnmap, Delete, IoFlush,
                    LogLevel, LogPut, LogReopen, TableCreate, TableRename, TableRemove, TableList,
                    ColumnCopy, ColumnCreate, ColumnList, ColumnRename, ColumnRemove,
                    RequestCancel, Normalize, NormalizerList, ObjectExist, ObjectInspect,
                    ObjectRemove, PluginRegister, PluginUnregister, ThreadLimit, Tokenize,
                    TokenizerList, Truncate, Schema, Shutdown, Extension};
#[cfg(feature="sharding")]
use self::Command::{LogicalSelect, LogicalShardList, LogicalTableRemove};

#[derive (Debug)]
pub enum CommandError {
    Empty,
}

#[derive (Clone, PartialEq, Eq, Debug)]
pub enum Command {
    Select,
    Load,
    Status,
    CacheLimit,
    Dump,
    DatabaseUnmap,
    Delete,
    IoFlush,
    LogLevel,
    LogPut,
    LogReopen,
    TableCreate,
    TableList,
    TableRename,
    TableRemove,
    ColumnCopy,
    ColumnCreate,
    ColumnList,
    ColumnRename,
    ColumnRemove,
    RequestCancel,
    Normalize,
    NormalizerList,
    ObjectExist,
    ObjectInspect,
    ObjectRemove,
    PluginRegister,
    PluginUnregister,
    ThreadLimit,
    Tokenize,
    TokenizerList,
    Truncate,
    Schema,
    Shutdown,
    #[cfg(feature="sharding")]
    LogicalSelect,
    #[cfg(feature="sharding")]
    LogicalShardList,
    #[cfg(feature="sharding")]
    LogicalTableRemove,
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
            CacheLimit => "cache_limit",
            Dump => "dump",
            DatabaseUnmap => "database_unmap",
            Delete => "delete",
            IoFlush => "io_flush",
            LogLevel => "log_level",
            LogPut => "log_put",
            LogReopen => "log_reopen",
            TableCreate => "table_create",
            TableList => "table_list",
            TableRename => "table_rename",
            TableRemove => "table_remove",
            ColumnCopy => "column_copy",
            ColumnCreate => "column_create",
            ColumnList => "column_list",
            ColumnRename => "column_rename",
            ColumnRemove => "column_remove",
            RequestCancel => "request_cancel",
            ObjectExist => "object_exist",
            ObjectInspect => "object_inspect",
            ObjectRemove => "object_remove",
            Normalize => "normalize",
            NormalizerList => "normalizer_list",
            PluginRegister => "plugin_register",
            PluginUnregister => "plugin_unregister",
            ThreadLimit => "thread_limit",
            Tokenize => "tokenize",
            TokenizerList => "tokenizer_list",
            Truncate => "truncate",
            Schema => "schema",
            Shutdown => "shutdown",
            #[cfg(feature="sharding")]
            LogicalSelect => "logical_select",
            #[cfg(feature="sharding")]
            LogicalShardList => "logical_shard_list",
            #[cfg(feature="sharding")]
            LogicalTableRemove => "logical_table_remove",
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
                "cache_limit" => CacheLimit,
                "dump" => Dump,
                "database_unmap" => DatabaseUnmap,
                "delete" => Delete,
                "io_flush" => IoFlush,
                "log_level" => LogLevel,
                "log_put" => LogPut,
                "log_reopen" => LogReopen,
                "table_create" => TableCreate,
                "table_list" => TableList,
                "table_rename" => TableRename,
                "table_remove" => TableRemove,
                "column_copy" => ColumnCopy,
                "column_create" => ColumnCreate,
                "column_list" => ColumnList,
                "column_rename" => ColumnRename,
                "column_remove" => ColumnRemove,
                "object_exist" => ObjectExist,
                "object_inspect" => ObjectInspect,
                "object_remove" => ObjectRemove,
                "normalize" => Normalize,
                "normalizer_list" => NormalizerList,
                "plugin_register" => PluginRegister,
                "plugin_unregister" => PluginUnregister,
                "thread_limit" => ThreadLimit,
                "tokenize" => Tokenize,
                "tokenizer_list" => TokenizerList,
                "truncate" => Truncate,
                "request_cancel" => RequestCancel,
                "schema" => Schema,
                "shutdown" => Shutdown,
                #[cfg(feature="sharding")]
                "logical_select" => LogicalSelect,
                #[cfg(feature="sharding")]
                "logical_shard_list" => LogicalShardList,
                #[cfg(feature="sharding")]
                "logical_table_remove" => LogicalTableRemove,
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
            CacheLimit => "cache_limit",
            Dump => "dump",
            DatabaseUnmap => "database_unmap",
            Delete => "delete",
            IoFlush => "io_flush",
            LogLevel => "log_level",
            LogPut => "log_put",
            LogReopen => "log_reopen",
            TableCreate => "table_create",
            TableList => "table_list",
            TableRename => "table_rename",
            TableRemove => "table_remove",
            ColumnCopy => "column_copy",
            ColumnCreate => "column_create",
            ColumnList => "column_list",
            ColumnRename => "column_rename",
            ColumnRemove => "column_remove",
            RequestCancel => "request_cancel",
            ObjectExist => "object_exist",
            ObjectInspect => "object_inspect",
            ObjectRemove => "object_remove",
            Normalize => "normalize",
            NormalizerList => "normalizer_list",
            PluginRegister => "plugin_register",
            PluginUnregister => "plugin_unregister",
            ThreadLimit => "thread_limit",
            Tokenize => "tokenize",
            TokenizerList => "tokenizer_list",
            Truncate => "truncate",
            Schema => "schema",
            Shutdown => "shutdown",
            #[cfg(feature="sharding")]
            LogicalSelect => "logical_select",
            #[cfg(feature="sharding")]
            LogicalShardList => "logical_shard_list",
            #[cfg(feature="sharding")]
            LogicalTableRemove => "logical_table_remove",
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
    use super::Command::{Select, Load, Status, CacheLimit, Dump, DatabaseUnmap, Delete, IoFlush,
                         LogLevel, LogPut, LogReopen, TableCreate, TableList, TableRemove,
                         TableRename, ColumnCopy, ColumnCreate, ColumnList, ColumnRename,
                         ColumnRemove, RequestCancel, ObjectExist, ObjectInspect, ObjectRemove,
                         Normalize, NormalizerList, PluginRegister, PluginUnregister, ThreadLimit,
                         Tokenize, TokenizerList, Truncate, Schema, Shutdown, Extension};
    #[cfg(feature="sharding")]
    use super::Command::{LogicalSelect, LogicalShardList, LogicalTableRemove};

    #[test]
    fn test_from_str() {
        assert_eq!(Select, FromStr::from_str("select").unwrap());
        assert_eq!(CacheLimit, FromStr::from_str("cache_limit").unwrap());
        assert_eq!(DatabaseUnmap, FromStr::from_str("database_unmap").unwrap());
        assert_eq!(ColumnCopy, FromStr::from_str("column_copy").unwrap());
        assert_eq!(ColumnCreate, FromStr::from_str("column_create").unwrap());
        assert_eq!(ColumnList, FromStr::from_str("column_list").unwrap());
        assert_eq!(ColumnRename, FromStr::from_str("column_rename").unwrap());
        assert_eq!(ColumnRemove, FromStr::from_str("column_remove").unwrap());
        assert_eq!(IoFlush, FromStr::from_str("io_flush").unwrap());
        assert_eq!(LogLevel, FromStr::from_str("log_level").unwrap());
        assert_eq!(LogPut, FromStr::from_str("log_put").unwrap());
        assert_eq!(LogReopen, FromStr::from_str("log_reopen").unwrap());
        assert_eq!(RequestCancel, FromStr::from_str("request_cancel").unwrap());
        assert_eq!(TableCreate, FromStr::from_str("table_create").unwrap());
        assert_eq!(TableRename, FromStr::from_str("table_rename").unwrap());
        assert_eq!(TableRemove, FromStr::from_str("table_remove").unwrap());
        assert_eq!(TableList, FromStr::from_str("table_list").unwrap());
        assert_eq!(ObjectExist, FromStr::from_str("object_exist").unwrap());
        assert_eq!(ObjectInspect, FromStr::from_str("object_inspect").unwrap());
        assert_eq!(Normalize, FromStr::from_str("normalize").unwrap());
        assert_eq!(NormalizerList,
                   FromStr::from_str("normalizer_list").unwrap());
        assert_eq!(PluginRegister,
                   FromStr::from_str("plugin_register").unwrap());
        assert_eq!(PluginUnregister,
                   FromStr::from_str("plugin_unregister").unwrap());
        assert_eq!(ThreadLimit, FromStr::from_str("thread_limit").unwrap());
        assert_eq!(Tokenize, FromStr::from_str("tokenize").unwrap());
        assert_eq!(TokenizerList, FromStr::from_str("tokenizer_list").unwrap());
        assert_eq!(Truncate, FromStr::from_str("truncate").unwrap());
        assert_eq!(Schema, FromStr::from_str("schema").unwrap());
        assert_eq!(Shutdown, FromStr::from_str("shutdown").unwrap());
        assert_eq!(Extension("added-command".to_owned()),
                   FromStr::from_str("added-command").unwrap());
        let x: Result<Command, _> = FromStr::from_str("");
        if let Err(CommandError::Empty) = x {
        } else {
            panic!("An empty command is invalid!")
        }
    }

    #[cfg(feature="sharding")]
    #[test]
    fn test_from_str_with_sharding() {
        assert_eq!(LogicalSelect, FromStr::from_str("logical_select").unwrap());
        assert_eq!(LogicalShardList,
                   FromStr::from_str("logical_shard_list").unwrap());
        assert_eq!(LogicalTableRemove,
                   FromStr::from_str("logical_table_remove").unwrap());
    }

    #[test]
    fn test_fmt() {
        assert_eq!("load".to_owned(), format!("{}", Load));
        assert_eq!("cache_limit".to_owned(), format!("{}", CacheLimit));
        assert_eq!("database_unmap".to_owned(), format!("{}", DatabaseUnmap));
        assert_eq!("table_create".to_owned(), format!("{}", TableCreate));
        assert_eq!("column_copy".to_owned(), format!("{}", ColumnCopy));
        assert_eq!("column_create".to_owned(), format!("{}", ColumnCreate));
        assert_eq!("column_list".to_owned(), format!("{}", ColumnList));
        assert_eq!("column_rename".to_owned(), format!("{}", ColumnRename));
        assert_eq!("column_remove".to_owned(), format!("{}", ColumnRemove));
        assert_eq!("io_flush".to_owned(), format!("{}", IoFlush));
        assert_eq!("log_level".to_owned(), format!("{}", LogLevel));
        assert_eq!("log_put".to_owned(), format!("{}", LogPut));
        assert_eq!("log_reopen".to_owned(), format!("{}", LogReopen));
        assert_eq!("request_cancel".to_owned(), format!("{}", RequestCancel));
        assert_eq!("table_list".to_owned(), format!("{}", TableList));
        assert_eq!("table_rename".to_owned(), format!("{}", TableRename));
        assert_eq!("table_remove".to_owned(), format!("{}", TableRemove));
        assert_eq!("object_exist".to_owned(), format!("{}", ObjectExist));
        assert_eq!("object_inspect".to_owned(), format!("{}", ObjectInspect));
        assert_eq!("object_remove".to_owned(), format!("{}", ObjectRemove));
        assert_eq!("normalize".to_owned(), format!("{}", Normalize));
        assert_eq!("normalizer_list".to_owned(), format!("{}", NormalizerList));
        assert_eq!("plugin_register".to_owned(), format!("{}", PluginRegister));
        assert_eq!("plugin_unregister".to_owned(),
                   format!("{}", PluginUnregister));
        assert_eq!("thread_limit".to_owned(), format!("{}", ThreadLimit));
        assert_eq!("tokenize".to_owned(), format!("{}", Tokenize));
        assert_eq!("tokenizer_list".to_owned(), format!("{}", TokenizerList));
        assert_eq!("truncate".to_owned(), format!("{}", Truncate));
        assert_eq!("schema".to_owned(), format!("{}", Schema));
        assert_eq!("shutdown".to_owned(), format!("{}", Shutdown));
        assert_eq!("added-command".to_owned(),
                   format!("{}", Extension("added-command".to_owned())));
    }

    #[cfg(feature="sharding")]
    #[test]
    fn test_fmt_with_sharding() {
        assert_eq!("logical_select".to_owned(), format!("{}", LogicalSelect));
        assert_eq!("logical_shard_list".to_owned(),
                   format!("{}", LogicalShardList));
        assert_eq!("logical_table_remove".to_owned(),
                   format!("{}", LogicalTableRemove));
    }

    #[test]
    fn test_as_str() {
        assert_eq!(Select.as_ref(), "select");
        assert_eq!(Load.as_ref(), "load");
        assert_eq!(Dump.as_ref(), "dump");
        assert_eq!(CacheLimit.as_ref(), "cache_limit");
        assert_eq!(DatabaseUnmap.as_ref(), "database_unmap");
        assert_eq!(Delete.as_ref(), "delete");
        assert_eq!(Status.as_ref(), "status");
        assert_eq!(IoFlush.as_ref(), "io_flush");
        assert_eq!(LogLevel.as_ref(), "log_level");
        assert_eq!(LogPut.as_ref(), "log_put");
        assert_eq!(LogReopen.as_ref(), "log_reopen");
        assert_eq!(TableCreate.as_ref(), "table_create");
        assert_eq!(TableList.as_ref(), "table_list");
        assert_eq!(TableRename.as_ref(), "table_rename");
        assert_eq!(TableRemove.as_ref(), "table_remove");
        assert_eq!(ColumnCopy.as_ref(), "column_copy");
        assert_eq!(ColumnCreate.as_ref(), "column_create");
        assert_eq!(ColumnList.as_ref(), "column_list");
        assert_eq!(ColumnRename.as_ref(), "column_rename");
        assert_eq!(ColumnRemove.as_ref(), "column_remove");
        assert_eq!(RequestCancel.as_ref(), "request_cancel");
        assert_eq!(ObjectExist.as_ref(), "object_exist");
        assert_eq!(ObjectInspect.as_ref(), "object_inspect");
        assert_eq!(ObjectRemove.as_ref(), "object_remove");
        assert_eq!(Normalize.as_ref(), "normalize");
        assert_eq!(NormalizerList.as_ref(), "normalizer_list");
        assert_eq!(PluginRegister.as_ref(), "plugin_register");
        assert_eq!(PluginUnregister.as_ref(), "plugin_unregister");
        assert_eq!(ThreadLimit.as_ref(), "thread_limit");
        assert_eq!(Tokenize.as_ref(), "tokenize");
        assert_eq!(TokenizerList.as_ref(), "tokenizer_list");
        assert_eq!(Truncate.as_ref(), "truncate");
        assert_eq!(Schema.as_ref(), "schema");
        assert_eq!(Shutdown.as_ref(), "shutdown");
        assert_eq!(Extension("added-command".to_owned()).as_ref(),
                   "added-command");
    }

    #[cfg(feature="sharding")]
    #[test]
    fn test_as_str_with_sharding() {
        assert_eq!(LogicalSelect.as_ref(), "logical_select");
        assert_eq!(LogicalShardList.as_ref(), "logical_shard_list");
        assert_eq!(LogicalTableRemove.as_ref(), "logical_table_remove");
    }
}
