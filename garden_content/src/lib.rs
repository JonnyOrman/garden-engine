use std::rc::Rc;

use garden::GetName;

pub trait GetVertexData {
    fn get_vertex_data(&self) -> Vec<f32>;
}

pub trait GetNumberOfVertices {
    fn get_number_of_vertices(&self) -> i32;
}

pub trait Scale {
    fn scale(&mut self, x: f32, y: f32);
}

pub trait GetContentInstanceData:
    GetVertexData + GetNumberOfVertices + Scale + GetNumberOfObjects
{
}

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

pub trait GetR {
    fn get_r(&self) -> f32;
}

pub trait GetG {
    fn get_g(&self) -> f32;
}

pub trait GetB {
    fn get_b(&self) -> f32;
}

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

pub trait GetRgb<TRgb> {
    fn get_rgb(&self) -> &TRgb;
}

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
    pub fn new(point: TTwoDPoint, rgb: TRgb) -> Self {
        let mut vertex_data = vec![];

        vertex_data.append(&mut point.get_vertex_data());
        vertex_data.append(&mut rgb.get_vertex_data());

        let number_of_vertices = point.get_number_of_vertices() + rgb.get_number_of_vertices();

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

impl<TTwoDPoint: GetY, TRgb> GetRgb<TRgb> for TrianglePoint<TTwoDPoint, TRgb> {
    fn get_rgb(&self) -> &TRgb {
        &self.rgb
    }
}

pub struct Triangle<TTrianglePoint> {
    name: String,
    point_1: TTrianglePoint,
    point_2: TTrianglePoint,
    point_3: TTrianglePoint,
    number_of_vertices: i32,
    vertex_data: Vec<f32>,
}

impl<TTrianglePoint: GetVertexData + GetNumberOfVertices> Triangle<TTrianglePoint> {
    pub fn new(
        name: String,
        point_1: TTrianglePoint,
        point_2: TTrianglePoint,
        point_3: TTrianglePoint,
    ) -> Self {
        let mut vertex_data = vec![];

        vertex_data.append(&mut point_1.get_vertex_data().clone());
        vertex_data.append(&mut point_2.get_vertex_data().clone());
        vertex_data.append(&mut point_3.get_vertex_data().clone());

        let number_of_vertices = point_1.get_number_of_vertices()
            + point_2.get_number_of_vertices()
            + point_3.get_number_of_vertices();

        Self {
            name,
            point_1,
            point_2,
            point_3,
            number_of_vertices,
            vertex_data,
        }
    }

    pub fn get_point_1(&self) -> &TTrianglePoint {
        &self.point_1
    }

    pub fn get_point_2(&self) -> &TTrianglePoint {
        &self.point_2
    }

    pub fn get_point_3(&self) -> &TTrianglePoint {
        &self.point_3
    }
}

impl<TTrianglePoint> GetName for Triangle<TTrianglePoint> {
    fn get_name(&self) -> &str {
        &self.name
    }
}

impl<TTrianglePoint> GetVertexData for Triangle<TTrianglePoint> {
    fn get_vertex_data(&self) -> Vec<f32> {
        self.vertex_data.clone()
    }
}

impl<TTrianglePoint> GetNumberOfVertices for Triangle<TTrianglePoint> {
    fn get_number_of_vertices(&self) -> i32 {
        self.number_of_vertices
    }
}

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

pub trait GetScale {
    fn get_scale(&self) -> f32;
}

pub trait GetPosition<TPosition> {
    fn get_position(&self) -> &TPosition;
}

pub struct TriangleInstance<TPosition, TTrianglePoint> {
    name: String,
    contentName: String,
    scale: f32,
    position: TPosition,
    point_1: TTrianglePoint,
    point_2: TTrianglePoint,
    point_3: TTrianglePoint,
    number_of_vertices: i32,
    vertex_data: Vec<f32>,
}

impl<TPosition, TTrianglePoint: GetVertexData + GetNumberOfVertices>
    TriangleInstance<TPosition, TTrianglePoint>
{
    pub fn new(
        name: String,
        contentName: String,
        scale: f32,
        position: TPosition,
        point_1: TTrianglePoint,
        point_2: TTrianglePoint,
        point_3: TTrianglePoint,
    ) -> Self {
        let mut vertex_data = vec![];

        vertex_data.append(&mut point_1.get_vertex_data().clone());
        vertex_data.append(&mut point_2.get_vertex_data().clone());
        vertex_data.append(&mut point_3.get_vertex_data().clone());

        let number_of_vertices = point_1.get_number_of_vertices()
            + point_2.get_number_of_vertices()
            + point_3.get_number_of_vertices();

        Self {
            name,
            contentName,
            scale,
            position,
            point_1,
            point_2,
            point_3,
            number_of_vertices,
            vertex_data,
        }
    }

    pub fn get_content_name(&self) -> &str {
        &self.contentName
    }

    pub fn get_point_1(&self) -> &TTrianglePoint {
        &self.point_1
    }

    pub fn get_point_2(&self) -> &TTrianglePoint {
        &self.point_2
    }

    pub fn get_point_3(&self) -> &TTrianglePoint {
        &self.point_3
    }
}

impl<TPosition, TTrianglePoint> GetName for TriangleInstance<TPosition, TTrianglePoint> {
    fn get_name(&self) -> &str {
        &self.name
    }
}

impl<TPosition, TTrianglePoint> GetScale for TriangleInstance<TPosition, TTrianglePoint> {
    fn get_scale(&self) -> f32 {
        self.scale
    }
}

impl<TPosition, TTrianglePoint> GetPosition<TPosition>
    for TriangleInstance<TPosition, TTrianglePoint>
{
    fn get_position(&self) -> &TPosition {
        &self.position
    }
}

impl<TPosition, TTrianglePoint> GetVertexData for TriangleInstance<TPosition, TTrianglePoint> {
    fn get_vertex_data(&self) -> Vec<f32> {
        self.vertex_data.clone()
    }
}

impl<TPosition, TTrianglePoint> GetNumberOfVertices
    for TriangleInstance<TPosition, TTrianglePoint>
{
    fn get_number_of_vertices(&self) -> i32 {
        self.number_of_vertices
    }
}

impl Scale for TriangleInstance<TwoDPoint, TrianglePoint<TwoDPoint, Rgb>> {
    fn scale(&mut self, x: f32, y: f32) {
        let new_position = TwoDPoint::new(
            self.get_position().get_x() / x,
            self.get_position().get_y() / y,
        );
        let new_point_1 = TrianglePoint::<TwoDPoint, Rgb>::new(
            TwoDPoint::new(
                self.get_point_1().get_x() / x,
                self.get_point_1().get_y() / y,
            ),
            Rgb::new(
                self.get_point_1().get_rgb().get_r(),
                self.get_point_1().get_rgb().get_g(),
                self.get_point_1().get_rgb().get_b(),
            ),
        );
        let new_point_2 = TrianglePoint::<TwoDPoint, Rgb>::new(
            TwoDPoint::new(
                self.get_point_2().get_x() / x,
                self.get_point_2().get_y() / y,
            ),
            Rgb::new(
                self.get_point_2().get_rgb().get_r(),
                self.get_point_2().get_rgb().get_g(),
                self.get_point_2().get_rgb().get_b(),
            ),
        );
        let new_point_3 = TrianglePoint::<TwoDPoint, Rgb>::new(
            TwoDPoint::new(
                self.get_point_3().get_x() / x,
                self.get_point_3().get_y() / y,
            ),
            Rgb::new(
                self.get_point_3().get_rgb().get_r(),
                self.get_point_3().get_rgb().get_g(),
                self.get_point_3().get_rgb().get_b(),
            ),
        );

        let mut new_vertex_data = vec![];

        new_vertex_data.append(&mut new_point_1.get_vertex_data().clone());
        new_vertex_data.append(&mut new_point_2.get_vertex_data().clone());
        new_vertex_data.append(&mut new_point_3.get_vertex_data().clone());

        self.position = new_position;
        self.point_1 = new_point_1;
        self.point_2 = new_point_2;
        self.point_3 = new_point_3;

        self.vertex_data = new_vertex_data;
    }
}

impl GetNumberOfObjects for TriangleInstance<TwoDPoint, TrianglePoint<TwoDPoint, Rgb>> {
    fn get_number_of_objects(&self) -> i32 {
        1
    }
}

impl GetContentInstanceData for TriangleInstance<TwoDPoint, TrianglePoint<TwoDPoint, Rgb>> {}

pub struct RectangleInstance<TPosition, TPoint, TTriangle, TRgb> {
    name: String,
    content_name: String,
    scale: f32,
    position: TPosition,
    number_of_vertices: i32,
    vertex_data: Vec<f32>,
    point_1: TPoint,
    point_2: TPoint,
    point_3: TPoint,
    point_4: TPoint,
    triangle_instance_1: TTriangle,
    triangle_instance_2: TTriangle,
    rgb: TRgb,
}

impl<TPosition: Get2DCoordiantes, TRgb: GetR + GetG + GetB>
    RectangleInstance<
        TPosition,
        TrianglePoint<TwoDPoint, Rgb>,
        TriangleInstance<TwoDPoint, TrianglePoint<TwoDPoint, Rgb>>,
        TRgb,
    >
{
    pub fn new(
        name: String,
        content_name: String,
        scale: f32,
        position: TPosition,
        width: f32,
        height: f32,
        rgb: TRgb,
    ) -> Self {
        let mut vertex_data = vec![];

        let x = width / 2.0;
        let y = height / 2.0;

        let point_1 = TrianglePoint::new(
            TwoDPoint::new(position.get_x() + x, position.get_y() + y),
            Rgb::new(rgb.get_r(), rgb.get_g(), rgb.get_b()),
        );
        let point_2 = TrianglePoint::new(
            TwoDPoint::new(position.get_x() - x, position.get_y() + y),
            Rgb::new(rgb.get_r(), rgb.get_g(), rgb.get_b()),
        );
        let point_3 = TrianglePoint::new(
            TwoDPoint::new(position.get_x() - x, position.get_y() - y),
            Rgb::new(rgb.get_r(), rgb.get_g(), rgb.get_b()),
        );
        let point_4 = TrianglePoint::new(
            TwoDPoint::new(position.get_x() + x, position.get_y() - y),
            Rgb::new(rgb.get_r(), rgb.get_g(), rgb.get_b()),
        );

        let triangle_instance_1 = TriangleInstance::new(
            name.clone() + "-triangle-1",
            "".to_string(),
            scale,
            TwoDPoint::new(0.0, 0.0),
            TrianglePoint::new(
                TwoDPoint::new(point_1.get_x(), point_1.get_y()),
                Rgb::new(
                    point_1.get_rgb().get_r(),
                    point_1.get_rgb().get_g(),
                    point_1.get_rgb().get_b(),
                ),
            ),
            TrianglePoint::new(
                TwoDPoint::new(point_2.get_x(), point_2.get_y()),
                Rgb::new(
                    point_2.get_rgb().get_r(),
                    point_2.get_rgb().get_g(),
                    point_2.get_rgb().get_b(),
                ),
            ),
            TrianglePoint::new(
                TwoDPoint::new(point_3.get_x(), point_3.get_y()),
                Rgb::new(
                    point_3.get_rgb().get_r(),
                    point_3.get_rgb().get_g(),
                    point_3.get_rgb().get_b(),
                ),
            ),
        );

        let triangle_instance_2 = TriangleInstance::new(
            name.clone() + "-triangle-2",
            "".to_string(),
            scale,
            TwoDPoint::new(0.0, 0.0),
            TrianglePoint::new(
                TwoDPoint::new(point_1.get_x(), point_1.get_y()),
                Rgb::new(
                    point_1.get_rgb().get_r(),
                    point_1.get_rgb().get_g(),
                    point_1.get_rgb().get_b(),
                ),
            ),
            TrianglePoint::new(
                TwoDPoint::new(point_3.get_x(), point_3.get_y()),
                Rgb::new(
                    point_3.get_rgb().get_r(),
                    point_3.get_rgb().get_g(),
                    point_3.get_rgb().get_b(),
                ),
            ),
            TrianglePoint::new(
                TwoDPoint::new(point_4.get_x(), point_4.get_y()),
                Rgb::new(
                    point_4.get_rgb().get_r(),
                    point_4.get_rgb().get_g(),
                    point_4.get_rgb().get_b(),
                ),
            ),
        );

        vertex_data.append(&mut triangle_instance_1.get_vertex_data().clone());
        vertex_data.append(&mut triangle_instance_2.get_vertex_data().clone());

        let number_of_vertices = triangle_instance_1.get_number_of_vertices()
            + triangle_instance_2.get_number_of_vertices();

        Self {
            name,
            content_name,
            scale,
            position,
            vertex_data,
            number_of_vertices,
            point_1,
            point_2,
            point_3,
            point_4,
            triangle_instance_1,
            triangle_instance_2,
            rgb,
        }
    }
}

impl<TPosition, TPoint, TRgb> GetVertexData
    for RectangleInstance<
        TPosition,
        TPoint,
        TriangleInstance<TwoDPoint, TrianglePoint<TwoDPoint, Rgb>>,
        TRgb,
    >
{
    fn get_vertex_data(&self) -> Vec<f32> {
        self.vertex_data.clone()
    }
}

impl<TPosition, TPoint, TRgb> GetNumberOfVertices
    for RectangleInstance<
        TPosition,
        TPoint,
        TriangleInstance<TwoDPoint, TrianglePoint<TwoDPoint, Rgb>>,
        TRgb,
    >
{
    fn get_number_of_vertices(&self) -> i32 {
        self.number_of_vertices
    }
}

impl<TPosition, TRgb> Scale
    for RectangleInstance<
        TPosition,
        TrianglePoint<TwoDPoint, Rgb>,
        TriangleInstance<TwoDPoint, TrianglePoint<TwoDPoint, Rgb>>,
        TRgb,
    >
{
    fn scale(&mut self, x: f32, y: f32) {
        let mut new_vertex_data = vec![];

        let new_point_1 = TrianglePoint::new(
            TwoDPoint::new(self.point_1.get_x() / x, self.point_1.get_y() / y),
            Rgb::new(
                self.point_1.get_rgb().get_r(),
                self.point_1.get_rgb().get_g(),
                self.point_1.get_rgb().get_b(),
            ),
        );
        let new_point_2 = TrianglePoint::new(
            TwoDPoint::new(self.point_2.get_x() / x, self.point_2.get_y() / y),
            Rgb::new(
                self.point_2.get_rgb().get_r(),
                self.point_2.get_rgb().get_g(),
                self.point_2.get_rgb().get_b(),
            ),
        );
        let new_point_3 = TrianglePoint::new(
            TwoDPoint::new(self.point_3.get_x() / x, self.point_3.get_y() / y),
            Rgb::new(
                self.point_3.get_rgb().get_r(),
                self.point_3.get_rgb().get_g(),
                self.point_3.get_rgb().get_b(),
            ),
        );
        let new_point_4 = TrianglePoint::new(
            TwoDPoint::new(self.point_4.get_x() / x, self.point_4.get_y() / y),
            Rgb::new(
                self.point_4.get_rgb().get_r(),
                self.point_4.get_rgb().get_g(),
                self.point_4.get_rgb().get_b(),
            ),
        );

        let new_triangle_instance_1 = TriangleInstance::new(
            self.name.clone() + "-triangle-1",
            "".to_string(),
            self.scale,
            TwoDPoint::new(0.0, 0.0),
            TrianglePoint::new(
                TwoDPoint::new(new_point_1.get_x(), new_point_1.get_y()),
                Rgb::new(
                    new_point_1.get_rgb().get_r(),
                    new_point_1.get_rgb().get_g(),
                    new_point_1.get_rgb().get_b(),
                ),
            ),
            TrianglePoint::new(
                TwoDPoint::new(new_point_2.get_x(), new_point_2.get_y()),
                Rgb::new(
                    new_point_2.get_rgb().get_r(),
                    new_point_2.get_rgb().get_g(),
                    new_point_2.get_rgb().get_b(),
                ),
            ),
            TrianglePoint::new(
                TwoDPoint::new(new_point_3.get_x(), new_point_3.get_y()),
                Rgb::new(
                    new_point_3.get_rgb().get_r(),
                    new_point_3.get_rgb().get_g(),
                    new_point_3.get_rgb().get_b(),
                ),
            ),
        );

        let new_triangle_instance_2 = TriangleInstance::new(
            self.name.clone() + "-triangle-2",
            "".to_string(),
            self.scale,
            TwoDPoint::new(0.0, 0.0),
            TrianglePoint::new(
                TwoDPoint::new(new_point_1.get_x(), new_point_1.get_y()),
                Rgb::new(
                    new_point_1.get_rgb().get_r(),
                    new_point_1.get_rgb().get_g(),
                    new_point_1.get_rgb().get_b(),
                ),
            ),
            TrianglePoint::new(
                TwoDPoint::new(new_point_3.get_x(), new_point_3.get_y()),
                Rgb::new(
                    new_point_3.get_rgb().get_r(),
                    new_point_3.get_rgb().get_g(),
                    new_point_3.get_rgb().get_b(),
                ),
            ),
            TrianglePoint::new(
                TwoDPoint::new(new_point_4.get_x(), new_point_4.get_y()),
                Rgb::new(
                    new_point_4.get_rgb().get_r(),
                    new_point_4.get_rgb().get_g(),
                    new_point_4.get_rgb().get_b(),
                ),
            ),
        );

        new_vertex_data.append(&mut new_triangle_instance_1.get_vertex_data().clone());
        new_vertex_data.append(&mut new_triangle_instance_2.get_vertex_data().clone());

        let number_of_vertices = new_triangle_instance_1.get_number_of_vertices()
            + new_triangle_instance_2.get_number_of_vertices();

        self.triangle_instance_1 = new_triangle_instance_1;
        self.triangle_instance_2 = new_triangle_instance_2;

        self.number_of_vertices = number_of_vertices;

        self.vertex_data = new_vertex_data;
    }
}

impl<TPosition, TRgb> GetNumberOfObjects
    for RectangleInstance<
        TPosition,
        TrianglePoint<TwoDPoint, Rgb>,
        TriangleInstance<TwoDPoint, TrianglePoint<TwoDPoint, Rgb>>,
        TRgb,
    >
{
    fn get_number_of_objects(&self) -> i32 {
        2
    }
}

impl<TPosition, TRgb> GetContentInstanceData
    for RectangleInstance<
        TPosition,
        TrianglePoint<TwoDPoint, Rgb>,
        TriangleInstance<TwoDPoint, TrianglePoint<TwoDPoint, Rgb>>,
        TRgb,
    >
{
}

pub struct Content {
    objects: Option<Vec<Rc<Box<dyn GetName>>>>,
    object_instances: Option<Vec<Box<dyn GetContentInstanceData>>>,
    vertex_data: Vec<f32>,
    number_of_vertices: i32,
    number_of_objects: i32,
}

impl Content {
    pub fn new(
        objects: Vec<Rc<Box<dyn GetName>>>,
        object_instances: Vec<Box<dyn GetContentInstanceData>>,
    ) -> Self {
        let mut number_of_vertices = 0;

        let mut vertex_data = vec![];

        let mut number_of_objects = 0;

        for object_instance in object_instances.iter() {
            number_of_vertices += object_instance.get_number_of_vertices();
            vertex_data.append(&mut object_instance.get_vertex_data());
            number_of_objects += object_instance.get_number_of_objects();
        }

        Self {
            objects: Some(objects),
            object_instances: Some(object_instances),
            vertex_data,
            number_of_vertices,
            number_of_objects,
        }
    }

    pub fn get_objects(&self) -> &Option<Vec<Rc<Box<dyn GetName>>>> {
        &self.objects
    }

    pub fn get_object_instances(&self) -> &Option<Vec<Box<dyn GetContentInstanceData>>> {
        &self.object_instances
    }

    pub fn scale_object_instances(&mut self, x: f32, y: f32) {
        let mut new_object_instances = Vec::<Box<dyn GetContentInstanceData>>::new();

        let mut new_vertex_data = vec![];

        let mut i = self.object_instances.as_ref().unwrap().len();

        let mut object_instances = self.object_instances.take().unwrap();

        while i > 0 {
            let mut t = object_instances.remove(i - 1);
            t.scale(x, y);
            new_vertex_data.append(&mut t.get_vertex_data());
            new_object_instances.push(t);
            i = i - 1;
        }

        self.object_instances = Some(new_object_instances);

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

#[cfg(test)]
mod tests {
    use garden::GetName;
    use mockall::mock;

    use crate::{
        Content, GetContentInstanceData, GetNumberOfObjects, GetNumberOfVertices, GetVertexData,
        GetX, GetY, Rgb, Triangle, TriangleInstance, TrianglePoint, TwoDPoint,
    };

    #[test]
    fn when_a_two_d_point_gets_its_vertex_data_then_the_vertex_data_is_returned() {
        let x = 1.0;
        let y = -0.5;

        let expected_vertex_data = vec![x, y];

        let two_d_point = TwoDPoint::new(x, y);

        let result = two_d_point.get_vertex_data();

        assert_eq!(result, expected_vertex_data);
    }

    #[test]
    fn when_a_two_d_point_gets_its_number_of_vertices_then_the_number_of_vertices_is_returned() {
        let x = 1.0;
        let y = -0.5;

        let expected_number_of_vertices = 2;

        let two_d_point = TwoDPoint::new(x, y);

        let result = two_d_point.get_number_of_vertices();

        assert_eq!(result, expected_number_of_vertices);
    }

    #[test]
    fn when_a_rgb_gets_its_vertex_data_then_the_vertex_data_is_returned() {
        let r = 1.0;
        let g = 0.5;
        let b = 0.0;

        let expected_vertex_data = vec![r, g, b];

        let rgb = Rgb::new(r, g, b);

        let result = rgb.get_vertex_data();

        assert_eq!(result, expected_vertex_data);
    }

    #[test]
    fn when_a_rgb_gets_its_number_of_vertices_then_the_number_of_vertices_is_returned() {
        let r = 1.0;
        let g = 0.5;
        let b = 0.0;

        let expected_number_of_vertices = 3;

        let rgb = Rgb::new(r, g, b);

        let result = rgb.get_number_of_vertices();

        assert_eq!(result, expected_number_of_vertices);
    }

    #[test]
    fn when_a_triangle_point_gets_its_vertex_data_then_the_vertex_data_is_returned() {
        let x = 1.0;
        let y = -0.5;
        let r = 1.0;
        let g = 0.5;
        let b = 0.0;

        let two_d_point = create_mock_vertex_object(vec![x, y], 0);

        let rgb = create_mock_vertex_object(vec![r, g, b], 0);

        let expected_vertex_data = vec![x, y, r, g, b];

        let triangle_point =
            TrianglePoint::<MockVertexObject, MockVertexObject>::new(two_d_point, rgb);

        let result = triangle_point.get_vertex_data();

        assert_eq!(result, expected_vertex_data);
    }

    #[test]
    fn when_a_triangle_point_gets_its_number_of_vertices_then_the_number_of_vertices_is_returned() {
        let two_d_point = create_mock_vertex_object(vec![], 2);

        let rgb = create_mock_vertex_object(vec![], 3);

        let expected_number_of_vertices = 5;

        let triangle_point =
            TrianglePoint::<MockVertexObject, MockVertexObject>::new(two_d_point, rgb);

        let result = triangle_point.get_number_of_vertices();

        assert_eq!(result, expected_number_of_vertices);
    }

    #[test]
    fn when_a_triangle_point_gets_x_then_point_x_is_returned() {
        let x = 1.23;

        let mut two_d_point = create_mock_vertex_object(vec![], 2);
        two_d_point.expect_get_x().times(1).returning(move || x);

        let rgb = create_mock_vertex_object(vec![], 3);

        let triangle_point =
            TrianglePoint::<MockVertexObject, MockVertexObject>::new(two_d_point, rgb);

        let result = triangle_point.get_x();

        assert_eq!(result, x);
    }

    #[test]
    fn when_a_triangle_point_gets_y_then_point_y_is_returned() {
        let y = 1.23;

        let mut two_d_point = create_mock_vertex_object(vec![], 2);
        two_d_point.expect_get_y().times(1).returning(move || y);

        let rgb = create_mock_vertex_object(vec![], 3);

        let triangle_point =
            TrianglePoint::<MockVertexObject, MockVertexObject>::new(two_d_point, rgb);

        let result = triangle_point.get_y();

        assert_eq!(result, y);
    }

    #[test]
    fn when_a_triangle_gets_its_name_then_the_name_is_returned() {
        let triangle_name = "SomeTriangle";

        let triangle_point_1 = create_mock_vertex_object(vec![], 0);

        let triangle_point_2 = create_mock_vertex_object(vec![], 0);

        let triangle_point_3 = create_mock_vertex_object(vec![], 0);

        let triangle = Triangle::<MockVertexObject>::new(
            triangle_name.to_string(),
            triangle_point_1,
            triangle_point_2,
            triangle_point_3,
        );

        let result = triangle.get_name();

        assert_eq!(result, triangle_name);
    }

    #[test]
    fn when_a_triangle_gets_its_vertex_data_then_the_vertex_data_is_returned() {
        let triangle_name = "SomeTriangle";

        let triangle_point_1_x = 0.0;
        let triangle_point_1_y = 0.5;
        let triangle_point_1_r = 1.0;
        let triangle_point_1_g = 0.0;
        let triangle_point_1_b = 0.0;

        let triangle_point_2_x = 0.5;
        let triangle_point_2_y = 1.0;
        let triangle_point_2_r = 0.0;
        let triangle_point_2_g = 1.0;
        let triangle_point_2_b = 0.0;

        let triangle_point_3_x = 1.0;
        let triangle_point_3_y = 0.0;
        let triangle_point_3_r = 0.0;
        let triangle_point_3_g = 0.0;
        let triangle_point_3_b = 1.0;

        let triangle_point_1 = create_mock_vertex_object(
            vec![
                triangle_point_1_x,
                triangle_point_1_y,
                triangle_point_1_r,
                triangle_point_1_g,
                triangle_point_1_b,
            ],
            0,
        );

        let triangle_point_2 = create_mock_vertex_object(
            vec![
                triangle_point_2_x,
                triangle_point_2_y,
                triangle_point_2_r,
                triangle_point_2_g,
                triangle_point_2_b,
            ],
            0,
        );

        let triangle_point_3 = create_mock_vertex_object(
            vec![
                triangle_point_3_x,
                triangle_point_3_y,
                triangle_point_3_r,
                triangle_point_3_g,
                triangle_point_3_b,
            ],
            0,
        );

        let expected_vertex_data = vec![
            triangle_point_1_x,
            triangle_point_1_y,
            triangle_point_1_r,
            triangle_point_1_g,
            triangle_point_1_b,
            triangle_point_2_x,
            triangle_point_2_y,
            triangle_point_2_r,
            triangle_point_2_g,
            triangle_point_2_b,
            triangle_point_3_x,
            triangle_point_3_y,
            triangle_point_3_r,
            triangle_point_3_g,
            triangle_point_3_b,
        ];

        let triangle = Triangle::<MockVertexObject>::new(
            triangle_name.to_string(),
            triangle_point_1,
            triangle_point_2,
            triangle_point_3,
        );

        let result = triangle.get_vertex_data();

        assert_eq!(result, expected_vertex_data);
    }

    #[test]
    fn when_a_triangle_gets_its_number_of_vertices_then_the_number_of_vertices_is_returned() {
        let triangle_name = "SomeTriangle";

        let triangle_point_1 = create_mock_vertex_object(vec![], 5);

        let triangle_point_2 = create_mock_vertex_object(vec![], 5);

        let triangle_point_3 = create_mock_vertex_object(vec![], 5);

        let expected_number_of_vertices = 15;

        let triangle = Triangle::<MockVertexObject>::new(
            triangle_name.to_string(),
            triangle_point_1,
            triangle_point_2,
            triangle_point_3,
        );

        let result = triangle.get_number_of_vertices();

        assert_eq!(result, expected_number_of_vertices);
    }

    #[test]
    fn when_a_triangle_instance_gets_its_name_then_the_name_is_returned() {
        let triangle_instance_name = "SomeTriangle";

        let triangle_name = "";

        let scale = 0.5;

        let position = MockVertexObject::new();

        let triangle_point_1 = create_mock_vertex_object(vec![], 0);

        let triangle_point_2 = create_mock_vertex_object(vec![], 0);

        let triangle_point_3 = create_mock_vertex_object(vec![], 0);

        let triangle_instance = TriangleInstance::<MockVertexObject, MockVertexObject>::new(
            triangle_instance_name.to_string(),
            triangle_name.to_string(),
            scale,
            position,
            triangle_point_1,
            triangle_point_2,
            triangle_point_3,
        );

        let result = triangle_instance.get_name();

        assert_eq!(result, triangle_instance_name);
    }

    #[test]
    fn when_a_triangle_instance_gets_its_content_name_then_the_name_is_returned() {
        let triangle_instance_name = "";

        let triangle_name = "SomeContent";

        let scale = 0.5;

        let position = MockVertexObject::new();

        let triangle_point_1 = create_mock_vertex_object(vec![], 0);

        let triangle_point_2 = create_mock_vertex_object(vec![], 0);

        let triangle_point_3 = create_mock_vertex_object(vec![], 0);

        let triangle_instance = TriangleInstance::<MockVertexObject, MockVertexObject>::new(
            triangle_instance_name.to_string(),
            triangle_name.to_string(),
            scale,
            position,
            triangle_point_1,
            triangle_point_2,
            triangle_point_3,
        );

        let result = triangle_instance.get_content_name();

        assert_eq!(result, triangle_name);
    }

    #[test]
    fn when_a_triangle_instance_gets_its_vertex_data_then_the_vertex_data_is_returned() {
        let triangle_instance_name = "SomeTriangle";

        let triangle_name = "";

        let triangle_point_1_x = 0.0;
        let triangle_point_1_y = 0.5;
        let triangle_point_1_r = 1.0;
        let triangle_point_1_g = 0.0;
        let triangle_point_1_b = 0.0;

        let triangle_point_2_x = 0.5;
        let triangle_point_2_y = 1.0;
        let triangle_point_2_r = 0.0;
        let triangle_point_2_g = 1.0;
        let triangle_point_2_b = 0.0;

        let triangle_point_3_x = 1.0;
        let triangle_point_3_y = 0.0;
        let triangle_point_3_r = 0.0;
        let triangle_point_3_g = 0.0;
        let triangle_point_3_b = 1.0;

        let triangle_point_1 = create_mock_vertex_object(
            vec![
                triangle_point_1_x,
                triangle_point_1_y,
                triangle_point_1_r,
                triangle_point_1_g,
                triangle_point_1_b,
            ],
            0,
        );

        let triangle_point_2 = create_mock_vertex_object(
            vec![
                triangle_point_2_x,
                triangle_point_2_y,
                triangle_point_2_r,
                triangle_point_2_g,
                triangle_point_2_b,
            ],
            0,
        );

        let triangle_point_3 = create_mock_vertex_object(
            vec![
                triangle_point_3_x,
                triangle_point_3_y,
                triangle_point_3_r,
                triangle_point_3_g,
                triangle_point_3_b,
            ],
            0,
        );

        let expected_vertex_data = vec![
            triangle_point_1_x,
            triangle_point_1_y,
            triangle_point_1_r,
            triangle_point_1_g,
            triangle_point_1_b,
            triangle_point_2_x,
            triangle_point_2_y,
            triangle_point_2_r,
            triangle_point_2_g,
            triangle_point_2_b,
            triangle_point_3_x,
            triangle_point_3_y,
            triangle_point_3_r,
            triangle_point_3_g,
            triangle_point_3_b,
        ];

        let scale = 0.5;

        let position = MockVertexObject::new();

        let triangle_instance = TriangleInstance::<MockVertexObject, MockVertexObject>::new(
            triangle_instance_name.to_string(),
            triangle_name.to_string(),
            scale,
            position,
            triangle_point_1,
            triangle_point_2,
            triangle_point_3,
        );

        let result = triangle_instance.get_vertex_data();

        assert_eq!(result, expected_vertex_data);
    }

    #[test]
    fn when_a_triangle_instance_gets_its_number_of_vertices_then_the_number_of_vertices_is_returned(
    ) {
        let triangle_instance_name = "";

        let triangle_name = "";

        let scale = 0.5;

        let position = MockVertexObject::new();

        let triangle_point_1 = create_mock_vertex_object(vec![], 5);

        let triangle_point_2 = create_mock_vertex_object(vec![], 5);

        let triangle_point_3 = create_mock_vertex_object(vec![], 5);

        let expected_number_of_vertices = 15;

        let triangle_instance = TriangleInstance::<MockVertexObject, MockVertexObject>::new(
            triangle_instance_name.to_string(),
            triangle_name.to_string(),
            scale,
            position,
            triangle_point_1,
            triangle_point_2,
            triangle_point_3,
        );

        let result = triangle_instance.get_number_of_vertices();

        assert_eq!(result, expected_number_of_vertices);
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

        let triangle_instance_1 = Box::new(create_mock_vertex_object(
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

        let triangle_instance_2 = Box::new(create_mock_vertex_object(
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

        let triangle_instance_3 = Box::new(create_mock_vertex_object(
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

        let objects = vec![];

        let mut object_instances = Vec::<Box<dyn GetContentInstanceData>>::new();
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
        let object_instance_1 = Box::new(create_mock_vertex_object(vec![], 15));

        let object_instance_2 = Box::new(create_mock_vertex_object(vec![], 15));

        let object_instance_3 = Box::new(create_mock_vertex_object(vec![], 15));

        let objects = vec![];

        let mut object_instances = Vec::<Box<dyn GetContentInstanceData>>::new();
        object_instances.push(object_instance_1);
        object_instances.push(object_instance_2);
        object_instances.push(object_instance_3);

        let expected_number_of_vertices = 45;

        let content = Content::new(objects, object_instances);

        let result = content.get_number_of_vertices();

        assert_eq!(result, expected_number_of_vertices);
    }

    #[test]
    fn when_content_gets_its_number_of_objects_then_the_number_of_objects_is_returned() {
        let object_instance_1 = Box::new(create_mock_vertex_object(vec![], 0));

        let object_instance_2 = Box::new(create_mock_vertex_object(vec![], 0));

        let object_instance_3 = Box::new(create_mock_vertex_object(vec![], 0));

        let objects = vec![];

        let mut object_instances = Vec::<Box<dyn GetContentInstanceData>>::new();
        object_instances.push(object_instance_1);
        object_instances.push(object_instance_2);
        object_instances.push(object_instance_3);

        let expected_number_of_objects = 3;

        let content = Content::new(objects, object_instances);

        let result = content.get_number_of_objects();

        assert_eq!(result, expected_number_of_objects);
    }

    mock! {
        VertexObject {}
        impl GetVertexData for VertexObject {
            fn get_vertex_data(&self) -> Vec<f32>;
        }
        impl GetNumberOfVertices for VertexObject {
            fn get_number_of_vertices(&self) -> i32;
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

    fn create_mock_vertex_object(
        vertex_data: Vec<f32>,
        number_of_vertices: i32,
    ) -> MockVertexObject {
        let mut triangle_point_1 = MockVertexObject::new();
        triangle_point_1
            .expect_get_vertex_data()
            .times(1)
            .returning(move || vertex_data.clone());
        triangle_point_1
            .expect_get_number_of_vertices()
            .times(1)
            .returning(move || number_of_vertices);

        triangle_point_1
    }
}
