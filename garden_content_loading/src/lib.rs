use garden::{GetHeight, GetName, GetWidth};
use garden_content::{
    rectangles::{
        ContentProvider, Rectangle, RectangleConstructor, RectangleInstanceConstructor,
        RectangleInstanceParameters, RectangleInstanceScaler, RectangleParameters,
    },
    triangles::{
        CalculateTriangleInstancePoint, GeometryTriangleConstructor, GeometryTrianglesCreator,
        GetTrianglePoints, Triangle, TriangleConstructor, TriangleInstanceConstructor,
        TriangleInstanceParameters, TriangleInstancePointCalculator, TriangleInstancePointCreator,
        TriangleInstanceScaler, TriangleInstanceVertexCounter, TriangleInstanceVertexDataGenerator,
        TriangleParameters,
    },
    Content, CreateObject, CreateRgb, CreateTrianglePoint, CreateTwoDPoint, Get2DCoordiantes,
    GetContent, GetNumberOfVertices, GetRgbValues, GetTrianglePointProperties, GetVertexData,
    ObjectCreator, ObjectInstanceRunner, Rgb, RgbCreator, RunObjectInstance, Store, TrianglePoint,
    TrianglePointConstructor, TrianglePointCreator, TwoDPoint, TwoDPointCreator,
    TwoDPointTranslator,
};
use garden_json::{ConvertJsonToValue, JsonToF32Converter, JsonToStringConverter};
use garden_loading::Load;
use serde_json::Value;
use std::{cell::RefCell, collections::HashMap, fs, marker::PhantomData, rc::Rc};

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

