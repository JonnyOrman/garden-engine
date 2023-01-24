use std::fs;

use garden_content::{Content, Rgb, Triangle, TrianglePoint, TwoDPoint};
use garden_json::ConvertJsonToValue;
use serde_json::Value;

pub trait LoadContent<TContent> {
    fn load_content(self) -> TContent;
}

pub struct ContentLoader<TJsonToContentConverter> {
    json_to_content_converter: TJsonToContentConverter,
}

impl<
        TJsonToContentConverter: ConvertJsonToValue<Content<Triangle<TrianglePoint<TwoDPoint, Rgb>>>>,
    > ContentLoader<TJsonToContentConverter>
{
    pub fn new(json_to_content_converter: TJsonToContentConverter) -> Self {
        Self {
            json_to_content_converter,
        }
    }
}

impl<
        TJsonToContentConverter: ConvertJsonToValue<Content<Triangle<TrianglePoint<TwoDPoint, Rgb>>>>,
    > LoadContent<Content<Triangle<TrianglePoint<TwoDPoint, Rgb>>>>
    for ContentLoader<TJsonToContentConverter>
{
    fn load_content(self) -> Content<Triangle<TrianglePoint<TwoDPoint, Rgb>>> {
        let file_contents = fs::read_to_string("content.json").unwrap();

        let json: Value = serde_json::from_str(file_contents.as_str()).unwrap();

        self.json_to_content_converter.convert_json_to_value(&json)
    }
}

pub struct JsonToContentConverter<TJsonToTriangleConverter> {
    json_to_triangle_converter: TJsonToTriangleConverter,
}

impl<TJsonToTriangleConverter> JsonToContentConverter<TJsonToTriangleConverter> {
    pub fn new(json_to_triangle_converter: TJsonToTriangleConverter) -> Self {
        Self {
            json_to_triangle_converter,
        }
    }
}

impl<TJsonToTriangleConverter: ConvertJsonToValue<Triangle<TrianglePoint<TwoDPoint, Rgb>>>>
    ConvertJsonToValue<Content<Triangle<TrianglePoint<TwoDPoint, Rgb>>>>
    for JsonToContentConverter<TJsonToTriangleConverter>
{
    fn convert_json_to_value(
        &self,
        json: &Value,
    ) -> Content<Triangle<TrianglePoint<TwoDPoint, Rgb>>> {
        let mut triangles = vec![];

        if let Some(triangle_json_array) = json["content"]["triangles"].as_array() {
            for triangle_json in triangle_json_array {
                triangles.push(
                    self.json_to_triangle_converter
                        .convert_json_to_value(triangle_json),
                );
            }
        }

        Content::new(triangles)
    }
}

pub struct JsonToTriangleConverter<TJsonToStringConverter, TJsonToTrianglePointConverter> {
    json_to_string_converter: TJsonToStringConverter,
    json_to_triangle_point_converter: TJsonToTrianglePointConverter,
}

impl<TJsonToStringConverter, TJsonToTrianglePointConverter>
    JsonToTriangleConverter<TJsonToStringConverter, TJsonToTrianglePointConverter>
{
    pub fn new(
        json_to_string_converter: TJsonToStringConverter,
        json_to_triangle_point_converter: TJsonToTrianglePointConverter,
    ) -> Self {
        Self {
            json_to_string_converter,
            json_to_triangle_point_converter,
        }
    }
}

impl<
        TJsonToStringConverter: ConvertJsonToValue<String>,
        TJsonToTrianglePointConverter: ConvertJsonToValue<TrianglePoint<TwoDPoint, Rgb>>,
    > ConvertJsonToValue<Triangle<TrianglePoint<TwoDPoint, Rgb>>>
    for JsonToTriangleConverter<TJsonToStringConverter, TJsonToTrianglePointConverter>
{
    fn convert_json_to_value(&self, json: &Value) -> Triangle<TrianglePoint<TwoDPoint, Rgb>> {
        Triangle::new(
            self.json_to_string_converter
                .convert_json_to_value(&json["name"]),
            //json.as_str().unwrap(),
            self.json_to_triangle_point_converter
                .convert_json_to_value(&json["point1"]),
            self.json_to_triangle_point_converter
                .convert_json_to_value(&json["point2"]),
            self.json_to_triangle_point_converter
                .convert_json_to_value(&json["point3"]),
        )
    }
}

