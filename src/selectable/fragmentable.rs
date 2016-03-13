use std::collections::HashMap;

pub type QueryFragment = HashMap<String, String>;
pub trait Fragmentable {
    fn to_fragment(self) -> QueryFragment;
}
