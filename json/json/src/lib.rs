use serde_json::Value;

pub trait ConvertJsonToValue<TValue> {
    fn convert_json_to_value(&self, json: &Value) -> TValue;
}

pub struct JsonToF32Converter {}

impl JsonToF32Converter {
    pub fn new() -> Self {
        Self {}
    }
}

impl ConvertJsonToValue<f32> for JsonToF32Converter {
    fn convert_json_to_value(&self, json: &Value) -> f32 {
        json.as_f64().unwrap() as f32
    }
}

pub struct JsonToStringConverter {}

impl JsonToStringConverter {
    pub fn new() -> Self {
        Self {}
    }
}

impl ConvertJsonToValue<String> for JsonToStringConverter {
    fn convert_json_to_value(&self, json: &Value) -> String {
        json.as_str().unwrap().to_string()
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use serde_json::json;

    use crate::{ConvertJsonToValue, JsonToF32Converter, JsonToStringConverter};

    #[rstest]
    #[case(1.23)]
    #[case(45.67)]
    #[case(0.0)]
    fn when_a_json_to_f32_converter_converts_an_f32_json_value_to_f32_then_it_is_converted_to_f32(
        #[case] value: f32,
    ) {
        let f32_value = json!(value);

        let json_to_f32_converter = JsonToF32Converter::new();

        let result = json_to_f32_converter.convert_json_to_value(&f32_value);

        assert_eq!(value, result);
    }

    #[rstest]
    #[case(1, 1.0)]
    #[case(23, 23.0)]
    #[case(127, 127.0)]
    #[case(-127, -127.0)]
    #[case(0, 0.0)]
    fn when_a_json_to_f32_converter_converts_an_i8_json_value_to_f32_then_it_is_converted_to_f32(
        #[case] value: i8,
        #[case] expected_result: f32,
    ) {
        let f32_value = json!(value);

        let json_to_f32_converter = JsonToF32Converter::new();

        let result = json_to_f32_converter.convert_json_to_value(&f32_value);

        assert_eq!(expected_result, result);
    }

    #[rstest]
    #[case("val")]
    #[case(" ")]
    #[case("")]
    fn when_a_json_to_string_converter_converts_a_string_json_value_to_string_then_it_is_converted_to_string(
        #[case] value: String,
    ) {
        let string_value = json!(value);

        let json_to_string_converter = JsonToStringConverter::new();

        let result = json_to_string_converter.convert_json_to_value(&string_value);

        assert_eq!(value, result);
    }

    #[rstest]
    #[case("val")]
    #[case(" ")]
    #[case("")]
    fn when_a_json_to_string_converter_converts_a_str_json_value_to_string_then_it_is_converted_to_string(
        #[case] value: &str,
    ) {
        let string_value = json!(value);

        let json_to_string_converter = JsonToStringConverter::new();

        let result = json_to_string_converter.convert_json_to_value(&string_value);

        assert_eq!(value, result);
    }
}
