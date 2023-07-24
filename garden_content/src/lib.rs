pub mod circles;
pub mod equilateral_triangles;
pub mod rectangles;
pub mod triangles;

use std::{cell::RefCell, marker::PhantomData, rc::Rc};

use garden::GetName;

pub trait GetVertexData {
    fn get_vertex_data(&self) -> Vec<f32>;
}

pub trait GetNumberOfVertices {
    fn get_number_of_vertices(&self) -> i32;
}

pub trait GetScale {
    fn get_scale(&self) -> f32;
}

pub trait Scale {
    fn scale(&mut self, x: f32, y: f32);
}

pub trait ScaleObjectInstance<TObjectInstance> {
    fn scale_object_instance(
        &self,
        object_instance: Rc<RefCell<TObjectInstance>>,
        x: f32,
        y: f32,
    ) -> Rc<RefCell<TObjectInstance>>;
}

pub trait GetContentInstanceData: GetVertexData + GetNumberOfVertices + GetNumberOfObjects {}

pub trait GetNumberOfObjects {
    fn get_number_of_objects(&self) -> i32;
}

pub trait GetVertexDataPtr {
    fn get_vertex_data_ptr(&self) -> *const f32;
}

pub trait GetX {
    fn get_x(&self) -> f32;
}

pub trait GetY {
    fn get_y(&self) -> f32;
}

pub trait Get2DCoordiantes: GetX + GetY {}

pub struct TwoDPoint {
    x: f32,
    y: f32,
    vertex_data: Vec<f32>,
}

impl TwoDPoint {
    const NUMBER_OF_VERTICES: i32 = 2;

    pub fn new(x: f32, y: f32) -> Self {
        let vertex_data = vec![x, y];
        Self { x, y, vertex_data }
    }
}

impl GetX for TwoDPoint {
    fn get_x(&self) -> f32 {
        self.x
    }
}

impl GetY for TwoDPoint {
    fn get_y(&self) -> f32 {
        self.y
    }
}

impl Get2DCoordiantes for TwoDPoint {}

impl GetVertexData for TwoDPoint {
    fn get_vertex_data(&self) -> Vec<f32> {
        self.vertex_data.clone()
    }
}

impl GetNumberOfVertices for TwoDPoint {
    fn get_number_of_vertices(&self) -> i32 {
        TwoDPoint::NUMBER_OF_VERTICES
    }
}

pub trait GetTwoDPointProperties: GetVertexData + GetNumberOfVertices {}

impl GetTwoDPointProperties for TwoDPoint {}

pub trait CreateTwoDPoint<TTwoDPoint> {
    fn create_two_d_point(&self, x: f32, y: f32) -> TTwoDPoint;
}

pub struct TwoDPointCreator {}

impl TwoDPointCreator {
    pub fn new() -> Self {
        Self {}
    }
}

impl CreateTwoDPoint<TwoDPoint> for TwoDPointCreator {
    fn create_two_d_point(&self, x: f32, y: f32) -> TwoDPoint {
        TwoDPoint::new(x, y)
    }
}

pub trait TranslateTwoDPoint<TTwoDPoint> {
    fn translate_two_d_point(&self, position: &TTwoDPoint, x: f32, y: f32) -> TTwoDPoint;
}

pub struct TwoDPointTranslator {}

impl TwoDPointTranslator {
    pub fn new() -> Self {
        Self {}
    }
}

impl TranslateTwoDPoint<TwoDPoint> for TwoDPointTranslator {
    fn translate_two_d_point(&self, position: &TwoDPoint, x: f32, y: f32) -> TwoDPoint {
        TwoDPoint::new(position.get_x() / x, position.get_y() / y)
    }
}

pub trait GetR {
    fn get_r(&self) -> f32;
}

pub trait GetG {
    fn get_g(&self) -> f32;
}

pub trait GetB {
    fn get_b(&self) -> f32;
}

pub trait GetRgbValues: GetR + GetG + GetB {}

pub struct Rgb {
    r: f32,
    g: f32,
    b: f32,
}

impl Rgb {
    const NUMBER_OF_VERTICES: i32 = 3;

    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b }
    }
}

impl GetR for Rgb {
    fn get_r(&self) -> f32 {
        self.r
    }
}

impl GetG for Rgb {
    fn get_g(&self) -> f32 {
        self.g
    }
}

impl GetB for Rgb {
    fn get_b(&self) -> f32 {
        self.b
    }
}

impl GetVertexData for Rgb {
    fn get_vertex_data(&self) -> Vec<f32> {
        vec![self.r, self.g, self.b]
    }
}

impl GetNumberOfVertices for Rgb {
    fn get_number_of_vertices(&self) -> i32 {
        Rgb::NUMBER_OF_VERTICES
    }
}

