pub fn convert_bool(value: bool) -> String {
    convert_bool_as(value, "0", "1")
}

pub fn convert_bool_as(value: bool, false_value: &'static str, true_value: &'static str) -> String {
    match value {
        true => true_value.to_string(),
        false => false_value.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::env::{convert_bool, convert_bool_as};

    #[test]
    fn convert_bool_should_map_to_0_or_1() {
        assert_eq!(convert_bool(false), "0");
        assert_eq!(convert_bool(true), "1");
    }

    #[test]
    fn convert_bool_as_should_support_custom_values() {
        assert_eq!(convert_bool_as(false, "false", "true"), "false");
        assert_eq!(convert_bool_as(true, "false", "true"), "true");
    }
}
