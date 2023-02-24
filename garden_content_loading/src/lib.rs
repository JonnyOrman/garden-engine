use garden::GetName;
use garden_content::{
    Content, Get2DCoordiantes, GetB, GetContentInstanceData, GetG, GetR, GetRgb, GetX, GetY,
    Rectangle, RectangleInstance, Rgb, Triangle, TriangleInstance, TrianglePoint, TwoDPoint,
};
use garden_json::{ConvertJsonToValue, JsonToF32Converter};
use garden_loading::Load;
use serde_json::Value;
use std::{collections::HashMap, fs, rc::Rc};

pub struct ContentLoader<TJsonToContentConverter> {
    json_to_content_converter: TJsonToContentConverter,
}

impl<'a, TJsonToContentConverter> ContentLoader<TJsonToContentConverter> {
    pub fn new(json_to_content_converter: TJsonToContentConverter) -> Self {
        Self {
            json_to_content_converter,
        }
    }
}

impl<'a, TJsonToContentConverter: ConvertJsonToValue<Content>> Load<Content>
    for ContentLoader<TJsonToContentConverter>
{
    fn load(self) -> Content {
        let file_contents = fs::read_to_string("content.json").unwrap();

        let json: Value = serde_json::from_str(file_contents.as_str()).unwrap();

        self.json_to_content_converter.convert_json_to_value(&json)
    }
}

pub struct JsonToContentConverter<TJsonToObjectConverter, TJsonToObjectInstanceConverter> {
    json_to_object_converter: TJsonToObjectConverter,
    json_to_object_instance_converter: TJsonToObjectInstanceConverter,
}

impl<
        'a,
        TJsonToObjectConverter: ConvertJsonToValue<Box<dyn GetName>>,
        TJsonToObjectInstanceConverter: ConvertJsonToValue<Box<dyn GetContentInstanceData>>,
    > JsonToContentConverter<TJsonToObjectConverter, TJsonToObjectInstanceConverter>
{
    pub fn new(
        json_to_object_converter: TJsonToObjectConverter,
        json_to_object_instance_converter: TJsonToObjectInstanceConverter,
    ) -> Self {
        Self {
            json_to_object_converter,
            json_to_object_instance_converter,
        }
    }
}

impl<
        'a,
        TJsonToObjectConverter: ConvertJsonToValue<Box<dyn GetName>>,
        TJsonToObjectInstanceConverter: ConvertJsonToValue<Box<dyn GetContentInstanceData>>,
    > ConvertJsonToValue<Content>
    for JsonToContentConverter<TJsonToObjectConverter, TJsonToObjectInstanceConverter>
{
    fn convert_json_to_value(&self, json: &Value) -> Content {
        let mut objects = Vec::<Rc<Box<dyn GetName>>>::new();

        if let Some(object_json_array) = json["content"]["objects"].as_array() {
            for object_json in object_json_array {
                objects.push(Rc::new(
                    self.json_to_object_converter
                        .convert_json_to_value(object_json),
                ));
            }
        }

        let mut object_instances = Vec::<Box<dyn GetContentInstanceData>>::new();

        if let Some(object_instance_json_array) = json["objects"].as_array() {
            for object_instance_json in object_instance_json_array {
                object_instances.push(
                    self.json_to_object_instance_converter
                        .convert_json_to_value(object_instance_json),
                );
            }
        }

        Content::new(objects, object_instances)
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
            self.json_to_triangle_point_converter
                .convert_json_to_value(&json["point1"]),
            self.json_to_triangle_point_converter
                .convert_json_to_value(&json["point2"]),
            self.json_to_triangle_point_converter
                .convert_json_to_value(&json["point3"]),
        )
    }
}

pub struct JsonToBoxedTriangleConverter<TJsonToTriangleConverter> {
    json_to_triangle_converter: TJsonToTriangleConverter,
}