pub trait GetRgbProperties: GetRgbValues + GetVertexData + GetNumberOfVertices {}

impl GetRgbValues for Rgb {}

impl GetRgbProperties for Rgb {}

pub trait CreateRgb<TRgb> {
    fn create_rgb(&self, r: f32, g: f32, b: f32) -> TRgb;
}

pub struct RgbCreator {}

impl RgbCreator {
    pub fn new() -> Self {
        Self {}
    }
}

impl CreateRgb<Rgb> for RgbCreator {
    fn create_rgb(&self, r: f32, g: f32, b: f32) -> Rgb {
        Rgb::new(r, g, b)
    }
}

pub trait GetRgb<TRgb> {
    fn get_rgb(&self) -> &TRgb;
}

pub trait GetTrianglePointProperties: Get2DCoordiantes + GetRgbValues {}

pub struct TrianglePoint<TTwoDPoint, TRgb> {
    point: TTwoDPoint,
    rgb: TRgb,
    number_of_vertices: i32,
    vertex_data: Vec<f32>,
}

impl<
        TTwoDPoint: GetVertexData + GetNumberOfVertices,
        TRgb: GetVertexData + GetNumberOfVertices,
    > TrianglePoint<TTwoDPoint, TRgb>
{
    pub fn new(
        point: TTwoDPoint,
        rgb: TRgb,
        number_of_vertices: i32,
        vertex_data: Vec<f32>,
    ) -> Self {
        Self {
            point,
            rgb,
            number_of_vertices,
            vertex_data,
        }
    }
}

impl<TTwoDPoint, TRgb> GetVertexData for TrianglePoint<TTwoDPoint, TRgb> {
    fn get_vertex_data(&self) -> Vec<f32> {
        self.vertex_data.clone()
    }
}

impl<TTwoDPoint, TRgb> GetNumberOfVertices for TrianglePoint<TTwoDPoint, TRgb> {
    fn get_number_of_vertices(&self) -> i32 {
        self.number_of_vertices
    }
}

impl<TTwoDPoint: GetX, TRgb> GetX for TrianglePoint<TTwoDPoint, TRgb> {
    fn get_x(&self) -> f32 {
        self.point.get_x()
    }
}

impl<TTwoDPoint: GetY, TRgb> GetY for TrianglePoint<TTwoDPoint, TRgb> {
    fn get_y(&self) -> f32 {
        self.point.get_y()
    }
}

impl<TTwoDPoint: Get2DCoordiantes, TRgb> Get2DCoordiantes for TrianglePoint<TTwoDPoint, TRgb> {}

impl<TTwoDPoint: GetY, TRgb> GetRgb<TRgb> for TrianglePoint<TTwoDPoint, TRgb> {
    fn get_rgb(&self) -> &TRgb {
        &self.rgb
    }
}

impl<TTwoDPoint, TRgb: GetR> GetR for TrianglePoint<TTwoDPoint, TRgb> {
    fn get_r(&self) -> f32 {
        self.rgb.get_r()
    }
}

impl<TTwoDPoint, TRgb: GetG> GetG for TrianglePoint<TTwoDPoint, TRgb> {
    fn get_g(&self) -> f32 {
        self.rgb.get_g()
    }
}

impl<TTwoDPoint, TRgb: GetB> GetB for TrianglePoint<TTwoDPoint, TRgb> {
    fn get_b(&self) -> f32 {
        self.rgb.get_b()
    }
}

impl<TTwoDPoint, TRgb: GetRgbValues> GetRgbValues for TrianglePoint<TTwoDPoint, TRgb> {}

impl<TTwoDPoint: Get2DCoordiantes, TRgb: GetRgbValues> GetTrianglePointProperties
    for TrianglePoint<TTwoDPoint, TRgb>
{
}

pub trait ConstructTrianglePoint<TTwoDPoint, TRgb, TTrianglePoint> {
    fn construct_triangle_point(
        &self,
        point: TTwoDPoint,
        rgb: TRgb,
        number_of_vertices: i32,
        vertex_data: Vec<f32>,
    ) -> TTrianglePoint;
}

pub struct TrianglePointConstructor {}

impl TrianglePointConstructor {
    pub fn new() -> Self {
        Self {}
    }
}

impl<
        TTwoDPoint: GetVertexData + GetNumberOfVertices,
        TRgb: GetVertexData + GetNumberOfVertices,
    > ConstructTrianglePoint<TTwoDPoint, TRgb, TrianglePoint<TTwoDPoint, TRgb>>
    for TrianglePointConstructor
{
    fn construct_triangle_point(
        &self,
        point: TTwoDPoint,
        rgb: TRgb,
        number_of_vertices: i32,
        vertex_data: Vec<f32>,
    ) -> TrianglePoint<TTwoDPoint, TRgb> {
        TrianglePoint::new(point, rgb, number_of_vertices, vertex_data)
    }
}

