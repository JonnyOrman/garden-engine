use serde_json::Value;

pub trait ConvertJsonToValue<TValue> {
    fn convert_json_to_value(&self, json: &Value) -> TValue;
}
