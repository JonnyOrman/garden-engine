use std::rc::Rc;

use garden::GetName;

use crate::{
    CreateTrianglePoint, GetB, GetContentInstanceData, GetContentName, GetG, GetNumberOfObjects,
    GetNumberOfVertices, GetPosition, GetR, GetRgb, GetScale, GetVertexData, GetX, GetY, Rgb,
    ScaleObjectInstance, TrianglePoint, TwoDPoint,
};

pub trait GetPoint1<TPoint> {
    fn get_point_1(&self) -> &TPoint;
}

pub trait GetPoint2<TPoint> {
    fn get_point_2(&self) -> &TPoint;
}

pub trait GetPoint3<TPoint> {
    fn get_point_3(&self) -> &TPoint;
}

pub trait GetTrianglePoints<TPoint>:
    GetPoint1<TPoint> + GetPoint2<TPoint> + GetPoint3<TPoint>
{
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
        vertex_data: Vec<f32>,
        number_of_vertices: i32,
    ) -> Self {
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

pub trait CreateTriangle<TTrianglePoint> {
    fn create_triangle(
        &self,
        name: String,
        point_1: TTrianglePoint,
        point_2: TTrianglePoint,
        point_3: TTrianglePoint,
    ) -> Triangle<TTrianglePoint>;
}

pub struct TriangleCreator {}

impl TriangleCreator {
    pub fn new() -> Self {
        Self {}
    }
}

impl<TTrianglePoint: GetVertexData + GetNumberOfVertices> CreateTriangle<TTrianglePoint>
    for TriangleCreator
{
    fn create_triangle(
        &self,
        name: String,
        point_1: TTrianglePoint,
        point_2: TTrianglePoint,
        point_3: TTrianglePoint,
    ) -> Triangle<TTrianglePoint> {
        let mut vertex_data = vec![];

        vertex_data.append(&mut point_1.get_vertex_data().clone());
        vertex_data.append(&mut point_2.get_vertex_data().clone());
        vertex_data.append(&mut point_3.get_vertex_data().clone());

        let number_of_vertices = point_1.get_number_of_vertices()
            + point_2.get_number_of_vertices()
            + point_3.get_number_of_vertices();

        Triangle::<TTrianglePoint>::new(
            name,
            point_1,
            point_2,
            point_3,
            vertex_data,
            number_of_vertices,
        )
    }
}

pub struct TriangleInstance<TPosition, TTrianglePoint> {
    name: String,
    content_name: String,
    scale: f32,
    position: TPosition,
    point_1: TTrianglePoint,
    point_2: TTrianglePoint,
    point_3: TTrianglePoint,
    number_of_vertices: i32,
    vertex_data: Vec<f32>,
}

impl<TPosition, TTrianglePoint> TriangleInstance<TPosition, TTrianglePoint> {
    pub fn new(
        name: String,
        content_name: String,
        scale: f32,
        position: TPosition,
        point_1: TTrianglePoint,
        point_2: TTrianglePoint,
        point_3: TTrianglePoint,
        number_of_vertices: i32,
        vertex_data: Vec<f32>,
    ) -> Self {
        Self {
            name,
            content_name,
            scale,
            position,
            point_1,
            point_2,
            point_3,
            number_of_vertices,
            vertex_data,
        }
    }
}

impl<TPosition, TTrianglePoint> GetName for TriangleInstance<TPosition, TTrianglePoint> {
    fn get_name(&self) -> &str {
        &self.name
    }
}

impl<TPosition, TTrianglePoint> GetContentName for TriangleInstance<TPosition, TTrianglePoint> {
    fn get_content_name(&self) -> &str {
        &self.content_name
    }
}

impl<TPosition, TTrianglePoint> GetScale for TriangleInstance<TPosition, TTrianglePoint> {
    fn get_scale(&self) -> f32 {
        self.scale
    }
}

impl<TPosition, TTrianglePoint> GetPoint1<TTrianglePoint>
    for TriangleInstance<TPosition, TTrianglePoint>
{
    fn get_point_1(&self) -> &TTrianglePoint {
        &self.point_1
    }
}

impl<TPosition, TTrianglePoint> GetPoint2<TTrianglePoint>
    for TriangleInstance<TPosition, TTrianglePoint>
{
    fn get_point_2(&self) -> &TTrianglePoint {
        &self.point_2
    }
}

impl<TPosition, TTrianglePoint> GetPoint3<TTrianglePoint>
    for TriangleInstance<TPosition, TTrianglePoint>
{
    fn get_point_3(&self) -> &TTrianglePoint {
        &self.point_3
    }
}

impl<TPosition, TTrianglePoint> GetTrianglePoints<TTrianglePoint>
    for TriangleInstance<TPosition, TTrianglePoint>
{
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

impl GetNumberOfObjects for TriangleInstance<TwoDPoint, TrianglePoint<TwoDPoint, Rgb>> {
    fn get_number_of_objects(&self) -> i32 {
        1
    }
}

impl GetContentInstanceData for TriangleInstance<TwoDPoint, TrianglePoint<TwoDPoint, Rgb>> {}

pub trait CreateTriangleInstance<TPosition, TTrianglePoint, TTriangleInstance> {
    fn create_triangle_instance(
        &self,
        name: String,
        content_name: String,
        scale: f32,
        position: TPosition,
        point_1: TTrianglePoint,
        point_2: TTrianglePoint,
        point_3: TTrianglePoint,
    ) -> TTriangleInstance;
}

pub struct TriangleInstanceCreator {}

impl TriangleInstanceCreator {
    pub fn new() -> Self {
        Self {}
    }
}

impl<TPosition, TTrianglePoint: GetVertexData + GetNumberOfVertices>
    CreateTriangleInstance<TPosition, TTrianglePoint, TriangleInstance<TPosition, TTrianglePoint>>
    for TriangleInstanceCreator
{
    fn create_triangle_instance(
        &self,
        name: String,
        content_name: String,
        scale: f32,
        position: TPosition,
        point_1: TTrianglePoint,
        point_2: TTrianglePoint,
        point_3: TTrianglePoint,
    ) -> TriangleInstance<TPosition, TTrianglePoint> {
        let mut vertex_data = vec![];

        vertex_data.append(&mut point_1.get_vertex_data().clone());
        vertex_data.append(&mut point_2.get_vertex_data().clone());
        vertex_data.append(&mut point_3.get_vertex_data().clone());

        let number_of_vertices = point_1.get_number_of_vertices()
            + point_2.get_number_of_vertices()
            + point_3.get_number_of_vertices();

        TriangleInstance::new(
            name,
            content_name,
            scale,
            position,
            point_1,
            point_2,
            point_3,
            number_of_vertices,
            vertex_data,
        )
    }
}

pub struct TriangleInstanceScaler<TTriangleInstanceCreator, TTrianglePointCreator> {
    triangle_instance_creator: Rc<TTriangleInstanceCreator>,
    triangle_point_creator: Rc<TTrianglePointCreator>,
}

impl<TTriangleInstanceCreator, TTrianglePointCreator>
    TriangleInstanceScaler<TTriangleInstanceCreator, TTrianglePointCreator>
{
    pub fn new(
        triangle_instance_creator: Rc<TTriangleInstanceCreator>,
        triangle_point_creator: Rc<TTrianglePointCreator>,
    ) -> Self {
        Self {
            triangle_instance_creator,
            triangle_point_creator,
        }
    }
}

impl<
        TTriangleInstanceCreator: CreateTriangleInstance<TwoDPoint, TrianglePoint<TwoDPoint, Rgb>, TTriangleInstance>,
        TTrianglePointCreator: CreateTrianglePoint<TrianglePoint<TwoDPoint, Rgb>>,
        TTriangleInstance: GetContentInstanceData
            + GetPosition<TwoDPoint>
            + GetTrianglePoints<TrianglePoint<TwoDPoint, Rgb>>
            + GetName
            + GetContentName
            + GetScale,
    > ScaleObjectInstance<TTriangleInstance>
    for TriangleInstanceScaler<TTriangleInstanceCreator, TTrianglePointCreator>
{
    fn scale_object_instance(
        &self,
        triangle_instance: &TTriangleInstance,
        x: f32,
        y: f32,
    ) -> TTriangleInstance {
        let new_position = TwoDPoint::new(
            triangle_instance.get_position().get_x() / x,
            triangle_instance.get_position().get_y() / y,
        );

        let new_point_1 = self.triangle_point_creator.create_triangle_point(
            triangle_instance.get_point_1().get_x() / x,
            triangle_instance.get_point_1().get_y() / y,
            triangle_instance.get_point_1().get_rgb().get_r(),
            triangle_instance.get_point_1().get_rgb().get_g(),
            triangle_instance.get_point_1().get_rgb().get_b(),
        );

        let new_point_2 = self.triangle_point_creator.create_triangle_point(
            triangle_instance.get_point_2().get_x() / x,
            triangle_instance.get_point_2().get_y() / y,
            triangle_instance.get_point_2().get_rgb().get_r(),
            triangle_instance.get_point_2().get_rgb().get_g(),
            triangle_instance.get_point_2().get_rgb().get_b(),
        );

        let new_point_3 = self.triangle_point_creator.create_triangle_point(
            triangle_instance.get_point_3().get_x() / x,
            triangle_instance.get_point_3().get_y() / y,
            triangle_instance.get_point_3().get_rgb().get_r(),
            triangle_instance.get_point_3().get_rgb().get_g(),
            triangle_instance.get_point_3().get_rgb().get_b(),
        );

        self.triangle_instance_creator.create_triangle_instance(
            triangle_instance.get_name().to_string(),
            triangle_instance.get_content_name().to_string(),
            triangle_instance.get_scale(),
            new_position,
            new_point_1,
            new_point_2,
            new_point_3,
        )
    }
}

#[cfg(test)]
mod tests {
    use garden::GetName;
    use mockall::mock;

    use crate::{
        GetContentInstanceData, GetContentName, GetNumberOfObjects, GetNumberOfVertices,
        GetVertexData, GetX, GetY, Scale,
    };

    use crate::triangles::{Triangle, TriangleInstance};

    use super::{CreateTriangle, CreateTriangleInstance, TriangleCreator, TriangleInstanceCreator};

    #[test]
    fn when_a_triangle_gets_its_name_then_the_name_is_returned() {
        let name = "SomeTriangle";

        let point_1 = MockVertexObject::new();

        let point_2 = MockVertexObject::new();

        let point_3 = MockVertexObject::new();

        let vertex_data = vec![];

        let number_of_vertices = 0;

        let triangle = Triangle::<MockVertexObject>::new(
            name.to_string(),
            point_1,
            point_2,
            point_3,
            vertex_data,
            number_of_vertices,
        );

        let result = triangle.get_name();

        assert_eq!(name, result);
    }

    #[test]
    fn when_a_triangle_gets_its_vertex_data_then_the_vertex_data_is_returned() {
        let triangle_name = "";

        let point_1 = MockVertexObject::new();

        let point_2 = MockVertexObject::new();

        let point_3 = MockVertexObject::new();

        let vertex_data = vec![
            0.0, 0.5, 1.0, 0.0, 0.0, 0.5, 1.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0,
        ];

        let number_of_vertices = 0;

        let triangle = Triangle::new(
            triangle_name.to_string(),
            point_1,
            point_2,
            point_3,
            vertex_data.clone(),
            number_of_vertices,
        );

        let result = triangle.get_vertex_data();

        assert_eq!(vertex_data, result);
    }

    #[test]
    fn when_a_triangle_gets_its_number_of_vertices_then_the_number_of_vertices_is_returned() {
        let triangle_name = "";

        let point_1 = MockVertexObject::new();

        let point_2 = MockVertexObject::new();

        let point_3 = MockVertexObject::new();

        let vertex_data = vec![];

        let number_of_vertices = 15;

        let triangle = Triangle::<MockVertexObject>::new(
            triangle_name.to_string(),
            point_1,
            point_2,
            point_3,
            vertex_data,
            15,
        );

        let result = triangle.get_number_of_vertices();

        assert_eq!(number_of_vertices, result);
    }

    #[test]
    fn when_a_triangle_creator_creates_a_triangle_then_the_triangle_is_created() {
        let name = "TriangleName";

        let point_1_x = 0.0;
        let point_1_y = 0.5;
        let point_1_r = 1.0;
        let point_1_g = 0.0;
        let point_1_b = 0.0;

        let point_2_x = 0.5;
        let point_2_y = 1.0;
        let point_2_r = 0.0;
        let point_2_g = 1.0;
        let point_2_b = 0.0;

        let point_3_x = 1.0;
        let point_3_y = 0.0;
        let point_3_r = 0.0;
        let point_3_g = 0.0;
        let point_3_b = 1.0;

        let point_1 = create_mock_vertex_object(
            vec![point_1_x, point_1_y, point_1_r, point_1_g, point_1_b],
            5,
        );

        let point_2 = create_mock_vertex_object(
            vec![point_2_x, point_2_y, point_2_r, point_2_g, point_2_b],
            5,
        );

        let point_3 = create_mock_vertex_object(
            vec![point_3_x, point_3_y, point_3_r, point_3_g, point_3_b],
            5,
        );

        let expected_vertex_data = vec![
            point_1_x, point_1_y, point_1_r, point_1_g, point_1_b, point_2_x, point_2_y, point_2_r,
            point_2_g, point_2_b, point_3_x, point_3_y, point_3_r, point_3_g, point_3_b,
        ];

        let triangle_creator = TriangleCreator::new();

        let triangle =
            triangle_creator.create_triangle(name.to_string(), point_1, point_2, point_3);

        assert_eq!(name, triangle.get_name());
        assert_eq!(expected_vertex_data, triangle.get_vertex_data());
        assert_eq!(15, triangle.get_number_of_vertices());
    }

    #[test]
    fn when_a_triangle_instance_gets_its_name_then_the_name_is_returned() {
        let name = "Name";

        let content_name = "";

        let scale = 0.0;

        let position = MockVertexObject::new();

        let point_1 = MockVertexObject::new();

        let point_2 = MockVertexObject::new();

        let point_3 = MockVertexObject::new();

        let number_of_vertices = 0;

        let vertex_data = vec![];

        let triangle_instance = TriangleInstance::<MockVertexObject, MockVertexObject>::new(
            name.to_string(),
            content_name.to_string(),
            scale,
            position,
            point_1,
            point_2,
            point_3,
            number_of_vertices,
            vertex_data,
        );

        let result = triangle_instance.get_name();

        assert_eq!(name, result);
    }

    #[test]
    fn when_a_triangle_instance_gets_its_content_name_then_the_name_is_returned() {
        let name = "";

        let content_name = "SomeContent";

        let scale = 0.0;

        let position = MockVertexObject::new();

        let point_1 = MockVertexObject::new();

        let point_2 = MockVertexObject::new();

        let point_3 = MockVertexObject::new();

        let number_of_vertices = 0;

        let vertex_data = vec![];

        let triangle_instance = TriangleInstance::<MockVertexObject, MockVertexObject>::new(
            name.to_string(),
            content_name.to_string(),
            scale,
            position,
            point_1,
            point_2,
            point_3,
            number_of_vertices,
            vertex_data,
        );

        let result = triangle_instance.get_content_name();

        assert_eq!(content_name, result);
    }

    #[test]
    fn when_a_triangle_instance_gets_its_vertex_data_then_the_vertex_data_is_returned() {
        let name = "SomeTriangle";

        let content_name = "";

        let scale = 0.0;

        let position = MockVertexObject::new();

        let point_1 = MockVertexObject::new();

        let point_2 = MockVertexObject::new();

        let point_3 = MockVertexObject::new();

        let number_of_vertices = 0;

        let vertex_data = vec![
            0.0, 0.5, 1.0, 0.0, 0.0, 0.5, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0,
        ];

        let position = MockVertexObject::new();

        let triangle_instance = TriangleInstance::<MockVertexObject, MockVertexObject>::new(
            name.to_string(),
            content_name.to_string(),
            scale,
            position,
            point_1,
            point_2,
            point_3,
            number_of_vertices,
            vertex_data.clone(),
        );

        let result = triangle_instance.get_vertex_data();

        assert_eq!(vertex_data, result);
    }

    #[test]
    fn when_a_triangle_instance_gets_its_number_of_vertices_then_the_number_of_vertices_is_returned(
    ) {
        let name = "";

        let content_name = "";

        let scale = 0.0;

        let position = MockVertexObject::new();

        let point_1 = MockVertexObject::new();

        let point_2 = MockVertexObject::new();

        let point_3 = MockVertexObject::new();

        let number_of_vertices = 15;

        let vertex_data = vec![];

        let triangle_instance = TriangleInstance::<MockVertexObject, MockVertexObject>::new(
            name.to_string(),
            content_name.to_string(),
            scale,
            position,
            point_1,
            point_2,
            point_3,
            number_of_vertices,
            vertex_data,
        );

        let result = triangle_instance.get_number_of_vertices();

        assert_eq!(number_of_vertices, result);
    }

    #[test]
    fn when_a_triangle_instance_creator_creates_a_triangle_instance_then_the_triangle_instance_is_created(
    ) {
        let name = "SomeTriangle";

        let content_name = "";

        let scale = 0.0;

        let position = MockVertexObject::new();

        let point_1_x = 0.0;
        let point_1_y = 0.5;
        let point_1_r = 1.0;
        let point_1_g = 0.0;
        let point_1_b = 0.0;

        let point_2_x = 0.5;
        let point_2_y = 1.0;
        let point_2_r = 0.0;
        let point_2_g = 1.0;
        let point_2_b = 0.0;

        let point_3_x = 1.0;
        let point_3_y = 0.0;
        let point_3_r = 0.0;
        let point_3_g = 0.0;
        let point_3_b = 1.0;

        let point_1 = create_mock_vertex_object(
            vec![point_1_x, point_1_y, point_1_r, point_1_g, point_1_b],
            5,
        );

        let point_2 = create_mock_vertex_object(
            vec![point_2_x, point_2_y, point_2_r, point_2_g, point_2_b],
            5,
        );

        let point_3 = create_mock_vertex_object(
            vec![point_3_x, point_3_y, point_3_r, point_3_g, point_3_b],
            5,
        );

        let expected_vertex_data = vec![
            0.0, 0.5, 1.0, 0.0, 0.0, 0.5, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0,
        ];

        let triangle_instance_creator = TriangleInstanceCreator::new();

        let triangle_instance = triangle_instance_creator.create_triangle_instance(
            name.to_string(),
            content_name.to_string(),
            scale,
            position,
            point_1,
            point_2,
            point_3,
        );

        assert_eq!(name, triangle_instance.get_name());
        assert_eq!(content_name, triangle_instance.get_content_name());
        assert_eq!(expected_vertex_data, triangle_instance.get_vertex_data());
        assert_eq!(15, triangle_instance.get_number_of_vertices());
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
