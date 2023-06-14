use std::{cell::RefCell, marker::PhantomData, rc::Rc};

use garden::{GetHeight, GetName, GetWidth};

use crate::{
    triangles::CreateGeometryTriangles, ConstructObject, CreateObject, CreateTwoDPoint,
    Get2DCoordiantes, GetB, GetContent, GetContentInstanceData, GetG, GetNumberOfObjects,
    GetNumberOfVertices, GetPosition, GetR, GetRgb, GetRgbValues, GetScale, GetVertexData, Rgb,
    ScaleObjectInstance, StoreObject,
};

pub struct Rectangle<TRgb> {
    name: String,
    width: f32,
    height: f32,
    rgb: TRgb,
}

impl<TRgb> Rectangle<TRgb> {
    pub fn new(name: String, width: f32, height: f32, rgb: TRgb) -> Self {
        Self {
            name,
            width,
            height,
            rgb,
        }
    }
}

impl<TRgb> GetName for Rectangle<TRgb> {
    fn get_name(&self) -> &str {
        &self.name
    }
}

impl<TRgb> GetWidth for Rectangle<TRgb> {
    fn get_width(&self) -> f32 {
        self.width
    }
}

impl<TRgb> GetHeight for Rectangle<TRgb> {
    fn get_height(&self) -> f32 {
        self.height
    }
}

impl<TRgb> GetRgb<TRgb> for Rectangle<TRgb> {
    fn get_rgb(&self) -> &TRgb {
        &self.rgb
    }
}

impl<TRgb: GetR> GetR for Rectangle<TRgb> {
    fn get_r(&self) -> f32 {
        self.get_rgb().get_r()
    }
}

impl<TRgb: GetG> GetG for Rectangle<TRgb> {
    fn get_g(&self) -> f32 {
        self.get_rgb().get_g()
    }
}

impl<TRgb: GetB> GetB for Rectangle<TRgb> {
    fn get_b(&self) -> f32 {
        self.get_rgb().get_b()
    }
}

impl<TRgb: GetRgbValues> GetRgbValues for Rectangle<TRgb> {}

pub struct RectangleParameters<TRgb> {
    name: String,
    width: f32,
    height: f32,
    rgb: TRgb,
}

impl<TRgb> RectangleParameters<TRgb> {
    pub fn new(name: String, width: f32, height: f32, rgb: TRgb) -> Self {
        Self {
            name,
            width,
            height,
            rgb,
        }
    }
}

pub struct RectangleConstructor {}

impl RectangleConstructor {
    pub fn new() -> Self {
        Self {}
    }
}

impl<TRgb> ConstructObject<Rectangle<TRgb>, RectangleParameters<TRgb>> for RectangleConstructor {
    fn construct_object(&self, parameters: RectangleParameters<TRgb>) -> Rectangle<TRgb> {
        Rectangle::new(
            parameters.name,
            parameters.width,
            parameters.height,
            parameters.rgb,
        )
    }
}

pub struct RectangleInstanceParameters<TRectangle, TTwoDPoint> {
    name: String,
    rectangle: Rc<RefCell<TRectangle>>,
    scale: f32,
    position: TTwoDPoint,
    width: f32,
    height: f32,
}

impl<TRectangle, TTwoDPoint> RectangleInstanceParameters<TRectangle, TTwoDPoint> {
    pub fn new(
        name: String,
        rectangle: Rc<RefCell<TRectangle>>,
        scale: f32,
        position: TTwoDPoint,
        width: f32,
        height: f32,
    ) -> Self {
        Self {
            name,
            rectangle,
            scale,
            position,
            width,
            height,
        }
    }
}

pub struct RectangleInstance<TPosition, TRectangle, TGeometryTriangle> {
    name: String,
    rectangle: Rc<RefCell<TRectangle>>,
    scale: f32,
    position: TPosition,
    number_of_vertices: i32,
    vertex_data: Vec<f32>,
    geometry_triangles: Vec<TGeometryTriangle>,
}

