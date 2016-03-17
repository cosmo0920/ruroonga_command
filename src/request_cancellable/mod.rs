pub trait RequestCancellable {
    type Output;

    fn append_request_id(self, request_id: String) -> Self::Output;
}

macro_rules! request_cancellable {
    ($($t:ty), +) => (
        $(impl RequestCancellable for $t {
            type Output = $t;

            fn append_request_id(mut self, request_id: String) -> Self::Output {
                self.arguments.insert("request_id".to_string(), request_id);
                self
            }
        })+
    )
}