impl<TJsonToTriangleConverter> JsonToBoxedTriangleConverter<TJsonToTriangleConverter> {
    fn new(json_to_triangle_converter: TJsonToTriangleConverter) -> Self {
        Self {
            json_to_triangle_converter,
        }
    }
}

impl<TJsonToTriangleConverter: ConvertJsonToValue<Triangle<TrianglePoint<TwoDPoint, Rgb>>>>
    ConvertJsonToValue<Box<dyn GetName>>
    for JsonToBoxedTriangleConverter<TJsonToTriangleConverter>
{
    fn convert_json_to_value(&self, json: &Value) -> Box<dyn GetName> {
        Box::new(self.json_to_triangle_converter.convert_json_to_value(json))
    }
}

pub struct JsonToRectangleConverter<
    TJsonToStringConverter,
    TJsonToF32Converter,
    TJsonToRgbConverter,
> {
    json_to_string_converter: TJsonToStringConverter,
    json_to_f32_converter: TJsonToF32Converter,
    json_to_rgb_converter: TJsonToRgbConverter,
}

impl<TJsonToStringConverter, TJsonToF32Converter, TJsonToRgbConverter>
    JsonToRectangleConverter<TJsonToStringConverter, TJsonToF32Converter, TJsonToRgbConverter>
{
    fn new(
        json_to_string_converter: TJsonToStringConverter,
        json_to_f32_converter: TJsonToF32Converter,
        json_to_rgb_converter: TJsonToRgbConverter,
    ) -> Self {
        Self {
            json_to_string_converter,
            json_to_f32_converter,
            json_to_rgb_converter,
        }
    }
}

impl<
        TJsonToStringConverter: ConvertJsonToValue<String>,
        TJsonToF32Converter: ConvertJsonToValue<f32>,
        TJsonToRgbConverter: ConvertJsonToValue<TRgb>,
        TRgb,
    > ConvertJsonToValue<Rectangle<TRgb>>
    for JsonToRectangleConverter<TJsonToStringConverter, TJsonToF32Converter, TJsonToRgbConverter>
{
    fn convert_json_to_value(&self, json: &Value) -> Rectangle<TRgb> {
        Rectangle::<TRgb>::new(
            self.json_to_string_converter
                .convert_json_to_value(&json["name"]),
            self.json_to_f32_converter
                .convert_json_to_value(&json["width"]),
            self.json_to_f32_converter
                .convert_json_to_value(&json["height"]),
            self.json_to_rgb_converter
                .convert_json_to_value(&json["rgb"]),
        )
    }
}

pub struct JsonToBoxedRectangleConverter<TJsonToRectangleConverter> {
    json_to_rectangle_converter: TJsonToRectangleConverter,
}

impl<TJsonToRectangleConverter> JsonToBoxedRectangleConverter<TJsonToRectangleConverter> {
    fn new(json_to_rectangle_converter: TJsonToRectangleConverter) -> Self {
        Self {
            json_to_rectangle_converter,
        }
    }
}

impl<TJsonToRectangleConverter: ConvertJsonToValue<Rectangle<Rgb>>>
    ConvertJsonToValue<Box<dyn GetName>>
    for JsonToBoxedRectangleConverter<TJsonToRectangleConverter>
{
    fn convert_json_to_value(&self, json: &Value) -> Box<dyn GetName> {
        Box::new(self.json_to_rectangle_converter.convert_json_to_value(json))
    }
}

pub struct JsonToTriangleInstanceConverter<
    TJsonToStringConverter,
    TJsonToTwoDPointConverter,
    TJsonToTrianglePointConverter,
    TJsonToF32Converter,
> {
    json_to_string_converter: TJsonToStringConverter,
    json_to_two_d_point_converter: TJsonToTwoDPointConverter,
    json_to_triangle_point_converter: TJsonToTrianglePointConverter,
    json_to_f32_converter: TJsonToF32Converter,
}

