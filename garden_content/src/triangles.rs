use std::{cell::RefCell, marker::PhantomData, rc::Rc};

use garden::GetName;

use crate::{
    ConstructObject, CreateObject, CreateTrianglePoint, Get2DCoordiantes, GetContentInstanceData,
    GetNumberOfObjects, GetNumberOfVertices, GetPosition, GetRgbValues, GetScale,
    GetTrianglePointProperties, GetVertexData, ScaleObjectInstance, TranslateTwoDPoint,
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

impl<TTrianglePoint> GetPoint1<TTrianglePoint> for Triangle<TTrianglePoint> {
    fn get_point_1(&self) -> &TTrianglePoint {
        &self.point_1
    }
}

impl<TTrianglePoint> GetPoint2<TTrianglePoint> for Triangle<TTrianglePoint> {
    fn get_point_2(&self) -> &TTrianglePoint {
        &self.point_2
    }
}

impl<TTrianglePoint> GetPoint3<TTrianglePoint> for Triangle<TTrianglePoint> {
    fn get_point_3(&self) -> &TTrianglePoint {
        &self.point_3
    }
}

impl<TTrianglePoint> GetTrianglePoints<TTrianglePoint> for Triangle<TTrianglePoint> {}

pub struct TriangleParameters<TTrianglePoint> {
    name: String,
    point_1: TTrianglePoint,
    point_2: TTrianglePoint,
    point_3: TTrianglePoint,
}

impl<TTrianglePoint> TriangleParameters<TTrianglePoint> {
    pub fn new(
        name: String,
        point_1: TTrianglePoint,
        point_2: TTrianglePoint,
        point_3: TTrianglePoint,
    ) -> Self {
        Self {
            name,
            point_1,
            point_2,
            point_3,
        }
    }
}

pub struct TriangleConstructor {}

impl TriangleConstructor {
    pub fn new() -> Self {
        Self {}
    }
}

impl<TTrianglePoint: GetVertexData + GetNumberOfVertices>
    ConstructObject<Triangle<TTrianglePoint>, TriangleParameters<TTrianglePoint>>
    for TriangleConstructor
{
    fn construct_object(
        &self,
        parameters: TriangleParameters<TTrianglePoint>,
    ) -> Triangle<TTrianglePoint> {
        let name = parameters.name;

        let mut vertex_data = vec![];

        let point_1 = parameters.point_1;
        let point_2 = parameters.point_2;
        let point_3 = parameters.point_3;

        vertex_data.append(&mut point_1.get_vertex_data().clone());
        vertex_data.append(&mut point_2.get_vertex_data().clone());
        vertex_data.append(&mut point_3.get_vertex_data().clone());

        let number_of_vertices = point_1.get_number_of_vertices()
            + point_2.get_number_of_vertices()
            + point_3.get_number_of_vertices();

        Triangle::new(
            name,
            point_1,
            point_2,
            point_3,
            vertex_data,
            number_of_vertices,
        )
    }
}

pub trait CalculateTriangleInstancePoint<TTrianglePoint, TTwoDPoint> {
    fn calculate_triangle_instance_point(
        &self,
        point: &TTrianglePoint,
        position: &TTwoDPoint,
        scale: f32,
    ) -> TTrianglePoint;
}

pub struct TriangleInstancePointCalculator<TTrianglePointCreator> {
    triangle_point_creator: Rc<TTrianglePointCreator>,
}

impl<TTrianglePointCreator> TriangleInstancePointCalculator<TTrianglePointCreator> {
    pub fn new(triangle_point_creator: Rc<TTrianglePointCreator>) -> Self {
        Self {
            triangle_point_creator,
        }
    }
}

impl<
        TTrianglePoint: GetRgbValues + Get2DCoordiantes,
        TTwoDPoint: Get2DCoordiantes,
        TTrianglePointCreator: CreateTrianglePoint<TTrianglePoint>,
    > CalculateTriangleInstancePoint<TTrianglePoint, TTwoDPoint>
    for TriangleInstancePointCalculator<TTrianglePointCreator>
{
    fn calculate_triangle_instance_point(
        &self,
        point: &TTrianglePoint,
        position: &TTwoDPoint,
        scale: f32,
    ) -> TTrianglePoint {
        self.triangle_point_creator.create_triangle_point(
            point.get_x() * scale + position.get_x(),
            point.get_y() * scale + position.get_y(),
            point.get_r(),
            point.get_g(),
            point.get_b(),
        )
    }
}

pub struct TriangleInstanceParameters<TTriangle, TPosition, TTrianglePoint> {
    name: String,
    triangle: Rc<RefCell<TTriangle>>,
    scale: f32,
    position: TPosition,
    point_1: TTrianglePoint,
    point_2: TTrianglePoint,
    point_3: TTrianglePoint,
}

impl<TTriangle, TPosition, TTrianglePoint>
    TriangleInstanceParameters<TTriangle, TPosition, TTrianglePoint>
{
    pub fn new(
        name: String,
        triangle: Rc<RefCell<TTriangle>>,
        scale: f32,
        position: TPosition,
        point_1: TTrianglePoint,
        point_2: TTrianglePoint,
        point_3: TTrianglePoint,
    ) -> Self {
        Self {
            name,
            triangle,
            scale,
            position,
            point_1,
            point_2,
            point_3,
        }
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

impl<TPosition, TTrianglePoint, TTriangle> GetNumberOfObjects
    for TriangleInstance<TPosition, TTrianglePoint, TTriangle>
{
    fn get_number_of_objects(&self) -> i32 {
        1
    }
}

impl<TPosition, TTrianglePoint, TTriangle> GetContentInstanceData
    for TriangleInstance<TPosition, TTrianglePoint, TTriangle>
{
}

impl<TTriangle, TTwoDPoint, TTrianglePoint> GetTriangle<TTriangle>
    for TriangleInstance<TTwoDPoint, TTrianglePoint, TTriangle>
{
    fn get_triangle(&self) -> Rc<RefCell<TTriangle>> {
        Rc::clone(&self.triangle)
    }
}

pub struct TriangleInstanceConstructor<
    TTriangleInstanceVertexDataGenerator,
    TTriangleInstanceVertexCounter,
> {
    triangle_instance_vertex_data_generator: Rc<TTriangleInstanceVertexDataGenerator>,
    triangle_instance_vertex_counter: Rc<TTriangleInstanceVertexCounter>,
}

impl<TTriangleInstanceVertexDataGenerator, TTriangleInstanceVertexCounter>
    TriangleInstanceConstructor<
        TTriangleInstanceVertexDataGenerator,
        TTriangleInstanceVertexCounter,
    >
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
        TTriangle,
        TPosition,
        TTrianglePoint,
        TTriangleInstanceVertexDataGenerator: GenerateTriangleInstanceVertexData<TTrianglePoint>,
        TTriangleInstanceVertexCounter: CountTriangleInstanceVertices<TTrianglePoint>,
    >
    ConstructObject<
        TriangleInstance<TPosition, TTrianglePoint, TTriangle>,
        TriangleInstanceParameters<TTriangle, TPosition, TTrianglePoint>,
    >
    for TriangleInstanceConstructor<
        TTriangleInstanceVertexDataGenerator,
        TTriangleInstanceVertexCounter,
    >
{
    fn construct_object(
        &self,
        parameters: TriangleInstanceParameters<TTriangle, TPosition, TTrianglePoint>,
    ) -> TriangleInstance<TPosition, TTrianglePoint, TTriangle> {
        let vertex_data = self
            .triangle_instance_vertex_data_generator
            .generate_triangle_instance_vertex_data(
                &parameters.point_1,
                &parameters.point_2,
                &parameters.point_3,
            );

        let number_of_vertices = self
            .triangle_instance_vertex_counter
            .count_triangle_instance_vertices(
                &parameters.point_1,
                &parameters.point_2,
                &parameters.point_3,
            );

        TriangleInstance::new(
            parameters.name,
            parameters.triangle,
            parameters.scale,
            parameters.position,
            parameters.point_1,
            parameters.point_2,
            parameters.point_3,
            number_of_vertices,
            vertex_data,
        )
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

// pub trait CreateTriangleInstance<TPosition, TTrianglePoint, TTriangleInstance, TTriangle> {
//     fn create_triangle_instance(
//         &self,
//         name: String,
//         triangle: Rc<RefCell<TTriangle>>,
//         scale: f32,
//         position: TPosition,
//         point_1: TTrianglePoint,
//         point_2: TTrianglePoint,
//         point_3: TTrianglePoint,
//     ) -> Rc<RefCell<TTriangleInstance>>;
// }

// pub struct TriangleInstanceCreator<TTriangleInstanceConstructor> {
//     triangle_instance_constructor: Rc<TTriangleInstanceConstructor>,
// }

// impl<TTriangleInstanceConstructor> TriangleInstanceCreator<TTriangleInstanceConstructor> {
//     pub fn new(triangle_instance_constructor: Rc<TTriangleInstanceConstructor>) -> Self {
//         Self {
//             triangle_instance_constructor,
//         }
//     }
// }

// impl<
//         TPosition,
//         TTrianglePoint: GetVertexData + GetNumberOfVertices,
//         TTriangle,
//         TTriangleInstance,
//         TTriangleInstanceConstructor: ConstructObject<
//             TTriangleInstance,
//             TriangleInstanceParameters<TTriangle, TPosition, TTrianglePoint>,
//         >,
//     > CreateTriangleInstance<TPosition, TTrianglePoint, TTriangleInstance, TTriangle>
//     for TriangleInstanceCreator<TTriangleInstanceConstructor>
// {
//     fn create_triangle_instance(
//         &self,
//         name: String,
//         triangle: Rc<RefCell<TTriangle>>,
//         scale: f32,
//         position: TPosition,
//         point_1: TTrianglePoint,
//         point_2: TTrianglePoint,
//         point_3: TTrianglePoint,
//     ) -> Rc<RefCell<TTriangleInstance>> {
//         Rc::new(RefCell::new(
//             self.triangle_instance_constructor
//                 .construct_object(TriangleInstanceParameters::new(
//                     name, triangle, scale, position, point_1, point_2, point_3,
//                 )),
//         ))
//     }
// }

pub struct TriangleInstanceScaler<
    TTriangleInstanceCreator,
    TTriangleInstancePointCreator,
    TTwoDPointTranslator,
    TTwoDPoint,
    TTrianglePoint,
    TTriangle,
> {
    triangle_instance_creator: Rc<TTriangleInstanceCreator>,
    triangle_instance_point_creator: Rc<TTriangleInstancePointCreator>,
    two_d_point_translator: Rc<TTwoDPointTranslator>,
    two_d_point_type: PhantomData<TTwoDPoint>,
    triangle_point_type: PhantomData<TTrianglePoint>,
    triangle_type: PhantomData<TTriangle>,
}

impl<
        TTriangleInstanceCreator,
        TTriangleInstancePointCreator,
        TTwoDPointTranslator,
        TTwoDPoint,
        TTrianglePoint,
        TTriangle,
    >
    TriangleInstanceScaler<
        TTriangleInstanceCreator,
        TTriangleInstancePointCreator,
        TTwoDPointTranslator,
        TTwoDPoint,
        TTrianglePoint,
        TTriangle,
    >
{
    pub fn new(
        triangle_instance_creator: Rc<TTriangleInstanceCreator>,
        triangle_instance_point_creator: Rc<TTriangleInstancePointCreator>,
        two_d_point_translator: Rc<TTwoDPointTranslator>,
    ) -> Self {
        Self {
            triangle_instance_creator: triangle_instance_creator,
            triangle_instance_point_creator: triangle_instance_point_creator,
            two_d_point_translator: two_d_point_translator,
            two_d_point_type: PhantomData,
            triangle_point_type: PhantomData,
            triangle_type: PhantomData,
        }
    }
}

impl<
        TTriangleInstanceCreator: CreateObject<
            TTriangleInstance,
            TriangleInstanceParameters<TTriangle, TTwoDPoint, TTrianglePoint>,
        >,
        TTriangleInstance: GetContentInstanceData
            + GetPosition<TTwoDPoint>
            + GetTrianglePoints<TTrianglePoint>
            + GetName
            + GetScale
            + GetTriangle<TTriangle>,
        TTriangleInstancePointCreator: CreateTriangleInstancePoint<TTrianglePoint>,
        TPositionTranslator: TranslateTwoDPoint<TTwoDPoint>,
        TTwoDPoint,
        TTrianglePoint,
        TTriangle,
    > ScaleObjectInstance<TTriangleInstance>
    for TriangleInstanceScaler<
        TTriangleInstanceCreator,
        TTriangleInstancePointCreator,
        TPositionTranslator,
        TTwoDPoint,
        TTrianglePoint,
        TTriangle,
    >
{
    fn scale_object_instance(
        &self,
        triangle_instance: Rc<RefCell<TTriangleInstance>>,
        x: f32,
        y: f32,
    ) -> Rc<RefCell<TTriangleInstance>> {
        let new_position = self.two_d_point_translator.translate_two_d_point(
            triangle_instance.borrow().get_position(),
            x,
            y,
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

        self.triangle_instance_creator
            .create_object(TriangleInstanceParameters::new(
                triangle_instance.borrow().get_name().to_string(),
                triangle_instance.borrow().get_triangle(),
                triangle_instance.borrow().get_scale(),
                new_position,
                new_point_1,
                new_point_2,
                new_point_3,
            ))
    }
}

pub trait GetTriangle<TTriangle> {
    fn get_triangle(&self) -> Rc<RefCell<TTriangle>>;
}

pub struct GeometryTriangle<TTrianglePoint> {
    triangle_point_1: TTrianglePoint,
    triangle_point_2: TTrianglePoint,
    triangle_point_3: TTrianglePoint,
    vertex_data: Vec<f32>,
    number_of_vertices: i32,
}

impl<TTrianglePoint> GeometryTriangle<TTrianglePoint> {
    fn new(
        triangle_point_1: TTrianglePoint,
        triangle_point_2: TTrianglePoint,
        triangle_point_3: TTrianglePoint,
        vertex_data: Vec<f32>,
        number_of_vertices: i32,
    ) -> Self {
        Self {
            triangle_point_1,
            triangle_point_2,
            triangle_point_3,
            vertex_data,
            number_of_vertices,
        }
    }
}

impl<TTrianglePoint> GetVertexData for GeometryTriangle<TTrianglePoint> {
    fn get_vertex_data(&self) -> Vec<f32> {
        self.vertex_data.clone()
    }
}

impl<TTrianglePoint> GetNumberOfVertices for GeometryTriangle<TTrianglePoint> {
    fn get_number_of_vertices(&self) -> i32 {
        self.number_of_vertices
    }
}

pub trait CreateGeometryTriangle<TTrianglePoint> {
    fn create_geometry_triangle(
        &self,
        triangle_point_1: TTrianglePoint,
        triangle_point_2: TTrianglePoint,
        triangle_point_3: TTrianglePoint,
    ) -> GeometryTriangle<TTrianglePoint>;
}

pub struct GeometryTriangleCreator {}

impl GeometryTriangleCreator {
    pub fn new() -> Self {
        Self {}
    }
}

impl<TTrianglePoint: GetVertexData + GetNumberOfVertices> CreateGeometryTriangle<TTrianglePoint>
    for GeometryTriangleCreator
{
    fn create_geometry_triangle(
        &self,
        triangle_point_1: TTrianglePoint,
        triangle_point_2: TTrianglePoint,
        triangle_point_3: TTrianglePoint,
    ) -> GeometryTriangle<TTrianglePoint> {
        let mut vertex_data = vec![];

        vertex_data.append(&mut triangle_point_1.get_vertex_data().clone());
        vertex_data.append(&mut triangle_point_2.get_vertex_data().clone());
        vertex_data.append(&mut triangle_point_3.get_vertex_data().clone());

        let number_of_vertices = triangle_point_1.get_number_of_vertices()
            + triangle_point_2.get_number_of_vertices()
            + triangle_point_3.get_number_of_vertices();

        GeometryTriangle::new(
            triangle_point_1,
            triangle_point_2,
            triangle_point_3,
            vertex_data,
            number_of_vertices,
        )
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;

    use garden::GetName;
    use mockall::mock;

    use crate::triangles::{
        Triangle, TriangleInstance, TriangleInstanceConstructor, TriangleInstanceVertexCounter,
        TriangleInstanceVertexDataGenerator, TriangleParameters,
    };
    use crate::{
        ConstructObject, CreateObject, GetContentInstanceData, GetNumberOfObjects,
        GetNumberOfVertices, GetVertexData, ObjectCreator, Scale, StoreObject,
    };
    use crate::{GetX, GetY};

    use super::TriangleInstanceCreator;

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

        let mut triangle_constructor = MockTriangleConstructor::new();
        triangle_constructor
            .expect_construct_object()
            .times(1)
            .returning(move |_| {
                Triangle::new(
                    name.to_string(),
                    MockVertexObject::new(),
                    MockVertexObject::new(),
                    MockVertexObject::new(),
                    vec![
                        point_1_x, point_1_y, point_1_r, point_1_g, point_1_b, point_2_x,
                        point_2_y, point_2_r, point_2_g, point_2_b, point_3_x, point_3_y,
                        point_3_r, point_3_g, point_3_b,
                    ],
                    15,
                )
            });

        let triangle_provider = Rc::new(RefCell::new(MockTriangleProvider::new()));
        triangle_provider
            .borrow_mut()
            .expect_store_object()
            .times(1)
            .returning(move |_| {});

        let triangle_creator = ObjectCreator::new(Rc::new(triangle_constructor), triangle_provider);

        let parameters = TriangleParameters::new(name.to_string(), point_1, point_2, point_3);

        let result = triangle_creator.create_object(parameters);

        assert_eq!(name, result.borrow().get_name());
        assert_eq!(expected_vertex_data, result.borrow().get_vertex_data());
        assert_eq!(15, result.borrow().get_number_of_vertices());
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

        let triangle_instance_constructor = TriangleInstanceConstructor::new();

        let triangle_instance_creator = TriangleInstanceCreator::new(
            Rc::new(triangle_instance_vertex_data_generator),
            Rc::new(triangle_instance_vertex_counter),
            Rc::new(triangle_instance_constructor),
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
        impl<TTrianglePoint> StoreObject<Triangle<TTrianglePoint>> for TriangleProvider<TTrianglePoint> {
            fn store_object(&mut self, content: Rc<RefCell<Triangle<TTrianglePoint>>>);
        }
    }

    mock! {
        TriangleConstructor {}
        impl ConstructObject<Triangle<MockVertexObject>, TriangleParameters<MockVertexObject>> for TriangleConstructor {
            fn construct_object(
                &self,
               parameters: TriangleParameters<MockVertexObject>
            ) -> Triangle<MockVertexObject>;
        }
    }
}
