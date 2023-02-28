use garden::GetName;
use garden_content::{
    rectangles::{
        CreateRectangle, CreateRectangleInstance, Rectangle, RectangleCreator, RectangleInstance,
        RectangleInstanceCreator, RectangleInstanceScaler,
    },
    triangles::{
        CreateTriangle, CreateTriangleInstance, Triangle, TriangleCreator, TriangleInstance,
        TriangleInstanceCreator, TriangleInstanceScaler,
    },
    Content, CreateTrianglePoint, GetB, GetG, GetNumberOfVertices, GetR, GetRgb, GetVertexData,
    GetX, GetY, ObjectInstanceRunner, Rgb, RgbCreator, RunObjectInstance, TrianglePoint,
    TrianglePointCreator, TwoDPoint, TwoDPointCreator,
};
use garden_json::{ConvertJsonToValue, JsonToF32Converter, JsonToStringConverter};
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

pub struct JsonToContentConverter<TJsonToObjectConverter, TJsonToObjectInstanceRunnerConverter> {
    json_to_object_converter: TJsonToObjectConverter,
    json_to_object_instance_runner_converter: TJsonToObjectInstanceRunnerConverter,
}

impl<
        'a,
        TJsonToObjectConverter: ConvertJsonToValue<Box<dyn GetName>>,
        TJsonToObjectInstanceRunnerConverter: ConvertJsonToValue<Box<dyn RunObjectInstance>>,
    > JsonToContentConverter<TJsonToObjectConverter, TJsonToObjectInstanceRunnerConverter>
{
    pub fn new(
        json_to_object_converter: TJsonToObjectConverter,
        json_to_object_instance_runner_converter: TJsonToObjectInstanceRunnerConverter,
    ) -> Self {
        Self {
            json_to_object_converter,
            json_to_object_instance_runner_converter,
        }
    }
}

impl<
        'a,
        TJsonToObjectConverter: ConvertJsonToValue<Box<dyn GetName>>,
        TJsonToObjectInstanceRunnerConverter: ConvertJsonToValue<Box<dyn RunObjectInstance>>,
    > ConvertJsonToValue<Content>
    for JsonToContentConverter<TJsonToObjectConverter, TJsonToObjectInstanceRunnerConverter>
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

        let mut object_instance_runners = Vec::<Box<dyn RunObjectInstance>>::new();

        if let Some(object_instance_json_array) = json["objects"].as_array() {
            for object_instance_json in object_instance_json_array {
                object_instance_runners.push(
                    self.json_to_object_instance_runner_converter
                        .convert_json_to_value(object_instance_json),
                );
            }
        }

        Content::new(objects, object_instance_runners)
    }
}

pub struct JsonToTriangleConverter<
    TJsonToStringConverter,
    TJsonToTrianglePointConverter,
    TTriangleCreator,
> {
    json_to_string_converter: Rc<TJsonToStringConverter>,
    json_to_triangle_point_converter: Rc<TJsonToTrianglePointConverter>,
    triangle_creator: TTriangleCreator,
}

impl<TJsonToStringConverter, TJsonToTrianglePointConverter, TTriangleCreator>
    JsonToTriangleConverter<TJsonToStringConverter, TJsonToTrianglePointConverter, TTriangleCreator>
{
    pub fn new(
        json_to_string_converter: Rc<TJsonToStringConverter>,
        json_to_triangle_point_converter: Rc<TJsonToTrianglePointConverter>,
        triangle_creator: TTriangleCreator,
    ) -> Self {
        Self {
            json_to_string_converter,
            json_to_triangle_point_converter,
            triangle_creator,
        }
    }
}

