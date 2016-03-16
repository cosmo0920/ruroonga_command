use std::collections::HashMap;

pub trait Extendable {
    type Output;

    unsafe fn set_arguments(self, arguments: HashMap<String, String>) -> Self::Output;
}

macro_rules! extendable {
    ($($t:ty), +) => (
        $(impl Extendable for $t {
            type Output = $t;

            unsafe fn set_arguments(mut self, arguments: HashMap<String, String>) -> Self::Output {
                self.arguments = arguments;
                self
            }
        })+
    )
}
