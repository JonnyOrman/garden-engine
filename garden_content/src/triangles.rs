use std::{cell::RefCell, rc::Rc};

use garden::GetName;

use crate::{
    AddContent, CreateTrianglePoint, GetContentInstanceData, GetNumberOfObjects,
    GetNumberOfVertices, GetPosition, GetScale, GetTrianglePointProperties, GetVertexData, GetX,
    GetY, Rgb, ScaleObjectInstance, TrianglePoint, TwoDPoint,
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
    ) -> Rc<RefCell<Triangle<TTrianglePoint>>>;
}

pub struct TriangleCreator<TTriangleProvider> {
    triangle_provider: Rc<RefCell<TTriangleProvider>>,
}

impl<TTriangleProvider> TriangleCreator<TTriangleProvider> {
    pub fn new(triangle_provider: Rc<RefCell<TTriangleProvider>>) -> Self {
        Self { triangle_provider }
    }
}

impl<
        TTriangleProvider: AddContent<Triangle<TTrianglePoint>>,
        TTrianglePoint: GetVertexData + GetNumberOfVertices,
    > CreateTriangle<TTrianglePoint> for TriangleCreator<TTriangleProvider>
{
    fn create_triangle(
        &self,
        name: String,
        point_1: TTrianglePoint,
        point_2: TTrianglePoint,
        point_3: TTrianglePoint,
    ) -> Rc<RefCell<Triangle<TTrianglePoint>>> {
        let mut vertex_data = vec![];

        vertex_data.append(&mut point_1.get_vertex_data().clone());
        vertex_data.append(&mut point_2.get_vertex_data().clone());
        vertex_data.append(&mut point_3.get_vertex_data().clone());

        let number_of_vertices = point_1.get_number_of_vertices()
            + point_2.get_number_of_vertices()
            + point_3.get_number_of_vertices();

        let triangle = Rc::new(RefCell::new(Triangle::<TTrianglePoint>::new(
            name,
            point_1,
            point_2,
            point_3,
            vertex_data,
            number_of_vertices,
        )));

        let triangle_ref = triangle.borrow();

        self.triangle_provider
            .borrow_mut()
            .add_content(Rc::clone(&triangle));

        Rc::clone(&triangle)
    }
}

pub struct TriangleInstance<TPosition, TTrianglePoint, TTriangle> {
    name: String,
    triangle: Rc<RefCell<TTriangle>>,
    scale: f32,
    position: TPosition,
    point_1: TTrianglePoint,
    point_2: TTrianglePoint,
    point_3: TTrianglePoint,
    number_of_vertices: i32,
    vertex_data: Vec<f32>,
}