pub trait CreateTrianglePoint<TTrianglePoint> {
    fn create_triangle_point(&self, x: f32, y: f32, r: f32, g: f32, b: f32) -> TTrianglePoint;
}

pub struct TrianglePointCreator<
    TTwoDPointCreator,
    TRgbCreator,
    TTrianglePoint,
    TTrianglePointConstructor,
    TTwoDPoint,
    TRgb,
> {
    two_d_point_creator: Rc<TTwoDPointCreator>,
    rgb_creator: Rc<TRgbCreator>,
    triangle_point_constructor: Rc<TTrianglePointConstructor>,
    triangle_point_type: PhantomData<TTrianglePoint>,
    two_d_point_type: PhantomData<TTwoDPoint>,
    rgb_type: PhantomData<TRgb>,
}

impl<
        TTwoDPointCreator,
        TRgbCreator,
        TTrianglePoint,
        TTrianglePointConstructor,
        TTwoDPoint,
        TRgb,
    >
    TrianglePointCreator<
        TTwoDPointCreator,
        TRgbCreator,
        TTrianglePoint,
        TTrianglePointConstructor,
        TTwoDPoint,
        TRgb,
    >
{
    pub fn new(
        two_d_point_creator: Rc<TTwoDPointCreator>,
        rgb_creator: Rc<TRgbCreator>,
        triangle_point_constructor: Rc<TTrianglePointConstructor>,
    ) -> Self {
        Self {
            two_d_point_creator: two_d_point_creator,
            rgb_creator: rgb_creator,
            triangle_point_constructor: triangle_point_constructor,
            triangle_point_type: PhantomData,
            two_d_point_type: PhantomData,
            rgb_type: PhantomData,
        }
    }
}

impl<
        TTwoDPointCreator: CreateTwoDPoint<TTwoDPoint>,
        TRgbCreator: CreateRgb<TRgb>,
        TTwoDPoint: GetTwoDPointProperties,
        TRgb: GetRgbProperties,
        TTrianglePointConstructor: ConstructTrianglePoint<TTwoDPoint, TRgb, TTrianglePoint>,
        TTrianglePoint,
    > CreateTrianglePoint<TTrianglePoint>
    for TrianglePointCreator<
        TTwoDPointCreator,
        TRgbCreator,
        TTrianglePoint,
        TTrianglePointConstructor,
        TTwoDPoint,
        TRgb,
    >
{
    fn create_triangle_point(&self, x: f32, y: f32, r: f32, g: f32, b: f32) -> TTrianglePoint {
        let two_d_point = self.two_d_point_creator.create_two_d_point(x, y);

        let rgb = self.rgb_creator.create_rgb(r, g, b);

        let mut vertex_data = vec![];

        vertex_data.append(&mut two_d_point.get_vertex_data());
        vertex_data.append(&mut rgb.get_vertex_data());

        let number_of_vertices =
            two_d_point.get_number_of_vertices() + rgb.get_number_of_vertices();

        self.triangle_point_constructor.construct_triangle_point(
            two_d_point,
            rgb,
            number_of_vertices,
            vertex_data,
        )
    }
}

pub trait GetPosition<TPosition> {
    fn get_position(&self) -> &TPosition;
}

pub struct Content {
    objects: Option<Vec<Box<Rc<RefCell<dyn GetName>>>>>,
    object_instance_runners: Option<Vec<Box<dyn RunObjectInstance>>>,
    vertex_data: Vec<f32>,
    number_of_vertices: i32,
    number_of_objects: i32,
}

impl Content {
    pub fn new(
        objects: Vec<Box<Rc<RefCell<dyn GetName>>>>,
        object_instance_runners: Vec<Box<dyn RunObjectInstance>>,
    ) -> Self {
        let mut number_of_vertices = 0;

        let mut vertex_data = vec![];

        let mut number_of_objects = 0;

        for object_instance in object_instance_runners.iter() {
            number_of_vertices += object_instance.get_number_of_vertices();
            vertex_data.append(&mut object_instance.get_vertex_data());
            number_of_objects += object_instance.get_number_of_objects();
        }

        Self {
            objects: Some(objects),
            object_instance_runners: Some(object_instance_runners),
            vertex_data,
            number_of_vertices,
            number_of_objects,
        }
    }

    pub fn get_objects(&self) -> &Option<Vec<Box<Rc<RefCell<dyn GetName>>>>> {
        &self.objects
    }

