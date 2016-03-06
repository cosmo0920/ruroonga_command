extern crate url;

pub trait Queryable {
    fn to_query(self) -> String;
}

mod util;
pub mod types;
pub mod command;
pub mod command_query;
pub mod dump;
pub mod select;
pub mod load;
pub mod status;
pub mod delete;
pub mod table_create;
