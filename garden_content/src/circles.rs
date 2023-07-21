use std::{cell::RefCell, marker::PhantomData, rc::Rc};

use garden::GetName;

use crate::{
    triangles::CreateGeometryTriangles, ConstructObject, CreateObject, CreateTwoDPoint,
    Get2DCoordiantes, GetB, GetContentInstanceData, GetG, GetNumberOfObjects, GetNumberOfVertices,
    GetPosition, GetR, GetRgb, GetRgbValues, GetScale, GetVertexData, Rgb, ScaleObjectInstance,
};

pub trait GetDiameter {
    fn get_diameter(&self) -> f32;
}

pub trait GetCircle<TCircle> {
    fn get_circle(&self) -> Rc<RefCell<TCircle>>;
}

pub struct Circle<TRgb> {
    name: String,
    diameter: f32,
    rgb: TRgb,
}

impl<TRgb> Circle<TRgb> {
    fn new(name: String, diameter: f32, rgb: TRgb) -> Self {
        Self {
            name,
            diameter,
            rgb,
        }
    }
}

impl<TRgb> GetName for Circle<TRgb> {
    fn get_name(&self) -> &str {
        &self.name
    }
}

impl<TRgb> GetDiameter for Circle<TRgb> {
    fn get_diameter(&self) -> f32 {
        self.diameter
    }
}

impl<TRgb> GetRgb<TRgb> for Circle<TRgb> {
    fn get_rgb(&self) -> &TRgb {
        &self.rgb
    }
}

impl<TRgb: GetR> GetR for Circle<TRgb> {
    fn get_r(&self) -> f32 {
        self.get_rgb().get_r()
    }
}

impl<TRgb: GetG> GetG for Circle<TRgb> {
    fn get_g(&self) -> f32 {
        self.get_rgb().get_g()
    }
}

impl<TRgb: GetB> GetB for Circle<TRgb> {
    fn get_b(&self) -> f32 {
        self.get_rgb().get_b()
    }
}

impl<TRgb: GetRgbValues> GetRgbValues for Circle<TRgb> {}

pub struct CircleParameters<TRgb> {
    name: String,
    diameter: f32,
    rgb: TRgb,
}

impl<TRgb> CircleParameters<TRgb> {
    pub fn new(name: String, diameter: f32, rgb: TRgb) -> Self {
        Self {
            name,
            diameter,
            rgb,
        }
    }
}

pub struct CircleConstructor {}

impl CircleConstructor {
    pub fn new() -> Self {
        Self {}
    }
}

impl<TRgb> ConstructObject<Circle<TRgb>, CircleParameters<TRgb>> for CircleConstructor {
    fn construct_object(&self, parameters: CircleParameters<TRgb>) -> Circle<TRgb> {
        Circle::new(parameters.name, parameters.diameter, parameters.rgb)
    }
}

pub struct CircleInstanceParameters<TCircle, TTwoDPoint> {
    name: String,
    circle: Rc<RefCell<TCircle>>,
    scale: f32,
    position: TTwoDPoint,
    diameter: f32,
}

impl<TCircle, TTwoDPoint> CircleInstanceParameters<TCircle, TTwoDPoint> {
    pub fn new(
        name: String,
        circle: Rc<RefCell<TCircle>>,
        scale: f32,
        position: TTwoDPoint,
        diameter: f32,
    ) -> Self {
        Self {
            name,
            circle,
            scale,
            position,
            diameter,
        }
    }
}

pub struct CircleInstance<TPosition, TCircle, TGeometryTriangle> {
    name: String,
    circle: Rc<RefCell<TCircle>>,
    scale: f32,
    position: TPosition,
    number_of_vertices: i32,
    vertex_data: Vec<f32>,
    geometry_triangles: Vec<TGeometryTriangle>,
}