pub struct JsonToTrianglePointConverter<TJsonToTwoDPointConverter, TJsonToRgbConverter> {
    json_to_two_d_point_converter: TJsonToTwoDPointConverter,
    json_to_rgb_converter: TJsonToRgbConverter,
}

impl<TJsonToTwoDPointConverter, TJsonToRgbConverter>
    JsonToTrianglePointConverter<TJsonToTwoDPointConverter, TJsonToRgbConverter>
{
    pub fn new(
        json_to_two_d_point_converter: TJsonToTwoDPointConverter,
        json_to_rgb_converter: TJsonToRgbConverter,
    ) -> Self {
        Self {
            json_to_two_d_point_converter,
            json_to_rgb_converter,
        }
    }
}

impl<
        TJsonToTwoDPointConverter: ConvertJsonToValue<TwoDPoint>,
        TJsonToRgbConverter: ConvertJsonToValue<Rgb>,
    > ConvertJsonToValue<TrianglePoint<TwoDPoint, Rgb>>
    for JsonToTrianglePointConverter<TJsonToTwoDPointConverter, TJsonToRgbConverter>
{
    fn convert_json_to_value(&self, json: &Value) -> TrianglePoint<TwoDPoint, Rgb> {
        TrianglePoint::new(
            self.json_to_two_d_point_converter
                .convert_json_to_value(&json["twoDPoint"]),
            self.json_to_rgb_converter
                .convert_json_to_value(&json["rgb"]),
        )
    }
}

pub struct JsonToTwoDPointConverter<TJsonToF32Converter> {
    json_to_f32_converter: TJsonToF32Converter,
}

impl<TJsonToF32Converter> JsonToTwoDPointConverter<TJsonToF32Converter> {
    pub fn new(json_to_f32_converter: TJsonToF32Converter) -> Self {
        Self {
            json_to_f32_converter,
        }
    }
}

impl<TJsonToF32Converter: ConvertJsonToValue<f32>> ConvertJsonToValue<TwoDPoint>
    for JsonToTwoDPointConverter<TJsonToF32Converter>
{
    fn convert_json_to_value(&self, json: &Value) -> TwoDPoint {
        TwoDPoint::new(
            self.json_to_f32_converter.convert_json_to_value(&json["x"]),
            self.json_to_f32_converter.convert_json_to_value(&json["y"]),
        )
    }
}

pub struct JsonToRgbConverter<TJsonToF32Converter> {
    json_to_f32_converter: TJsonToF32Converter,
}

impl<TJsonToF32Converter> JsonToRgbConverter<TJsonToF32Converter> {
    pub fn new(json_to_f32_converter: TJsonToF32Converter) -> Self {
        Self {
            json_to_f32_converter,
        }
    }
}