impl<'a, TJsonToContentConverter: ConvertJsonToValue<TContent>, TContent> Load<TContent>
    for ContentLoader<TJsonToContentConverter>
{
    fn load(self) -> TContent {
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
        TJsonToObjectConverter: ConvertJsonToValue<Box<Rc<RefCell<dyn GetName>>>>,
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
        TJsonToObjectConverter: ConvertJsonToValue<Box<Rc<RefCell<dyn GetName>>>>,
        TJsonToObjectInstanceRunnerConverter: ConvertJsonToValue<Box<dyn RunObjectInstance>>,
    > ConvertJsonToValue<Content>
    for JsonToContentConverter<TJsonToObjectConverter, TJsonToObjectInstanceRunnerConverter>
{
    fn convert_json_to_value(&self, json: &Value) -> Content {
        let mut objects = Vec::<Box<Rc<RefCell<dyn GetName>>>>::new();

        if let Some(object_json_array) = json["content"]["objects"].as_array() {
            for object_json in object_json_array {
                objects.push(
                    self.json_to_object_converter
                        .convert_json_to_value(object_json),
                );
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
    TTrianglePoint,
    TTriangle,
> {
    json_to_string_converter: Rc<TJsonToStringConverter>,
    json_to_triangle_point_converter: Rc<TJsonToTrianglePointConverter>,
    triangle_creator: TTriangleCreator,
    triangle_point_type: PhantomData<TTrianglePoint>,
    triangle_type: PhantomData<TTriangle>,
}

impl<
        TJsonToStringConverter,
        TJsonToTrianglePointConverter,
        TTriangleCreator,
        TTrianglePoint,
        TTriangle,
    >
    JsonToTriangleConverter<
        TJsonToStringConverter,
        TJsonToTrianglePointConverter,
        TTriangleCreator,
        TTrianglePoint,
        TTriangle,
    >
{
    pub fn new(
        json_to_string_converter: Rc<TJsonToStringConverter>,
        json_to_triangle_point_converter: Rc<TJsonToTrianglePointConverter>,
        triangle_creator: TTriangleCreator,
    ) -> Self {
        Self {
            json_to_string_converter: json_to_string_converter,
            json_to_triangle_point_converter: json_to_triangle_point_converter,
            triangle_creator: triangle_creator,
            triangle_point_type: PhantomData,
            triangle_type: PhantomData,
        }
    }
}

impl<
        TJsonToStringConverter: ConvertJsonToValue<String>,
        TJsonToTrianglePointConverter: ConvertJsonToValue<TTrianglePoint>,
        TTriangleCreator: CreateObject<TTriangle, TriangleParameters<TTrianglePoint>>,
        TTrianglePoint,
        TTriangle,
    > ConvertJsonToValue<Rc<RefCell<TTriangle>>>
    for JsonToTriangleConverter<
        TJsonToStringConverter,
        TJsonToTrianglePointConverter,
        TTriangleCreator,
        TTrianglePoint,
        TTriangle,
    >
{
    fn convert_json_to_value(&self, json: &Value) -> Rc<RefCell<TTriangle>> {
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

        let parameters = TriangleParameters::new(name, point_1, point_2, point_3);

        self.triangle_creator.create_object(parameters)
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

impl<
        TJsonToTriangleConverter: ConvertJsonToValue<Rc<RefCell<Triangle<TrianglePoint<TwoDPoint, Rgb>>>>>,
    > ConvertJsonToValue<Box<Rc<RefCell<dyn GetName>>>>
    for JsonToBoxedTriangleConverter<TJsonToTriangleConverter>
{
    fn convert_json_to_value(&self, json: &Value) -> Box<Rc<RefCell<dyn GetName>>> {
        Box::new(self.json_to_triangle_converter.convert_json_to_value(json))
    }
}

pub struct JsonToRectangleConverter<
    TJsonToStringConverter,
    TJsonToF32Converter,
    TJsonToRgbConverter,
    TRgb,
    TRectangleCreator,
> {
    json_to_string_converter: Rc<TJsonToStringConverter>,
    json_to_f32_converter: Rc<TJsonToF32Converter>,
    json_to_rgb_converter: Rc<TJsonToRgbConverter>,
    rgb_type: PhantomData<TRgb>,
    rectangle_creator: Rc<TRectangleCreator>,
}

impl<TJsonToStringConverter, TJsonToF32Converter, TJsonToRgbConverter, TRgb, TRectangleCreator>
    JsonToRectangleConverter<
        TJsonToStringConverter,
        TJsonToF32Converter,
        TJsonToRgbConverter,
        TRgb,
        TRectangleCreator,
    >
{
    fn new(
        json_to_string_converter: Rc<TJsonToStringConverter>,
        json_to_f32_converter: Rc<TJsonToF32Converter>,
        json_to_rgb_converter: Rc<TJsonToRgbConverter>,
        rectangle_creator: Rc<TRectangleCreator>,
    ) -> Self {
        Self {
            json_to_string_converter: json_to_string_converter,
            json_to_f32_converter: json_to_f32_converter,
            json_to_rgb_converter: json_to_rgb_converter,
            rgb_type: PhantomData,
            rectangle_creator: rectangle_creator,
        }
    }
}

impl<
        TJsonToStringConverter: ConvertJsonToValue<String>,
        TJsonToF32Converter: ConvertJsonToValue<f32>,
        TJsonToRgbConverter: ConvertJsonToValue<TRgb>,
        TRgb,
        TRectangleCreator: CreateObject<TRectangle, RectangleParameters<TRgb>>,
        TRectangle,
    > ConvertJsonToValue<Rc<RefCell<TRectangle>>>
    for JsonToRectangleConverter<
        TJsonToStringConverter,
        TJsonToF32Converter,
        TJsonToRgbConverter,
        TRgb,
        TRectangleCreator,
    >
{
    fn convert_json_to_value(&self, json: &Value) -> Rc<RefCell<TRectangle>> {
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

        let parameters = RectangleParameters::new(name, width, height, rgb);

        self.rectangle_creator.create_object(parameters)
    }
}

pub struct JsonToBoxedRectangleConverter<TJsonToRectangleConverter, TRectangle> {
    json_to_rectangle_converter: TJsonToRectangleConverter,
    rectangle_type: PhantomData<TRectangle>,
}

impl<TJsonToRectangleConverter, TRectangle>
    JsonToBoxedRectangleConverter<TJsonToRectangleConverter, TRectangle>
{
    fn new(json_to_rectangle_converter: TJsonToRectangleConverter) -> Self {
        Self {
            json_to_rectangle_converter: json_to_rectangle_converter,
            rectangle_type: PhantomData,
        }
    }
}

impl<
        TJsonToRectangleConverter: ConvertJsonToValue<Rc<RefCell<TRectangle>>>,
        TRectangle: GetName + 'static,
    > ConvertJsonToValue<Box<Rc<RefCell<dyn GetName>>>>
    for JsonToBoxedRectangleConverter<TJsonToRectangleConverter, TRectangle>
{
    fn convert_json_to_value(&self, json: &Value) -> Box<Rc<RefCell<dyn GetName>>> {
        Box::new(self.json_to_rectangle_converter.convert_json_to_value(json))
    }
}

pub struct JsonToTriangleInstanceConverter<
    TJsonToStringConverter,
    TJsonToTwoDPointConverter,
    TJsonToF32Converter,
    TTriangleInstanceCreator,
    TTriangleProvider,
    TTriangle,
    TTwoDPoint,
    TTrianglePoint,
    TTriangleInstancePointCalculator,
> {
    json_to_string_converter: Rc<TJsonToStringConverter>,
    json_to_two_d_point_converter: Rc<TJsonToTwoDPointConverter>,
    json_to_f32_converter: Rc<TJsonToF32Converter>,
    triangle_instance_creator: Rc<TTriangleInstanceCreator>,
    triangle_provider: Rc<RefCell<TTriangleProvider>>,
    triangle_type: PhantomData<TTriangle>,
    two_d_point_type: PhantomData<TTwoDPoint>,
    triangle_point_type: PhantomData<TTrianglePoint>,
    triangle_instance_point_calculator: Rc<TTriangleInstancePointCalculator>,
}

impl<
        TJsonToStringConverter,
        TJsonToTwoDPointConverter,
        TJsonToF32Converter,
        TTriangleInstanceCreator,
        TTriangleProvider,
        TTriangle,
        TTwoDPoint,
        TTrianglePoint,
        TTriangleInstancePointCalculator,
    >
    JsonToTriangleInstanceConverter<
        TJsonToStringConverter,
        TJsonToTwoDPointConverter,
        TJsonToF32Converter,
        TTriangleInstanceCreator,
        TTriangleProvider,
        TTriangle,
        TTwoDPoint,
        TTrianglePoint,
        TTriangleInstancePointCalculator,
    >
{
    fn new(
        json_to_string_converter: Rc<TJsonToStringConverter>,
        json_to_two_d_point_converter: Rc<TJsonToTwoDPointConverter>,
        json_to_f32_converter: Rc<TJsonToF32Converter>,
        triangle_instance_creator: Rc<TTriangleInstanceCreator>,
        triangle_provider: Rc<RefCell<TTriangleProvider>>,
        triangle_instance_point_calculator: Rc<TTriangleInstancePointCalculator>,
    ) -> Self {
        Self {
            json_to_string_converter: json_to_string_converter,
            json_to_two_d_point_converter: json_to_two_d_point_converter,
            json_to_f32_converter: json_to_f32_converter,
            triangle_instance_creator: triangle_instance_creator,
            triangle_provider: triangle_provider,
            triangle_type: PhantomData,
            two_d_point_type: PhantomData,
            triangle_point_type: PhantomData,
            triangle_instance_point_calculator: triangle_instance_point_calculator,
        }
    }
}

impl<
        TJsonToStringConverter: ConvertJsonToValue<String>,
        TJsonToTwoDPointConverter: ConvertJsonToValue<TTwoDPoint>,
        TJsonToF32Converter: ConvertJsonToValue<f32>,
        TTriangleInstanceCreator: CreateObject<
            TTriangleInstance,
            TriangleInstanceParameters<TTriangle, TTwoDPoint, TTrianglePoint>,
        >,
        TTriangleInstance,
        TTriangleProvider: GetContent<TTriangle>,
        TTriangle: GetTrianglePoints<TTrianglePoint>,
        TTwoDPoint: Get2DCoordiantes,
        TTrianglePoint: GetRgbValues + Get2DCoordiantes,
        TTriangleInstancePointCalculator: CalculateTriangleInstancePoint<TTrianglePoint, TTwoDPoint>,
    > ConvertJsonToValue<Rc<RefCell<TTriangleInstance>>>
    for JsonToTriangleInstanceConverter<
        TJsonToStringConverter,
        TJsonToTwoDPointConverter,
        TJsonToF32Converter,
        TTriangleInstanceCreator,
        TTriangleProvider,
        TTriangle,
        TTwoDPoint,
        TTrianglePoint,
        TTriangleInstancePointCalculator,
    >
{
    fn convert_json_to_value(&self, json: &Value) -> Rc<RefCell<TTriangleInstance>> {
        let content_name = self
            .json_to_string_converter
            .convert_json_to_value(&json["contentName"]);

        let triangle = self
            .triangle_provider
            .borrow_mut()
            .get_content(content_name);

        let scale = self
            .json_to_f32_converter
            .convert_json_to_value(&json["scale"]);

        let position = self
            .json_to_two_d_point_converter
            .convert_json_to_value(&json["position"]);

        let point_1 = self
            .triangle_instance_point_calculator
            .calculate_triangle_instance_point(
                triangle.borrow_mut().get_point_1(),
                &position,
                scale,
            );

        let point_2 = self
            .triangle_instance_point_calculator
            .calculate_triangle_instance_point(
                triangle.borrow_mut().get_point_2(),
                &position,
                scale,
            );

        let point_3 = self
            .triangle_instance_point_calculator
            .calculate_triangle_instance_point(
                triangle.borrow_mut().get_point_3(),
                &position,
                scale,
            );

        let name = self
            .json_to_string_converter
            .convert_json_to_value(&json["name"]);

        let position = self
            .json_to_two_d_point_converter
            .convert_json_to_value(&json["position"]);

        self.triangle_instance_creator
            .create_object(TriangleInstanceParameters::new(
                name, triangle, scale, position, point_1, point_2, point_3,
            ))
    }
}

pub struct JsonToBoxedObjectInstanceRunnerConverter<
    TJsonToObjectInstanceRunnerConverter,
    TObjectInstanceRunner,
> {
    json_to_object_instance_runner_converter: TJsonToObjectInstanceRunnerConverter,
    phantom_data: PhantomData<TObjectInstanceRunner>,
}

impl<TJsonToObjectInstanceRunnerConverter, TObjectInstanceRunner>
    JsonToBoxedObjectInstanceRunnerConverter<
        TJsonToObjectInstanceRunnerConverter,
        TObjectInstanceRunner,
    >
{
    fn new(json_to_object_instance_runner_converter: TJsonToObjectInstanceRunnerConverter) -> Self {
        Self {
            json_to_object_instance_runner_converter: json_to_object_instance_runner_converter,
            phantom_data: PhantomData,
        }
    }
}

impl<
        TJsonToObjectInstanceRunnerConverter: ConvertJsonToValue<TObjectInstanceRunner>,
        TObjectInstanceRunner: RunObjectInstance + 'static,
    > ConvertJsonToValue<Box<dyn RunObjectInstance>>
    for JsonToBoxedObjectInstanceRunnerConverter<
        TJsonToObjectInstanceRunnerConverter,
        TObjectInstanceRunner,
    >
{
    fn convert_json_to_value(&self, json: &Value) -> Box<dyn RunObjectInstance> {
        Box::new(
            self.json_to_object_instance_runner_converter
                .convert_json_to_value(json),
        )
    }
}

pub struct JsonToRectangleInstanceConverter<
    TJsonToStringConverter,
    TJsonToF32Converter,
    TJsonToPositionConverter,
    TRectangleInstanceCreator,
    TRectangleProvider,
    TTwoDPoint,
    TRectangle,
> {
    json_to_string_converter: Rc<TJsonToStringConverter>,
    json_to_f32_converter: Rc<TJsonToF32Converter>,
    json_to_position_converter: Rc<TJsonToPositionConverter>,
    rectangle_instance_creator: Rc<TRectangleInstanceCreator>,
    rectangle_provider: Rc<RefCell<TRectangleProvider>>,
    two_d_point_type: PhantomData<TTwoDPoint>,
    rectangle_type: PhantomData<TRectangle>,
}

impl<
        'r,
        TJsonToStringConverter,
        TJsonToF32Converter,
        TJsonToPositionConverter,
        TRectangleInstanceCreator,
        TRectangleProvider,
        TTwoDPoint,
        TRectangle,
    >
    JsonToRectangleInstanceConverter<
        TJsonToStringConverter,
        TJsonToF32Converter,
        TJsonToPositionConverter,
        TRectangleInstanceCreator,
        TRectangleProvider,
        TTwoDPoint,
        TRectangle,
    >
{
    fn new(
        json_to_string_converter: Rc<TJsonToStringConverter>,
        json_to_f32_converter: Rc<TJsonToF32Converter>,
        json_to_position_converter: Rc<TJsonToPositionConverter>,
        rectangle_instance_creator: Rc<TRectangleInstanceCreator>,
        rectangle_provider: Rc<RefCell<TRectangleProvider>>,
    ) -> Self {
        Self {
            json_to_string_converter: json_to_string_converter,
            json_to_f32_converter: json_to_f32_converter,
            json_to_position_converter: json_to_position_converter,
            rectangle_instance_creator: rectangle_instance_creator,
            rectangle_provider: rectangle_provider,
            two_d_point_type: PhantomData,
            rectangle_type: PhantomData,
        }
    }
}

impl<
        TJsonToStringConverter: ConvertJsonToValue<String>,
        TJsonToF32Converter: ConvertJsonToValue<f32>,
        TJsonToPositionConverter: ConvertJsonToValue<TTwoDPoint>,
        TRectangleInstanceCreator: CreateObject<TRectangleInstance, RectangleInstanceParameters<TRectangle, TTwoDPoint>>,
        TRectangleProvider: GetContent<TRectangle>,
        TTwoDPoint,
        TRectangleInstance,
        TRectangle: GetWidth + GetHeight,
    > ConvertJsonToValue<Rc<RefCell<TRectangleInstance>>>
    for JsonToRectangleInstanceConverter<
        TJsonToStringConverter,
        TJsonToF32Converter,
        TJsonToPositionConverter,
        TRectangleInstanceCreator,
        TRectangleProvider,
        TTwoDPoint,
        TRectangle,
    >
{
    fn convert_json_to_value(&self, json: &Value) -> Rc<RefCell<TRectangleInstance>> {
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

        let rectangle = self
            .rectangle_provider
            .borrow_mut()
            .get_content(content_name);

        let width = rectangle.borrow().get_width();
        let height = rectangle.borrow().get_height();

        self.rectangle_instance_creator
            .create_object(RectangleInstanceParameters::new(
                name, rectangle, scale, position, width, height, /*rgb*/
            ))
    }
}

pub struct JsonToObjectInstanceRunnerConverter<
    TJsonToObjectInstanceConverter,
    TObjectInstanceScaler,
> {
    json_to_object_instance_converter: TJsonToObjectInstanceConverter,
    object_instance_scaler: Rc<TObjectInstanceScaler>,
}

impl<TJsonToObjectInstanceConverter, TObjectInstanceScaler>
    JsonToObjectInstanceRunnerConverter<TJsonToObjectInstanceConverter, TObjectInstanceScaler>
{
    fn new(
        json_to_object_instance_converter: TJsonToObjectInstanceConverter,
        object_instance_scaler: Rc<TObjectInstanceScaler>,
    ) -> Self {
        Self {
            json_to_object_instance_converter,
            object_instance_scaler,
        }
    }
}

impl<
        TJsonToObjectInstanceConverter: ConvertJsonToValue<Rc<RefCell<TObjectInstance>>>,
        TObjectInstanceScaler,
        TObjectInstance,
    > ConvertJsonToValue<ObjectInstanceRunner<TObjectInstance, TObjectInstanceScaler>>
    for JsonToObjectInstanceRunnerConverter<TJsonToObjectInstanceConverter, TObjectInstanceScaler>
{
    fn convert_json_to_value(
        &self,
        json: &Value,
    ) -> ObjectInstanceRunner<TObjectInstance, TObjectInstanceScaler> {
        ObjectInstanceRunner::new(
            self.json_to_object_instance_converter
                .convert_json_to_value(json),
            Rc::clone(&self.object_instance_scaler),
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
        TJsonToTwoDPointConverter: ConvertJsonToValue<TTwoDPoint>,
        TJsonToRgbConverter: ConvertJsonToValue<TRgb>,
        TTwoDPoint: GetVertexData + GetNumberOfVertices,
        TRgb: GetVertexData + GetNumberOfVertices,
    > ConvertJsonToValue<TrianglePoint<TTwoDPoint, TRgb>>
    for JsonToTrianglePointConverter<TJsonToTwoDPointConverter, TJsonToRgbConverter>
{
    fn convert_json_to_value(&self, json: &Value) -> TrianglePoint<TTwoDPoint, TRgb> {
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

pub fn compose_rectangles<
    TJsonToStringConverter: ConvertJsonToValue<String> + 'static,
    TJsonToF32Converter: ConvertJsonToValue<f32> + 'static,
    TJsontoRgbConverter: ConvertJsonToValue<Rgb> + 'static,
    TJsonToTwoDPointConverter: ConvertJsonToValue<TTwoDPoint> + 'static,
    TTwoDPointCreator: CreateTwoDPoint<TTwoDPoint> + 'static,
    TTrianglePointCreator: CreateTrianglePoint<TTrianglePoint> + 'static,
    TTwoDPoint: Get2DCoordiantes + 'static,
    TTrianglePoint: GetTrianglePointProperties + GetVertexData + GetNumberOfVertices + 'static,
>(
    object_converters: &mut HashMap<
        String,
        Box<dyn ConvertJsonToValue<Box<Rc<RefCell<dyn GetName>>>>>,
    >,
    object_instance_runner_converters: &mut HashMap<
        String,
        Box<dyn ConvertJsonToValue<Box<dyn RunObjectInstance>>>,
    >,
    json_to_string_converter: Rc<TJsonToStringConverter>,
    json_to_f32_converter: Rc<TJsonToF32Converter>,
    json_to_rgb_converter: Rc<TJsontoRgbConverter>,
    json_to_two_d_point_converter: Rc<TJsonToTwoDPointConverter>,
    two_d_point_creator: Rc<TTwoDPointCreator>,
    triangle_point_creator: Rc<TTrianglePointCreator>,
) {
    let rectangle_provider = ContentProvider::<Rectangle<Rgb>>::new(vec![]);

    let rectangle_provider_ref_cell = Rc::new(RefCell::new(rectangle_provider));

    let rectangle_constructor = Rc::new(RectangleConstructor::new());

    let rectangle_creator = Rc::new(ObjectCreator::new(
        Rc::clone(&rectangle_constructor),
        Rc::clone(&rectangle_provider_ref_cell),
    ));

    let json_to_rectangle_converter = JsonToRectangleConverter::new(
        Rc::clone(&json_to_string_converter),
        Rc::clone(&json_to_f32_converter),
        Rc::clone(&json_to_rgb_converter),
        Rc::clone(&rectangle_creator),
    );

    let json_to_boxed_rectangle_converter =
        JsonToBoxedRectangleConverter::new(json_to_rectangle_converter);

    let geometry_triangle_constructor = Rc::new(GeometryTriangleConstructor::new());

    let geometry_triangles_creator = Rc::new(GeometryTrianglesCreator::new(
        Rc::clone(&geometry_triangle_constructor),
        Rc::clone(&triangle_point_creator),
    ));

    let rectangle_instance_constructor = Rc::new(RectangleInstanceConstructor::new(Rc::clone(
        &geometry_triangles_creator,
    )));

    let rectangle_instance_store = Rc::new(RefCell::new(Store::new(vec![])));

    let rectangle_instance_creator = Rc::new(ObjectCreator::new(
        Rc::clone(&rectangle_instance_constructor),
        Rc::clone(&rectangle_instance_store),
    ));

    let json_to_rectangle_instance_converter = JsonToRectangleInstanceConverter::new(
        Rc::clone(&json_to_string_converter),
        Rc::clone(&json_to_f32_converter),
        Rc::clone(&json_to_two_d_point_converter),
        Rc::clone(&rectangle_instance_creator),
        Rc::clone(&rectangle_provider_ref_cell),
    );

    let rectangle_instance_scaler = Rc::new(RectangleInstanceScaler::new(
        Rc::clone(&rectangle_instance_creator),
        Rc::clone(&two_d_point_creator),
    ));

    let json_to_rectangle_instance_runner_converter = JsonToObjectInstanceRunnerConverter::new(
        json_to_rectangle_instance_converter,
        Rc::clone(&rectangle_instance_scaler),
    );

    let json_to_boxed_rectangle_instance_runner_converter =
        JsonToBoxedObjectInstanceRunnerConverter::new(json_to_rectangle_instance_runner_converter);

    let b = Box::new(json_to_boxed_rectangle_converter);
    object_converters.insert("rectangle".to_string(), b);

    object_instance_runner_converters.insert(
        "rectangle".to_string(),
        Box::new(json_to_boxed_rectangle_instance_runner_converter),
    );
}

pub fn compose_triangles<
    TJsonToStringConverter: ConvertJsonToValue<String> + 'static,
    TJsonToTrianglePointConverter: ConvertJsonToValue<TrianglePoint<TwoDPoint, Rgb>> + 'static,
    TTwoDPointCreator: CreateTwoDPoint<TwoDPoint> + 'static,
    TRgbCreator: CreateRgb<Rgb> + 'static,
    TJsonToTwoDPointConverter: ConvertJsonToValue<TwoDPoint> + 'static,
    TJsonToF32Converter: ConvertJsonToValue<f32> + 'static,
>(
    object_converters: &mut HashMap<
        String,
        Box<dyn ConvertJsonToValue<Box<Rc<RefCell<dyn GetName>>>>>,
    >,
    object_instance_runner_converters: &mut HashMap<
        String,
        Box<dyn ConvertJsonToValue<Box<dyn RunObjectInstance>>>,
    >,
    json_to_string_converter: Rc<TJsonToStringConverter>,
    json_to_triangle_point_converter: Rc<TJsonToTrianglePointConverter>,
    two_d_point_creator: Rc<TTwoDPointCreator>,
    rgb_creator: Rc<TRgbCreator>,
    json_to_two_d_point_converter: Rc<TJsonToTwoDPointConverter>,
    json_to_f32_converter: Rc<TJsonToF32Converter>,
) {
    let triangle_provider = ContentProvider::<Triangle<TrianglePoint<TwoDPoint, Rgb>>>::new(vec![]);

    let triangle_provider_ref_cell = Rc::new(RefCell::new(triangle_provider));

    let triangle_constructor = Rc::new(TriangleConstructor::new());

    let triangle_creator = ObjectCreator::new(
        Rc::clone(&triangle_constructor),
        Rc::clone(&triangle_provider_ref_cell),
    );

    let json_to_triangle_converter = JsonToTriangleConverter::new(
        Rc::clone(&json_to_string_converter),
        Rc::clone(&json_to_triangle_point_converter),
        triangle_creator,
    );

    let json_to_boxed_triangle_converter =
        JsonToBoxedTriangleConverter::new(json_to_triangle_converter);

    let triangle_instance_vertex_data_generator =
        Rc::new(TriangleInstanceVertexDataGenerator::new());

    let triangle_instance_vertex_counter = Rc::new(TriangleInstanceVertexCounter::new());

    let triangle_instance_constructor = Rc::new(TriangleInstanceConstructor::new(
        Rc::clone(&triangle_instance_vertex_data_generator),
        Rc::clone(&triangle_instance_vertex_counter),
    ));

    let triangle_instance_store = Rc::new(RefCell::new(Store::new(vec![])));

    let triangle_instance_creator = Rc::new(ObjectCreator::new(
        Rc::clone(&triangle_instance_constructor),
        Rc::clone(&triangle_instance_store),
    ));

    let triangle_point_constructor = Rc::new(TrianglePointConstructor::new());

    let triangle_point_creator = Rc::new(TrianglePointCreator::new(
        Rc::clone(&two_d_point_creator),
        Rc::clone(&rgb_creator),
        Rc::clone(&triangle_point_constructor),
    ));

    let triangle_instance_point_creator = Rc::new(TriangleInstancePointCreator::new(Rc::clone(
        &triangle_point_creator,
    )));

    let triangle_instance_point_calculator = Rc::new(TriangleInstancePointCalculator::new(
        Rc::clone(&triangle_point_creator),
    ));

    let json_to_triangle_instance_converter = JsonToTriangleInstanceConverter::new(
        Rc::clone(&json_to_string_converter),
        Rc::clone(&json_to_two_d_point_converter),
        Rc::clone(&json_to_f32_converter),
        Rc::clone(&triangle_instance_creator),
        Rc::clone(&triangle_provider_ref_cell),
        Rc::clone(&triangle_instance_point_calculator),
    );

    let two_d_point_translator = Rc::new(TwoDPointTranslator::new());

    let triangle_instance_scaler = Rc::new(TriangleInstanceScaler::new(
        Rc::clone(&triangle_instance_creator),
        Rc::clone(&triangle_instance_point_creator),
        Rc::clone(&two_d_point_translator),
    ));

    let json_to_triangle_instance_runner_converter = JsonToObjectInstanceRunnerConverter::new(
        json_to_triangle_instance_converter,
        Rc::clone(&triangle_instance_scaler),
    );

    let json_to_boxed_triangle_instance_runner_converter =
        JsonToBoxedObjectInstanceRunnerConverter::new(json_to_triangle_instance_runner_converter);

    let c = Box::new(json_to_boxed_triangle_converter);
    object_converters.insert("triangle".to_string(), c);

    object_instance_runner_converters.insert(
        "triangle".to_string(),
        Box::new(json_to_boxed_triangle_instance_runner_converter),
    );
}

pub fn compose_json_to_content_converter(
    json_to_f32_converter: Rc<JsonToF32Converter>,
    json_to_string_converter: Rc<JsonToStringConverter>,
) -> JsonToContentConverter<
    TypedJsonToValueConverter<JsonToStringConverter, Box<Rc<RefCell<dyn GetName>>>>,
    TypedJsonToValueConverter<JsonToStringConverter, Box<dyn RunObjectInstance>>,
> {
    let json_to_rgb_converter = Rc::new(JsonToRgbConverter::new(Rc::clone(&json_to_f32_converter)));

    let two_d_point_creator = Rc::new(TwoDPointCreator::new());

    let rgb_creator = Rc::new(RgbCreator::new());

    let mut object_instance_runner_converters =
        HashMap::<String, Box<dyn ConvertJsonToValue<Box<dyn RunObjectInstance>>>>::new();

    let mut object_converters =
        HashMap::<String, Box<dyn ConvertJsonToValue<Box<Rc<RefCell<dyn GetName>>>>>>::new();

    let json_to_two_d_point_converter = Rc::new(JsonToTwoDPointConverter::new(Rc::clone(
        &json_to_f32_converter,
    )));

    let json_to_triangle_point_converter = Rc::new(JsonToTrianglePointConverter::new(
        Rc::clone(&json_to_two_d_point_converter),
        Rc::clone(&json_to_rgb_converter),
    ));

    let triangle_point_constructor = Rc::new(TrianglePointConstructor::new());

    let triangle_point_creator = Rc::new(TrianglePointCreator::new(
        Rc::clone(&two_d_point_creator),
        Rc::clone(&rgb_creator),
        Rc::clone(&triangle_point_constructor),
    ));

    compose_rectangles(
        &mut object_converters,
        &mut object_instance_runner_converters,
        Rc::clone(&json_to_string_converter),
        Rc::clone(&json_to_f32_converter),
        Rc::clone(&json_to_rgb_converter),
        Rc::clone(&json_to_two_d_point_converter),
        Rc::clone(&two_d_point_creator),
        Rc::clone(&triangle_point_creator),
    );

    compose_triangles(
        &mut object_converters,
        &mut object_instance_runner_converters,
        Rc::clone(&json_to_string_converter),
        Rc::clone(&json_to_triangle_point_converter),
        Rc::clone(&two_d_point_creator),
        Rc::clone(&&rgb_creator),
        Rc::clone(&json_to_two_d_point_converter),
        Rc::clone(&json_to_f32_converter),
    );

    let json_to_object_converter =
        TypedJsonToValueConverter::new(Rc::clone(&json_to_string_converter), object_converters);

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
        TypedJsonToValueConverter<JsonToStringConverter, Box<Rc<RefCell<dyn GetName>>>>,
        TypedJsonToValueConverter<JsonToStringConverter, Box<dyn RunObjectInstance>>,
    >,
> {
    let json_to_content_converter =
        compose_json_to_content_converter(json_to_f32_converter, json_to_string_converter);

    ContentLoader::<
        JsonToContentConverter<
            TypedJsonToValueConverter<JsonToStringConverter, Box<Rc<RefCell<dyn GetName>>>>,
            TypedJsonToValueConverter<JsonToStringConverter, Box<dyn RunObjectInstance>>,
        >,
    >::new(json_to_content_converter)
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use garden_content::{
        rectangles::{
            Rectangle, RectangleInstance, RectangleInstanceConstructor, RectangleInstanceScaler,
        },
        triangles::{
            Triangle, TriangleInstance, TriangleInstanceConstructor, TriangleInstancePointCreator,
            TriangleInstanceScaler, TriangleInstanceVertexCounter,
            TriangleInstanceVertexDataGenerator,
        },
        Content, GetVertexData, ObjectInstanceRunner, Rgb, RgbCreator, TrianglePoint,
        TrianglePointConstructor, TrianglePointCreator, TwoDPoint, TwoDPointCreator,
        TwoDPointTranslator,
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
                    }
                },
                {
                    "name": "Rectangle1-a",
                    "contentName": "Rectangle1",
                    "type": "rectangle",
                    "scale": 1.0,
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
                    "position": {
                        "x": 5.0,
                        "y": -5.0
                    }
                }
            ]
        });

        let expected_result = Content::new(
            vec![
                Box::new(Rc::new(RefCell::new(Triangle::<
                    TrianglePoint<TwoDPoint, Rgb>,
                >::new(
                    "Triangle1".to_string(),
                    TrianglePoint::<TwoDPoint, Rgb>::new(
                        TwoDPoint::new(-1.0, -1.0),
                        Rgb::new(1.0, 0.0, 0.0),
                        5,
                        vec![-1.0, -1.0, 1.0, 0.0, 0.0],
                    ),
                    TrianglePoint::<TwoDPoint, Rgb>::new(
                        TwoDPoint::new(0.0, 1.0),
                        Rgb::new(0.0, 1.0, 0.0),
                        5,
                        vec![0.0, 1.0, 0.0, 1.0, 0.0],
                    ),
                    TrianglePoint::<TwoDPoint, Rgb>::new(
                        TwoDPoint::new(1.0, 0.0),
                        Rgb::new(0.0, 0.0, 1.0),
                        5,
                        vec![1.0, 0.0, 0.0, 0.0, 1.0],
                    ),
                    vec![],
                    15,
                )))),
                Box::new(Rc::new(RefCell::new(Rectangle::new(
                    "Rectangle1".to_string(),
                    2.0,
                    5.0,
                    Rgb::new(0.0, 0.0, 0.1),
                )))),
                Box::new(Rc::new(RefCell::new(Rectangle::new(
                    "Rectangle2".to_string(),
                    3.0,
                    2.0,
                    Rgb::new(1.0, 0.0, 0.0),
                )))),
            ],
            vec![
                Box::new(ObjectInstanceRunner::new(
                    Rc::new(RefCell::new(TriangleInstance::<
                        TwoDPoint,
                        TrianglePoint<TwoDPoint, Rgb>,
                        Triangle<TrianglePoint<TwoDPoint, Rgb>>,
                    >::new(
                        "Triangle1-a".to_string(),
                        Rc::new(RefCell::new(Triangle::new(
                            "triangle".to_string(),
                            TrianglePoint::new(
                                TwoDPoint::new(1.0, 1.0),
                                Rgb::new(1.0, 1.0, 1.0),
                                0,
                                vec![],
                            ),
                            TrianglePoint::new(
                                TwoDPoint::new(1.0, 1.0),
                                Rgb::new(1.0, 1.0, 1.0),
                                0,
                                vec![],
                            ),
                            TrianglePoint::new(
                                TwoDPoint::new(1.0, 1.0),
                                Rgb::new(1.0, 1.0, 1.0),
                                0,
                                vec![],
                            ),
                            vec![],
                            0,
                        ))),
                        0.5,
                        TwoDPoint::new(-5.0, -5.0),
                        TrianglePoint::<TwoDPoint, Rgb>::new(
                            TwoDPoint::new(-5.5, -5.5),
                            Rgb::new(1.0, 0.0, 0.0),
                            5,
                            vec![-5.5, -5.5, 1.0, 0.0, 0.0],
                        ),
                        TrianglePoint::<TwoDPoint, Rgb>::new(
                            TwoDPoint::new(-5.0, -4.5),
                            Rgb::new(0.0, 1.0, 0.0),
                            5,
                            vec![-5.0, -4.5, 0.0, 1.0, 0.0],
                        ),
                        TrianglePoint::<TwoDPoint, Rgb>::new(
                            TwoDPoint::new(-4.5, -5.5),
                            Rgb::new(0.0, 0.0, 1.0),
                            5,
                            vec![-4.5, -5.5, 0.0, 0.0, 1.0],
                        ),
                        15,
                        vec![
                            -5.5, -5.5, 1.0, 0.0, 0.0, -5.0, -4.5, 0.0, 1.0, 0.0, -4.5, -5.5, 0.0,
                            0.0, 1.0,
                        ],
                    ))),
                    Rc::new(TriangleInstanceScaler::new(
                        Rc::new(TriangleInstanceCreator::new(
                            Rc::new(TriangleInstanceVertexDataGenerator::new()),
                            Rc::new(TriangleInstanceVertexCounter::new()),
                            Rc::new(TriangleInstanceConstructor::new()),
                        )),
                        Rc::new(TriangleInstancePointCreator::new(Rc::new(
                            TrianglePointCreator::new(
                                Rc::new(TwoDPointCreator::new()),
                                Rc::new(RgbCreator::new()),
                                Rc::new(TrianglePointConstructor::new()),
                            ),
                        ))),
                        Rc::new(TwoDPointTranslator::new()),
                    )),
                )),
                Box::new(ObjectInstanceRunner::new(
                    Rc::new(RefCell::new(TriangleInstance::new(
                        "Triangle1-b".to_string(),
                        Rc::new(RefCell::new(Triangle::new(
                            "triangle".to_string(),
                            TrianglePoint::new(
                                TwoDPoint::new(1.0, 1.0),
                                Rgb::new(1.0, 1.0, 1.0),
                                0,
                                vec![],
                            ),
                            TrianglePoint::new(
                                TwoDPoint::new(1.0, 1.0),
                                Rgb::new(1.0, 1.0, 1.0),
                                0,
                                vec![],
                            ),
                            TrianglePoint::new(
                                TwoDPoint::new(1.0, 1.0),
                                Rgb::new(1.0, 1.0, 1.0),
                                0,
                                vec![],
                            ),
                            vec![],
                            0,
                        ))),
                        0.5,
                        TwoDPoint::new(0.5, 0.5),
                        TrianglePoint::<TwoDPoint, Rgb>::new(
                            TwoDPoint::new(2.0, 2.0),
                            Rgb::new(1.0, 0.0, 0.0),
                            5,
                            vec![2.0, 2.0, 1.0, 0.0, 0.0],
                        ),
                        TrianglePoint::<TwoDPoint, Rgb>::new(
                            TwoDPoint::new(5.0, 8.0),
                            Rgb::new(0.0, 1.0, 0.0),
                            5,
                            vec![5.0, 8.0, 0.0, 1.0, 0.0],
                        ),
                        TrianglePoint::<TwoDPoint, Rgb>::new(
                            TwoDPoint::new(8.0, 2.0),
                            Rgb::new(0.0, 0.0, 1.0),
                            5,
                            vec![8.0, 2.0, 0.0, 0.0, 1.0],
                        ),
                        15,
                        vec![
                            2.0, 2.0, 1.0, 0.0, 0.0, 5.0, 8.0, 0.0, 1.0, 0.0, 8.0, 2.0, 0.0, 0.0,
                            1.0,
                        ],
                    ))),
                    Rc::new(TriangleInstanceScaler::new(
                        Rc::new(TriangleInstanceCreator::new(
                            Rc::new(TriangleInstanceVertexDataGenerator::new()),
                            Rc::new(TriangleInstanceVertexCounter::new()),
                            Rc::new(TriangleInstanceConstructor::new()),
                        )),
                        Rc::new(TriangleInstancePointCreator::new(Rc::new(
                            TrianglePointCreator::new(
                                Rc::new(TwoDPointCreator::new()),
                                Rc::new(RgbCreator::new()),
                                Rc::new(TrianglePointConstructor::new()),
                            ),
                        ))),
                        Rc::new(TwoDPointTranslator::new()),
                    )),
                )),
                Box::new(ObjectInstanceRunner::new(
                    Rc::new(RefCell::new(RectangleInstance::new(
                        "Rectangle1-a".to_string(),
                        Rc::new(RefCell::new(Rectangle::new(
                            "Rectangle1".to_string(),
                            2.5,
                            5.0,
                            Rgb::new(0.0, 0.0, 1.0),
                        ))),
                        1.0,
                        TwoDPoint::new(-5.0, 5.0),
                        TrianglePoint::new(
                            TwoDPoint::new(0.0, 0.0),
                            Rgb::new(1.0, 0.0, 0.0),
                            5,
                            vec![0.0, 0.0, 1.0, 0.0, 0.0],
                        ),
                        TrianglePoint::new(
                            TwoDPoint::new(0.0, 0.0),
                            Rgb::new(1.0, 0.0, 0.0),
                            5,
                            vec![0.0, 0.0, 1.0, 0.0, 0.0],
                        ),
                        TrianglePoint::new(
                            TwoDPoint::new(0.0, 0.0),
                            Rgb::new(1.0, 0.0, 0.0),
                            5,
                            vec![0.0, 0.0, 1.0, 0.0, 0.0],
                        ),
                        TrianglePoint::new(
                            TwoDPoint::new(0.0, 0.0),
                            Rgb::new(1.0, 0.0, 0.0),
                            5,
                            vec![0.0, 0.0, 1.0, 0.0, 0.0],
                        ),
                        30,
                        vec![
                            -4.0, 7.5, 0.0, 0.0, 1.0, -6.0, 7.5, 0.0, 0.0, 1.0, -6.0, 2.5, 0.0,
                            0.0, 1.0, -4.0, 7.5, 0.0, 0.0, 1.0, -6.0, 2.5, 0.0, 0.0, 1.0, -4.0,
                            2.5, 0.0, 0.0, 1.0,
                        ],
                        Rc::new(RefCell::new(TriangleInstance::new(
                            "".to_string(),
                            Rc::new(RefCell::new(Triangle::new(
                                "triangle".to_string(),
                                TrianglePoint::new(
                                    TwoDPoint::new(1.0, 1.0),
                                    Rgb::new(1.0, 1.0, 1.0),
                                    0,
                                    vec![],
                                ),
                                TrianglePoint::new(
                                    TwoDPoint::new(1.0, 1.0),
                                    Rgb::new(1.0, 1.0, 1.0),
                                    0,
                                    vec![],
                                ),
                                TrianglePoint::new(
                                    TwoDPoint::new(1.0, 1.0),
                                    Rgb::new(1.0, 1.0, 1.0),
                                    0,
                                    vec![],
                                ),
                                vec![],
                                0,
                            ))),
                            0.0,
                            TwoDPoint::new(0.0, 0.0),
                            TrianglePoint::new(
                                TwoDPoint::new(0.0, 0.0),
                                Rgb::new(1.0, 0.0, 0.0),
                                5,
                                vec![0.0, 0.0, 1.0, 0.0, 0.0],
                            ),
                            TrianglePoint::new(
                                TwoDPoint::new(0.0, 0.0),
                                Rgb::new(1.0, 0.0, 0.0),
                                5,
                                vec![0.0, 0.0, 1.0, 0.0, 0.0],
                            ),
                            TrianglePoint::new(
                                TwoDPoint::new(0.0, 0.0),
                                Rgb::new(1.0, 0.0, 0.0),
                                5,
                                vec![0.0, 0.0, 1.0, 0.0, 0.0],
                            ),
                            15,
                            vec![],
                        ))),
                        Rc::new(RefCell::new(TriangleInstance::new(
                            "".to_string(),
                            Rc::new(RefCell::new(Triangle::new(
                                "triangle".to_string(),
                                TrianglePoint::new(
                                    TwoDPoint::new(1.0, 1.0),
                                    Rgb::new(1.0, 1.0, 1.0),
                                    0,
                                    vec![],
                                ),
                                TrianglePoint::new(
                                    TwoDPoint::new(1.0, 1.0),
                                    Rgb::new(1.0, 1.0, 1.0),
                                    0,
                                    vec![],
                                ),
                                TrianglePoint::new(
                                    TwoDPoint::new(1.0, 1.0),
                                    Rgb::new(1.0, 1.0, 1.0),
                                    0,
                                    vec![],
                                ),
                                vec![],
                                0,
                            ))),
                            0.0,
                            TwoDPoint::new(0.0, 0.0),
                            TrianglePoint::new(
                                TwoDPoint::new(0.0, 0.0),
                                Rgb::new(1.0, 0.0, 0.0),
                                5,
                                vec![0.0, 0.0, 1.0, 0.0, 0.0],
                            ),
                            TrianglePoint::new(
                                TwoDPoint::new(0.0, 0.0),
                                Rgb::new(1.0, 0.0, 0.0),
                                5,
                                vec![0.0, 0.0, 1.0, 0.0, 0.0],
                            ),
                            TrianglePoint::new(
                                TwoDPoint::new(0.0, 0.0),
                                Rgb::new(1.0, 0.0, 0.0),
                                5,
                                vec![0.0, 0.0, 1.0, 0.0, 0.0],
                            ),
                            15,
                            vec![],
                        ))),
                    ))),
                    Rc::new(RectangleInstanceScaler::new(
                        Rc::new(RectangleInstanceCreator::new(
                            Rc::new(TriangleInstanceCreator::new(
                                Rc::new(TriangleInstanceVertexDataGenerator::new()),
                                Rc::new(TriangleInstanceVertexCounter::new()),
                                Rc::new(TriangleInstanceConstructor::new()),
                            )),
                            Rc::new(TrianglePointCreator::new(
                                Rc::new(TwoDPointCreator::new()),
                                Rc::new(RgbCreator::new()),
                                Rc::new(TrianglePointConstructor::new()),
                            )),
                            Rc::new(TwoDPointCreator::new()),
                            Rc::new(RectangleInstanceConstructor::new()),
                        )),
                        Rc::new(TwoDPointCreator::new()),
                    )),
                )),
                Box::new(ObjectInstanceRunner::new(
                    Rc::new(RefCell::new(RectangleInstance::new(
                        "Rectangle2-a".to_string(),
                        Rc::new(RefCell::new(Rectangle::new(
                            "Rectangle1".to_string(),
                            3.0,
                            2.0,
                            Rgb::new(1.0, 0.0, 0.0),
                        ))),
                        1.0,
                        TwoDPoint::new(5.0, -5.0),
                        TrianglePoint::new(
                            TwoDPoint::new(0.0, 0.0),
                            Rgb::new(1.0, 0.0, 0.0),
                            5,
                            vec![0.0, 0.0, 1.0, 0.0, 0.0],
                        ),
                        TrianglePoint::new(
                            TwoDPoint::new(0.0, 0.0),
                            Rgb::new(1.0, 0.0, 0.0),
                            5,
                            vec![0.0, 0.0, 1.0, 0.0, 0.0],
                        ),
                        TrianglePoint::new(
                            TwoDPoint::new(0.0, 0.0),
                            Rgb::new(1.0, 0.0, 0.0),
                            5,
                            vec![0.0, 0.0, 1.0, 0.0, 0.0],
                        ),
                        TrianglePoint::new(
                            TwoDPoint::new(0.0, 0.0),
                            Rgb::new(1.0, 0.0, 0.0),
                            5,
                            vec![0.0, 0.0, 1.0, 0.0, 0.0],
                        ),
                        30,
                        vec![
                            6.5, -4.0, 1.0, 0.0, 0.0, 3.5, -4.0, 1.0, 0.0, 0.0, 3.5, -6.0, 1.0,
                            0.0, 0.0, 6.5, -4.0, 1.0, 0.0, 0.0, 3.5, -6.0, 1.0, 0.0, 0.0, 6.5,
                            -6.0, 1.0, 0.0, 0.0,
                        ],
                        Rc::new(RefCell::new(TriangleInstance::new(
                            "".to_string(),
                            Rc::new(RefCell::new(Triangle::new(
                                "triangle".to_string(),
                                TrianglePoint::new(
                                    TwoDPoint::new(1.0, 1.0),
                                    Rgb::new(1.0, 1.0, 1.0),
                                    0,
                                    vec![],
                                ),
                                TrianglePoint::new(
                                    TwoDPoint::new(1.0, 1.0),
                                    Rgb::new(1.0, 1.0, 1.0),
                                    0,
                                    vec![],
                                ),
                                TrianglePoint::new(
                                    TwoDPoint::new(1.0, 1.0),
                                    Rgb::new(1.0, 1.0, 1.0),
                                    0,
                                    vec![],
                                ),
                                vec![],
                                0,
                            ))),
                            0.0,
                            TwoDPoint::new(0.0, 0.0),
                            TrianglePoint::new(
                                TwoDPoint::new(0.0, 0.0),
                                Rgb::new(1.0, 0.0, 0.0),
                                5,
                                vec![0.0, 0.0, 1.0, 0.0, 0.0],
                            ),
                            TrianglePoint::new(
                                TwoDPoint::new(0.0, 0.0),
                                Rgb::new(1.0, 0.0, 0.0),
                                5,
                                vec![0.0, 0.0, 1.0, 0.0, 0.0],
                            ),
                            TrianglePoint::new(
                                TwoDPoint::new(0.0, 0.0),
                                Rgb::new(1.0, 0.0, 0.0),
                                5,
                                vec![0.0, 0.0, 1.0, 0.0, 0.0],
                            ),
                            15,
                            vec![],
                        ))),
                        Rc::new(RefCell::new(TriangleInstance::new(
                            "".to_string(),
                            Rc::new(RefCell::new(Triangle::new(
                                "triangle".to_string(),
                                TrianglePoint::new(
                                    TwoDPoint::new(1.0, 1.0),
                                    Rgb::new(1.0, 1.0, 1.0),
                                    0,
                                    vec![],
                                ),
                                TrianglePoint::new(
                                    TwoDPoint::new(1.0, 1.0),
                                    Rgb::new(1.0, 1.0, 1.0),
                                    0,
                                    vec![],
                                ),
                                TrianglePoint::new(
                                    TwoDPoint::new(1.0, 1.0),
                                    Rgb::new(1.0, 1.0, 1.0),
                                    0,
                                    vec![],
                                ),
                                vec![],
                                0,
                            ))),
                            0.0,
                            TwoDPoint::new(0.0, 0.0),
                            TrianglePoint::new(
                                TwoDPoint::new(0.0, 0.0),
                                Rgb::new(1.0, 0.0, 0.0),
                                5,
                                vec![0.0, 0.0, 1.0, 0.0, 0.0],
                            ),
                            TrianglePoint::new(
                                TwoDPoint::new(0.0, 0.0),
                                Rgb::new(1.0, 0.0, 0.0),
                                5,
                                vec![0.0, 0.0, 1.0, 0.0, 0.0],
                            ),
                            TrianglePoint::new(
                                TwoDPoint::new(0.0, 0.0),
                                Rgb::new(1.0, 0.0, 0.0),
                                5,
                                vec![0.0, 0.0, 1.0, 0.0, 0.0],
                            ),
                            15,
                            vec![],
                        ))),
                    ))),
                    Rc::new(RectangleInstanceScaler::new(
                        Rc::new(RectangleInstanceCreator::new(
                            Rc::new(TriangleInstanceCreator::new(
                                Rc::new(TriangleInstanceVertexDataGenerator::new()),
                                Rc::new(TriangleInstanceVertexCounter::new()),
                                Rc::new(TriangleInstanceConstructor::new()),
                            )),
                            Rc::new(TrianglePointCreator::new(
                                Rc::new(TwoDPointCreator::new()),
                                Rc::new(RgbCreator::new()),
                                Rc::new(TrianglePointConstructor::new()),
                            )),
                            Rc::new(TwoDPointCreator::new()),
                            Rc::new(RectangleInstanceConstructor::new()),
                        )),
                        Rc::new(TwoDPointCreator::new()),
                    )),
                )),
            ],
        );

        let result = json_to_content_converter.convert_json_to_value(&json);

        assert_eq!(expected_result.get_vertex_data(), result.get_vertex_data());
    }
}