impl<TPosition, TCircle, TGeometryTriangle> CircleInstance<TPosition, TCircle, TGeometryTriangle> {
    pub fn new(
        name: String,
        circle: Rc<RefCell<TCircle>>,
        scale: f32,
        position: TPosition,
        number_of_vertices: i32,
        vertex_data: Vec<f32>,
        geometry_triangles: Vec<TGeometryTriangle>,
    ) -> Self {
        Self {
            name,
            circle,
            scale,
            position,
            vertex_data,
            number_of_vertices,
            geometry_triangles,
        }
    }
}

pub struct CircleInstanceConstructor<TGeometryTrianglesCreator, TGeometryTriangle> {
    geometry_triangles_creator: Rc<TGeometryTrianglesCreator>,
    geometry_triangle_type: PhantomData<TGeometryTriangle>,
}

impl<TGeometryTrianglesCreator, TGeometryTriangle>
    CircleInstanceConstructor<TGeometryTrianglesCreator, TGeometryTriangle>
{
    pub fn new(geometry_triangles_creator: Rc<TGeometryTrianglesCreator>) -> Self {
        Self {
            geometry_triangles_creator: geometry_triangles_creator,
            geometry_triangle_type: PhantomData,
        }
    }
}

impl<
        TCircle: GetRgbValues,
        TGeometryTrianglesCreator: CreateGeometryTriangles<TGeometryTriangle, TCircle, TTwoDPoint>,
        TTwoDPoint: Get2DCoordiantes,
        TGeometryTriangle: GetNumberOfVertices + GetVertexData,
    >
    ConstructObject<
        CircleInstance<TTwoDPoint, TCircle, TGeometryTriangle>,
        CircleInstanceParameters<TCircle, TTwoDPoint>,
    > for CircleInstanceConstructor<TGeometryTrianglesCreator, TGeometryTriangle>
{
    fn construct_object(
        &self,
        parameters: CircleInstanceParameters<TCircle, TTwoDPoint>,
    ) -> CircleInstance<TTwoDPoint, TCircle, TGeometryTriangle> {
        let mut vertex_data = vec![];

        let mut number_of_vertices = 0;

        let geometry_triangles = self.geometry_triangles_creator.create_geometry_triangles(
            &parameters.circle.borrow(),
            &parameters.position,
            parameters.diameter,
            parameters.diameter,
        );

        for geometry_triangle in geometry_triangles.iter() {
            number_of_vertices += geometry_triangle.get_number_of_vertices();
            vertex_data.append(&mut geometry_triangle.get_vertex_data());
        }

        CircleInstance::new(
            parameters.name,
            parameters.circle,
            parameters.scale,
            parameters.position,
            number_of_vertices,
            vertex_data,
            geometry_triangles,
        )
    }
}

impl<TPosition, TCircle, TGeometryTriangle> GetName
    for CircleInstance<TPosition, TCircle, TGeometryTriangle>
{
    fn get_name(&self) -> &str {
        &self.name
    }
}

impl<TPosition, TCircle, TGeometryTriangle> GetVertexData
    for CircleInstance<TPosition, TCircle, TGeometryTriangle>
{
    fn get_vertex_data(&self) -> Vec<f32> {
        self.vertex_data.clone()
    }
}

impl<TPosition, TCircle, TGeometryTriangle> GetNumberOfVertices
    for CircleInstance<TPosition, TCircle, TGeometryTriangle>
{
    fn get_number_of_vertices(&self) -> i32 {
        self.number_of_vertices
    }
}

impl<TPosition, TCircle, TGeometryTriangle> GetNumberOfObjects
    for CircleInstance<TPosition, TCircle, TGeometryTriangle>
{
    fn get_number_of_objects(&self) -> i32 {
        2
    }
}

impl<TPosition, TCircle, TGeometryTriangle> GetContentInstanceData
    for CircleInstance<TPosition, TCircle, TGeometryTriangle>
{
}

impl<TPosition, TCircle, TGeometryTriangle> GetScale
    for CircleInstance<TPosition, TCircle, TGeometryTriangle>
{
    fn get_scale(&self) -> f32 {
        self.scale
    }
}

impl<TPosition, TCircle, TGeometryTriangle> GetPosition<TPosition>
    for CircleInstance<TPosition, TCircle, TGeometryTriangle>
{
    fn get_position(&self) -> &TPosition {
        &self.position
    }
}

