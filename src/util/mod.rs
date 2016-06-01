use std::convert::AsRef;

#[inline]
pub fn split_values_vec(values: Vec<String>) -> String {
    let string = values.into_iter()
                       .collect::<Vec<String>>()
                       .join(",");
    string.clone()
}

#[inline]
pub fn split_flags_vec<T>(values: Vec<T>) -> String
    where T: AsRef<str>
{
    let string = values.into_iter()
                       .map(|v| v.as_ref().to_string())
                       .collect::<Vec<String>>()
                       .join("|");
    string.clone()
}

#[inline]
pub fn labeled_key(label: String, key: String) -> String {
    let string = format!("drilldowns[{}].{}", label.to_owned(), key.to_owned());
    string.clone()
}
