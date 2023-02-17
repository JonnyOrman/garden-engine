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

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use serde_json::json;

    use crate::{ConvertJsonToValue, JsonToF32Converter};

    #[rstest]
    #[case(1.23)]
    #[case(45.67)]
    #[case(0.0)]
    fn when_a_json_to_f32_converter_converts_an_f32_json_value_to_f32_then_it_is_converted_to_f32(
        #[case] val: f32,
    ) {
        let f32_value = json!(val);

        let json_to_f32_converter = JsonToF32Converter::new();

        let result = json_to_f32_converter.convert_json_to_value(&f32_value);

        assert_eq!(val, result);
    }

    #[rstest]
    #[case(1, 1.0)]
    #[case(23, 23.0)]
    #[case(127, 127.0)]
    #[case(-127, -127.0)]
    #[case(0, 0.0)]
    fn when_a_json_to_f32_converter_converts_an_i8_json_value_to_f32_then_it_is_converted_to_f32(
        #[case] val: i8,
        #[case] expected_result: f32,
    ) {
        let f32_value = json!(val);

        let json_to_f32_converter = JsonToF32Converter::new();

        let result = json_to_f32_converter.convert_json_to_value(&f32_value);

        assert_eq!(expected_result, result);
    }
}
