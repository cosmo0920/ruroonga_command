use std::fmt;
use std::str::FromStr;
use std::convert::AsRef;
use std::collections::HashMap;
use command::{Command, Query};
use util;
use selectable::fragmentable::Fragmentable;
use selectable::fragmentable::{OrderedFragment, QueryFragment};
use types::data_type::DataType;
use types::column_flag_type::ColumnFlagType;
use self::WindowFunction::{RecordNumber, ExtWindowFunction};
use self::WindowableColumn::{Window, Value};

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub enum WindowableColumn<W, V> {
    Window(W),
    Value(V),
}

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub enum WindowFunction {
    RecordNumber,
    /// For future extensibility.
    ExtWindowFunction(String),
}

impl AsRef<str> for WindowFunction {
    fn as_ref(&self) -> &str {
        match *self {
            RecordNumber => "record_number()",
            ExtWindowFunction(ref s) => s.as_ref(),
        }
    }
}

#[derive (Debug)]
pub enum WindowFunctionError {
    Empty,
}

impl FromStr for WindowFunction {
    type Err = WindowFunctionError;
    fn from_str(s: &str) -> Result<WindowFunction, WindowFunctionError> {
        if s == "" {
            Err(WindowFunctionError::Empty)
        } else {
            Ok(match s {
                "record_number" | "RecordNumber" => RecordNumber,
                /// For future extensibility.
                _ => ExtWindowFunction(s.to_owned()),
            })
        }
    }
}

impl fmt::Display for WindowFunction {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(match *self {
            RecordNumber => "record_number()",
            ExtWindowFunction(ref s) => s.as_ref(),
        })
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct StagedColumns {
    label: String,
    stage: String,
    column_type: DataType,
    value: WindowableColumn<WindowFunction, String>,
    arguments: HashMap<String, String>,
}

impl StagedColumns {
    pub fn new(label: String,
               stage: String,
               column_type: DataType,
               value: WindowableColumn<WindowFunction, String>)
               -> StagedColumns {
        StagedColumns {
            label: label,
            stage: stage,
            column_type: column_type,
            value: value,
            arguments: HashMap::new(),
        }
    }

    pub fn flags(mut self, flags: Vec<ColumnFlagType>) -> StagedColumns {
        let flags_key = util::labeled_staged_key(self.label.to_owned(), "flags".to_string());
        self.arguments.insert(flags_key,
                              format!("\'{}\'", util::split_flags_vec(flags.clone())));
        self
    }

    pub fn sort_keys(mut self, sort_keys: Vec<String>) -> StagedColumns {
        let value = util::split_values_vec(sort_keys);
        let sort_keys_key = match self.value {
            Window(_) => util::labeled_window_key(self.label.to_owned(), "sort_keys".to_string()),
            Value(_) => util::labeled_staged_key(self.label.to_owned(), "sort_keys".to_string()),
        };
        self.arguments.insert(sort_keys_key, value);
        self
    }

    pub fn construct(mut self) -> StagedColumns {
        let stage_key = util::labeled_staged_key(self.label.to_owned(), "stage".to_string());
        let column_type_key = util::labeled_staged_key(self.label.to_owned(), "type".to_string());
        let value_key = util::labeled_staged_key(self.label.to_owned(), "value".to_string());
        let value_str = match self.value.clone() {
            Window(w) => format!("{}", w.as_ref()),
            Value(v) => v,
        };
        self.arguments.insert(stage_key, self.stage.clone());
        self.arguments.insert(column_type_key, format!("{}", self.column_type));
        self.arguments.insert(value_key, value_str.clone());
        self
    }

    pub fn build(self) -> Query {
        let this = self.construct();
        let mut query: Query = vec![];
        for (key, value) in &this.arguments {
            query.push((key.to_owned(), value.to_owned()));
        }
        query
    }
}

impl Fragmentable for StagedColumns {
    // Before calling this, it must call construct().
    fn to_fragment(self) -> (Command, OrderedFragment, QueryFragment) {
        // Command::Extension is `Command` type requirement. It should be ignored.
        (Command::Extension("staged_columns".to_string()), vec![], self.arguments.to_owned())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::str::FromStr;
    use std::collections::HashMap;
    use types::data_type::DataType;
    use types::column_flag_type::ColumnFlagType;
    use super::WindowableColumn::Value;

    #[test]
    fn test_as_str() {
        assert_eq!(WindowFunction::RecordNumber.as_ref(), "record_number()");
        assert_eq!(WindowFunction::ExtWindowFunction("ext_func()".to_owned()).as_ref(),
                   "ext_func()");
    }

    #[test]
    fn test_from_str() {
        assert_eq!(WindowFunction::RecordNumber, FromStr::from_str("record_number").unwrap());
        assert_eq!(WindowFunction::RecordNumber, FromStr::from_str("RecordNumber").unwrap());
        assert_eq!(WindowFunction::ExtWindowFunction("ext_func()".to_owned()),
                   FromStr::from_str("ext_func()").unwrap());
        let x: Result<WindowFunction, _> = FromStr::from_str("");
        if let Err(WindowFunctionError::Empty) = x {
        } else {
            panic!("An empty window function is invalid!")
        }
    }

    #[test]
    fn test_fmt() {
        assert_eq!("record_number()".to_owned(),
                   format!("{}", WindowFunction::RecordNumber));
        assert_eq!("ext_func()".to_owned(),
                   format!("{}",
                           WindowFunction::ExtWindowFunction("ext_func()".to_owned())));
    }

    #[test]
    fn test_new() {
        let label = "label1".to_string();
        let stage = "staged".to_string();
        let value = "'_id'".to_string();
        let staged_columns = StagedColumns::new(label.clone(),
                                                stage.clone(),
                                                DataType::UInt32,
                                                Value(value.clone()))
            .flags(vec![(ColumnFlagType::Scalar)]);
        let mut arg: HashMap<String, String> = HashMap::new();
        arg.insert(format!("columns[{}].flags", label.clone()),
                   "\'COLUMN_SCALAR\'".to_string());
        let expected = StagedColumns {
            label: label.to_owned(),
            stage: stage.clone(),
            column_type: DataType::UInt32,
            value: Value(value.clone()),
            arguments: arg,
        };
        assert_eq!(expected, staged_columns);
    }

    #[test]
    fn test_construct() {
        let label = "label1".to_string();
        let stage = "staged".to_string();
        let value = "'_id'".to_string();
        let staged_columns = StagedColumns::new(label.clone(),
                                                stage.clone(),
                                                DataType::UInt32,
                                                Value(value.clone()))
            .flags(vec![(ColumnFlagType::Scalar)])
            .construct();
        let mut arg: HashMap<String, String> = HashMap::new();
        arg.insert(format!("columns[{}].flags", label.clone()),
                   "\'COLUMN_SCALAR\'".to_string());
        assert_eq!(staged_columns.arguments
                       .get(&format!("columns[{}].stage", label.to_owned()))
                       .unwrap(),
                   &"staged");
        assert_eq!(staged_columns.arguments
                       .get(&format!("columns[{}].type", label.to_owned()))
                       .unwrap(),
                   &"UInt32");
        assert_eq!(staged_columns.arguments
                       .get(&format!("columns[{}].flags", label.to_owned()))
                       .unwrap(),
                   &"\'COLUMN_SCALAR\'");
        assert_eq!(staged_columns.arguments
                       .get(&format!("columns[{}].value", label.to_owned()))
                       .unwrap(),
                   &"\'_id\'");
    }
}