impl<
        TJsonToStringConverter,
        TJsonToTwoDPointConverter,
        TJsonToTrianglePointConverter,
        TJsonToF32Converter,
    >
    JsonToTriangleInstanceConverter<
        TJsonToStringConverter,
        TJsonToTwoDPointConverter,
        TJsonToTrianglePointConverter,
        TJsonToF32Converter,
    >
{
    fn new(
        json_to_string_converter: TJsonToStringConverter,
        json_to_two_d_point_converter: TJsonToTwoDPointConverter,
        json_to_triangle_point_converter: TJsonToTrianglePointConverter,
        json_to_f32_converter: TJsonToF32Converter,
    ) -> Self {
        Self {
            json_to_string_converter,
            json_to_two_d_point_converter,
            json_to_triangle_point_converter,
            json_to_f32_converter,
        }
    }
}

impl<
        TJsonToStringConverter: ConvertJsonToValue<String>,
        TJsonToTwoDPointConverter: ConvertJsonToValue<TwoDPoint>,
        TJsonToTrianglePointConverter: ConvertJsonToValue<TrianglePoint<TwoDPoint, Rgb>>,
        TJsonToF32Converter: ConvertJsonToValue<f32>,
    > ConvertJsonToValue<TriangleInstance<TwoDPoint, TrianglePoint<TwoDPoint, Rgb>>>
    for JsonToTriangleInstanceConverter<
        TJsonToStringConverter,
        TJsonToTwoDPointConverter,
        TJsonToTrianglePointConverter,
        TJsonToF32Converter,
    >
{
    fn convert_json_to_value(
        &self,
        json: &Value,
    ) -> TriangleInstance<TwoDPoint, TrianglePoint<TwoDPoint, Rgb>> {
        let scale = self
            .json_to_f32_converter
            .convert_json_to_value(&json["scale"]);

        let position = self
            .json_to_two_d_point_converter
            .convert_json_to_value(&json["position"]);

        let point_1 = self
            .json_to_triangle_point_converter
            .convert_json_to_value(&json["point1"]);

        let point_1_translated = TrianglePoint::<TwoDPoint, Rgb>::new(
            TwoDPoint::new(
                point_1.get_x() * scale + position.get_x(),
                point_1.get_y() * scale + position.get_y(),
            ),
            Rgb::new(
                point_1.get_rgb().get_r(),
                point_1.get_rgb().get_g(),
                point_1.get_rgb().get_b(),
            ),
        );

        let point_2 = self
            .json_to_triangle_point_converter
            .convert_json_to_value(&json["point2"]);

        let point_2_translated = TrianglePoint::<TwoDPoint, Rgb>::new(
            TwoDPoint::new(
                point_2.get_x() * scale + position.get_x(),
                point_2.get_y() * scale + position.get_y(),
            ),
            Rgb::new(
                point_2.get_rgb().get_r(),
                point_2.get_rgb().get_g(),
                point_2.get_rgb().get_b(),
            ),
        );

        let point_3 = self
            .json_to_triangle_point_converter
            .convert_json_to_value(&json["point3"]);

        let point_3_translated = TrianglePoint::new(
            TwoDPoint::new(
                point_3.get_x() * scale + position.get_x(),
                point_3.get_y() * scale + position.get_y(),
            ),
            Rgb::new(
                point_3.get_rgb().get_r(),
                point_3.get_rgb().get_g(),
                point_3.get_rgb().get_b(),
            ),
        );

        TriangleInstance::new(
            self.json_to_string_converter
                .convert_json_to_value(&json["name"]),
            self.json_to_string_converter
                .convert_json_to_value(&json["contentName"]),
            scale,
            self.json_to_two_d_point_converter
                .convert_json_to_value(&json["position"]),
            point_1_translated,
            point_2_translated,
            point_3_translated,
        )
    }
}

pub struct JsonToBoxedTriangleInstanceConverter<TJsonToTriangleInstanceConverter> {
    json_to_triangle_instance_converter: TJsonToTriangleInstanceConverter,
}

