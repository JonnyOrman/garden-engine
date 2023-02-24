use garden::GetName;

use crate::{
    GetB, GetContentInstanceData, GetG, GetNumberOfObjects, GetNumberOfVertices, GetPosition, GetR,
    GetRgb, GetScale, GetVertexData, GetX, GetY, Rgb, Scale, TrianglePoint, TwoDPoint,
};

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

#[cfg(test)]
mod tests {
    use garden::GetName;
    use mockall::mock;

    use crate::{
        GetContentInstanceData, GetNumberOfObjects, GetNumberOfVertices, GetVertexData, GetX, GetY,
        Scale,
    };

    use crate::triangles::{Triangle, TriangleInstance};

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
