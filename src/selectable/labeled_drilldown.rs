use std::collections::HashMap;
use command::{Command, Query};
use util;
use selectable::fragmentable::Fragmentable;
use selectable::fragmentable::{OrderedFragment, QueryFragment};
use types::drilldown_calc_type::CalcType;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LabeledDrilldown {
    label: String,
    arguments: HashMap<String, String>,
}

impl Default for LabeledDrilldown {
    fn default() -> LabeledDrilldown {
        LabeledDrilldown {
            label: "".to_string(),
            arguments: HashMap::new(),
        }
    }
}

impl LabeledDrilldown {
    pub fn new(label: String) -> LabeledDrilldown {
        let default: LabeledDrilldown = Default::default();
        LabeledDrilldown {
            label: label,
            arguments: default.arguments,
        }
    }

    pub fn keys(mut self, targets: Vec<String>) -> LabeledDrilldown {
        let key = util::labeled_key(self.label.clone(), "keys".to_string());
        let string = format!("\'{}\'", util::split_values_vec(targets));
        self.arguments.insert(key, string.clone());
        self
    }

    pub fn sortby(mut self, targets: Vec<String>) -> LabeledDrilldown {
        let string = format!("\'{}\'", util::split_values_vec(targets));
        let key = util::labeled_key(self.label.clone(), "sortby".to_string());
        self.arguments.insert(key, string.clone());
        self
    }

    pub fn output_columns(mut self, columns: Vec<String>) -> LabeledDrilldown {
        let string = format!("\'{}\'", util::split_values_vec(columns));
        let key = util::labeled_key(self.label.clone(), "output_columns".to_string());

        self.arguments.insert(key, string.clone());
        self
    }

    pub fn offset(mut self, offset: i64) -> LabeledDrilldown {
        let offset = format!("{}", offset);
        let key = util::labeled_key(self.label.clone(), "offset".to_string());
        self.arguments.insert(key, offset.clone());
        self
    }

    pub fn limit(mut self, limit: i64) -> LabeledDrilldown {
        let limit = format!("{}", limit);
        let key = util::labeled_key(self.label.clone(), "limit".to_string());
        self.arguments.insert(key, limit.clone());
        self
    }

    pub fn calc_types(mut self, calc_types: CalcType) -> LabeledDrilldown {
        let key = util::labeled_key(self.label.clone(), "calc_types".to_string());
        let types = format!("{}", calc_types);
        self.arguments.insert(key, types);
        self
    }

    pub fn calc_target(mut self, target: String) -> LabeledDrilldown {
        let key = util::labeled_key(self.label.clone(), "calc_target".to_string());
        self.arguments.insert(key, target.clone());
        self
    }

    // TODO: where is the right place for pseudo-table?
    pub fn table(mut self, table: String) -> LabeledDrilldown {
        let key = util::labeled_key(self.label.clone(), "table".to_string());
        self.arguments.insert(key, table.clone());
        self
    }

    pub fn build(self) -> Query {
        let mut query: Query = vec![];
        for (key, value) in self.arguments.iter() {
            query.push((key.to_owned(), value.to_owned()));
        }
        query
    }
}

impl Fragmentable for LabeledDrilldown {
    fn to_fragment(self) -> (Command, OrderedFragment, QueryFragment) {
        // Command::Extension is `Command` type requirement. It should be ignored.
        (Command::Extension("labeled_drilldown".to_string()), vec![], self.arguments.clone())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::collections::HashMap;
    use command::Query;
    use types::drilldown_calc_type::CalcType;

    #[test]
    fn test_new() {
        let label = "label1".to_string();
        let vanilla_drilldown = LabeledDrilldown::new(label.clone());
        let expected = LabeledDrilldown {
            label: label.clone(),
            arguments: HashMap::new(),
        };
        assert_eq!(expected, vanilla_drilldown);
    }

    #[test]
    fn test_keys() {
        let label = "label1".to_string();
        let drilldown = LabeledDrilldown::new(label.clone()).keys(vec![("tag".to_string())]);
        let mut arg: HashMap<String, String> = HashMap::new();
        arg.insert(format!("drilldown[{}].keys", label.clone()),
                   "\'tag\'".to_string());
        let expected = LabeledDrilldown {
            label: label.clone(),
            arguments: arg,
        };
        assert_eq!(expected, drilldown);
    }

    #[test]
    fn test_sortby() {
        let label = "label1".to_string();
        let drilldown = LabeledDrilldown::new(label.clone()).sortby(vec![("tag".to_string())]);
        let mut arg: HashMap<String, String> = HashMap::new();
        arg.insert(format!("drilldown[{}].sortby", label),
                   "\'tag\'".to_string());
        let expected = LabeledDrilldown {
            label: label.clone(),
            arguments: arg,
        };
        assert_eq!(expected, drilldown);
    }

    #[test]
    fn test_output_columns() {
        let label = "label1".to_string();
        let drilldown = LabeledDrilldown::new(label.clone())
                            .output_columns(vec![("tag".to_string()), ("category".to_string())]);
        let mut arg: HashMap<String, String> = HashMap::new();
        arg.insert(format!("drilldown[{}].output_columns", label.clone()),
                   "\'tag,category\'".to_string());
        let expected = LabeledDrilldown {
            label: label.clone(),
            arguments: arg,
        };
        assert_eq!(expected, drilldown);
    }

    #[test]
    fn test_offset() {
        let label = "label1".to_string();
        let drilldown = LabeledDrilldown::new(label.clone()).offset(10);
        let mut arg: HashMap<String, String> = HashMap::new();
        arg.insert(format!("drilldown[{}].offset", label.clone()),
                   "10".to_string());
        let expected = LabeledDrilldown {
            label: label.clone(),
            arguments: arg,
        };
        assert_eq!(expected, drilldown);
    }

    #[test]
    fn test_limit() {
        let label = "label1".to_string();
        let drilldown = LabeledDrilldown::new(label.clone()).limit(30);
        let mut arg: HashMap<String, String> = HashMap::new();
        arg.insert(format!("drilldown[{}].limit", label.clone()),
                   "30".to_string());
        let expected = LabeledDrilldown {
            label: label.clone(),
            arguments: arg,
        };
        assert_eq!(expected, drilldown);
    }

    #[test]
    fn test_calc_target() {
        let label = "label1".to_string();
        let drilldown = LabeledDrilldown::new(label.clone()).calc_target("target".to_string());
        let mut arg: HashMap<String, String> = HashMap::new();
        arg.insert(format!("drilldown[{}].calc_target", label.clone()),
                   "target".to_string());
        let expected = LabeledDrilldown {
            label: label.clone(),
            arguments: arg,
        };
        assert_eq!(expected, drilldown);
    }

    #[test]
    fn test_calc_types() {
        let label = "label1".to_string();
        let drilldown = LabeledDrilldown::new(label.clone()).calc_types(CalcType::None);
        let mut arg: HashMap<String, String> = HashMap::new();
        arg.insert(format!("drilldown[{}].calc_types", label.clone()),
                   "NONE".to_string());
        let expected = LabeledDrilldown {
            label: label.clone(),
            arguments: arg,
        };
        assert_eq!(expected, drilldown);
    }

    #[test]
    fn test_build() {
        let label = "label1".to_string();
        let drilldown = LabeledDrilldown::new(label.clone()).limit(30).build();
        let expected: Query = vec![(format!("drilldown[{}].limit", label.clone()),
                                    "30".to_string())];
        assert_eq!(expected, drilldown);
    }
}