impl<TPosition, TRectangle, TGeometryTriangle>
    RectangleInstance<TPosition, TRectangle, TGeometryTriangle>
{
    pub fn new(
        name: String,
        rectangle: Rc<RefCell<TRectangle>>,
        scale: f32,
        position: TPosition,
        number_of_vertices: i32,
        vertex_data: Vec<f32>,
        geometry_triangles: Vec<TGeometryTriangle>,
    ) -> Self {
        Self {
            name,
            rectangle,
            scale,
            position,
            vertex_data,
            number_of_vertices,
            geometry_triangles,
        }
    }
}

impl<TPosition, TRectangle, TGeometryTriangle> GetName
    for RectangleInstance<TPosition, TRectangle, TGeometryTriangle>
{
    fn get_name(&self) -> &str {
        &self.name
    }
}

impl<TPosition, TRectangle, TGeometryTriangle> GetVertexData
    for RectangleInstance<TPosition, TRectangle, TGeometryTriangle>
{
    fn get_vertex_data(&self) -> Vec<f32> {
        self.vertex_data.clone()
    }
}

impl<TPosition, TRectangle, TGeometryTriangle> GetNumberOfVertices
    for RectangleInstance<TPosition, TRectangle, TGeometryTriangle>
{
    fn get_number_of_vertices(&self) -> i32 {
        self.number_of_vertices
    }
}

impl<TPosition, TRectangle, TGeometryTriangle> GetNumberOfObjects
    for RectangleInstance<TPosition, TRectangle, TGeometryTriangle>
{
    fn get_number_of_objects(&self) -> i32 {
        2
    }
}

impl<TPosition, TRectangle, TGeometryTriangle> GetContentInstanceData
    for RectangleInstance<TPosition, TRectangle, TGeometryTriangle>
{
}

impl<TPosition, TRectangle, TGeometryTriangle> GetScale
    for RectangleInstance<TPosition, TRectangle, TGeometryTriangle>
{
    fn get_scale(&self) -> f32 {
        self.scale
    }
}

impl<TPosition, TRectangle: GetWidth, TGeometryTriangle> GetWidth
    for RectangleInstance<TPosition, TRectangle, TGeometryTriangle>
{
    fn get_width(&self) -> f32 {
        self.rectangle.borrow().get_width()
    }
}

impl<TPosition, TRectangle: GetHeight, TGeometryTriangle> GetHeight
    for RectangleInstance<TPosition, TRectangle, TGeometryTriangle>
{
    fn get_height(&self) -> f32 {
        self.rectangle.borrow().get_height()
    }
}

impl<TPosition, TRectangle, TGeometryTriangle> GetPosition<TPosition>
    for RectangleInstance<TPosition, TRectangle, TGeometryTriangle>
{
    fn get_position(&self) -> &TPosition {
        &self.position
    }
}

impl<TPosition, TRectangle: GetRgb<Rgb>, TGeometryTriangle> GetR
    for RectangleInstance<TPosition, TRectangle, TGeometryTriangle>
{
    fn get_r(&self) -> f32 {
        self.rectangle.borrow().get_rgb().get_r()
    }
}

impl<TPosition, TRectangle: GetRgb<Rgb>, TGeometryTriangle> GetG
    for RectangleInstance<TPosition, TRectangle, TGeometryTriangle>
{
    fn get_g(&self) -> f32 {
        self.rectangle.borrow().get_rgb().get_g()
    }
}

impl<TPosition, TRectangle: GetRgb<Rgb>, TGeometryTriangle> GetB
    for RectangleInstance<TPosition, TRectangle, TGeometryTriangle>
{
    fn get_b(&self) -> f32 {
        self.rectangle.borrow().get_rgb().get_b()
    }
}

impl<TPosition, TRectangle: GetRgb<Rgb>, TGeometryTriangle> GetRgbValues
    for RectangleInstance<TPosition, TRectangle, TGeometryTriangle>
{
}

impl<TPosition, TRectangle, TGeometryTriangle> GetRectangle<TRectangle>
    for RectangleInstance<TPosition, TRectangle, TGeometryTriangle>
{
    fn get_rectangle(&self) -> Rc<RefCell<TRectangle>> {
        Rc::clone(&self.rectangle)
    }
}

