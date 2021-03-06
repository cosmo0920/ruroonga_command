use std::collections::HashMap;
use command::Command;
use command::Query;
use util;
use selectable::fragmentable::Fragmentable;
use selectable::fragmentable::{OrderedFragment, QueryFragment};
use types::drilldown_calc_type::CalcType;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Drilldown {
    arguments: HashMap<String, String>,
}

impl Default for Drilldown {
    fn default() -> Drilldown {
        Drilldown { arguments: HashMap::new() }
    }
}
impl Drilldown {
    pub fn new() -> Drilldown {
        Default::default()
    }

    pub fn drilldown(mut self, targets: Vec<String>) -> Drilldown {
        let string = format!("\'{}\'", util::split_values_vec(targets));
        self.arguments.insert("drilldown".to_string(), string.to_owned());
        self
    }

    #[deprecated]
    #[allow(deprecated)]
    pub fn sortby(mut self, targets: Vec<String>) -> Drilldown {
        let string = format!("\'{}\'", util::split_values_vec(targets));
        self.arguments.insert("drilldown_sortby".to_string(), string.to_owned());
        self
    }

    pub fn sort_keys(mut self, targets: Vec<String>) -> Drilldown {
        let string = format!("\'{}\'", util::split_values_vec(targets));
        self.arguments.insert("drilldown_sort_keys".to_string(), string.to_owned());
        self
    }

    pub fn output_columns(mut self, columns: Vec<String>) -> Drilldown {
        let string = format!("\'{}\'", util::split_values_vec(columns));
        self.arguments.insert("drilldown_output_columns".to_string(), string.to_owned());
        self
    }

    pub fn offset(mut self, offset: i64) -> Drilldown {
        let offset = format!("{}", offset);
        self.arguments.insert("drilldown_offset".to_string(), offset.to_owned());
        self
    }

    pub fn limit(mut self, limit: i64) -> Drilldown {
        let limit = format!("{}", limit);
        self.arguments.insert("drilldown_limit".to_string(), limit.to_owned());
        self
    }

    pub fn calc_types(mut self, calc_types: Vec<CalcType>) -> Drilldown {
        let string = format!("\'{}\'", util::split_flags_vec(calc_types));
        self.arguments.insert("drilldown_calc_types".to_string(), string.to_owned());
        self
    }

    pub fn calc_target(mut self, target: String) -> Drilldown {
        self.arguments.insert("drilldown_calc_target".to_string(), target.to_owned());
        self
    }

    pub fn build(self) -> Query {
        let mut query: Query = vec![];
        for (key, value) in &self.arguments {
            query.push((key.to_owned(), value.to_owned()));
        }
        query
    }
}

impl Fragmentable for Drilldown {
    fn to_fragment(self) -> (Command, OrderedFragment, QueryFragment) {
        // Command::Extension is `Command` type requirement. It should be ignored.
        (Command::Extension("drilldown".to_string()), vec![], self.arguments.to_owned())
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
        let expected = Drilldown { arguments: HashMap::new() };
        assert_eq!(expected, vanilla_drilldown);
    }

    #[test]
    fn test_drilldown() {
        let drilldown = Drilldown::new().drilldown(vec![("tag".to_string())]);
        let mut arg: HashMap<String, String> = HashMap::new();
        arg.insert("drilldown".to_string(), "\'tag\'".to_string());
        let expected = Drilldown { arguments: arg };
        assert_eq!(expected, drilldown);
    }

    #[test]
    #[allow(deprecated)]
    fn test_sortby() {
        let drilldown = Drilldown::new().sortby(vec![("tag".to_string())]);
        let mut arg: HashMap<String, String> = HashMap::new();
        arg.insert("drilldown_sortby".to_string(), "\'tag\'".to_string());
        let expected = Drilldown { arguments: arg };
        assert_eq!(expected, drilldown);
    }

    #[test]
    fn test_sort_keys() {
        let drilldown = Drilldown::new().sort_keys(vec![("tag".to_string())]);
        let mut arg: HashMap<String, String> = HashMap::new();
        arg.insert("drilldown_sort_keys".to_string(), "\'tag\'".to_string());
        let expected = Drilldown { arguments: arg };
        assert_eq!(expected, drilldown);
    }

    #[test]
    fn test_output_columns() {
        let drilldown = Drilldown::new()
            .output_columns(vec![("tag".to_string()), ("category".to_string())]);
        let mut arg: HashMap<String, String> = HashMap::new();
        arg.insert("drilldown_output_columns".to_string(),
                   "\'tag,category\'".to_string());
        let expected = Drilldown { arguments: arg };
        assert_eq!(expected, drilldown);
    }

    #[test]
    fn test_offset() {
        let drilldown = Drilldown::new().offset(10);
        let mut arg: HashMap<String, String> = HashMap::new();
        arg.insert("drilldown_offset".to_string(), "10".to_string());
        let expected = Drilldown { arguments: arg };
        assert_eq!(expected, drilldown);
    }

    #[test]
    fn test_limit() {
        let drilldown = Drilldown::new().limit(30);
        let mut arg: HashMap<String, String> = HashMap::new();
        arg.insert("drilldown_limit".to_string(), "30".to_string());
        let expected = Drilldown { arguments: arg };
        assert_eq!(expected, drilldown);
    }

    #[test]
    fn test_calc_target() {
        let drilldown = Drilldown::new().calc_target("target".to_string());
        let mut arg: HashMap<String, String> = HashMap::new();
        arg.insert("drilldown_calc_target".to_string(), "target".to_string());
        let expected = Drilldown { arguments: arg };
        assert_eq!(expected, drilldown);
    }

    #[test]
    fn test_calc_types() {
        let drilldown = Drilldown::new().calc_types(vec![(CalcType::None)]);
        let mut arg: HashMap<String, String> = HashMap::new();
        arg.insert("drilldown_calc_types".to_string(), "\'NONE\'".to_string());
        let expected = Drilldown { arguments: arg };
        assert_eq!(expected, drilldown);
    }

    #[test]
    fn test_build() {
        let drilldown = Drilldown::new().limit(30).build();
        let expected: Query = vec![("drilldown_limit".to_string(), "30".to_string())];
        assert_eq!(expected, drilldown);
    }
}
