use std::collections::HashMap;
use command::Query;
use command::Command;

pub type OrderedFragment = Query;
pub type QueryFragment = HashMap<String, String>;
pub trait Fragmentable {
    fn to_fragment(self) -> (Command, OrderedFragment, QueryFragment);
}