    pub fn scale_object_instances(&mut self, x: f32, y: f32) {
        let mut new_object_instance_runners = Vec::<Box<dyn RunObjectInstance>>::new();

        let mut new_vertex_data = vec![];

        let mut i = self.object_instance_runners.as_ref().unwrap().len();

        let mut object_instance_runners = self.object_instance_runners.take().unwrap();

        while i > 0 {
            let mut t = object_instance_runners.remove(i - 1);
            t.scale(x, y);
            new_vertex_data.append(&mut t.get_vertex_data());
            new_object_instance_runners.push(t);
            i = i - 1;
        }

        self.object_instance_runners = Some(new_object_instance_runners);

        self.vertex_data = new_vertex_data;
    }
}

impl GetVertexData for Content {
    fn get_vertex_data(&self) -> Vec<f32> {
        self.vertex_data.clone()
    }
}

impl GetNumberOfVertices for Content {
    fn get_number_of_vertices(&self) -> i32 {
        self.number_of_vertices
    }
}

impl GetNumberOfObjects for Content {
    fn get_number_of_objects(&self) -> i32 {
        self.number_of_objects
    }
}

impl GetVertexDataPtr for Content {
    fn get_vertex_data_ptr(&self) -> *const f32 {
        self.vertex_data.as_ptr()
    }
}

pub trait RunObjectInstance: GetContentInstanceData + Scale {}

pub struct ObjectInstanceRunner<TObjectInstance, TObjectInstanceScaler> {
    object_instance: Rc<RefCell<TObjectInstance>>,
    object_instance_scaler: Rc<TObjectInstanceScaler>,
}

impl<TObjectInstance, TObjectInstanceScaler>
    ObjectInstanceRunner<TObjectInstance, TObjectInstanceScaler>
{
    pub fn new(
        object_instance: Rc<RefCell<TObjectInstance>>,
        object_instance_scaler: Rc<TObjectInstanceScaler>,
    ) -> Self {
        Self {
            object_instance,
            object_instance_scaler,
        }
    }
}

impl<TObjectInstance, TObjectInstanceScaler: ScaleObjectInstance<TObjectInstance>> Scale
    for ObjectInstanceRunner<TObjectInstance, TObjectInstanceScaler>
{
    fn scale(&mut self, x: f32, y: f32) {
        self.object_instance = self.object_instance_scaler.scale_object_instance(
            Rc::clone(&self.object_instance),
            x,
            y,
        );
    }
}

impl<TObjectInstance: GetNumberOfObjects, TObjectInstanceScaler> GetNumberOfObjects
    for ObjectInstanceRunner<TObjectInstance, TObjectInstanceScaler>
{
    fn get_number_of_objects(&self) -> i32 {
        self.object_instance.borrow().get_number_of_objects()
    }
}

impl<TObjectInstance: GetNumberOfVertices, TObjectInstanceScaler> GetNumberOfVertices
    for ObjectInstanceRunner<TObjectInstance, TObjectInstanceScaler>
{
    fn get_number_of_vertices(&self) -> i32 {
        self.object_instance.borrow().get_number_of_vertices()
    }
}

impl<TObjectInstance: GetVertexData, TObjectInstanceScaler> GetVertexData
    for ObjectInstanceRunner<TObjectInstance, TObjectInstanceScaler>
{
    fn get_vertex_data(&self) -> Vec<f32> {
        self.object_instance.borrow().get_vertex_data()
    }
}

impl<TObjectInstance: GetContentInstanceData, TObjectInstanceScaler> GetContentInstanceData
    for ObjectInstanceRunner<TObjectInstance, TObjectInstanceScaler>
{
}

impl<
        TObjectInstance: GetContentInstanceData,
        TObjectInstanceScaler: ScaleObjectInstance<TObjectInstance>,
    > RunObjectInstance for ObjectInstanceRunner<TObjectInstance, TObjectInstanceScaler>
{
}

pub trait GetContent<TContent> {
    fn get_content(&self, content_name: String) -> Rc<RefCell<TContent>>;
}

pub trait ConstructObject<TObject, TParameters> {
    fn construct_object(&self, parameters: TParameters) -> TObject;
}

pub trait StoreObject<TObject> {
    fn store_object(&mut self, object: Rc<RefCell<TObject>>);
}

pub trait CreateObject<TObject, TParameters> {
    fn create_object(&self, parameters: TParameters) -> Rc<RefCell<TObject>>;
}

pub struct ObjectCreator<TObjectConstructor, TObjectStore, TObject, TParameters> {
    object_constructor: Rc<TObjectConstructor>,
    object_store: Rc<RefCell<TObjectStore>>,
    object_type: PhantomData<TObject>,
    parameters_type: PhantomData<TParameters>,
}

