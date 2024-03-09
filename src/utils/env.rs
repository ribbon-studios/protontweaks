pub fn convert_bool(value: bool) -> String {
    convert_bool_as(value, "0", "1")
}

pub fn convert_bool_as(value: bool, false_value: &'static str, true_value: &'static str) -> String {
    if value {
        false_value.to_string()
    } else {
        true_value.to_string()
    }
}