impl<TPosition, TTrianglePoint, TTriangle> TriangleInstance<TPosition, TTrianglePoint, TTriangle> {
    pub fn new(
        name: String,
        triangle: Rc<RefCell<TTriangle>>,
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
            triangle,
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

impl<TPosition, TTrianglePoint, TTriangle> GetName
    for TriangleInstance<TPosition, TTrianglePoint, TTriangle>
{
    fn get_name(&self) -> &str {
        &self.name
    }
}

impl<TPosition, TTrianglePoint, TTriangle> GetScale
    for TriangleInstance<TPosition, TTrianglePoint, TTriangle>
{
    fn get_scale(&self) -> f32 {
        self.scale
    }
}

impl<TPosition, TTrianglePoint, TTriangle> GetPoint1<TTrianglePoint>
    for TriangleInstance<TPosition, TTrianglePoint, TTriangle>
{
    fn get_point_1(&self) -> &TTrianglePoint {
        &self.point_1
    }
}

impl<TPosition, TTrianglePoint, TTriangle> GetPoint2<TTrianglePoint>
    for TriangleInstance<TPosition, TTrianglePoint, TTriangle>
{
    fn get_point_2(&self) -> &TTrianglePoint {
        &self.point_2
    }
}

impl<TPosition, TTrianglePoint, TTriangle> GetPoint3<TTrianglePoint>
    for TriangleInstance<TPosition, TTrianglePoint, TTriangle>
{
    fn get_point_3(&self) -> &TTrianglePoint {
        &self.point_3
    }
}

impl<TPosition, TTrianglePoint, TTriangle> GetTrianglePoints<TTrianglePoint>
    for TriangleInstance<TPosition, TTrianglePoint, TTriangle>
{
}

impl<TPosition, TTrianglePoint, TTriangle> GetPosition<TPosition>
    for TriangleInstance<TPosition, TTrianglePoint, TTriangle>
{
    fn get_position(&self) -> &TPosition {
        &self.position
    }
}

impl<TPosition, TTrianglePoint, TTriangle> GetVertexData
    for TriangleInstance<TPosition, TTrianglePoint, TTriangle>
{
    fn get_vertex_data(&self) -> Vec<f32> {
        self.vertex_data.clone()
    }
}

impl<TPosition, TTrianglePoint, TTriangle> GetNumberOfVertices
    for TriangleInstance<TPosition, TTrianglePoint, TTriangle>
{
    fn get_number_of_vertices(&self) -> i32 {
        self.number_of_vertices
    }
}

impl GetNumberOfObjects
    for TriangleInstance<
        TwoDPoint,
        TrianglePoint<TwoDPoint, Rgb>,
        Triangle<TrianglePoint<TwoDPoint, Rgb>>,
    >
{
    fn get_number_of_objects(&self) -> i32 {
        1
    }
}

impl GetContentInstanceData
    for TriangleInstance<
        TwoDPoint,
        TrianglePoint<TwoDPoint, Rgb>,
        Triangle<TrianglePoint<TwoDPoint, Rgb>>,
    >
{
}

impl GetTriangle<Triangle<TrianglePoint<TwoDPoint, Rgb>>>
    for TriangleInstance<
        TwoDPoint,
        TrianglePoint<TwoDPoint, Rgb>,
        Triangle<TrianglePoint<TwoDPoint, Rgb>>,
    >
{
    fn get_triangle(&self) -> Rc<RefCell<Triangle<TrianglePoint<TwoDPoint, Rgb>>>> {
        Rc::clone(&self.triangle)
    }
}

pub trait GenerateTriangleInstanceVertexData<TTrianglePoint> {
    fn generate_triangle_instance_vertex_data(
        &self,
        triangle_point_1: &TTrianglePoint,
        triangle_point_2: &TTrianglePoint,
        triangle_point_3: &TTrianglePoint,
    ) -> Vec<f32>;
}

pub struct TriangleInstanceVertexDataGenerator {}

impl TriangleInstanceVertexDataGenerator {
    pub fn new() -> Self {
        Self {}
    }
}

impl<TTrianglePoint: GetVertexData> GenerateTriangleInstanceVertexData<TTrianglePoint>
    for TriangleInstanceVertexDataGenerator
{
    fn generate_triangle_instance_vertex_data(
        &self,
        triangle_point_1: &TTrianglePoint,
        triangle_point_2: &TTrianglePoint,
        triangle_point_3: &TTrianglePoint,
    ) -> Vec<f32> {
        let mut vertex_data = vec![];

        vertex_data.append(&mut triangle_point_1.get_vertex_data().clone());
        vertex_data.append(&mut triangle_point_2.get_vertex_data().clone());
        vertex_data.append(&mut triangle_point_3.get_vertex_data().clone());

        return vertex_data;
    }
}

pub trait CountTriangleInstanceVertices<TTrianglePoint> {
    fn count_triangle_instance_vertices(
        &self,
        triangle_point_1: &TTrianglePoint,
        triangle_point_2: &TTrianglePoint,
        triangle_point_3: &TTrianglePoint,
    ) -> i32;
}

pub struct TriangleInstanceVertexCounter {}

impl TriangleInstanceVertexCounter {
    pub fn new() -> Self {
        Self {}
    }
}

impl<TTrianglePoint: GetNumberOfVertices> CountTriangleInstanceVertices<TTrianglePoint>
    for TriangleInstanceVertexCounter
{
    fn count_triangle_instance_vertices(
        &self,
        triangle_point_1: &TTrianglePoint,
        triangle_point_2: &TTrianglePoint,
        triangle_point_3: &TTrianglePoint,
    ) -> i32 {
        triangle_point_1.get_number_of_vertices()
            + triangle_point_2.get_number_of_vertices()
            + triangle_point_3.get_number_of_vertices()
    }
}

pub trait CreateTriangleInstancePoint<TTrianglePoint> {
    fn create_triangle_instance_point(
        &self,
        triangle_point: &TTrianglePoint,
        x: f32,
        y: f32,
    ) -> TTrianglePoint;
}

pub struct TriangleInstancePointCreator<TTrianglePointCreator> {
    triangle_point_creator: Rc<TTrianglePointCreator>,
}

impl<TTrianglePointCreator> TriangleInstancePointCreator<TTrianglePointCreator> {
    pub fn new(triangle_point_creator: Rc<TTrianglePointCreator>) -> Self {
        Self {
            triangle_point_creator,
        }
    }
}

impl<
        TTrianglePoint: GetTrianglePointProperties,
        TTrianglePointCreator: CreateTrianglePoint<TTrianglePoint>,
    > CreateTriangleInstancePoint<TTrianglePoint>
    for TriangleInstancePointCreator<TTrianglePointCreator>
{
    fn create_triangle_instance_point(
        &self,
        triangle_point: &TTrianglePoint,
        x: f32,
        y: f32,
    ) -> TTrianglePoint {
        self.triangle_point_creator.create_triangle_point(
            triangle_point.get_x() / x,
            triangle_point.get_y() / y,
            triangle_point.get_r(),
            triangle_point.get_g(),
            triangle_point.get_b(),
        )
    }
}

pub trait CreateTriangleInstance<TPosition, TTrianglePoint, TTriangleInstance, TTriangle> {
    fn create_triangle_instance(
        &self,
        name: String,
        triangle: Rc<RefCell<TTriangle>>,
        scale: f32,
        position: TPosition,
        point_1: TTrianglePoint,
        point_2: TTrianglePoint,
        point_3: TTrianglePoint,
    ) -> Rc<RefCell<TTriangleInstance>>;
}

pub struct TriangleInstanceCreator<
    TTriangleInstanceVertexDataGenerator,
    TTriangleInstanceVertexCounter,
> {
    triangle_instance_vertex_data_generator: Rc<TTriangleInstanceVertexDataGenerator>,
    triangle_instance_vertex_counter: Rc<TTriangleInstanceVertexCounter>,
}

impl<TTriangleInstanceVertexDataGenerator, TTriangleInstanceVertexCounter>
    TriangleInstanceCreator<TTriangleInstanceVertexDataGenerator, TTriangleInstanceVertexCounter>
{
    pub fn new(
        triangle_instance_vertex_data_generator: Rc<TTriangleInstanceVertexDataGenerator>,
        triangle_instance_vertex_counter: Rc<TTriangleInstanceVertexCounter>,
    ) -> Self {
        Self {
            triangle_instance_vertex_data_generator,
            triangle_instance_vertex_counter,
        }
    }
}

impl<
        TPosition,
        TTrianglePoint: GetVertexData + GetNumberOfVertices,
        TTriangle,
        TTriangleInstanceVertexDataGenerator: GenerateTriangleInstanceVertexData<TTrianglePoint>,
        TTriangleInstanceVertexCounter: CountTriangleInstanceVertices<TTrianglePoint>,
    >
    CreateTriangleInstance<
        TPosition,
        TTrianglePoint,
        TriangleInstance<TPosition, TTrianglePoint, TTriangle>,
        TTriangle,
    >
    for TriangleInstanceCreator<
        TTriangleInstanceVertexDataGenerator,
        TTriangleInstanceVertexCounter,
    >
{
    fn create_triangle_instance(
        &self,
        name: String,
        triangle: Rc<RefCell<TTriangle>>,
        scale: f32,
        position: TPosition,
        point_1: TTrianglePoint,
        point_2: TTrianglePoint,
        point_3: TTrianglePoint,
    ) -> Rc<RefCell<TriangleInstance<TPosition, TTrianglePoint, TTriangle>>> {
        let vertex_data = self
            .triangle_instance_vertex_data_generator
            .generate_triangle_instance_vertex_data(&point_1, &point_2, &point_3);

        let number_of_vertices = self
            .triangle_instance_vertex_counter
            .count_triangle_instance_vertices(&point_1, &point_2, &point_3);

        Rc::new(RefCell::new(TriangleInstance::new(
            name,
            triangle,
            scale,
            position,
            point_1,
            point_2,
            point_3,
            number_of_vertices,
            vertex_data,
        )))
    }
}

pub struct TriangleInstanceScaler<TTriangleInstanceCreator, TTriangleInstancePointCreator> {
    triangle_instance_creator: Rc<TTriangleInstanceCreator>,
    triangle_instance_point_creator: Rc<TTriangleInstancePointCreator>,
}

impl<TTriangleInstanceCreator, TTriangleInstancePointCreator>
    TriangleInstanceScaler<TTriangleInstanceCreator, TTriangleInstancePointCreator>
{
    pub fn new(
        triangle_instance_creator: Rc<TTriangleInstanceCreator>,
        triangle_instance_point_creator: Rc<TTriangleInstancePointCreator>,
    ) -> Self {
        Self {
            triangle_instance_creator,
            triangle_instance_point_creator,
        }
    }
}

impl<
        TTriangleInstanceCreator: CreateTriangleInstance<
            TwoDPoint,
            TrianglePoint<TwoDPoint, Rgb>,
            TTriangleInstance,
            Triangle<TrianglePoint<TwoDPoint, Rgb>>,
        >,
        TTriangleInstance: GetContentInstanceData
            + GetPosition<TwoDPoint>
            + GetTrianglePoints<TrianglePoint<TwoDPoint, Rgb>>
            + GetName
            + GetScale
            + GetTriangle<Triangle<TrianglePoint<TwoDPoint, Rgb>>>,
        TTriangleInstancePointCreator: CreateTriangleInstancePoint<TrianglePoint<TwoDPoint, Rgb>>,
    > ScaleObjectInstance<TTriangleInstance>
    for TriangleInstanceScaler<TTriangleInstanceCreator, TTriangleInstancePointCreator>
{
    fn scale_object_instance(
        &self,
        triangle_instance: Rc<RefCell<TTriangleInstance>>,
        x: f32,
        y: f32,
    ) -> Rc<RefCell<TTriangleInstance>> {
        let new_position = TwoDPoint::new(
            triangle_instance.borrow().get_position().get_x() / x,
            triangle_instance.borrow().get_position().get_y() / y,
        );

        let new_point_1 = self
            .triangle_instance_point_creator
            .create_triangle_instance_point(triangle_instance.borrow().get_point_1(), x, y);

        let new_point_2 = self
            .triangle_instance_point_creator
            .create_triangle_instance_point(triangle_instance.borrow().get_point_2(), x, y);

        let new_point_3 = self
            .triangle_instance_point_creator
            .create_triangle_instance_point(triangle_instance.borrow().get_point_3(), x, y);

        self.triangle_instance_creator.create_triangle_instance(
            triangle_instance.borrow().get_name().to_string(),
            triangle_instance.borrow().get_triangle(),
            triangle_instance.borrow().get_scale(),
            new_position,
            new_point_1,
            new_point_2,
            new_point_3,
        )
    }
}

pub trait GetTriangle<TTriangle> {
    fn get_triangle(&self) -> Rc<RefCell<TTriangle>>;
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;

    use garden::GetName;
    use mockall::mock;

    use crate::triangles::{
        Triangle, TriangleInstance, TriangleInstanceVertexCounter,
        TriangleInstanceVertexDataGenerator,
    };
    use crate::{AddContent, GetX, GetY};
    use crate::{
        GetContentInstanceData, GetNumberOfObjects, GetNumberOfVertices, GetVertexData, Scale,
    };

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

        let triangle_provider = Rc::new(RefCell::new(MockTriangleProvider::new()));
        triangle_provider
            .borrow_mut()
            .expect_add_content()
            .times(1)
            .returning(move |_| {});

        let triangle_creator = TriangleCreator::new(triangle_provider);

        let triangle =
            triangle_creator.create_triangle(name.to_string(), point_1, point_2, point_3);

        assert_eq!(name, triangle.borrow().get_name());
        assert_eq!(expected_vertex_data, triangle.borrow().get_vertex_data());
        assert_eq!(15, triangle.borrow().get_number_of_vertices());
    }

    #[test]
    fn when_a_triangle_instance_gets_its_name_then_the_name_is_returned() {
        let name = "Name";

        let triangel_point_1 = MockVertexObject::new();

        let triangel_point_2 = MockVertexObject::new();

        let triangel_point_3 = MockVertexObject::new();

        let triangle = Rc::new(RefCell::new(Triangle::new(
            "Triangle".to_string(),
            triangel_point_1,
            triangel_point_2,
            triangel_point_3,
            vec![],
            0,
        )));

        let scale = 0.0;

        let position = MockVertexObject::new();

        let point_1 = MockVertexObject::new();

        let point_2 = MockVertexObject::new();

        let point_3 = MockVertexObject::new();

        let number_of_vertices = 0;

        let vertex_data = vec![];

        let triangle_instance = TriangleInstance::<
            MockVertexObject,
            MockVertexObject,
            Triangle<MockVertexObject>,
        >::new(
            name.to_string(),
            triangle,
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
    fn when_a_triangle_instance_gets_its_vertex_data_then_the_vertex_data_is_returned() {
        let name = "SomeTriangle";

        let triangle_point_1 = MockVertexObject::new();

        let triangle_point_2 = MockVertexObject::new();

        let triangle_point_3 = MockVertexObject::new();

        let triangle = Rc::new(RefCell::new(Triangle::new(
            name.to_string(),
            triangle_point_1,
            triangle_point_2,
            triangle_point_3,
            vec![],
            0,
        )));

        let scale = 0.0;

        let point_1 = MockVertexObject::new();

        let point_2 = MockVertexObject::new();

        let point_3 = MockVertexObject::new();

        let number_of_vertices = 0;

        let vertex_data = vec![
            0.0, 0.5, 1.0, 0.0, 0.0, 0.5, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0,
        ];

        let position = MockVertexObject::new();

        let triangle_instance = TriangleInstance::new(
            name.to_string(),
            triangle,
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

        let triangle_point_1 = MockVertexObject::new();

        let triangle_point_2 = MockVertexObject::new();

        let triangle_point_3 = MockVertexObject::new();

        let triangle = Rc::new(RefCell::new(Triangle::new(
            name.to_string(),
            triangle_point_1,
            triangle_point_2,
            triangle_point_3,
            vec![],
            0,
        )));

        let scale = 0.0;

        let position = MockVertexObject::new();

        let point_1 = MockVertexObject::new();

        let point_2 = MockVertexObject::new();

        let point_3 = MockVertexObject::new();

        let number_of_vertices = 15;

        let vertex_data = vec![];

        let triangle_instance = TriangleInstance::new(
            name.to_string(),
            triangle,
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

        let triangle_point_1 = MockVertexObject::new();

        let triangle_point_2 = MockVertexObject::new();

        let triangle_point_3 = MockVertexObject::new();

        let triangle = Rc::new(RefCell::new(Triangle::new(
            name.to_string(),
            triangle_point_1,
            triangle_point_2,
            triangle_point_3,
            vec![],
            0,
        )));

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

        let triangle_instance_vertex_data_generator = TriangleInstanceVertexDataGenerator::new();

        let triangle_instance_vertex_counter = TriangleInstanceVertexCounter::new();

        let triangle_instance_creator = TriangleInstanceCreator::new(
            Rc::new(triangle_instance_vertex_data_generator),
            Rc::new(triangle_instance_vertex_counter),
        );

        let triangle_instance = triangle_instance_creator.create_triangle_instance(
            name.to_string(),
            triangle,
            scale,
            position,
            point_1,
            point_2,
            point_3,
        );

        assert_eq!(name, triangle_instance.borrow().get_name());
        assert_eq!(
            expected_vertex_data,
            triangle_instance.borrow().get_vertex_data()
        );
        assert_eq!(15, triangle_instance.borrow().get_number_of_vertices());
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

    mock! {
        TriangleProvider<TTrianglePoint> {}
        impl<TTrianglePoint> AddContent<Triangle<TTrianglePoint>> for TriangleProvider<TTrianglePoint> {
            fn add_content(&mut self, content: Rc<RefCell<Triangle<TTrianglePoint>>>);
        }
    }
}