impl<TObjectConstructor, TObjectStore, TObject, TParameters>
    ObjectCreator<TObjectConstructor, TObjectStore, TObject, TParameters>
{
    pub fn new(
        object_constructor: Rc<TObjectConstructor>,
        object_store: Rc<RefCell<TObjectStore>>,
    ) -> Self {
        Self {
            object_constructor: object_constructor,
            object_store: object_store,
            object_type: PhantomData,
            parameters_type: PhantomData,
        }
    }
}

impl<
        TObjectConstructor: ConstructObject<TObject, TParameters>,
        TObjectStore: StoreObject<TObject>,
        TObject,
        TParameters,
    > CreateObject<TObject, TParameters>
    for ObjectCreator<TObjectConstructor, TObjectStore, TObject, TParameters>
{
    fn create_object(&self, parameters: TParameters) -> Rc<RefCell<TObject>> {
        let object = Rc::new(RefCell::new(
            self.object_constructor.construct_object(parameters),
        ));

        self.object_store
            .borrow_mut()
            .store_object(Rc::clone(&object));

        Rc::clone(&object)
    }
}

pub struct Store<T> {
    objects: Vec<Rc<RefCell<T>>>,
}

impl<T> Store<T> {
    pub fn new(objects: Vec<Rc<RefCell<T>>>) -> Self {
        Self { objects }
    }
}

impl<TObject> StoreObject<TObject> for Store<TObject> {
    fn store_object(&mut self, object: Rc<RefCell<TObject>>) {
        self.objects.push(object)
    }
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use mockall::mock;

    use crate::{
        ConstructObject, Content, CreateObject, GetContentInstanceData, GetNumberOfObjects,
        GetNumberOfVertices, GetVertexData, GetX, GetY, ObjectCreator, Rgb, RunObjectInstance,
        Scale, StoreObject, TrianglePoint, TwoDPoint,
    };

    #[test]
    fn when_a_two_d_point_gets_its_vertex_data_then_the_vertex_data_is_returned() {
        let x = 1.0;
        let y = -0.5;

        let expected_vertex_data = vec![x, y];

        let two_d_point = TwoDPoint::new(x, y);

        let result = two_d_point.get_vertex_data();

        assert_eq!(expected_vertex_data, result);
    }

    #[test]
    fn when_a_two_d_point_gets_its_number_of_vertices_then_the_number_of_vertices_is_returned() {
        let x = 1.0;
        let y = -0.5;

        let expected_number_of_vertices = 2;

        let two_d_point = TwoDPoint::new(x, y);

        let result = two_d_point.get_number_of_vertices();

        assert_eq!(expected_number_of_vertices, result);
    }

    #[test]
    fn when_a_rgb_gets_its_vertex_data_then_the_vertex_data_is_returned() {
        let r = 1.0;
        let g = 0.5;
        let b = 0.0;

        let expected_vertex_data = vec![r, g, b];

        let rgb = Rgb::new(r, g, b);

        let result = rgb.get_vertex_data();

        assert_eq!(expected_vertex_data, result);
    }

    #[test]
    fn when_a_rgb_gets_its_number_of_vertices_then_the_number_of_vertices_is_returned() {
        let expected_number_of_vertices = 3;

        let rgb = Rgb::new(0.0, 0.0, 0.0);

        let result = rgb.get_number_of_vertices();

        assert_eq!(expected_number_of_vertices, result);
    }

    #[test]
    fn when_a_triangle_point_gets_its_vertex_data_then_the_vertex_data_is_returned() {
        let point = MockVertexObject::new();

        let rgb = MockVertexObject::new();

        let number_of_vertices = 0;

        let vertex_data = vec![1.0, -0.5, 1.0, 0.5, 0.0];

        let triangle_point = TrianglePoint::<MockVertexObject, MockVertexObject>::new(
            point,
            rgb,
            number_of_vertices,
            vertex_data.clone(),
        );

        let result = triangle_point.get_vertex_data();

        assert_eq!(vertex_data, result);
    }

    #[test]
    fn when_a_triangle_point_gets_its_number_of_vertices_then_the_number_of_vertices_is_returned() {
        let point = MockVertexObject::new();

        let rgb = MockVertexObject::new();

        let number_of_vertices = 5;

        let vertex_data = vec![];

        let triangle_point = TrianglePoint::<MockVertexObject, MockVertexObject>::new(
            point,
            rgb,
            number_of_vertices,
            vertex_data,
        );

        let result = triangle_point.get_number_of_vertices();

        assert_eq!(number_of_vertices, result);
    }

    #[test]
    fn when_a_triangle_point_gets_x_then_point_x_is_returned() {
        let x = 1.23;

        let mut point = MockVertexObject::new();
        point.expect_get_x().times(1).returning(move || x);

        let rgb = MockVertexObject::new();

        let number_of_vertices = 0;

        let vertex_data = vec![];

        let triangle_point = TrianglePoint::<MockVertexObject, MockVertexObject>::new(
            point,
            rgb,
            number_of_vertices,
            vertex_data,
        );

        let result = triangle_point.get_x();

        assert_eq!(x, result);
    }