impl<TJsonToTriangleInstanceConverter>
    JsonToBoxedTriangleInstanceConverter<TJsonToTriangleInstanceConverter>
{
    fn new(json_to_triangle_instance_converter: TJsonToTriangleInstanceConverter) -> Self {
        Self {
            json_to_triangle_instance_converter,
        }
    }
}

impl<
        TJsonToTriangleInstanceConverter: ConvertJsonToValue<TriangleInstance<TwoDPoint, TrianglePoint<TwoDPoint, Rgb>>>,
    > ConvertJsonToValue<Box<dyn GetContentInstanceData>>
    for JsonToBoxedTriangleInstanceConverter<TJsonToTriangleInstanceConverter>
{
    fn convert_json_to_value(&self, json: &Value) -> Box<dyn GetContentInstanceData> {
        Box::new(
            self.json_to_triangle_instance_converter
                .convert_json_to_value(json),
        )
    }
}

pub struct JsonToRectangleInstanceConverter<
    TJsonToStringConverter,
    TJsonToF32Converter,
    TJsonToPositionConverter,
> {
    json_to_string_converter: TJsonToStringConverter,
    json_to_f32_converter: TJsonToF32Converter,
    json_to_position_converter: TJsonToPositionConverter,
}

impl<TJsonToStringConverter, TJsonToF32Converter, TJsonToPositionConverter>
    JsonToRectangleInstanceConverter<
        TJsonToStringConverter,
        TJsonToF32Converter,
        TJsonToPositionConverter,
    >
{
    fn new(
        json_to_string_converter: TJsonToStringConverter,
        json_to_f32_converter: TJsonToF32Converter,
        json_to_position_converter: TJsonToPositionConverter,
    ) -> Self {
        Self {
            json_to_string_converter,
            json_to_f32_converter,
            json_to_position_converter,
        }
    }
}

impl<
        TJsonToStringConverter: ConvertJsonToValue<String>,
        TJsonToF32Converter: ConvertJsonToValue<f32>,
        TJsonToPositionConverter: ConvertJsonToValue<TPosition>,
        TPosition: Get2DCoordiantes,
    >
    ConvertJsonToValue<
        RectangleInstance<
            TPosition,
            TrianglePoint<TwoDPoint, Rgb>,
            TriangleInstance<TwoDPoint, TrianglePoint<TwoDPoint, Rgb>>,
        >,
    >
    for JsonToRectangleInstanceConverter<
        TJsonToStringConverter,
        TJsonToF32Converter,
        TJsonToPositionConverter,
    >
{
    fn convert_json_to_value(
        &self,
        json: &Value,
    ) -> RectangleInstance<
        TPosition,
        TrianglePoint<TwoDPoint, Rgb>,
        TriangleInstance<TwoDPoint, TrianglePoint<TwoDPoint, Rgb>>,
    > {
        let scale = self
            .json_to_f32_converter
            .convert_json_to_value(&json["scale"]);

        RectangleInstance::<
            TPosition,
            TrianglePoint<TwoDPoint, Rgb>,
            TriangleInstance<TwoDPoint, TrianglePoint<TwoDPoint, Rgb>>,
        >::new(
            self.json_to_string_converter
                .convert_json_to_value(&json["name"]),
            self.json_to_string_converter
                .convert_json_to_value(&json["contentName"]),
            scale,
            self.json_to_position_converter
                .convert_json_to_value(&json["position"]),
            self.json_to_f32_converter
                .convert_json_to_value(&json["width"]),
            self.json_to_f32_converter
                .convert_json_to_value(&json["height"]),
        )
    }
}

pub struct JsonToBoxedRectangleInstanceConverter<TJsonToRectangleInstanceConverter> {
    json_to_rectangle_instance_converter: TJsonToRectangleInstanceConverter,
}

impl<TJsonToRectangleInstanceConverter>
    JsonToBoxedRectangleInstanceConverter<TJsonToRectangleInstanceConverter>
{
    fn new(json_to_rectangle_instance_converter: TJsonToRectangleInstanceConverter) -> Self {
        Self {
            json_to_rectangle_instance_converter,
        }
    }
}