impl<TJsonToF32Converter: ConvertJsonToValue<f32>> ConvertJsonToValue<Rgb>
    for JsonToRgbConverter<TJsonToF32Converter>
{
    fn convert_json_to_value(&self, json: &Value) -> Rgb {
        Rgb::new(
            self.json_to_f32_converter.convert_json_to_value(&json["r"]),
            self.json_to_f32_converter.convert_json_to_value(&json["g"]),
            self.json_to_f32_converter.convert_json_to_value(&json["b"]),
        )
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

pub fn compose_json_to_content_converter() -> JsonToContentConverter<
    JsonToTriangleConverter<
        JsonToStringConverter,
        JsonToTrianglePointConverter<
            JsonToTwoDPointConverter<JsonToF32Converter>,
            JsonToRgbConverter<JsonToF32Converter>,
        >,
    >,
> {
    let json_to_two_d_point_converter = JsonToTwoDPointConverter::new(JsonToF32Converter::new());
    let json_to_rgb_converter = JsonToRgbConverter::new(JsonToF32Converter::new());
    let json_to_triangle_point_converter =
        JsonToTrianglePointConverter::new(json_to_two_d_point_converter, json_to_rgb_converter);
    let json_to_triangle_converter = JsonToTriangleConverter::new(
        JsonToStringConverter::new(),
        json_to_triangle_point_converter,
    );
    let json_to_content_converter = JsonToContentConverter::new(json_to_triangle_converter);

    json_to_content_converter
}

pub fn compose_content_loader() -> ContentLoader<
    JsonToContentConverter<
        JsonToTriangleConverter<
            JsonToStringConverter,
            JsonToTrianglePointConverter<
                JsonToTwoDPointConverter<JsonToF32Converter>,
                JsonToRgbConverter<JsonToF32Converter>,
            >,
        >,
    >,
> {
    ContentLoader::new(compose_json_to_content_converter())
}

#[cfg(test)]
mod tests {
    use garden_content::{Content, GetVertexData, Rgb, Triangle, TrianglePoint, TwoDPoint};
    use garden_json::ConvertJsonToValue;
    use serde_json::json;

    use crate::compose_json_to_content_converter;

    #[test]
    fn when_a_json_to_content_converter_converts_json_to_content_then_the_content_is_converted() {
        let json_to_content_converter = compose_json_to_content_converter();

        let json = json!({
            "content": {
                "triangles": [
                    {
                        "point1": {
                            "twoDPoint": {
                                "x": -1.0,
                                "y": -1.0
                            },
                            "rgb": {
                                "r": 1.0,
                                "g": 0.0,
                                "b": 0.0
                            }
                        },
                        "point2": {
                            "twoDPoint": {
                                "x": -0.5,
                                "y": 0.0
                            },
                            "rgb": {
                                "r": 0.0,
                                "g": 1.0,
                                "b": 0.0
                            }
                        },
                        "point3": {
                            "twoDPoint": {
                                "x": 0.0,
                                "y": -1.0
                            },
                            "rgb": {
                                "r": 0.0,
                                "g": 0.0,
                                "b": 1.0
                            }
                        }
                    },
                    {
                        "point1": {
                            "twoDPoint": {
                                "x": 0.0,
                                "y": 0.0
                            },
                            "rgb": {
                                "r": 1.0,
                                "g": 0.0,
                                "b": 0.0
                            }
                        },
                        "point2": {
                            "twoDPoint": {
                                "x": 0.5,
                                "y": 1.0
                            },
                            "rgb": {
                                "r": 0.0,
                                "g": 1.0,
                                "b": 0.0
                            }
                        },
                        "point3": {
                            "twoDPoint": {
                                "x": 1.0,
                                "y": 0.0
                            },
                            "rgb": {
                                "r": 0.0,
                                "g": 0.0,
                                "b": 1.0
                            }
                        }
                    }
                ]
            }
        });

        let expected_result = Content::<Triangle<TrianglePoint<TwoDPoint, Rgb>>>::new(vec![
            Triangle::<'static, TrianglePoint<TwoDPoint, Rgb>>::new(
                "Triangle1",
                TrianglePoint::<TwoDPoint, Rgb>::new(
                    TwoDPoint::new(-1.0, -1.0),
                    Rgb::new(1.0, 0.0, 0.0),
                ),
                TrianglePoint::<TwoDPoint, Rgb>::new(
                    TwoDPoint::new(-0.5, 0.0),
                    Rgb::new(0.0, 1.0, 0.0),
                ),
                TrianglePoint::<TwoDPoint, Rgb>::new(
                    TwoDPoint::new(0.0, -1.0),
                    Rgb::new(0.0, 0.0, 1.0),
                ),
            ),
            Triangle::<TrianglePoint<TwoDPoint, Rgb>>::new(
                "Triangle2",
                TrianglePoint::<TwoDPoint, Rgb>::new(
                    TwoDPoint::new(0.0, 0.0),
                    Rgb::new(1.0, 0.0, 0.0),
                ),
                TrianglePoint::<TwoDPoint, Rgb>::new(
                    TwoDPoint::new(0.5, 1.0),
                    Rgb::new(0.0, 1.0, 0.0),
                ),
                TrianglePoint::<TwoDPoint, Rgb>::new(
                    TwoDPoint::new(1.0, 0.0),
                    Rgb::new(0.0, 0.0, 1.0),
                ),
            ),
        ]);

        let result = json_to_content_converter.convert_json_to_value(&json);

        assert_eq!(expected_result.get_vertex_data(), result.get_vertex_data());
    }
}