    #[test]
    fn when_a_triangle_point_gets_y_then_point_y_is_returned() {
        let y = 4.56;

        let mut point = MockVertexObject::new();
        point.expect_get_y().times(1).returning(move || y);

        let rgb = MockVertexObject::new();

        let number_of_vertices = 0;

        let vertex_data = vec![];

        let triangle_point = TrianglePoint::<MockVertexObject, MockVertexObject>::new(
            point,
            rgb,
            number_of_vertices,
            vertex_data,
        );

        let result = triangle_point.get_y();

        assert_eq!(y, result);
    }

    #[test]
    fn when_content_gets_its_vertex_data_then_the_vertex_data_is_returned() {
        let triangle_1_point_1_x = 0.0;
        let triangle_1_point_1_y = 0.5;
        let triangle_1_point_1_r = 1.0;
        let triangle_1_point_1_g = 0.0;
        let triangle_1_point_1_b = 0.0;
        let triangle_1_point_2_x = 0.5;
        let triangle_1_point_2_y = 1.0;
        let triangle_1_point_2_r = 0.0;
        let triangle_1_point_2_g = 1.0;
        let triangle_1_point_2_b = 0.0;
        let triangle_1_point_3_x = 1.0;
        let triangle_1_point_3_y = 0.0;
        let triangle_1_point_3_r = 0.0;
        let triangle_1_point_3_g = 0.0;
        let triangle_1_point_3_b = 1.0;

        let triangle_2_point_1_x = -1.0;
        let triangle_2_point_1_y = 0.0;
        let triangle_2_point_1_r = 1.0;
        let triangle_2_point_1_g = 0.0;
        let triangle_2_point_1_b = 0.0;
        let triangle_2_point_2_x = -0.5;
        let triangle_2_point_2_y = 1.0;
        let triangle_2_point_2_r = 0.0;
        let triangle_2_point_2_g = 1.0;
        let triangle_2_point_2_b = 0.0;
        let triangle_2_point_3_x = 0.0;
        let triangle_2_point_3_y = 0.0;
        let triangle_2_point_3_r = 0.0;
        let triangle_2_point_3_g = 0.0;
        let triangle_2_point_3_b = 1.0;

        let triangle_3_point_1_x = -1.0;
        let triangle_3_point_1_y = -1.0;
        let triangle_3_point_1_r = 1.0;
        let triangle_3_point_1_g = 0.0;
        let triangle_3_point_1_b = 0.0;
        let triangle_3_point_2_x = -0.5;
        let triangle_3_point_2_y = 0.0;
        let triangle_3_point_2_r = 0.0;
        let triangle_3_point_2_g = 1.0;
        let triangle_3_point_2_b = 0.0;
        let triangle_3_point_3_x = 0.0;
        let triangle_3_point_3_y = -1.0;
        let triangle_3_point_3_r = 0.0;
        let triangle_3_point_3_g = 0.0;
        let triangle_3_point_3_b = 1.0;

        let mut triangle_instance_1 = Box::new(create_mock_object_instance_runner(
            vec![
                triangle_1_point_1_x,
                triangle_1_point_1_y,
                triangle_1_point_1_r,
                triangle_1_point_1_g,
                triangle_1_point_1_b,
                triangle_1_point_2_x,
                triangle_1_point_2_y,
                triangle_1_point_2_r,
                triangle_1_point_2_g,
                triangle_1_point_2_b,
                triangle_1_point_3_x,
                triangle_1_point_3_y,
                triangle_1_point_3_r,
                triangle_1_point_3_g,
                triangle_1_point_3_b,
            ],
            0,
        ));
        triangle_instance_1
            .expect_get_number_of_objects()
            .times(1)
            .returning(move || 0);

        let mut triangle_instance_2 = Box::new(create_mock_object_instance_runner(
            vec![
                triangle_2_point_1_x,
                triangle_2_point_1_y,
                triangle_2_point_1_r,
                triangle_2_point_1_g,
                triangle_2_point_1_b,
                triangle_2_point_2_x,
                triangle_2_point_2_y,
                triangle_2_point_2_r,
                triangle_2_point_2_g,
                triangle_2_point_2_b,
                triangle_2_point_3_x,
                triangle_2_point_3_y,
                triangle_2_point_3_r,
                triangle_2_point_3_g,
                triangle_2_point_3_b,
            ],
            0,
        ));
        triangle_instance_2
            .expect_get_number_of_objects()
            .times(1)
            .returning(move || 0);

        let mut triangle_instance_3 = Box::new(create_mock_object_instance_runner(
            vec![
                triangle_3_point_1_x,
                triangle_3_point_1_y,
                triangle_3_point_1_r,
                triangle_3_point_1_g,
                triangle_3_point_1_b,
                triangle_3_point_2_x,
                triangle_3_point_2_y,
                triangle_3_point_2_r,
                triangle_3_point_2_g,
                triangle_3_point_2_b,
                triangle_3_point_3_x,
                triangle_3_point_3_y,
                triangle_3_point_3_r,
                triangle_3_point_3_g,
                triangle_3_point_3_b,
            ],
            0,
        ));
        triangle_instance_3
            .expect_get_number_of_objects()
            .times(1)
            .returning(move || 0);

        let objects = vec![];

        let mut object_instances = Vec::<Box<dyn RunObjectInstance>>::new();
        object_instances.push(triangle_instance_1);
        object_instances.push(triangle_instance_2);
        object_instances.push(triangle_instance_3);

        let expected_vertex_data = vec![
            triangle_1_point_1_x,
            triangle_1_point_1_y,
            triangle_1_point_1_r,
            triangle_1_point_1_g,
            triangle_1_point_1_b,
            triangle_1_point_2_x,
            triangle_1_point_2_y,
            triangle_1_point_2_r,
            triangle_1_point_2_g,
            triangle_1_point_2_b,
            triangle_1_point_3_x,
            triangle_1_point_3_y,
            triangle_1_point_3_r,
            triangle_1_point_3_g,
            triangle_1_point_3_b,
            triangle_2_point_1_x,
            triangle_2_point_1_y,
            triangle_2_point_1_r,
            triangle_2_point_1_g,
            triangle_2_point_1_b,
            triangle_2_point_2_x,
            triangle_2_point_2_y,
            triangle_2_point_2_r,
            triangle_2_point_2_g,
            triangle_2_point_2_b,
            triangle_2_point_3_x,
            triangle_2_point_3_y,
            triangle_2_point_3_r,
            triangle_2_point_3_g,
            triangle_2_point_3_b,
            triangle_3_point_1_x,
            triangle_3_point_1_y,
            triangle_3_point_1_r,
            triangle_3_point_1_g,
            triangle_3_point_1_b,
            triangle_3_point_2_x,
            triangle_3_point_2_y,
            triangle_3_point_2_r,
            triangle_3_point_2_g,
            triangle_3_point_2_b,
            triangle_3_point_3_x,
            triangle_3_point_3_y,
            triangle_3_point_3_r,
            triangle_3_point_3_g,
            triangle_3_point_3_b,
        ];

        let content = Content::new(objects, object_instances);

        let result = content.get_vertex_data();

        assert_eq!(result, expected_vertex_data);
    }

