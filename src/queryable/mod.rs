pub trait Queryable {
    fn to_query(self) -> String;
}

pub type LoadValues = String;
pub trait PostQueryable {
    fn to_post_query(self) -> (String, LoadValues);
}
