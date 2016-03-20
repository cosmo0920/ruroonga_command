use std::collections::HashMap;
use command::Query;
use util;
use selectable::fragmentable::Fragmentable;
use selectable::fragmentable::{OrderedFragment, QueryFragment};
use types::drilldown_calc_type::CalcType;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Drilldown {
    label: Option<String>,
    arguments: HashMap<String, String>,
}

impl Default for Drilldown {
    fn default() -> Drilldown {
        Drilldown {
            label: None,
            arguments: HashMap::new(),
        }
    }
}
impl Drilldown {
    pub fn new() -> Drilldown {
        let default: Drilldown = Default::default();
        default
    }

    pub fn new_with_label(label: String) -> Drilldown {
        let default: Drilldown = Default::default();
        Drilldown {
            label: Some(label),
            arguments: default.arguments,
        }
    }

    pub fn drilldown(mut self, targets: Vec<String>) -> Drilldown {
        let string = format!("\'{}\'", util::split_values_vec(targets));
        let key = match self.label.clone() {
            Some(v) => format!("drilldown[{}].keys", v.clone()),
            None => "drilldown".to_string(),
        };
        self.arguments.insert(key, string.clone());
        self
    }

    pub fn sortby(mut self, targets: Vec<String>) -> Drilldown {
        let string = format!("\'{}\'", util::split_values_vec(targets));
        let key = util::labeled_key(self.label.clone(), "sortby".to_string());
        self.arguments.insert(key, string.clone());
        self
    }

    pub fn output_columns(mut self, columns: Vec<String>) -> Drilldown {
        let string = format!("\'{}\'", util::split_values_vec(columns));
        let key = util::labeled_key(self.label.clone(), "output_columns".to_string());

        self.arguments.insert(key, string.clone());
        self
    }

    pub fn offset(mut self, offset: i64) -> Drilldown {
        let offset = format!("{}", offset);
        let key = util::labeled_key(self.label.clone(), "offset".to_string());
        self.arguments.insert(key, offset.clone());
        self
    }

    pub fn limit(mut self, limit: i64) -> Drilldown {
        let limit = format!("{}", limit);
        let key = util::labeled_key(self.label.clone(), "limit".to_string());
        self.arguments.insert(key, limit.clone());
        self
    }

    pub fn calc_types(mut self, calc_types: CalcType) -> Drilldown {
        let key = util::labeled_key(self.label.clone(), "calc_types".to_string());
        let types = format!("{}", calc_types);
        self.arguments.insert(key, types);
        self
    }

    pub fn calc_target(mut self, target: String) -> Drilldown {
        let key = util::labeled_key(self.label.clone(), "calc_target".to_string());
        self.arguments.insert(key, target.clone());
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

impl Fragmentable for Drilldown {
    fn to_fragment(self) -> (OrderedFragment, QueryFragment) {
        (vec![], self.arguments.clone())
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
        let vanilla_drilldown = Drilldown::new();
        let expected = Drilldown {
            label: None,
            arguments: HashMap::new(),
        };
        assert_eq!(expected, vanilla_drilldown);
    }

    #[test]
    fn test_drilldown() {
        let drilldown = Drilldown::new().drilldown(vec![("tag".to_string())]);
        let mut arg: HashMap<String, String> = HashMap::new();
        arg.insert("drilldown".to_string(), "\'tag\'".to_string());
        let expected = Drilldown {
            label: None,
            arguments: arg,
        };
        assert_eq!(expected, drilldown);
    }

    #[test]
    fn test_drilldown_with_label() {
        let label = "label1".to_string();
        let drilldown = Drilldown::new_with_label(label.clone())
                            .drilldown(vec![("tag".to_string())]);
        let mut arg: HashMap<String, String> = HashMap::new();
        arg.insert(format!("drilldown[{}].keys", label.clone()),
                   "\'tag\'".to_string());
        let expected = Drilldown {
            label: Some(label.clone()),
            arguments: arg,
        };
        assert_eq!(expected, drilldown);
    }

    #[test]
    fn test_sortby() {
        let drilldown = Drilldown::new().sortby(vec![("tag".to_string())]);
        let mut arg: HashMap<String, String> = HashMap::new();
        arg.insert("drilldown_sortby".to_string(), "\'tag\'".to_string());
        let expected = Drilldown {
            label: None,
            arguments: arg,
        };
        assert_eq!(expected, drilldown);
    }

    #[test]
    fn test_output_columns() {
        let drilldown = Drilldown::new()
                            .output_columns(vec![("tag".to_string()), ("category".to_string())]);
        let mut arg: HashMap<String, String> = HashMap::new();
        arg.insert("drilldown_output_columns".to_string(),
                   "\'tag,category\'".to_string());
        let expected = Drilldown {
            label: None,
            arguments: arg,
        };
        assert_eq!(expected, drilldown);
    }

    #[test]
    fn test_offset() {
        let drilldown = Drilldown::new().offset(10);
        let mut arg: HashMap<String, String> = HashMap::new();
        arg.insert("drilldown_offset".to_string(), "10".to_string());
        let expected = Drilldown {
            label: None,
            arguments: arg,
        };
        assert_eq!(expected, drilldown);
    }

    #[test]
    fn test_limit() {
        let drilldown = Drilldown::new().limit(30);
        let mut arg: HashMap<String, String> = HashMap::new();
        arg.insert("drilldown_limit".to_string(), "30".to_string());
        let expected = Drilldown {
            label: None,
            arguments: arg,
        };
        assert_eq!(expected, drilldown);
    }

    #[test]
    fn test_calc_target() {
        let drilldown = Drilldown::new().calc_target("target".to_string());
        let mut arg: HashMap<String, String> = HashMap::new();
        arg.insert("drilldown_calc_target".to_string(), "target".to_string());
        let expected = Drilldown {
            label: None,
            arguments: arg,
        };
        assert_eq!(expected, drilldown);
    }

    #[test]
    fn test_calc_types() {
        let drilldown = Drilldown::new().calc_types(CalcType::None);
        let mut arg: HashMap<String, String> = HashMap::new();
        arg.insert("drilldown_calc_types".to_string(), "NONE".to_string());
        let expected = Drilldown {
            label: None,
            arguments: arg,
        };
        assert_eq!(expected, drilldown);
    }

    #[test]
    fn test_build() {
        let drilldown = Drilldown::new().limit(30).build();
        let expected: Query = vec![("drilldown_limit".to_string(), "30".to_string())];
        assert_eq!(expected, drilldown);
    }
}
