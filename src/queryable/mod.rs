pub trait Queryable {
    fn to_query(self) -> String;
}

pub type LoadValues = String;
pub trait PostQueryable {
    fn to_post_query(self) -> (String, LoadValues);
}

macro_rules! queryable {
    ($($t:ty), +) => (
        $(impl Queryable for $t {
            fn to_query(self) -> String {
                let (command, query) = self.build();
                let mut command = CommandQuery::new(command, query);
                command.encode()
            }
        })+
    )
}
