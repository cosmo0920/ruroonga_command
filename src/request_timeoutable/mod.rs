pub trait RequestTimeoutable {
    type Output;

    /// Specify `timeout` value. Unit: second
    ///
    /// Panics if specified negative float value.
    fn append_request_timeout(self, timeout: f64) -> Self::Output;
}

macro_rules! request_timeoutable {
    ($($t:ty), +) => (
        $(impl RequestTimeoutable for $t {
            type Output = $t;

            fn append_request_timeout(mut self, timeout: f64) -> Self::Output {
                if !timeout.is_sign_positive() {
                    panic!("`timeout` should be positive 64bit float number.")
                }
                let string = format!("{}", timeout);
                self.arguments.insert("request_timeout".to_string(), string.clone());
                self
            }
        })+
    )
}