    #[test]
    fn when_content_gets_its_number_of_vertices_then_the_number_of_vertices_is_returned() {
        let mut object_instance_1 = create_mock_object_instance_runner(vec![], 15);
        object_instance_1
            .expect_get_number_of_objects()
            .times(1)
            .returning(move || 0);

        let mut object_instance_2 = create_mock_object_instance_runner(vec![], 15);
        object_instance_2
            .expect_get_number_of_objects()
            .times(1)
            .returning(move || 0);

        let mut object_instance_3 = create_mock_object_instance_runner(vec![], 15);
        object_instance_3
            .expect_get_number_of_objects()
            .times(1)
            .returning(move || 0);

        let objects = vec![];

        let mut object_instances = Vec::<Box<dyn RunObjectInstance>>::new();
        object_instances.push(Box::new(object_instance_1));
        object_instances.push(Box::new(object_instance_2));
        object_instances.push(Box::new(object_instance_3));

        let expected_number_of_vertices = 45;

        let content = Content::new(objects, object_instances);

        let result = content.get_number_of_vertices();

        assert_eq!(result, expected_number_of_vertices);
    }

    #[test]
    fn when_content_gets_its_number_of_objects_then_the_number_of_objects_is_returned() {
        let mut object_instance_1 = create_mock_object_instance_runner(vec![], 0);
        object_instance_1
            .expect_get_number_of_objects()
            .times(1)
            .returning(move || 1);

        let mut object_instance_2 = create_mock_object_instance_runner(vec![], 0);
        object_instance_2
            .expect_get_number_of_objects()
            .times(1)
            .returning(move || 2);

        let mut object_instance_3 = create_mock_object_instance_runner(vec![], 0);
        object_instance_3
            .expect_get_number_of_objects()
            .times(1)
            .returning(move || 3);

        let objects = vec![];

        let mut object_instances = Vec::<Box<dyn RunObjectInstance>>::new();
        object_instances.push(Box::new(object_instance_1));
        object_instances.push(Box::new(object_instance_2));
        object_instances.push(Box::new(object_instance_3));

        let expected_number_of_objects = 6;

        let content = Content::new(objects, object_instances);

        let result = content.get_number_of_objects();

        assert_eq!(result, expected_number_of_objects);
    }

