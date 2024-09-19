use std::rc::Rc;

use garden_json::{JsonToF32Converter, JsonToStringConverter};

pub struct JsonComponent {
    json_to_f32_converter: Rc<JsonToF32Converter>,
    json_to_string_converter: Rc<JsonToStringConverter>,
}

impl JsonComponent {
    fn new(
        json_to_f32_converter: Rc<JsonToF32Converter>,
        json_to_string_converter: Rc<JsonToStringConverter>,
    ) -> Self {
        Self {
            json_to_f32_converter,
            json_to_string_converter,
        }
    }

    pub fn get_json_to_f32_converter(&self) -> Rc<JsonToF32Converter> {
        Rc::clone(&self.json_to_f32_converter)
    }

    pub fn get_json_to_string_converter(&self) -> Rc<JsonToStringConverter> {
        Rc::clone(&self.json_to_string_converter)
    }
}

pub fn compose_component() -> JsonComponent {
    let json_to_f32_converter = Rc::new(JsonToF32Converter::new());

    let json_to_string_converter = Rc::new(JsonToStringConverter::new());

    let content_component = JsonComponent::new(json_to_f32_converter, json_to_string_converter);

    content_component
}