impl<
        TJsonToRectangleInstanceConverter: ConvertJsonToValue<
            RectangleInstance<
                TwoDPoint,
                TrianglePoint<TwoDPoint, Rgb>,
                TriangleInstance<TwoDPoint, TrianglePoint<TwoDPoint, Rgb>>,
            >,
        >,
    > ConvertJsonToValue<Box<dyn GetContentInstanceData>>
    for JsonToBoxedRectangleInstanceConverter<TJsonToRectangleInstanceConverter>
{
    fn convert_json_to_value(&self, json: &Value) -> Box<dyn GetContentInstanceData> {
        Box::new(
            self.json_to_rectangle_instance_converter
                .convert_json_to_value(json),
        )
    }
}

pub struct TypedJsonToValueConverter<TJsonToStringConverter, TBox> {
    json_to_string_converter: TJsonToStringConverter,
    converter_map: HashMap<String, Box<dyn ConvertJsonToValue<TBox>>>,
}

impl<TJsonToStringConverter, TBox> TypedJsonToValueConverter<TJsonToStringConverter, TBox> {
    fn new(
        json_to_string_converter: TJsonToStringConverter,
        converter_map: HashMap<String, Box<dyn ConvertJsonToValue<TBox>>>,
    ) -> Self {
        Self {
            json_to_string_converter,
            converter_map,
        }
    }
}

