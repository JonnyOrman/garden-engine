pub mod rectangles;
pub mod triangles;

use std::rc::Rc;

use garden::GetName;

pub trait GetContentName {
    fn get_content_name(&self) -> &str;
}

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
        object_instance: &TObjectInstance,
        x: f32,
        y: f32,
    ) -> TObjectInstance;
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

pub trait GetPosition<TPosition> {
    fn get_position(&self) -> &TPosition;
}

pub struct Content {
    objects: Option<Vec<Rc<Box<dyn GetName>>>>,
    object_instance_runners: Option<Vec<Box<dyn RunObjectInstance>>>,
    vertex_data: Vec<f32>,
    number_of_vertices: i32,
    number_of_objects: i32,
}

impl Content {
    pub fn new(
        objects: Vec<Rc<Box<dyn GetName>>>,
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

    pub fn get_objects(&self) -> &Option<Vec<Rc<Box<dyn GetName>>>> {
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

#[cfg(test)]
mod tests {
    use mockall::mock;

    use crate::{
        Content, GetContentInstanceData, GetNumberOfObjects, GetNumberOfVertices, GetVertexData,
        GetX, GetY, Rgb, RunObjectInstance, Scale, TrianglePoint, TwoDPoint,
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

        let mut triangle_instance_1 = Box::new(create_mock_vertex_object(
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

        let mut triangle_instance_2 = Box::new(create_mock_vertex_object(
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

        let mut triangle_instance_3 = Box::new(create_mock_vertex_object(
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
        let mut object_instance_1 = create_mock_vertex_object(vec![], 15);
        object_instance_1
            .expect_get_number_of_objects()
            .times(1)
            .returning(move || 0);

        let mut object_instance_2 = create_mock_vertex_object(vec![], 15);
        object_instance_2
            .expect_get_number_of_objects()
            .times(1)
            .returning(move || 0);

        let mut object_instance_3 = create_mock_vertex_object(vec![], 15);
        object_instance_3
            .expect_get_number_of_objects()
            .times(1)
            .returning(move || 0);

        let objects = vec![];

        let mut object_instances = Vec::<Box<dyn GetContentInstanceData>>::new();
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
        let mut object_instance_1 = create_mock_vertex_object(vec![], 0);
        object_instance_1
            .expect_get_number_of_objects()
            .times(1)
            .returning(move || 1);

        let mut object_instance_2 = create_mock_vertex_object(vec![], 0);
        object_instance_2
            .expect_get_number_of_objects()
            .times(1)
            .returning(move || 2);

        let mut object_instance_3 = create_mock_vertex_object(vec![], 0);
        object_instance_3
            .expect_get_number_of_objects()
            .times(1)
            .returning(move || 3);

        let objects = vec![];

        let mut object_instances = Vec::<Box<dyn GetContentInstanceData>>::new();
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
            fn scale(&self, x: f32, y: f32) {}
        }
        impl
    }

    fn create_mock_vertex_object(
        vertex_data: Vec<f32>,
        number_of_vertices: i32,
    ) -> MockVertexObject {
        let mut triangle_point = MockVertexObject::new();
        triangle_point
            .expect_get_vertex_data()
            .times(1)
            .returning(move || vertex_data.clone());
        triangle_point
            .expect_get_number_of_vertices()
            .times(1)
            .returning(move || number_of_vertices);

        triangle_point
    }

    fn create_mock_run_object_instance() -> MockObjectInstanceRunner {}
}
