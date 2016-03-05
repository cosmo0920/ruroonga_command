use types::table_flag_type::TableFlagType;

#[inline]
pub fn split_values_vec(values: Vec<String>) -> String {
    let string = values.into_iter().collect::<Vec<String>>()
        .join(",").to_owned();
    string.clone()
}

#[inline]
pub fn split_flags_vec(values: Vec<TableFlagType>) -> String {
    let string = values.into_iter().map(|v| v.as_ref().to_string())
        .collect::<Vec<String>>().join("|").to_owned();
    string.clone()
}
