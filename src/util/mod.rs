use std::convert::AsRef;

#[inline]
pub fn split_values_vec(values: Vec<String>) -> String {
    let string = values.into_iter()
                       .collect::<Vec<String>>()
                       .join(",")
                       .to_owned();
    string.clone()
}

#[inline]
pub fn split_flags_vec<T>(values: Vec<T>) -> String
    where T: AsRef<str>
{
    let string = values.into_iter()
                       .map(|v| v.as_ref().to_string())
                       .collect::<Vec<String>>()
                       .join("|")
                       .to_owned();
    string.clone()
}

#[inline]
pub fn labeled_key(label: Option<String>, key: String) -> String {
    match label.clone() {
        Some(v) => return format!("drilldown[{}].{}", key.clone(), v.clone()),
        None => return format!("drilldown_{}", key.clone()),
    };
}