impl<TJsonToStringConverter: ConvertJsonToValue<String>, TBox> ConvertJsonToValue<TBox>
    for TypedJsonToValueConverter<TJsonToStringConverter, TBox>
{
    fn convert_json_to_value(&self, json: &Value) -> TBox {
        let value_type = self
            .json_to_string_converter
            .convert_json_to_value(&json["type"]);

        match self.converter_map.get(&value_type) {
            Some(converter) => converter.convert_json_to_value(json),
            None => todo!("JSON converter for type {value_type} not found."),
        }
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

pub fn compose_json_to_content_converter() -> JsonToContentConverter<
    TypedJsonToValueConverter<JsonToStringConverter, Box<dyn GetName>>,
    TypedJsonToValueConverter<JsonToStringConverter, Box<dyn GetContentInstanceData>>,
> {
    let json_to_triangle_converter = JsonToTriangleConverter::new(
        JsonToStringConverter::new(),
        JsonToTrianglePointConverter::new(
            JsonToTwoDPointConverter::new(JsonToF32Converter::new()),
            JsonToRgbConverter::new(JsonToF32Converter::new()),
        ),
    );

    let json_to_boxed_triangle_converter =
        JsonToBoxedTriangleConverter::new(json_to_triangle_converter);

    let json_to_triangle_instance_converter = JsonToTriangleInstanceConverter::new(
        JsonToStringConverter::new(),
        JsonToTwoDPointConverter::new(JsonToF32Converter::new()),
        JsonToTrianglePointConverter::new(
            JsonToTwoDPointConverter::new(JsonToF32Converter::new()),
            JsonToRgbConverter::new(JsonToF32Converter::new()),
        ),
        JsonToF32Converter::new(),
    );

    let json_to_boxed_triangle_instance_converter =
        JsonToBoxedTriangleInstanceConverter::new(json_to_triangle_instance_converter);

    let json_to_rectangle_converter = JsonToRectangleConverter::new(
        JsonToStringConverter::new(),
        JsonToF32Converter::new(),
        JsonToRgbConverter::new(JsonToF32Converter::new()),
    );

    let json_to_boxed_rectangle_converter =
        JsonToBoxedRectangleConverter::new(json_to_rectangle_converter);

    let json_to_rectangle_instance_converter = JsonToRectangleInstanceConverter::new(
        JsonToStringConverter::new(),
        JsonToF32Converter::new(),
        JsonToTwoDPointConverter::new(JsonToF32Converter::new()),
    );

    let json_to_boxed_rectangle_instance_converter =
        JsonToBoxedRectangleInstanceConverter::new(json_to_rectangle_instance_converter);

    let mut object_converters =
        HashMap::<String, Box<dyn ConvertJsonToValue<Box<dyn GetName>>>>::new();
    object_converters.insert(
        "triangle".to_string(),
        Box::new(json_to_boxed_triangle_converter),
    );
    object_converters.insert(
        "rectangle".to_string(),
        Box::new(json_to_boxed_rectangle_converter),
    );

    let json_to_object_converter =
        TypedJsonToValueConverter::new(JsonToStringConverter::new(), object_converters);

    let mut object_instance_converters =
        HashMap::<String, Box<dyn ConvertJsonToValue<Box<dyn GetContentInstanceData>>>>::new();
    object_instance_converters.insert(
        "triangle".to_string(),
        Box::new(json_to_boxed_triangle_instance_converter),
    );
    object_instance_converters.insert(
        "rectangle".to_string(),
        Box::new(json_to_boxed_rectangle_instance_converter),
    );

    let json_to_object_instance_converter =
        TypedJsonToValueConverter::new(JsonToStringConverter::new(), object_instance_converters);

    let json_to_content_converter =
        JsonToContentConverter::new(json_to_object_converter, json_to_object_instance_converter);

    json_to_content_converter
}

pub fn compose_content_loader() -> ContentLoader<
    JsonToContentConverter<
        TypedJsonToValueConverter<JsonToStringConverter, Box<dyn GetName>>,
        TypedJsonToValueConverter<JsonToStringConverter, Box<dyn GetContentInstanceData>>,
    >,
> {
    let json_to_content_converter = compose_json_to_content_converter();

    ContentLoader::<
        JsonToContentConverter<
            TypedJsonToValueConverter<JsonToStringConverter, Box<dyn GetName>>,
            TypedJsonToValueConverter<JsonToStringConverter, Box<dyn GetContentInstanceData>>,
        >,
    >::new(json_to_content_converter)
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use garden_content::{
        Content, GetVertexData, Rectangle, RectangleInstance, Rgb, Triangle, TriangleInstance,
        TrianglePoint, TwoDPoint,
    };
    use garden_json::ConvertJsonToValue;
    use serde_json::json;

    use crate::compose_json_to_content_converter;

    #[test]
    fn when_a_json_to_content_converter_converts_json_to_content_then_the_content_is_converted() {
        let json_to_content_converter = compose_json_to_content_converter();

        let json = json!({
            "content": {
                "objects": [
                    {
                        "name": "Triangle1",
                        "type": "triangle",
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
                                "x": 0.0,
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
                    },
                    {
                        "name": "Rectangle1",
                        "type": "rectangle",
                        "width": 2.0,
                        "height": 5.0,
                        "rgb": {
                            "r": 0.0,
                            "g": 0.0,
                            "b": 1.0
                        }
                    },
                    {
                        "name": "Rectangle2",
                        "type": "rectangle",
                        "width": 3.0,
                        "height": 2.0,
                        "rgb": {
                            "r": 1.0,
                            "g": 0.0,
                            "b": 0.0
                        }
                    }
                ]
            },
            "objects": [
                {
                    "name": "Triangle1-a",
                    "contentName": "Triangle1",
                    "type": "triangle",
                    "scale": 0.5,
                    "position": {
                        "x": -5.0,
                        "y": -5.0
                    },
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
                            "x": 0.0,
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
                    "name": "Triangle1-b",
                    "contentName": "Triangle1",
                    "type": "triangle",
                    "scale": 3.0,
                    "position": {
                        "x": 5.0,
                        "y": 5.0
                    },
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
                            "x": 0.0,
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
                    "name": "Rectangle1-a",
                    "contentName": "Rectangle1",
                    "type": "rectangle",
                    "scale": 1.0,
                    "width": 2.5,
                    "height": 5.0,
                    "position": {
                        "x": -5.0,
                        "y": 5.0
                    }
                },
                {
                    "name": "Rectangle2-a",
                    "contentName": "Rectangle2",
                    "type": "rectangle",
                    "scale": 1.0,
                    "width": 3.0,
                    "height": 2.0,
                    "position": {
                        "x": 5.0,
                        "y": -5.0
                    }
                }
            ]
        });

        let expected_result = Content::new(
            vec![
                Rc::new(Box::new(Triangle::<TrianglePoint<TwoDPoint, Rgb>>::new(
                    "Triangle1".to_string(),
                    TrianglePoint::<TwoDPoint, Rgb>::new(
                        TwoDPoint::new(-1.0, -1.0),
                        Rgb::new(1.0, 0.0, 0.0),
                    ),
                    TrianglePoint::<TwoDPoint, Rgb>::new(
                        TwoDPoint::new(0.0, 1.0),
                        Rgb::new(0.0, 1.0, 0.0),
                    ),
                    TrianglePoint::<TwoDPoint, Rgb>::new(
                        TwoDPoint::new(1.0, 0.0),
                        Rgb::new(0.0, 0.0, 1.0),
                    ),
                ))),
                Rc::new(Box::new(Rectangle::new(
                    "Rectangle1".to_string(),
                    2.0,
                    5.0,
                    Rgb::new(0.0, 0.0, 0.1),
                ))),
                Rc::new(Box::new(Rectangle::new(
                    "Rectangle2".to_string(),
                    3.0,
                    2.0,
                    Rgb::new(1.0, 0.0, 0.0),
                ))),
            ],
            vec![
                Box::new(
                    TriangleInstance::<TwoDPoint, TrianglePoint<TwoDPoint, Rgb>>::new(
                        "Triangle1-a".to_string(),
                        "Triangle1".to_string(),
                        0.5,
                        TwoDPoint::new(-5.0, -5.0),
                        TrianglePoint::<TwoDPoint, Rgb>::new(
                            TwoDPoint::new(-5.5, -5.5),
                            Rgb::new(1.0, 0.0, 0.0),
                        ),
                        TrianglePoint::<TwoDPoint, Rgb>::new(
                            TwoDPoint::new(-5.0, -4.5),
                            Rgb::new(0.0, 1.0, 0.0),
                        ),
                        TrianglePoint::<TwoDPoint, Rgb>::new(
                            TwoDPoint::new(-4.5, -5.5),
                            Rgb::new(0.0, 0.0, 1.0),
                        ),
                    ),
                ),
                Box::new(
                    TriangleInstance::<TwoDPoint, TrianglePoint<TwoDPoint, Rgb>>::new(
                        "Triangle1-b".to_string(),
                        "Triangle1".to_string(),
                        0.5,
                        TwoDPoint::new(0.5, 0.5),
                        TrianglePoint::<TwoDPoint, Rgb>::new(
                            TwoDPoint::new(2.0, 2.0),
                            Rgb::new(1.0, 0.0, 0.0),
                        ),
                        TrianglePoint::<TwoDPoint, Rgb>::new(
                            TwoDPoint::new(5.0, 8.0),
                            Rgb::new(0.0, 1.0, 0.0),
                        ),
                        TrianglePoint::<TwoDPoint, Rgb>::new(
                            TwoDPoint::new(8.0, 2.0),
                            Rgb::new(0.0, 0.0, 1.0),
                        ),
                    ),
                ),
                Box::new(RectangleInstance::new(
                    "Rectangle1-a".to_string(),
                    "Rectangle1".to_string(),
                    1.0,
                    TwoDPoint::new(-5.0, 5.0),
                    2.5,
                    5.0,
                )),
                Box::new(RectangleInstance::new(
                    "Rectangle2-a".to_string(),
                    "Rectangle2".to_string(),
                    1.0,
                    TwoDPoint::new(5.0, -5.0),
                    3.0,
                    2.0,
                )),
            ],
        );

        let result = json_to_content_converter.convert_json_to_value(&json);

        assert_eq!(expected_result.get_vertex_data(), result.get_vertex_data());
    }
}