    #[test]
    fn when_two_d_point_gets_x_then_x_is_returned() {
        let x = 1.23;

        let y = 0.0;

        let two_d_point = TwoDPoint::new(x, y);

        let result = two_d_point.get_x();

        assert_eq!(result, x);
    }

    #[test]
    fn when_two_d_point_gets_y_then_y_is_returned() {
        let x = 0.0;

        let y = 1.23;

        let two_d_point = TwoDPoint::new(x, y);

        let result = two_d_point.get_y();

        assert_eq!(result, y);
    }

    #[test]
    fn when_two_d_point_gets_vertex_data_then_the_vertex_data_is_returned() {
        let x = 1.23;

        let y = 4.56;

        let two_d_point = TwoDPoint::new(x, y);

        let result = two_d_point.get_vertex_data();

        assert_eq!(result, [x, y]);
    }

    #[test]
    fn when_two_d_point_gets_number_of_vertices_then_the_number_of_vertices_is_returned() {
        let x = 0.0;

        let y = 0.0;

        let two_d_point = TwoDPoint::new(x, y);

        let result = two_d_point.get_number_of_vertices();

        assert_eq!(result, 2);
    }

    #[test]
    fn when_an_object_creator_creates_an_object_then_the_object_is_created() {
        let name = "RectangleName";

        let mut object_constructor = MockObjectConstructor::new();
        object_constructor
            .expect_construct_object()
            .times(1)
            .returning(move |_| Object::new(name.to_string()));

        let mut object_store = MockObjectStore::new();
        object_store
            .expect_store_object()
            .times(1)
            .returning(move |_| {});

        let object_creator = ObjectCreator::new(
            Rc::new(object_constructor),
            Rc::new(RefCell::new(object_store)),
        );

        let parameters = MockParameters::new();

        let result = object_creator.create_object(parameters);

        assert_eq!("object", result.borrow().get_name());
    }

    mock! {
        VertexObject {}
        impl GetVertexData for VertexObject {
            fn get_vertex_data(&self) -> Vec<f32>;
        }
        impl GetNumberOfVertices for VertexObject {
            fn get_number_of_vertices(&self) -> i32;
        }
        impl Scale for VertexObject {
            fn scale(&mut self, x: f32, y: f32);
        }
        impl GetNumberOfObjects for VertexObject {
            fn get_number_of_objects(&self) -> i32;
        }
        impl GetContentInstanceData for VertexObject {
        }
        impl GetX for VertexObject {
            fn get_x(&self) -> f32;
        }
        impl GetY for VertexObject {
            fn get_y(&self) -> f32;
        }
    }

    mock! {
        ObjectInstanceRunner {}
        impl RunObjectInstance for ObjectInstanceRunner {}
        impl GetContentInstanceData for ObjectInstanceRunner {}
        impl Scale for ObjectInstanceRunner {
            fn scale(&mut self, x: f32, y: f32) {}
        }
        impl GetNumberOfObjects for ObjectInstanceRunner {
            fn get_number_of_objects(&self) -> i32;
        }
        impl GetNumberOfVertices for ObjectInstanceRunner {
            fn get_number_of_vertices(&self) -> i32;
        }
        impl GetVertexData for ObjectInstanceRunner {
            fn get_vertex_data(&self) -> Vec<f32>;
        }
    }

    struct Object {
        name: String,
    }

    impl Object {
        fn new(name: String) -> Self {
            Self { name }
        }

        fn get_name(&self) -> &str {
            &self.name
        }
    }

    mock! {
        Parameters {}
    }

    mock! {
        ObjectConstructor<TObject, TParameters> {}
        impl<TObject, TParameters> ConstructObject<TObject, TParameters> for ObjectConstructor<TObject, TParameters> {
            fn construct_object(&self, parameters: TParameters) -> TObject;
        }
    }

    mock! {
        ObjectStore<T> {}
        impl<T> StoreObject<T> for ObjectStore<T> {
            fn store_object(&mut self, object: Rc::<RefCell::<T>>);
        }
    }

    fn create_mock_object_instance_runner(
        vertex_data: Vec<f32>,
        number_of_vertices: i32,
    ) -> MockObjectInstanceRunner {
        let mut mock_object_instance_runner = MockObjectInstanceRunner::new();

        mock_object_instance_runner
            .expect_get_vertex_data()
            .times(1)
            .returning(move || vertex_data.clone());
        mock_object_instance_runner
            .expect_get_number_of_vertices()
            .times(1)
            .returning(move || number_of_vertices);

        mock_object_instance_runner
    }
}