pub struct RectangleInstanceConstructor<TGeometryTrianglesCreator, TGeometryTriangle> {
    geometry_triangles_creator: Rc<TGeometryTrianglesCreator>,
    geometry_triangle_type: PhantomData<TGeometryTriangle>,
}

impl<TGeometryTrianglesCreator, TGeometryTriangle>
    RectangleInstanceConstructor<TGeometryTrianglesCreator, TGeometryTriangle>
{
    pub fn new(geometry_triangles_creator: Rc<TGeometryTrianglesCreator>) -> Self {
        Self {
            geometry_triangles_creator: geometry_triangles_creator,
            geometry_triangle_type: PhantomData,
        }
    }
}

impl<
        TRectangle: GetRgbValues,
        TGeometryTrianglesCreator: CreateGeometryTriangles<TGeometryTriangle, TRectangle, TTwoDPoint>,
        TTwoDPoint: Get2DCoordiantes,
        TGeometryTriangle: GetNumberOfVertices + GetVertexData,
    >
    ConstructObject<
        RectangleInstance<TTwoDPoint, TRectangle, TGeometryTriangle>,
        RectangleInstanceParameters<TRectangle, TTwoDPoint>,
    > for RectangleInstanceConstructor<TGeometryTrianglesCreator, TGeometryTriangle>
{
    fn construct_object(
        &self,
        parameters: RectangleInstanceParameters<TRectangle, TTwoDPoint>,
    ) -> RectangleInstance<TTwoDPoint, TRectangle, TGeometryTriangle> {
        let mut vertex_data = vec![];

        let mut number_of_vertices = 0;

        let geometry_triangles = self.geometry_triangles_creator.create_geometry_triangles(
            &parameters.rectangle.borrow(),
            &parameters.position,
            parameters.width,
            parameters.height,
        );

        for geometry_triangle in geometry_triangles.iter() {
            number_of_vertices += geometry_triangle.get_number_of_vertices();
            vertex_data.append(&mut geometry_triangle.get_vertex_data());
        }

        RectangleInstance::new(
            parameters.name,
            parameters.rectangle,
            parameters.scale,
            parameters.position,
            number_of_vertices,
            vertex_data,
            geometry_triangles,
        )
    }
}

pub struct RectangleInstanceScaler<
    TRectangleInstanceCreator,
    TTwoDPointCreator,
    TTwoDPoint,
    TRectangle,
> {
    rectangle_instance_creator: Rc<TRectangleInstanceCreator>,
    two_d_point_creator: Rc<TTwoDPointCreator>,
    two_d_point_type: PhantomData<TTwoDPoint>,
    rectangle_type: PhantomData<TRectangle>,
}

impl<TRectangleInstanceCreator, TTwoDPointCreator, TTwoDPoint, TRectangle>
    RectangleInstanceScaler<TRectangleInstanceCreator, TTwoDPointCreator, TTwoDPoint, TRectangle>
{
    pub fn new(
        rectangle_instance_creator: Rc<TRectangleInstanceCreator>,
        two_d_point_creator: Rc<TTwoDPointCreator>,
    ) -> Self {
        Self {
            rectangle_instance_creator: rectangle_instance_creator,
            two_d_point_creator: two_d_point_creator,
            two_d_point_type: PhantomData,
            rectangle_type: PhantomData,
        }
    }
}

impl<
        'ri,
        'r,
        TRectangleInstance: GetName
            + GetScale
            + GetPosition<TTwoDPoint>
            + GetWidth
            + GetHeight
            + GetRgbValues
            + GetRectangle<TRectangle>,
        TRectangleInstanceCreator: CreateObject<TRectangleInstance, RectangleInstanceParameters<TRectangle, TTwoDPoint>>,
        TTwoDPointCreator: CreateTwoDPoint<TTwoDPoint>,
        TTwoDPoint: Get2DCoordiantes,
        TRectangle,
    > ScaleObjectInstance<TRectangleInstance>
    for RectangleInstanceScaler<
        TRectangleInstanceCreator,
        TTwoDPointCreator,
        TTwoDPoint,
        TRectangle,
    >
{
    fn scale_object_instance(
        &self,
        rectangle_instance: Rc<RefCell<TRectangleInstance>>,
        x: f32,
        y: f32,
    ) -> Rc<RefCell<TRectangleInstance>> {
        self.rectangle_instance_creator
            .create_object(RectangleInstanceParameters::new(
                rectangle_instance.borrow().get_name().to_string(),
                rectangle_instance.borrow().get_rectangle(),
                rectangle_instance.borrow().get_scale(),
                self.two_d_point_creator.create_two_d_point(
                    rectangle_instance.borrow().get_position().get_x() / x,
                    rectangle_instance.borrow().get_position().get_y() / y,
                ),
                rectangle_instance.borrow().get_width() / x,
                rectangle_instance.borrow().get_height() / y,
            ))
    }
}

