use std::fs;

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
    json_to_f32_converter: TJsonToF32Converter,
}

impl<TJsonToF32Converter> JsonToSceneConverter<TJsonToF32Converter> {
    fn new(json_to_f32_converter: TJsonToF32Converter) -> Self {
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

pub fn compose_json_to_scene_converter() -> JsonToSceneConverter<JsonToF32Converter> {
    JsonToSceneConverter::new(JsonToF32Converter::new())
}

pub fn compose_scene_loader() -> SceneLoader<JsonToSceneConverter<JsonToF32Converter>> {
    SceneLoader::new(compose_json_to_scene_converter())
}

#[cfg(test)]
mod tests {
    use garden_content::{
        Content, GetVertexData, Rgb, Triangle, TriangleInstance, TrianglePoint, TwoDPoint,
    };
    use garden_json::ConvertJsonToValue;
    use serde_json::json;

    use crate::compose_json_to_content_converter;

    #[test]
    fn when_a_json_to_scene_converter_converts_json_to_a_two_d_scene_then_the_two_d_scene_is_converted(
    ) {
        let json_to_content_converter = compose_json_to_content_converter();

        let json = json!({
            "width": 123.45,
            "height": 678.90
        });

        let expected_result = TwoDScene::new(123.45, 678.90);

        let result = json_to_scene_converter.convert_json_to_value(&json);

        assert_eq!(expected_result.get_width(), result.get_width());
        assert_eq!(expected_result.get_height(), result.get_height());
    }
}
