#[inline]
pub fn split_values_vec(values: Vec<String>) -> String {
    let string = values.into_iter().collect::<Vec<String>>()
        .join(",").to_owned();
    string.clone()
}