pub trait GetRectangle<TRectangle> {
    fn get_rectangle(&self) -> Rc<RefCell<TRectangle>>;
}

pub struct ContentProvider<TContent> {
    content: Vec<Rc<RefCell<TContent>>>,
}

impl<TContent> ContentProvider<TContent> {
    pub fn new(content: Vec<Rc<RefCell<TContent>>>) -> Self {
        Self { content }
    }
}

impl<'c, TContent: GetName> GetContent<TContent> for ContentProvider<TContent> {
    fn get_content(&self, content_name: String) -> Rc<RefCell<TContent>> {
        for content in self.content.iter() {
            if content.borrow().get_name() == content_name {
                return Rc::clone(&content);
            }
        }

        todo!()
    }
}

impl<TObject> StoreObject<TObject> for ContentProvider<TObject> {
    fn store_object(&mut self, content: Rc<RefCell<TObject>>) {
        self.content.push(content)
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;

    use garden::{GetHeight, GetName, GetWidth};
    use mockall::mock;

    use crate::{
        Get2DCoordiantes, GetB, GetG, GetNumberOfObjects, GetNumberOfVertices, GetR, GetRgb,
        GetScale, GetVertexData, GetX, GetY,
    };

    use crate::rectangles::{Rectangle, RectangleInstance};

    #[test]
    fn when_a_rectangle_gets_its_name_then_the_name_is_returned() {
        let name = "RectangleName";

        let width = 0.0;

        let height = 0.0;

        let rgb = MockRectangleRgb::new();

        let rectangle = Rectangle::<MockRectangleRgb>::new(name.to_string(), width, height, rgb);

        let result = rectangle.get_name();

        assert_eq!(name, result);
    }

    #[test]
    fn when_a_rectangle_gets_its_width_then_the_width_is_returned() {
        let name = "";

        let width = 123.45;

        let height = 0.0;

        let rgb = MockRectangleRgb::new();

        let rectangle = Rectangle::<MockRectangleRgb>::new(name.to_string(), width, height, rgb);

        let result = rectangle.get_width();

        assert_eq!(width, result);
    }

    #[test]
    fn when_a_rectangle_gets_its_height_then_the_height_is_returned() {
        let name = "";

        let width = 0.0;

        let height = 123.45;

        let rgb = MockRectangleRgb::new();

        let rectangle = Rectangle::<MockRectangleRgb>::new(name.to_string(), width, height, rgb);

        let result = rectangle.get_height();

        assert_eq!(height, result);
    }

    #[test]
    fn when_a_rectangle_gets_its_rgb_then_the_rgb_is_returned() {
        let name = "";

        let width = 0.0;

        let height = 123.45;

        let mut rgb = MockRectangleRgb::new();
        rgb.expect_get_r().returning(move || 1.23);
        rgb.expect_get_g().returning(move || 4.56);
        rgb.expect_get_b().returning(move || 7.89);

        let rectangle = Rectangle::<MockRectangleRgb>::new(name.to_string(), width, height, rgb);

        let result = rectangle.get_rgb();

        assert_eq!(1.23, result.get_r());
        assert_eq!(4.56, result.get_g());
        assert_eq!(7.89, result.get_b());
    }

    #[test]
    fn when_a_rectangle_instance_gets_its_name_then_the_name_is_returned() {
        let name = "RectangleInstanceName";

        let rectangle = Rc::new(RefCell::new(MockRectangle::new()));

        let scale = 0.0;

        let position = MockRectanglePosition::new();

        let number_of_vertices = 0;

        let vertex_data = vec![];

        let geometry_triangles = Vec::<MockGeometryTriangle>::new();

        let rectangle_instance = RectangleInstance::new(
            name.to_string(),
            rectangle,
            scale,
            position,
            number_of_vertices,
            vertex_data,
            geometry_triangles,
        );

        let result = rectangle_instance.get_name();

        assert_eq!(name, result);
    }

    #[test]
    fn when_a_rectangle_instance_gets_its_vertex_data_then_the_vertex_data_is_returned() {
        let name = "";

        let rectangle = Rc::new(RefCell::new(MockRectangle::new()));

        let scale = 0.0;

        let position = MockRectanglePosition::new();

        let number_of_vertices = 0;

        let vertex_data = vec![1.23, 4.56, 7.89];

        let geometry_triangles = Vec::<MockGeometryTriangle>::new();

        let rectangle_instance = RectangleInstance::new(
            name.to_string(),
            rectangle,
            scale,
            position,
            number_of_vertices,
            vertex_data.clone(),
            geometry_triangles,
        );

        let result = rectangle_instance.get_vertex_data();

        assert_eq!(vertex_data, result);
    }

    #[test]
    fn when_a_rectangle_instance_gets_its_number_of_vertices_then_the_number_of_vertices_is_returned(
    ) {
        let name = "";

        let rectangle = Rc::new(RefCell::new(MockRectangle::new()));

        let scale = 0.0;

        let position = MockRectanglePosition::new();

        let number_of_vertices = 123;

        let vertex_data = vec![];

        let geometry_triangles = Vec::<MockGeometryTriangle>::new();

        let rectangle_instance = RectangleInstance::new(
            name.to_string(),
            rectangle,
            scale,
            position,
            number_of_vertices,
            vertex_data.clone(),
            geometry_triangles,
        );

        let result = rectangle_instance.get_number_of_vertices();

        assert_eq!(number_of_vertices, result);
    }

    #[test]
    fn when_a_rectangle_instance_gets_its_number_of_objects_then_the_number_of_objects_is_returned()
    {
        let name = "";

        let rectangle = Rc::new(RefCell::new(MockRectangle::new()));

        let scale = 0.0;

        let position = MockRectanglePosition::new();

        let number_of_vertices = 0;

        let vertex_data = vec![];

        let geometry_triangles = Vec::<MockGeometryTriangle>::new();

        let rectangle_instance = RectangleInstance::new(
            name.to_string(),
            rectangle,
            scale,
            position,
            number_of_vertices,
            vertex_data.clone(),
            geometry_triangles,
        );

        let result = rectangle_instance.get_number_of_objects();

        assert_eq!(2, result);
    }

    #[test]
    fn when_a_rectangle_instance_gets_its_scale_then_the_scale_is_returned() {
        let name = "";

        let rectangle = Rc::new(RefCell::new(MockRectangle::new()));

        let scale = 1.23;

        let position = MockRectanglePosition::new();

        let number_of_vertices = 0;

        let vertex_data = vec![];

        let geometry_triangles = Vec::<MockGeometryTriangle>::new();

        let rectangle_instance = RectangleInstance::new(
            name.to_string(),
            rectangle,
            scale,
            position,
            number_of_vertices,
            vertex_data.clone(),
            geometry_triangles,
        );

        let result = rectangle_instance.get_scale();

        assert_eq!(scale, result);
    }

    mock! {
        RectangleRgb {}
        impl GetR for RectangleRgb{
            fn get_r(&self) -> f32;
        }
        impl GetG for RectangleRgb{
            fn get_g(&self) -> f32;
        }
        impl GetB for RectangleRgb{
            fn get_b(&self) -> f32;
        }
    }

    mock! {
        RectanglePosition {}
        impl GetX for RectanglePosition {
            fn get_x(&self) -> f32;
        }
        impl GetY for RectanglePosition {
            fn get_y(&self) -> f32;
        }
        impl Get2DCoordiantes for RectanglePosition {}
    }

    mock! {
        GeometryTriangle {}
    }

    mock! {
        Rectangle {}
    }
}
