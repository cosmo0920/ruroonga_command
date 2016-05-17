use std::collections::HashMap;
use command::{Command, Query};
use util;
use selectable::fragmentable::Fragmentable;
use selectable::fragmentable::{OrderedFragment, QueryFragment};
use selectable::pseudo_table::PseudoTable;
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
        let key = util::labeled_key(self.label.to_owned(), "keys".to_string());
        let string = format!("\'{}\'", util::split_values_vec(targets));
        self.arguments.insert(key, string.to_owned());
        self
    }

    pub fn sortby(mut self, targets: Vec<String>) -> LabeledDrilldown {
        let string = format!("\'{}\'", util::split_values_vec(targets));
        let key = util::labeled_key(self.label.to_owned(), "sortby".to_string());
        self.arguments.insert(key, string.to_owned());
        self
    }

    pub fn output_columns(mut self, columns: Vec<String>) -> LabeledDrilldown {
        let string = format!("\'{}\'", util::split_values_vec(columns));
        let key = util::labeled_key(self.label.to_owned(), "output_columns".to_string());

        self.arguments.insert(key, string.to_owned());
        self
    }

    pub fn offset(mut self, offset: i64) -> LabeledDrilldown {
        let offset = format!("{}", offset);
        let key = util::labeled_key(self.label.to_owned(), "offset".to_string());
        self.arguments.insert(key, offset.to_owned());
        self
    }

    pub fn limit(mut self, limit: i64) -> LabeledDrilldown {
        let limit = format!("{}", limit);
        let key = util::labeled_key(self.label.to_owned(), "limit".to_string());
        self.arguments.insert(key, limit.to_owned());
        self
    }

    pub fn calc_types(mut self, calc_types: CalcType) -> LabeledDrilldown {
        let key = util::labeled_key(self.label.to_owned(), "calc_types".to_string());
        let types = format!("{}", calc_types);
        self.arguments.insert(key, types);
        self
    }

    pub fn calc_target(mut self, target: String) -> LabeledDrilldown {
        let key = util::labeled_key(self.label.to_owned(), "calc_target".to_string());
        self.arguments.insert(key, target.to_owned());
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
        (Command::Extension("labeled_drilldown".to_string()), vec![], self.arguments.to_owned())
    }
}

impl PseudoTable for LabeledDrilldown {
    type Output = LabeledDrilldown;

    fn table(mut self, table: String) -> LabeledDrilldown {
        let key = util::labeled_key(self.label.to_owned(), "table".to_string());

        self.arguments.insert(key, table.to_owned());
        self
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
        let vanilla_drilldown = LabeledDrilldown::new(label.to_owned());
        let expected = LabeledDrilldown {
            label: label.to_owned(),
            arguments: HashMap::new(),
        };
        assert_eq!(expected, vanilla_drilldown);
    }

    #[test]
    fn test_keys() {
        let label = "label1".to_string();
        let drilldown = LabeledDrilldown::new(label.to_owned()).keys(vec![("tag".to_string())]);
        let mut arg: HashMap<String, String> = HashMap::new();
        arg.insert(format!("drilldown[{}].keys", label.to_owned()),
                   "\'tag\'".to_string());
        let expected = LabeledDrilldown {
            label: label.to_owned(),
            arguments: arg,
        };
        assert_eq!(expected, drilldown);
    }

    #[test]
    fn test_sortby() {
        let label = "label1".to_string();
        let drilldown = LabeledDrilldown::new(label.to_owned()).sortby(vec![("tag".to_string())]);
        let mut arg: HashMap<String, String> = HashMap::new();
        arg.insert(format!("drilldown[{}].sortby", label),
                   "\'tag\'".to_string());
        let expected = LabeledDrilldown {
            label: label.to_owned(),
            arguments: arg,
        };
        assert_eq!(expected, drilldown);
    }

    #[test]
    fn test_output_columns() {
        let label = "label1".to_string();
        let drilldown = LabeledDrilldown::new(label.to_owned())
                            .output_columns(vec![("tag".to_string()), ("category".to_string())]);
        let mut arg: HashMap<String, String> = HashMap::new();
        arg.insert(format!("drilldown[{}].output_columns", label.to_owned()),
                   "\'tag,category\'".to_string());
        let expected = LabeledDrilldown {
            label: label.to_owned(),
            arguments: arg,
        };
        assert_eq!(expected, drilldown);
    }

    #[test]
    fn test_offset() {
        let label = "label1".to_string();
        let drilldown = LabeledDrilldown::new(label.to_owned()).offset(10);
        let mut arg: HashMap<String, String> = HashMap::new();
        arg.insert(format!("drilldown[{}].offset", label.to_owned()),
                   "10".to_string());
        let expected = LabeledDrilldown {
            label: label.to_owned(),
            arguments: arg,
        };
        assert_eq!(expected, drilldown);
    }

    #[test]
    fn test_limit() {
        let label = "label1".to_string();
        let drilldown = LabeledDrilldown::new(label.to_owned()).limit(30);
        let mut arg: HashMap<String, String> = HashMap::new();
        arg.insert(format!("drilldown[{}].limit", label.to_owned()),
                   "30".to_string());
        let expected = LabeledDrilldown {
            label: label.to_owned(),
            arguments: arg,
        };
        assert_eq!(expected, drilldown);
    }

    #[test]
    fn test_calc_target() {
        let label = "label1".to_string();
        let drilldown = LabeledDrilldown::new(label.to_owned()).calc_target("target".to_string());
        let mut arg: HashMap<String, String> = HashMap::new();
        arg.insert(format!("drilldown[{}].calc_target", label.to_owned()),
                   "target".to_string());
        let expected = LabeledDrilldown {
            label: label.to_owned(),
            arguments: arg,
        };
        assert_eq!(expected, drilldown);
    }

    #[test]
    fn test_calc_types() {
        let label = "label1".to_string();
        let drilldown = LabeledDrilldown::new(label.to_owned()).calc_types(CalcType::None);
        let mut arg: HashMap<String, String> = HashMap::new();
        arg.insert(format!("drilldown[{}].calc_types", label.to_owned()),
                   "NONE".to_string());
        let expected = LabeledDrilldown {
            label: label.to_owned(),
            arguments: arg,
        };
        assert_eq!(expected, drilldown);
    }

    #[test]
    fn test_build() {
        let label = "label1".to_string();
        let drilldown = LabeledDrilldown::new(label.to_owned()).limit(30).build();
        let expected: Query = vec![(format!("drilldown[{}].limit", label.to_owned()),
                                    "30".to_string())];
        assert_eq!(expected, drilldown);
    }
}
