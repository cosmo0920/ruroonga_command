pub trait Commandable {
    fn to_command(self) -> String;
}

pub type DataValues = String;
pub trait PostCommandable {
    fn to_post_command(self) -> (String, DataValues);
}

macro_rules! commandable {
    ($($t:ty), +) => (
        $(impl Commandable for $t {
            fn to_command(self) -> String {
                let (command, query) = self.build();
                let mut command = CommandLine::new(command, query);
                command.encode()
            }
        })+
    )
}
