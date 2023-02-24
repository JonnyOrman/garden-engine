use std::rc::Rc;

use garden_json::JsonToF32Converter;

pub struct JsonComponent {
    json_to_f32_converter: Rc<JsonToF32Converter>,
}

impl JsonComponent {
    fn new(json_to_f32_converter: Rc<JsonToF32Converter>) -> Self {
        Self {
            json_to_f32_converter,
        }
    }

    pub fn get_json_to_f32_converter(&self) -> Rc<JsonToF32Converter> {
        Rc::clone(&self.json_to_f32_converter)
    }
}

pub fn compose_component() -> JsonComponent {
    let json_to_f32_converter = Rc::new(JsonToF32Converter::new());

    let content_component = JsonComponent::new(json_to_f32_converter);

    content_component
}
