pub trait GetVertexData {
    fn get_vertex_data(&self) -> Vec<f32>;
}

pub trait GetNumberOfVertices {
    fn get_number_of_vertices(&self) -> i32;
}

pub trait GetNumberOfObjects {
    fn get_number_of_objects(&self) -> i32;
}

pub trait GetVertexDataPtr {
    fn get_vertex_data_ptr(&self) -> *const f32;
}

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

pub struct Triangle<TTrianglePoint> {
    point_1: TTrianglePoint,
    point_2: TTrianglePoint,
    point_3: TTrianglePoint,
    number_of_vertices: i32,
    vertex_data: Vec<f32>,
}

impl<TTrianglePoint: GetVertexData + GetNumberOfVertices> Triangle<TTrianglePoint> {
    pub fn new(point_1: TTrianglePoint, point_2: TTrianglePoint, point_3: TTrianglePoint) -> Self {
        let mut vertex_data = vec![];

        vertex_data.append(&mut point_1.get_vertex_data().clone());
        vertex_data.append(&mut point_2.get_vertex_data().clone());
        vertex_data.append(&mut point_3.get_vertex_data().clone());

        let number_of_vertices = point_1.get_number_of_vertices()
            + point_2.get_number_of_vertices()
            + point_3.get_number_of_vertices();

        Self {
            point_1,
            point_2,
            point_3,
            number_of_vertices,
            vertex_data,
        }
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

pub struct Content<TTriangle> {
    triangles: Vec<TTriangle>,
    vertex_data: Vec<f32>,
    number_of_vertices: i32,
}

impl<TTriangle: GetVertexData + GetNumberOfVertices> Content<TTriangle> {
    pub fn new(triangles: Vec<TTriangle>) -> Self {
        let mut number_of_vertices = 0;

        let mut vertex_data = vec![];

        for triangle in triangles.iter() {
            number_of_vertices += triangle.get_number_of_vertices();
            vertex_data.append(&mut triangle.get_vertex_data());
        }

        Self {
            triangles,
            vertex_data,
            number_of_vertices,
        }
    }
}

impl<TTriangle> GetVertexData for Content<TTriangle> {
    fn get_vertex_data(&self) -> Vec<f32> {
        self.vertex_data.clone()
    }
}

impl<TTriangle> GetNumberOfVertices for Content<TTriangle> {
    fn get_number_of_vertices(&self) -> i32 {
        self.number_of_vertices
    }
}

impl<TTriangle> GetNumberOfObjects for Content<TTriangle> {
    fn get_number_of_objects(&self) -> i32 {
        self.triangles.len() as i32
    }
}

impl<TTriangle> GetVertexDataPtr for Content<TTriangle> {
    fn get_vertex_data_ptr(&self) -> *const f32 {
        self.vertex_data.as_ptr()
    }
}

#[cfg(test)]
mod tests {
    use mockall::mock;

    use crate::{
        Content, GetNumberOfObjects, GetNumberOfVertices, GetVertexData, Rgb, Triangle,
        TrianglePoint, TwoDPoint,
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
    fn when_a_triangle_gets_its_vertex_data_then_the_vertex_data_is_returned() {
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

        let triangle_point =
            Triangle::<MockVertexObject>::new(triangle_point_1, triangle_point_2, triangle_point_3);

        let result = triangle_point.get_vertex_data();

        assert_eq!(result, expected_vertex_data);
    }

    #[test]
    fn when_a_triangle_gets_its_number_of_vertices_then_the_number_of_vertices_is_returned() {
        let triangle_point_1 = create_mock_vertex_object(vec![], 5);

        let triangle_point_2 = create_mock_vertex_object(vec![], 5);

        let triangle_point_3 = create_mock_vertex_object(vec![], 5);

        let expected_number_of_vertices = 15;

        let triangle_point =
            Triangle::<MockVertexObject>::new(triangle_point_1, triangle_point_2, triangle_point_3);

        let result = triangle_point.get_number_of_vertices();

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

        let triangle_1 = create_mock_vertex_object(
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
        );

        let triangle_2 = create_mock_vertex_object(
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
        );

        let triangle_3 = create_mock_vertex_object(
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
        );

        let triangles = vec![triangle_1, triangle_2, triangle_3];

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

        let content = Content::<MockVertexObject>::new(triangles);

        let result = content.get_vertex_data();

        assert_eq!(result, expected_vertex_data);
    }

    #[test]
    fn when_content_gets_its_number_of_vertices_then_the_number_of_vertices_is_returned() {
        let triangle_1 = create_mock_vertex_object(vec![], 15);

        let triangle_2 = create_mock_vertex_object(vec![], 15);

        let triangle_3 = create_mock_vertex_object(vec![], 15);

        let triangles = vec![triangle_1, triangle_2, triangle_3];

        let expected_number_of_vertices = 45;

        let content = Content::<MockVertexObject>::new(triangles);

        let result = content.get_number_of_vertices();

        assert_eq!(result, expected_number_of_vertices);
    }

    #[test]
    fn when_content_gets_its_number_of_objects_then_the_number_of_objects_is_returned() {
        let triangle_1 = create_mock_vertex_object(vec![], 0);

        let triangle_2 = create_mock_vertex_object(vec![], 0);

        let triangle_3 = create_mock_vertex_object(vec![], 0);

        let triangles = vec![triangle_1, triangle_2, triangle_3];

        let expected_number_of_objects = 3;

        let content = Content::<MockVertexObject>::new(triangles);

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
