use std::{fs, rc::Rc};

use garden_json::{ConvertJsonToValue, JsonToF32Converter};
use garden_loading::Load;
use garden_scenes::TwoDScene;
use serde_json::Value;

pub struct SceneLoader<TJsonToSceneConverter> {
    json_to_scene_converter: TJsonToSceneConverter,
}

impl<TJsonToSceneConverter> SceneLoader<TJsonToSceneConverter> {
    fn new(json_to_scene_converter: TJsonToSceneConverter) -> Self {
        Self {
            json_to_scene_converter,
        }
    }
}

impl<TJsonToF32Converter: ConvertJsonToValue<TwoDScene>> Load<TwoDScene>
    for SceneLoader<TJsonToF32Converter>
{
    fn load(self) -> TwoDScene {
        let file_contents = fs::read_to_string("content.json").unwrap();

        let json: Value = serde_json::from_str(file_contents.as_str()).unwrap();

        self.json_to_scene_converter
            .convert_json_to_value(&json["scene"])
    }
}

pub struct JsonToSceneConverter<TJsonToF32Converter> {
    json_to_f32_converter: Rc<TJsonToF32Converter>,
}

impl<TJsonToF32Converter> JsonToSceneConverter<TJsonToF32Converter> {
    fn new(json_to_f32_converter: Rc<TJsonToF32Converter>) -> Self {
        Self {
            json_to_f32_converter,
        }
    }
}

impl<TJsonToF32Converter: ConvertJsonToValue<f32>> ConvertJsonToValue<TwoDScene>
    for JsonToSceneConverter<TJsonToF32Converter>
{
    fn convert_json_to_value(&self, json: &Value) -> TwoDScene {
        let width = self
            .json_to_f32_converter
            .convert_json_to_value(&json["width"]);

        let height = self
            .json_to_f32_converter
            .convert_json_to_value(&json["height"]);

        TwoDScene::new(width, height)
    }
}

pub fn compose_json_to_scene_converter(
    json_to_f32_converter: Rc<JsonToF32Converter>,
) -> JsonToSceneConverter<JsonToF32Converter> {
    JsonToSceneConverter::new(json_to_f32_converter)
}

pub fn compose_scene_loader(
    json_to_f32_converter: Rc<JsonToF32Converter>,
) -> SceneLoader<JsonToSceneConverter<JsonToF32Converter>> {
    SceneLoader::new(compose_json_to_scene_converter(json_to_f32_converter))
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use garden_json::{ConvertJsonToValue, JsonToF32Converter};
    use garden_scenes::{GetHeight, GetWidth, TwoDScene};
    use mockall::{mock, predicate};
    use serde_json::{json, Value};

    use crate::{compose_json_to_scene_converter, JsonToSceneConverter};

    #[test]
    fn when_a_json_to_scene_converter_is_composed_and_converts_json_to_a_two_d_scene_then_the_two_d_scene_is_converted(
    ) {
        let json_to_f32_converter = Rc::new(JsonToF32Converter::new());

        let json_to_scene_converter = compose_json_to_scene_converter(json_to_f32_converter);

        let json = json!({
            "width": 123.45,
            "height": 678.90
        });

        let expected_result = TwoDScene::new(123.45, 678.90);

        let result = json_to_scene_converter.convert_json_to_value(&json);

        assert_eq!(expected_result.get_width(), result.get_width());
        assert_eq!(expected_result.get_height(), result.get_height());
    }

    mock! {
        JsonToF32Converter {}
        impl ConvertJsonToValue<f32> for JsonToF32Converter {
            fn convert_json_to_value(&self, json: &Value) -> f32;
        }
    }

    #[test]
    fn when_a_json_to_scene_converter_converts_json_to_a_two_d_scene_then_the_two_d_scene_is_converted(
    ) {
        let json = json!({
            "width": 123.45,
            "height": 678.90
        });

        let width = json["width"].clone();
        let height = json["height"].clone();

        let mut json_to_f32_converter = MockJsonToF32Converter::new();
        json_to_f32_converter
            .expect_convert_json_to_value()
            .with(predicate::eq(width))
            .times(1)
            .returning(|x| 123.45);
        json_to_f32_converter
            .expect_convert_json_to_value()
            .with(predicate::eq(height))
            .times(1)
            .returning(|x| 678.90);

        let json_to_f32_converter_rc = Rc::new(json_to_f32_converter);

        let expected_result = TwoDScene::new(123.45, 678.90);

        let json_to_scene_converter =
            JsonToSceneConverter::new(Rc::clone(&json_to_f32_converter_rc));

        let result = json_to_scene_converter.convert_json_to_value(&json);

        assert_eq!(expected_result.get_width(), result.get_width());
        assert_eq!(expected_result.get_height(), result.get_height());
    }
}
