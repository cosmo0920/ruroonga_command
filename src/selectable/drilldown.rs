use std::collections::HashMap;
use command::Query;
use util;
use selectable::fragmentable::Fragmentable;
use selectable::fragmentable::QueryFragment;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Drilldown {
    arguments: HashMap<String, String>,
}

impl Default for Drilldown {
    fn default() -> Drilldown {
        Drilldown {
            arguments: HashMap::new(),
        }
    }
}

impl Drilldown {
    pub fn new() -> Drilldown {
        let default: Drilldown = Default::default();
        default
    }

    pub fn drilldown(mut self, target: String) -> Drilldown {
        self.arguments.insert("drilldown".to_string(), target.clone());
        self
    }

    pub fn sortby(mut self, target: String) -> Drilldown {
        self.arguments.insert("drilldown_sortby".to_string(), target.clone());
        self
    }

    pub fn output_columns(mut self, columns: Vec<String>) -> Drilldown {
        let string = util::split_values_vec(columns);
        self.arguments.insert("drilldown_output_columns".to_string(),
                              string.clone());
        self
    }

    pub fn offset(mut self, offset: u64) -> Drilldown {
        let offset = format!("{}", offset);
        self.arguments.insert("drilldown_offset".to_string(), offset.clone());
        self
    }

    pub fn limit(mut self, limit: u64) -> Drilldown {
        let limit = format!("{}", limit);
        self.arguments.insert("drilldown_limit".to_string(), limit.clone());
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
    fn to_fragment(self) -> QueryFragment {
        self.arguments.clone()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::collections::HashMap;
    use command::Query;

    #[test]
    fn test_new() {
        let vanilla_drilldown = Drilldown::new();
        let expected = Drilldown {
            arguments: HashMap::new(),
        };
        assert_eq!(expected, vanilla_drilldown);
    }

    #[test]
    fn test_drilldown() {
        let drilldown = Drilldown::new().drilldown("tag".to_string());
        let mut arg: HashMap<String, String> = HashMap::new();
        arg.insert("drilldown".to_string(), "tag".to_string());
        let expected = Drilldown {
            arguments: arg,
        };
        assert_eq!(expected, drilldown);
    }

    #[test]
    fn test_sortby() {
        let drilldown = Drilldown::new().sortby("tag".to_string());
        let mut arg: HashMap<String, String> = HashMap::new();
        arg.insert("drilldown_sortby".to_string(), "tag".to_string());
        let expected = Drilldown {
            arguments: arg,
        };
        assert_eq!(expected, drilldown);
    }

    #[test]
    fn test_output_columns() {
        let drilldown = Drilldown::new()
            .output_columns(vec![("tag".to_string()),
                                 ("category".to_string())]);
        let mut arg: HashMap<String, String> = HashMap::new();
        arg.insert("drilldown_output_columns".to_string(),
                   "tag,category".to_string());
        let expected = Drilldown {
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
            arguments: arg,
        };
        assert_eq!(expected, drilldown);
    }

    #[test]
    fn test_build() {
        let drilldown = Drilldown::new().limit(30).build();
        let expected: Query = vec![("drilldown_limit".to_string(),
                                    "30".to_string())];
        assert_eq!(expected, drilldown);
    }
}
