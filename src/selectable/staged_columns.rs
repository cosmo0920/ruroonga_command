use std::collections::HashMap;
use command::{Command, Query};
use util;
use selectable::fragmentable::Fragmentable;
use selectable::fragmentable::{OrderedFragment, QueryFragment};
use types::data_type::DataType;
use types::column_flag_type::ColumnFlagType;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct StagedColumns {
    label: String,
    stage: String,
    column_type: DataType,
    flags: Vec<ColumnFlagType>,
    value: String,
    arguments: HashMap<String, String>,
}

impl StagedColumns {
    pub fn new(label: String,
               stage: String,
               column_type: DataType,
               flags: Vec<ColumnFlagType>,
               value: String)
               -> StagedColumns {
        StagedColumns {
            label: label,
            stage: stage,
            column_type: column_type,
            flags: flags,
            value: value,
            arguments: HashMap::new(),
        }
    }

    pub fn construct(mut self) -> StagedColumns {
        let stage_key = util::labeled_staged_key(self.label.to_owned(), "stage".to_string());
        let column_type_key = util::labeled_staged_key(self.label.to_owned(), "type".to_string());
        let flags_key = util::labeled_staged_key(self.label.to_owned(), "flags".to_string());
        let value_key = util::labeled_staged_key(self.label.to_owned(), "value".to_string());
        self.arguments.insert(stage_key, self.stage.clone());
        self.arguments.insert(column_type_key, format!("{}", self.column_type));
        self.arguments.insert(flags_key,
                              format!("\'{}\'", util::split_flags_vec(self.flags.clone())));
        self.arguments.insert(value_key, self.value.clone());
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
    use std::collections::HashMap;
    use types::data_type::DataType;
    use types::column_flag_type::ColumnFlagType;

    #[test]
    fn test_new() {
        let label = "label1".to_string();
        let stage = "staged".to_string();
        let value = "'_id'".to_string();
        let staged_columns = StagedColumns::new(label.clone(),
                                                stage.clone(),
                                                DataType::UInt32,
                                                vec![(ColumnFlagType::Scalar)],
                                                value.clone());
        let expected = StagedColumns {
            label: label.to_owned(),
            stage: stage.clone(),
            column_type: DataType::UInt32,
            flags: vec![(ColumnFlagType::Scalar)],
            value: value.clone(),
            arguments: HashMap::new(),
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
                                                vec![(ColumnFlagType::Scalar)],
                                                value.clone())
                                 .construct();
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