impl<
        TJsonToStringConverter: ConvertJsonToValue<String>,
        TJsonToTrianglePointConverter: ConvertJsonToValue<TrianglePoint<TwoDPoint, Rgb>>,
        TTriangleCreator: CreateTriangle<TrianglePoint<TwoDPoint, Rgb>>,
    > ConvertJsonToValue<Triangle<TrianglePoint<TwoDPoint, Rgb>>>
    for JsonToTriangleConverter<
        TJsonToStringConverter,
        TJsonToTrianglePointConverter,
        TTriangleCreator,
    >
{
    fn convert_json_to_value(&self, json: &Value) -> Triangle<TrianglePoint<TwoDPoint, Rgb>> {
        let name = self
            .json_to_string_converter
            .convert_json_to_value(&json["name"]);

        let point_1 = self
            .json_to_triangle_point_converter
            .convert_json_to_value(&json["point1"]);

        let point_2 = self
            .json_to_triangle_point_converter
            .convert_json_to_value(&json["point2"]);

        let point_3 = self
            .json_to_triangle_point_converter
            .convert_json_to_value(&json["point3"]);

        self.triangle_creator
            .create_triangle(name, point_1, point_2, point_3)
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
    TRectangleCreator,
> {
    json_to_string_converter: Rc<TJsonToStringConverter>,
    json_to_f32_converter: Rc<TJsonToF32Converter>,
    json_to_rgb_converter: Rc<TJsonToRgbConverter>,
    rectangle_creator: TRectangleCreator,
}

impl<TJsonToStringConverter, TJsonToF32Converter, TJsonToRgbConverter, TRectangleCreator>
    JsonToRectangleConverter<
        TJsonToStringConverter,
        TJsonToF32Converter,
        TJsonToRgbConverter,
        TRectangleCreator,
    >
{
    fn new(
        json_to_string_converter: Rc<TJsonToStringConverter>,
        json_to_f32_converter: Rc<TJsonToF32Converter>,
        json_to_rgb_converter: Rc<TJsonToRgbConverter>,
        rectangle_creator: TRectangleCreator,
    ) -> Self {
        Self {
            json_to_string_converter,
            json_to_f32_converter,
            json_to_rgb_converter,
            rectangle_creator,
        }
    }
}

impl<
        TJsonToStringConverter: ConvertJsonToValue<String>,
        TJsonToF32Converter: ConvertJsonToValue<f32>,
        TJsonToRgbConverter: ConvertJsonToValue<TRgb>,
        TRgb,
        TRectangleCreator: CreateRectangle<TRgb>,
    > ConvertJsonToValue<Rectangle<TRgb>>
    for JsonToRectangleConverter<
        TJsonToStringConverter,
        TJsonToF32Converter,
        TJsonToRgbConverter,
        TRectangleCreator,
    >
{
    fn convert_json_to_value(&self, json: &Value) -> Rectangle<TRgb> {
        let name = self
            .json_to_string_converter
            .convert_json_to_value(&json["name"]);

        let width = self
            .json_to_f32_converter
            .convert_json_to_value(&json["width"]);

        let height = self
            .json_to_f32_converter
            .convert_json_to_value(&json["height"]);

        let rgb = self
            .json_to_rgb_converter
            .convert_json_to_value(&json["rgb"]);

        self.rectangle_creator
            .create_rectangle(name, width, height, rgb)
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
    TTriangleInstanceCreator,
    TTrianglePointCreator,
> {
    json_to_string_converter: Rc<TJsonToStringConverter>,
    json_to_two_d_point_converter: Rc<TJsonToTwoDPointConverter>,
    json_to_triangle_point_converter: Rc<TJsonToTrianglePointConverter>,
    json_to_f32_converter: Rc<TJsonToF32Converter>,
    triangle_instance_creator: TTriangleInstanceCreator,
    triangle_point_creator: TTrianglePointCreator,
}

impl<
        TJsonToStringConverter,
        TJsonToTwoDPointConverter,
        TJsonToTrianglePointConverter,
        TJsonToF32Converter,
        TTriangleInstanceCreator,
        TTrianglePointCreator,
    >
    JsonToTriangleInstanceConverter<
        TJsonToStringConverter,
        TJsonToTwoDPointConverter,
        TJsonToTrianglePointConverter,
        TJsonToF32Converter,
        TTriangleInstanceCreator,
        TTrianglePointCreator,
    >
{
    fn new(
        json_to_string_converter: Rc<TJsonToStringConverter>,
        json_to_two_d_point_converter: Rc<TJsonToTwoDPointConverter>,
        json_to_triangle_point_converter: Rc<TJsonToTrianglePointConverter>,
        json_to_f32_converter: Rc<TJsonToF32Converter>,
        triangle_instance_creator: TTriangleInstanceCreator,
        triangle_point_creator: TTrianglePointCreator,
    ) -> Self {
        Self {
            json_to_string_converter,
            json_to_two_d_point_converter,
            json_to_triangle_point_converter,
            json_to_f32_converter,
            triangle_instance_creator,
            triangle_point_creator,
        }
    }
}

impl<
        TJsonToStringConverter: ConvertJsonToValue<String>,
        TJsonToTwoDPointConverter: ConvertJsonToValue<TwoDPoint>,
        TJsonToTrianglePointConverter: ConvertJsonToValue<TrianglePoint<TwoDPoint, Rgb>>,
        TJsonToF32Converter: ConvertJsonToValue<f32>,
        TTriangleInstanceCreator: CreateTriangleInstance<TwoDPoint, TrianglePoint<TwoDPoint, Rgb>>,
        TTrianglePointCreator: CreateTrianglePoint<TrianglePoint<TwoDPoint, Rgb>>,
    > ConvertJsonToValue<TriangleInstance<TwoDPoint, TrianglePoint<TwoDPoint, Rgb>>>
    for JsonToTriangleInstanceConverter<
        TJsonToStringConverter,
        TJsonToTwoDPointConverter,
        TJsonToTrianglePointConverter,
        TJsonToF32Converter,
        TTriangleInstanceCreator,
        TTrianglePointCreator,
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

        let point_1_translated = self.triangle_point_creator.create_triangle_point(
            point_1.get_x() * scale + position.get_x(),
            point_1.get_y() * scale + position.get_y(),
            point_1.get_rgb().get_r(),
            point_1.get_rgb().get_g(),
            point_1.get_rgb().get_b(),
        );

        let point_2 = self
            .json_to_triangle_point_converter
            .convert_json_to_value(&json["point2"]);

        let point_2_translated = self.triangle_point_creator.create_triangle_point(
            point_2.get_x() * scale + position.get_x(),
            point_2.get_y() * scale + position.get_y(),
            point_2.get_rgb().get_r(),
            point_2.get_rgb().get_g(),
            point_2.get_rgb().get_b(),
        );

        let point_3 = self
            .json_to_triangle_point_converter
            .convert_json_to_value(&json["point3"]);

        let point_3_translated = self.triangle_point_creator.create_triangle_point(
            point_3.get_x() * scale + position.get_x(),
            point_3.get_y() * scale + position.get_y(),
            point_3.get_rgb().get_r(),
            point_3.get_rgb().get_g(),
            point_3.get_rgb().get_b(),
        );

        let name = self
            .json_to_string_converter
            .convert_json_to_value(&json["name"]);

        let content_name = self
            .json_to_string_converter
            .convert_json_to_value(&json["contentName"]);

        let position = self
            .json_to_two_d_point_converter
            .convert_json_to_value(&json["position"]);

        self.triangle_instance_creator.create_triangle_instance(
            name,
            content_name,
            scale,
            position,
            point_1_translated,
            point_2_translated,
            point_3_translated,
        )
    }
}

pub struct JsonToTriangleInstanceRunnerConverter<TJsonToTriangleInstanceConverter> {
    json_to_triangle_instance_converter: TJsonToTriangleInstanceConverter,
}

impl<TJsonToTriangleInstanceConverter>
    JsonToTriangleInstanceRunnerConverter<TJsonToTriangleInstanceConverter>
{
    fn new(json_to_triangle_instance_converter: TJsonToTriangleInstanceConverter) -> Self {
        Self {
            json_to_triangle_instance_converter,
        }
    }
}

impl<
        TJsonToTriangleInstanceConverter: ConvertJsonToValue<TriangleInstance<TwoDPoint, TrianglePoint<TwoDPoint, Rgb>>>,
    >
    ConvertJsonToValue<
        ObjectInstanceRunner<
            TriangleInstance<TwoDPoint, TrianglePoint<TwoDPoint, Rgb>>,
            TriangleInstanceScaler<
                TriangleInstanceCreator,
                TrianglePointCreator<TwoDPointCreator, RgbCreator>,
            >,
        >,
    > for JsonToTriangleInstanceRunnerConverter<TJsonToTriangleInstanceConverter>
{
    fn convert_json_to_value(
        &self,
        json: &Value,
    ) -> ObjectInstanceRunner<
        TriangleInstance<TwoDPoint, TrianglePoint<TwoDPoint, Rgb>>,
        TriangleInstanceScaler<
            TriangleInstanceCreator,
            TrianglePointCreator<TwoDPointCreator, RgbCreator>,
        >,
    > {
        ObjectInstanceRunner::new(
            self.json_to_triangle_instance_converter
                .convert_json_to_value(json),
            TriangleInstanceScaler::new(
                TriangleInstanceCreator::new(),
                TrianglePointCreator::new(TwoDPointCreator::new(), RgbCreator::new()),
            ),
        )
    }
}

pub struct JsonToBoxedTriangleInstanceRunnerConverter<TJsonToTriangleInstanceRunnerConverter> {
    json_to_triangle_instance_runner_converter: TJsonToTriangleInstanceRunnerConverter,
}

impl<TJsonToTriangleInstanceRunnerConverter>
    JsonToBoxedTriangleInstanceRunnerConverter<TJsonToTriangleInstanceRunnerConverter>
{
    fn new(
        json_to_triangle_instance_runner_converter: TJsonToTriangleInstanceRunnerConverter,
    ) -> Self {
        Self {
            json_to_triangle_instance_runner_converter,
        }
    }
}

impl<
        TJsonToTriangleInstanceRunnerConverter: ConvertJsonToValue<
            ObjectInstanceRunner<
                TriangleInstance<TwoDPoint, TrianglePoint<TwoDPoint, Rgb>>,
                TriangleInstanceScaler<
                    TriangleInstanceCreator,
                    TrianglePointCreator<TwoDPointCreator, RgbCreator>,
                >,
            >,
        >,
    > ConvertJsonToValue<Box<dyn RunObjectInstance>>
    for JsonToBoxedTriangleInstanceRunnerConverter<TJsonToTriangleInstanceRunnerConverter>
{
    fn convert_json_to_value(&self, json: &Value) -> Box<dyn RunObjectInstance> {
        Box::new(
            self.json_to_triangle_instance_runner_converter
                .convert_json_to_value(json),
        )
    }
}

pub struct JsonToRectangleInstanceConverter<
    TJsonToStringConverter,
    TJsonToF32Converter,
    TJsonToPositionConverter,
    TJsonToRgbConverter,
    TRectangleInstanceCreator,
> {
    json_to_string_converter: Rc<TJsonToStringConverter>,
    json_to_f32_converter: Rc<TJsonToF32Converter>,
    json_to_position_converter: Rc<TJsonToPositionConverter>,
    json_to_rgb_converter: Rc<TJsonToRgbConverter>,
    rectangle_instance_creator: TRectangleInstanceCreator,
}

impl<
        TJsonToStringConverter,
        TJsonToF32Converter,
        TJsonToPositionConverter,
        TJsonToRgbConverter,
        TRectangleInstanceCreator,
    >
    JsonToRectangleInstanceConverter<
        TJsonToStringConverter,
        TJsonToF32Converter,
        TJsonToPositionConverter,
        TJsonToRgbConverter,
        TRectangleInstanceCreator,
    >
{
    fn new(
        json_to_string_converter: Rc<TJsonToStringConverter>,
        json_to_f32_converter: Rc<TJsonToF32Converter>,
        json_to_position_converter: Rc<TJsonToPositionConverter>,
        json_to_rgb_converter: Rc<TJsonToRgbConverter>,
        rectangle_instance_creator: TRectangleInstanceCreator,
    ) -> Self {
        Self {
            json_to_string_converter,
            json_to_f32_converter,
            json_to_position_converter,
            json_to_rgb_converter,
            rectangle_instance_creator,
        }
    }
}

impl<
        TJsonToStringConverter: ConvertJsonToValue<String>,
        TJsonToF32Converter: ConvertJsonToValue<f32>,
        TJsonToPositionConverter: ConvertJsonToValue<TwoDPoint>,
        TJsonToRgbConverter: ConvertJsonToValue<Rgb>,
        TRectangleInstanceCreator: CreateRectangleInstance<
            TwoDPoint,
            Rgb,
            RectangleInstance<
                TwoDPoint,
                TrianglePoint<TwoDPoint, Rgb>,
                TriangleInstance<TwoDPoint, TrianglePoint<TwoDPoint, Rgb>>,
                Rgb,
            >,
        >,
    >
    ConvertJsonToValue<
        RectangleInstance<
            TwoDPoint,
            TrianglePoint<TwoDPoint, Rgb>,
            TriangleInstance<TwoDPoint, TrianglePoint<TwoDPoint, Rgb>>,
            Rgb,
        >,
    >
    for JsonToRectangleInstanceConverter<
        TJsonToStringConverter,
        TJsonToF32Converter,
        TJsonToPositionConverter,
        TJsonToRgbConverter,
        TRectangleInstanceCreator,
    >
{
    fn convert_json_to_value(
        &self,
        json: &Value,
    ) -> RectangleInstance<
        TwoDPoint,
        TrianglePoint<TwoDPoint, Rgb>,
        TriangleInstance<TwoDPoint, TrianglePoint<TwoDPoint, Rgb>>,
        Rgb,
    > {
        let name = self
            .json_to_string_converter
            .convert_json_to_value(&json["name"]);

        let content_name = self
            .json_to_string_converter
            .convert_json_to_value(&json["contentName"]);

        let scale = self
            .json_to_f32_converter
            .convert_json_to_value(&json["scale"]);

        let position = self
            .json_to_position_converter
            .convert_json_to_value(&json["position"]);

        let width = self
            .json_to_f32_converter
            .convert_json_to_value(&json["width"]);

        let height = self
            .json_to_f32_converter
            .convert_json_to_value(&json["height"]);

        let rgb = self
            .json_to_rgb_converter
            .convert_json_to_value(&json["rgb"]);

        self.rectangle_instance_creator.create_rectangle_instance(
            name,
            content_name,
            scale,
            position,
            width,
            height,
            rgb,
        )
    }
}

pub struct JsonToRectangleInstanceRunnerConverter<TJsonToRectangleInstanceConverter> {
    json_to_rectangle_instance_converter: TJsonToRectangleInstanceConverter,
}

impl<TJsonToRectangleInstanceConverter>
    JsonToRectangleInstanceRunnerConverter<TJsonToRectangleInstanceConverter>
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
                Rgb,
            >,
        >,
    >
    ConvertJsonToValue<
        ObjectInstanceRunner<
            RectangleInstance<
                TwoDPoint,
                TrianglePoint<TwoDPoint, Rgb>,
                TriangleInstance<TwoDPoint, TrianglePoint<TwoDPoint, Rgb>>,
                Rgb,
            >,
            RectangleInstanceScaler<
                RectangleInstanceCreator<
                    TriangleInstanceCreator,
                    TrianglePointCreator<TwoDPointCreator, RgbCreator>,
                >,
            >,
        >,
    > for JsonToRectangleInstanceRunnerConverter<TJsonToRectangleInstanceConverter>
{
    fn convert_json_to_value(
        &self,
        json: &Value,
    ) -> ObjectInstanceRunner<
        RectangleInstance<
            TwoDPoint,
            TrianglePoint<TwoDPoint, Rgb>,
            TriangleInstance<TwoDPoint, TrianglePoint<TwoDPoint, Rgb>>,
            Rgb,
        >,
        RectangleInstanceScaler<
            RectangleInstanceCreator<
                TriangleInstanceCreator,
                TrianglePointCreator<TwoDPointCreator, RgbCreator>,
            >,
        >,
    > {
        ObjectInstanceRunner::new(
            self.json_to_rectangle_instance_converter
                .convert_json_to_value(json),
            RectangleInstanceScaler::new(RectangleInstanceCreator::new(
                TriangleInstanceCreator::new(),
                TrianglePointCreator::new(TwoDPointCreator::new(), RgbCreator::new()),
            )),
        )
    }
}

pub struct JsonToBoxedRectangleInstanceRunnerConverter<TJsonToRectangleInstanceRunnerConverter> {
    json_to_rectangle_instance_runner_converter: TJsonToRectangleInstanceRunnerConverter,
}

impl<TJsonToRectangleInstanceRunnerConverter>
    JsonToBoxedRectangleInstanceRunnerConverter<TJsonToRectangleInstanceRunnerConverter>
{
    fn new(
        json_to_rectangle_instance_runner_converter: TJsonToRectangleInstanceRunnerConverter,
    ) -> Self {
        Self {
            json_to_rectangle_instance_runner_converter,
        }
    }
}

impl<
        TJsonToRectangleInstanceRunnerConverter: ConvertJsonToValue<
            ObjectInstanceRunner<
                RectangleInstance<
                    TwoDPoint,
                    TrianglePoint<TwoDPoint, Rgb>,
                    TriangleInstance<TwoDPoint, TrianglePoint<TwoDPoint, Rgb>>,
                    Rgb,
                >,
                RectangleInstanceScaler<
                    RectangleInstanceCreator<
                        TriangleInstanceCreator,
                        TrianglePointCreator<TwoDPointCreator, RgbCreator>,
                    >,
                >,
            >,
        >,
    > ConvertJsonToValue<Box<dyn RunObjectInstance>>
    for JsonToBoxedRectangleInstanceRunnerConverter<TJsonToRectangleInstanceRunnerConverter>
{
    fn convert_json_to_value(&self, json: &Value) -> Box<dyn RunObjectInstance> {
        Box::new(
            self.json_to_rectangle_instance_runner_converter
                .convert_json_to_value(json),
        )
    }
}

pub struct TypedJsonToValueConverter<TJsonToStringConverter, TBox> {
    json_to_string_converter: Rc<TJsonToStringConverter>,
    converter_map: HashMap<String, Box<dyn ConvertJsonToValue<TBox>>>,
}

impl<TJsonToStringConverter, TBox> TypedJsonToValueConverter<TJsonToStringConverter, TBox> {
    fn new(
        json_to_string_converter: Rc<TJsonToStringConverter>,
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
    json_to_two_d_point_converter: Rc<TJsonToTwoDPointConverter>,
    json_to_rgb_converter: Rc<TJsonToRgbConverter>,
}

impl<TJsonToTwoDPointConverter, TJsonToRgbConverter>
    JsonToTrianglePointConverter<TJsonToTwoDPointConverter, TJsonToRgbConverter>
{
    pub fn new(
        json_to_two_d_point_converter: Rc<TJsonToTwoDPointConverter>,
        json_to_rgb_converter: Rc<TJsonToRgbConverter>,
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
        let two_d_point = self
            .json_to_two_d_point_converter
            .convert_json_to_value(&json["twoDPoint"]);

        let rgb = self
            .json_to_rgb_converter
            .convert_json_to_value(&json["rgb"]);

        let mut vertex_data = vec![];

        vertex_data.append(&mut two_d_point.get_vertex_data());
        vertex_data.append(&mut rgb.get_vertex_data());

        let number_of_vertices =
            two_d_point.get_number_of_vertices() + rgb.get_number_of_vertices();

        TrianglePoint::new(two_d_point, rgb, number_of_vertices, vertex_data)
    }
}

pub struct JsonToTwoDPointConverter<TJsonToF32Converter> {
    json_to_f32_converter: Rc<TJsonToF32Converter>,
}

impl<TJsonToF32Converter> JsonToTwoDPointConverter<TJsonToF32Converter> {
    pub fn new(json_to_f32_converter: Rc<TJsonToF32Converter>) -> Self {
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
    json_to_f32_converter: Rc<TJsonToF32Converter>,
}

impl<TJsonToF32Converter> JsonToRgbConverter<TJsonToF32Converter> {
    pub fn new(json_to_f32_converter: Rc<TJsonToF32Converter>) -> Self {
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

pub fn compose_json_to_content_converter(
    json_to_f32_converter: Rc<JsonToF32Converter>,
    json_to_string_converter: Rc<JsonToStringConverter>,
) -> JsonToContentConverter<
    TypedJsonToValueConverter<JsonToStringConverter, Box<dyn GetName>>,
    TypedJsonToValueConverter<JsonToStringConverter, Box<dyn RunObjectInstance>>,
> {
    let json_to_two_d_point_converter = Rc::new(JsonToTwoDPointConverter::new(Rc::clone(
        &json_to_f32_converter,
    )));

    let json_to_rgb_converter = Rc::new(JsonToRgbConverter::new(Rc::clone(&json_to_f32_converter)));

    let json_to_triangle_point_converter = Rc::new(JsonToTrianglePointConverter::new(
        Rc::clone(&json_to_two_d_point_converter),
        Rc::clone(&json_to_rgb_converter),
    ));

    let triangle_creator = TriangleCreator::new();

    let json_to_triangle_converter = JsonToTriangleConverter::new(
        Rc::clone(&json_to_string_converter),
        Rc::clone(&json_to_triangle_point_converter),
        triangle_creator,
    );

    let json_to_boxed_triangle_converter =
        JsonToBoxedTriangleConverter::new(json_to_triangle_converter);

    let triangle_instance_creator = TriangleInstanceCreator::new();

    let json_to_triangle_instance_converter = JsonToTriangleInstanceConverter::new(
        Rc::clone(&json_to_string_converter),
        Rc::clone(&json_to_two_d_point_converter),
        Rc::clone(&json_to_triangle_point_converter),
        Rc::clone(&json_to_f32_converter),
        triangle_instance_creator,
        TrianglePointCreator::new(TwoDPointCreator::new(), RgbCreator::new()),
    );

    let json_to_triangle_instance_runner_converter =
        JsonToTriangleInstanceRunnerConverter::new(json_to_triangle_instance_converter);

    let json_to_boxed_triangle_instance_runner_converter =
        JsonToBoxedTriangleInstanceRunnerConverter::new(json_to_triangle_instance_runner_converter);

    let rectangle_creator = RectangleCreator::new();

    let json_to_rectangle_converter = JsonToRectangleConverter::new(
        Rc::clone(&json_to_string_converter),
        Rc::clone(&json_to_f32_converter),
        Rc::clone(&json_to_rgb_converter),
        rectangle_creator,
    );

    let json_to_boxed_rectangle_converter =
        JsonToBoxedRectangleConverter::new(json_to_rectangle_converter);

    let rectangle_instance_triangle_instance_creator = TriangleInstanceCreator::new();

    let rectangle_instance_creator = RectangleInstanceCreator::new(
        rectangle_instance_triangle_instance_creator,
        TrianglePointCreator::new(TwoDPointCreator::new(), RgbCreator::new()),
    );

    let json_to_rectangle_instance_converter = JsonToRectangleInstanceConverter::new(
        Rc::clone(&json_to_string_converter),
        Rc::clone(&json_to_f32_converter),
        Rc::clone(&json_to_two_d_point_converter),
        Rc::clone(&json_to_rgb_converter),
        rectangle_instance_creator,
    );

    let json_to_rectangle_instance_runner_converter =
        JsonToRectangleInstanceRunnerConverter::new(json_to_rectangle_instance_converter);

    let json_to_boxed_rectangle_instance_runner_converter =
        JsonToBoxedRectangleInstanceRunnerConverter::new(
            json_to_rectangle_instance_runner_converter,
        );

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
        TypedJsonToValueConverter::new(Rc::clone(&json_to_string_converter), object_converters);

    let mut object_instance_runner_converters =
        HashMap::<String, Box<dyn ConvertJsonToValue<Box<dyn RunObjectInstance>>>>::new();
    object_instance_runner_converters.insert(
        "triangle".to_string(),
        Box::new(json_to_boxed_triangle_instance_runner_converter),
    );
    object_instance_runner_converters.insert(
        "rectangle".to_string(),
        Box::new(json_to_boxed_rectangle_instance_runner_converter),
    );

    let json_to_object_instance_runner_converter = TypedJsonToValueConverter::new(
        Rc::clone(&json_to_string_converter),
        object_instance_runner_converters,
    );

    let json_to_content_converter = JsonToContentConverter::new(
        json_to_object_converter,
        json_to_object_instance_runner_converter,
    );

    json_to_content_converter
}

pub fn compose_content_loader(
    json_to_f32_converter: Rc<JsonToF32Converter>,
    json_to_string_converter: Rc<JsonToStringConverter>,
) -> ContentLoader<
    JsonToContentConverter<
        TypedJsonToValueConverter<JsonToStringConverter, Box<dyn GetName>>,
        TypedJsonToValueConverter<JsonToStringConverter, Box<dyn RunObjectInstance>>,
    >,
> {
    let json_to_content_converter =
        compose_json_to_content_converter(json_to_f32_converter, json_to_string_converter);

    ContentLoader::<
        JsonToContentConverter<
            TypedJsonToValueConverter<JsonToStringConverter, Box<dyn GetName>>,
            TypedJsonToValueConverter<JsonToStringConverter, Box<dyn RunObjectInstance>>,
        >,
    >::new(json_to_content_converter)
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use garden_content::{
        rectangles::{Rectangle, RectangleInstance},
        triangles::{Triangle, TriangleInstance},
        Content, GetVertexData, Rgb, TrianglePoint, TwoDPoint,
    };
    use garden_json::{ConvertJsonToValue, JsonToF32Converter, JsonToStringConverter};
    use serde_json::json;

    use crate::compose_json_to_content_converter;

    #[test]
    fn when_a_json_to_content_converter_converts_json_to_content_then_the_content_is_converted() {
        let json_to_content_converter = compose_json_to_content_converter(
            Rc::new(JsonToF32Converter::new()),
            Rc::new(JsonToStringConverter::new()),
        );

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
                    },
                    "rgb": {
                        "r": 0.0,
                        "g": 0.0,
                        "b": 1.0
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
                    },
                    "rgb": {
                        "r": 1.0,
                        "g": 0.0,
                        "b": 0.0
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
                    vec![],
                    15,
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
                        15,
                        vec![
                            -5.5, -5.5, 1.0, 0.0, 0.0, -5.0, -4.5, 0.0, 1.0, 0.0, -4.5, -5.5, 0.0,
                            0.0, 1.0,
                        ],
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
                        15,
                        vec![
                            2.0, 2.0, 1.0, 0.0, 0.0, 5.0, 8.0, 0.0, 1.0, 0.0, 8.0, 2.0, 0.0, 0.0,
                            1.0,
                        ],
                    ),
                ),
                Box::new(RectangleInstance::new(
                    "Rectangle1-a".to_string(),
                    "Rectangle1".to_string(),
                    1.0,
                    TwoDPoint::new(-5.0, 5.0),
                    2.5,
                    5.0,
                    Rgb::new(0.0, 0.0, 1.0),
                    TrianglePoint::new(TwoDPoint::new(0.0, 0.0), Rgb::new(1.0, 0.0, 0.0)),
                    TrianglePoint::new(TwoDPoint::new(0.0, 0.0), Rgb::new(1.0, 0.0, 0.0)),
                    TrianglePoint::new(TwoDPoint::new(0.0, 0.0), Rgb::new(1.0, 0.0, 0.0)),
                    TrianglePoint::new(TwoDPoint::new(0.0, 0.0), Rgb::new(1.0, 0.0, 0.0)),
                    30,
                    vec![
                        -3.75, 7.5, 0.0, 0.0, 1.0, -6.25, 7.5, 0.0, 0.0, 1.0, -6.25, 2.5, 0.0, 0.0,
                        1.0, -3.75, 7.5, 0.0, 0.0, 1.0, -6.25, 2.5, 0.0, 0.0, 1.0, -3.75, 2.5, 0.0,
                        0.0, 1.0,
                    ],
                    TriangleInstance::new(
                        "".to_string(),
                        "".to_string(),
                        0.0,
                        TwoDPoint::new(0.0, 0.0),
                        TrianglePoint::new(TwoDPoint::new(0.0, 0.0), Rgb::new(1.0, 0.0, 0.0)),
                        TrianglePoint::new(TwoDPoint::new(0.0, 0.0), Rgb::new(1.0, 0.0, 0.0)),
                        TrianglePoint::new(TwoDPoint::new(0.0, 0.0), Rgb::new(1.0, 0.0, 0.0)),
                        15,
                        vec![],
                    ),
                    TriangleInstance::new(
                        "".to_string(),
                        "".to_string(),
                        0.0,
                        TwoDPoint::new(0.0, 0.0),
                        TrianglePoint::new(TwoDPoint::new(0.0, 0.0), Rgb::new(1.0, 0.0, 0.0)),
                        TrianglePoint::new(TwoDPoint::new(0.0, 0.0), Rgb::new(1.0, 0.0, 0.0)),
                        TrianglePoint::new(TwoDPoint::new(0.0, 0.0), Rgb::new(1.0, 0.0, 0.0)),
                        15,
                        vec![],
                    ),
                )),
                Box::new(RectangleInstance::new(
                    "Rectangle2-a".to_string(),
                    "Rectangle2".to_string(),
                    1.0,
                    TwoDPoint::new(5.0, -5.0),
                    3.0,
                    2.0,
                    Rgb::new(1.0, 0.0, 0.0),
                    TrianglePoint::new(TwoDPoint::new(0.0, 0.0), Rgb::new(1.0, 0.0, 0.0)),
                    TrianglePoint::new(TwoDPoint::new(0.0, 0.0), Rgb::new(1.0, 0.0, 0.0)),
                    TrianglePoint::new(TwoDPoint::new(0.0, 0.0), Rgb::new(1.0, 0.0, 0.0)),
                    TrianglePoint::new(TwoDPoint::new(0.0, 0.0), Rgb::new(1.0, 0.0, 0.0)),
                    30,
                    vec![
                        6.5, -4.0, 1.0, 0.0, 0.0, 3.5, -4.0, 1.0, 0.0, 0.0, 3.5, -6.0, 1.0, 0.0,
                        0.0, 6.5, -4.0, 1.0, 0.0, 0.0, 3.5, -6.0, 1.0, 0.0, 0.0, 6.5, -6.0, 1.0,
                        0.0, 0.0,
                    ],
                    TriangleInstance::new(
                        "".to_string(),
                        "".to_string(),
                        0.0,
                        TwoDPoint::new(0.0, 0.0),
                        TrianglePoint::new(TwoDPoint::new(0.0, 0.0), Rgb::new(1.0, 0.0, 0.0)),
                        TrianglePoint::new(TwoDPoint::new(0.0, 0.0), Rgb::new(1.0, 0.0, 0.0)),
                        TrianglePoint::new(TwoDPoint::new(0.0, 0.0), Rgb::new(1.0, 0.0, 0.0)),
                        15,
                        vec![],
                    ),
                    TriangleInstance::new(
                        "".to_string(),
                        "".to_string(),
                        0.0,
                        TwoDPoint::new(0.0, 0.0),
                        TrianglePoint::new(TwoDPoint::new(0.0, 0.0), Rgb::new(1.0, 0.0, 0.0)),
                        TrianglePoint::new(TwoDPoint::new(0.0, 0.0), Rgb::new(1.0, 0.0, 0.0)),
                        TrianglePoint::new(TwoDPoint::new(0.0, 0.0), Rgb::new(1.0, 0.0, 0.0)),
                        15,
                        vec![],
                    ),
                )),
            ],
        );

        let result = json_to_content_converter.convert_json_to_value(&json);

        assert_eq!(expected_result.get_vertex_data(), result.get_vertex_data());
    }
}
