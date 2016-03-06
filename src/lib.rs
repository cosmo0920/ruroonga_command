extern crate url;

pub trait Queryable {
    fn to_query(self) -> String;
}

pub type LoadValues = String;
pub trait PostQueryable {
    fn to_post_query(self) -> (String, LoadValues);
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
pub mod column_create;
pub mod extension;
