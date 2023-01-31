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