impl<TPosition, TCircle: GetDiameter, TGeometryTriangle> GetDiameter
    for CircleInstance<TPosition, TCircle, TGeometryTriangle>
{
    fn get_diameter(&self) -> f32 {
        self.circle.borrow().get_diameter()
    }
}

impl<TPosition, TCircle: GetRgb<Rgb>, TGeometryTriangle> GetR
    for CircleInstance<TPosition, TCircle, TGeometryTriangle>
{
    fn get_r(&self) -> f32 {
        self.circle.borrow().get_rgb().get_r()
    }
}

impl<TPosition, TCircle: GetRgb<Rgb>, TGeometryTriangle> GetG
    for CircleInstance<TPosition, TCircle, TGeometryTriangle>
{
    fn get_g(&self) -> f32 {
        self.circle.borrow().get_rgb().get_g()
    }
}

impl<TPosition, TCircle: GetRgb<Rgb>, TGeometryTriangle> GetB
    for CircleInstance<TPosition, TCircle, TGeometryTriangle>
{
    fn get_b(&self) -> f32 {
        self.circle.borrow().get_rgb().get_b()
    }
}

impl<TPosition, TCircle: GetRgb<Rgb>, TGeometryTriangle> GetRgbValues
    for CircleInstance<TPosition, TCircle, TGeometryTriangle>
{
}

impl<TPosition, TCircle, TGeometryTriangle> GetCircle<TCircle>
    for CircleInstance<TPosition, TCircle, TGeometryTriangle>
{
    fn get_circle(&self) -> Rc<RefCell<TCircle>> {
        Rc::clone(&self.circle)
    }
}

pub struct CircleInstanceScaler<TCircleInstanceCreator, TTwoDPointCreator, TTwoDPoint, TCircle> {
    circle_instance_creator: Rc<TCircleInstanceCreator>,
    two_d_point_creator: Rc<TTwoDPointCreator>,
    two_d_point_type: PhantomData<TTwoDPoint>,
    circle_type: PhantomData<TCircle>,
}

impl<TCircleInstanceCreator, TTwoDPointCreator, TTwoDPoint, TCircle>
    CircleInstanceScaler<TCircleInstanceCreator, TTwoDPointCreator, TTwoDPoint, TCircle>
{
    pub fn new(
        circle_instance_creator: Rc<TCircleInstanceCreator>,
        two_d_point_creator: Rc<TTwoDPointCreator>,
    ) -> Self {
        Self {
            circle_instance_creator: circle_instance_creator,
            two_d_point_creator: two_d_point_creator,
            two_d_point_type: PhantomData,
            circle_type: PhantomData,
        }
    }
}

impl<
        TCircleInstance: GetName
            + GetScale
            + GetPosition<TTwoDPoint>
            + GetDiameter
            + GetRgbValues
            + GetCircle<TCircle>,
        TCircleInstanceCreator: CreateObject<TCircleInstance, CircleInstanceParameters<TCircle, TTwoDPoint>>,
        TTwoDPointCreator: CreateTwoDPoint<TTwoDPoint>,
        TTwoDPoint: Get2DCoordiantes,
        TCircle,
    > ScaleObjectInstance<TCircleInstance>
    for CircleInstanceScaler<TCircleInstanceCreator, TTwoDPointCreator, TTwoDPoint, TCircle>
{
    fn scale_object_instance(
        &self,
        circle_instance: Rc<RefCell<TCircleInstance>>,
        x: f32,
        y: f32,
    ) -> Rc<RefCell<TCircleInstance>> {
        self.circle_instance_creator
            .create_object(CircleInstanceParameters::new(
                circle_instance.borrow().get_name().to_string(),
                circle_instance.borrow().get_circle(),
                circle_instance.borrow().get_scale(),
                self.two_d_point_creator.create_two_d_point(
                    circle_instance.borrow().get_position().get_x() / x,
                    circle_instance.borrow().get_position().get_y() / y,
                ),
                circle_instance.borrow().get_diameter() / x,
            ))
    }
}
