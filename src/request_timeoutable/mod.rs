pub trait RequestTimeoutable {
    type Output;

    fn append_request_timeout(self, timeout: u64) -> Self::Output;
}

macro_rules! request_timeoutable {
    ($($t:ty), +) => (
        $(impl RequestTimeoutable for $t {
            type Output = $t;

            fn append_request_timeout(mut self, timeout: u64) -> Self::Output {
                let string = format!("{}", timeout);
                self.arguments.insert("request_timeout".to_string(), string.clone());
                self
            }
        })+
    )
}
