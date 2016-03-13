pub trait Commandable {
    fn to_command(self) -> String;
}

pub type DataValues = String;
pub trait PostCommandable {
    fn to_post_command(self) -> (String, DataValues);
}
