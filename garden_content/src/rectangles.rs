use std::{cell::RefCell, rc::Rc};

use garden::{GetHeight, GetName, GetWidth};

use crate::{
    triangles::{CreateTriangleInstance, Triangle},
    AddContent, CreateTrianglePoint, CreateTwoDPoint, Get2DCoordiantes, GetB, GetContent,
    GetContentInstanceData, GetG, GetNumberOfObjects, GetNumberOfVertices, GetPosition, GetR,
    GetRgb, GetRgbValues, GetScale, GetVertexData, GetX, GetY, Rgb, ScaleObjectInstance,
    TrianglePoint, TwoDPoint,
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

pub trait CreateRectangle<TRgb> {
    fn create_rectangle(
        &self,
        name: String,
        width: f32,
        height: f32,
        rgb: TRgb,
    ) -> Rc<RefCell<Rectangle<TRgb>>>;
}

pub struct RectangleCreator<TRectangleProvider> {
    rectangle_provider: Rc<RefCell<TRectangleProvider>>,
}

impl<TRectangleProvider> RectangleCreator<TRectangleProvider> {
    pub fn new(rectangle_provider: Rc<RefCell<TRectangleProvider>>) -> Self {
        Self { rectangle_provider }
    }
}

impl<TRectangleProvider: AddContent<Rectangle<TRgb>>, TRgb> CreateRectangle<TRgb>
    for RectangleCreator<TRectangleProvider>
{
    fn create_rectangle(
        &self,
        name: String,
        width: f32,
        height: f32,
        rgb: TRgb,
    ) -> Rc<RefCell<Rectangle<TRgb>>> {
        let rectangle = Rc::new(RefCell::new(Rectangle::new(name, width, height, rgb)));

        let rectangle_ref: std::cell::Ref<Rectangle<TRgb>> = rectangle.borrow();

        self.rectangle_provider
            .borrow_mut()
            .add_content(Rc::clone(&rectangle));

        Rc::clone(&rectangle)
    }
}

pub struct RectangleInstance<TPosition, TPoint, TTriangleInstance, TRectangle> {
    name: String,
    rectangle: Rc<RefCell<TRectangle>>,
    scale: f32,
    position: TPosition,
    number_of_vertices: i32,
    vertex_data: Vec<f32>,
    point_1: TPoint,
    point_2: TPoint,
    point_3: TPoint,
    point_4: TPoint,
    triangle_instance_1: Rc<RefCell<TTriangleInstance>>,
    triangle_instance_2: Rc<RefCell<TTriangleInstance>>,
}

impl<TPosition, TPoint, TTriangleInstance, TRectangle>
    RectangleInstance<TPosition, TPoint, TTriangleInstance, TRectangle>
{
    pub fn new(
        name: String,
        rectangle: Rc<RefCell<TRectangle>>,
        scale: f32,
        position: TPosition,
        point_1: TPoint,
        point_2: TPoint,
        point_3: TPoint,
        point_4: TPoint,
        number_of_vertices: i32,
        vertex_data: Vec<f32>,
        triangle_instance_1: Rc<RefCell<TTriangleInstance>>,
        triangle_instance_2: Rc<RefCell<TTriangleInstance>>,
    ) -> Self {
        Self {
            name,
            rectangle,
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
        }
    }
}

impl<TPosition, TPoint, TTrianglePoint, TRectangle> GetName
    for RectangleInstance<TPosition, TPoint, TTrianglePoint, TRectangle>
{
    fn get_name(&self) -> &str {
        &self.name
    }
}

impl<TPosition, TPoint, TRectangle, TTriangleInstance> GetVertexData
    for RectangleInstance<TPosition, TPoint, TTriangleInstance, TRectangle>
{
    fn get_vertex_data(&self) -> Vec<f32> {
        self.vertex_data.clone()
    }
}

impl<TPosition, TPoint, TRectangle, TTriangleInstance> GetNumberOfVertices
    for RectangleInstance<TPosition, TPoint, TTriangleInstance, TRectangle>
{
    fn get_number_of_vertices(&self) -> i32 {
        self.number_of_vertices
    }
}

impl<TPosition, TPoint, TRectangle, TTriangleInstance> GetNumberOfObjects
    for RectangleInstance<TPosition, TPoint, TRectangle, TTriangleInstance>
{
    fn get_number_of_objects(&self) -> i32 {
        2
    }
}

impl<TPosition, TPoint, TRectangle, TTriangleInstance> GetContentInstanceData
    for RectangleInstance<TPosition, TPoint, TRectangle, TTriangleInstance>
{
}

impl<TPosition, TPoint, TTrianglePoint, TRectangle> GetScale
    for RectangleInstance<TPosition, TPoint, TTrianglePoint, TRectangle>
{
    fn get_scale(&self) -> f32 {
        self.scale
    }
}

impl<TPosition, TPoint, TTrianglePoint, TRectangle: GetWidth> GetWidth
    for RectangleInstance<TPosition, TPoint, TTrianglePoint, TRectangle>
{
    fn get_width(&self) -> f32 {
        self.rectangle.borrow().get_width()
    }
}

impl<TPosition, TPoint, TTrianglePoint, TRectangle: GetHeight> GetHeight
    for RectangleInstance<TPosition, TPoint, TTrianglePoint, TRectangle>
{
    fn get_height(&self) -> f32 {
        self.rectangle.borrow().get_height()
    }
}

impl<TPosition, TPoint, TTrianglePoint, TRectangle> GetPosition<TPosition>
    for RectangleInstance<TPosition, TPoint, TTrianglePoint, TRectangle>
{
    fn get_position(&self) -> &TPosition {
        &self.position
    }
}

impl<TPosition, TPoint, TTrianglePoint, TRectangle: GetRgb<Rgb>> GetR
    for RectangleInstance<TPosition, TPoint, TTrianglePoint, TRectangle>
{
    fn get_r(&self) -> f32 {
        self.rectangle.borrow().get_rgb().get_r()
    }
}

impl<TPosition, TPoint, TTrianglePoint, TRectangle: GetRgb<Rgb>> GetG
    for RectangleInstance<TPosition, TPoint, TTrianglePoint, TRectangle>
{
    fn get_g(&self) -> f32 {
        self.rectangle.borrow().get_rgb().get_g()
    }
}

impl<TPosition, TPoint, TTrianglePoint, TRectangle: GetRgb<Rgb>> GetB
    for RectangleInstance<TPosition, TPoint, TTrianglePoint, TRectangle>
{
    fn get_b(&self) -> f32 {
        self.rectangle.borrow().get_rgb().get_b()
    }
}

impl<TPosition, TPoint, TTrianglePoint, TRectangle: GetRgb<Rgb>> GetRgbValues
    for RectangleInstance<TPosition, TPoint, TTrianglePoint, TRectangle>
{
}

impl<TPosition, TPoint, TTrianglePoint, TRectangle> GetRectangle<TRectangle>
    for RectangleInstance<TPosition, TPoint, TTrianglePoint, TRectangle>
{
    fn get_rectangle(&self) -> Rc<RefCell<TRectangle>> {
        Rc::clone(&self.rectangle)
    }
}

pub trait CreateRectangleInstance<TPosition, TRectangleInstance, TRectangle> {
    fn create_rectangle_instance(
        &self,
        name: String,
        rectangle: Rc<RefCell<TRectangle>>,
        scale: f32,
        position: TPosition,
        width: f32,
        height: f32,
    ) -> Rc<RefCell<TRectangleInstance>>;
}

pub struct RectangleInstanceCreator<
    TTriangleInstanceCreator,
    TTrianglePointCreator,
    TTwoDPointCreator,
> {
    triangle_instance_creator: Rc<TTriangleInstanceCreator>,
    triangle_point_creator: Rc<TTrianglePointCreator>,
    two_d_point_creator: Rc<TTwoDPointCreator>,
}

impl<TTriangleInstanceCreator, TTrianglePointCreator, TTwoDPointCreator>
    RectangleInstanceCreator<TTriangleInstanceCreator, TTrianglePointCreator, TTwoDPointCreator>
{
    pub fn new(
        triangle_instance_creator: Rc<TTriangleInstanceCreator>,
        triangle_point_creator: Rc<TTrianglePointCreator>,
        two_d_point_creator: Rc<TTwoDPointCreator>,
    ) -> Self {
        Self {
            triangle_instance_creator,
            triangle_point_creator,
            two_d_point_creator,
        }
    }
}

impl<
        TTriangleInstanceCreator: CreateTriangleInstance<
            TPosition,
            TrianglePoint<TwoDPoint, Rgb>,
            TTriangleInstance,
            Triangle<TrianglePoint<TwoDPoint, Rgb>>,
        >,
        TTrianglePointCreator: CreateTrianglePoint<TrianglePoint<TwoDPoint, Rgb>>,
        TTwoDPointCreator: CreateTwoDPoint<TPosition>,
        TPosition: Get2DCoordiantes,
        TTriangleInstance: GetContentInstanceData,
        TRectangle: GetWidth + GetHeight + GetRgb<Rgb>,
    >
    CreateRectangleInstance<
        TPosition,
        RectangleInstance<TPosition, TrianglePoint<TwoDPoint, Rgb>, TTriangleInstance, TRectangle>,
        TRectangle,
    >
    for RectangleInstanceCreator<TTriangleInstanceCreator, TTrianglePointCreator, TTwoDPointCreator>
{
    fn create_rectangle_instance(
        &self,
        name: String,
        rectangle: Rc<RefCell<TRectangle>>,
        scale: f32,
        position: TPosition,
        width: f32,
        height: f32,
    ) -> Rc<
        RefCell<
            RectangleInstance<
                TPosition,
                TrianglePoint<TwoDPoint, Rgb>,
                TTriangleInstance,
                TRectangle,
            >,
        >,
    > {
        let mut vertex_data = vec![];

        let x = width / 2.0;
        let y = height / 2.0;

        let point_1 = self.triangle_point_creator.create_triangle_point(
            position.get_x() + x,
            position.get_y() + y,
            rectangle.borrow().get_rgb().get_r(),
            rectangle.borrow().get_rgb().get_g(),
            rectangle.borrow().get_rgb().get_b(),
        );

        let point_2 = self.triangle_point_creator.create_triangle_point(
            position.get_x() - x,
            position.get_y() + y,
            rectangle.borrow().get_rgb().get_r(),
            rectangle.borrow().get_rgb().get_g(),
            rectangle.borrow().get_rgb().get_b(),
        );

        let point_3 = self.triangle_point_creator.create_triangle_point(
            position.get_x() - x,
            position.get_y() - y,
            rectangle.borrow().get_rgb().get_r(),
            rectangle.borrow().get_rgb().get_g(),
            rectangle.borrow().get_rgb().get_b(),
        );

        let point_4 = self.triangle_point_creator.create_triangle_point(
            position.get_x() + x,
            position.get_y() - y,
            rectangle.borrow().get_rgb().get_r(),
            rectangle.borrow().get_rgb().get_g(),
            rectangle.borrow().get_rgb().get_b(),
        );

        let triangle_instance_1_point_1 = self.triangle_point_creator.create_triangle_point(
            point_1.get_x(),
            point_1.get_y(),
            point_1.get_r(),
            point_1.get_g(),
            point_1.get_b(),
        );

        let triangle_instance_1_point_2 = self.triangle_point_creator.create_triangle_point(
            point_2.get_x(),
            point_2.get_y(),
            point_2.get_r(),
            point_2.get_g(),
            point_2.get_b(),
        );

        let triangle_instance_1_point_3 = self.triangle_point_creator.create_triangle_point(
            point_3.get_x(),
            point_3.get_y(),
            point_3.get_r(),
            point_3.get_g(),
            point_3.get_b(),
        );

        let triangle_instance_1 = self.triangle_instance_creator.create_triangle_instance(
            name.clone() + "-triangle-1",
            Rc::new(RefCell::new(
                Triangle::<TrianglePoint<TwoDPoint, Rgb>>::new(
                    "TEMP".to_string(),
                    TrianglePoint::<TwoDPoint, Rgb>::new(
                        TwoDPoint::new(0.0, 0.0),
                        Rgb::new(0.0, 0.0, 0.0),
                        0,
                        vec![],
                    ),
                    TrianglePoint::<TwoDPoint, Rgb>::new(
                        TwoDPoint::new(0.0, 0.0),
                        Rgb::new(0.0, 0.0, 0.0),
                        0,
                        vec![],
                    ),
                    TrianglePoint::<TwoDPoint, Rgb>::new(
                        TwoDPoint::new(0.0, 0.0),
                        Rgb::new(0.0, 0.0, 0.0),
                        0,
                        vec![],
                    ),
                    vec![],
                    0,
                ),
            )),
            scale,
            self.two_d_point_creator.create_two_d_point(0.0, 0.0),
            triangle_instance_1_point_1,
            triangle_instance_1_point_2,
            triangle_instance_1_point_3,
        );

        let triangle_instance_2_point_1 = self.triangle_point_creator.create_triangle_point(
            point_1.get_x(),
            point_1.get_y(),
            point_1.get_r(),
            point_1.get_g(),
            point_1.get_b(),
        );

        let triangle_instance_2_point_2 = self.triangle_point_creator.create_triangle_point(
            point_3.get_x(),
            point_3.get_y(),
            point_3.get_r(),
            point_3.get_g(),
            point_3.get_b(),
        );

        let triangle_instance_2_point_3 = self.triangle_point_creator.create_triangle_point(
            point_4.get_x(),
            point_4.get_y(),
            point_4.get_r(),
            point_4.get_g(),
            point_4.get_b(),
        );

        let triangle_instance_2 = self.triangle_instance_creator.create_triangle_instance(
            name.clone() + "-triangle-2",
            Rc::new(RefCell::new(
                Triangle::<TrianglePoint<TwoDPoint, Rgb>>::new(
                    "TEMP".to_string(),
                    TrianglePoint::<TwoDPoint, Rgb>::new(
                        TwoDPoint::new(0.0, 0.0),
                        Rgb::new(0.0, 0.0, 0.0),
                        0,
                        vec![],
                    ),
                    TrianglePoint::<TwoDPoint, Rgb>::new(
                        TwoDPoint::new(0.0, 0.0),
                        Rgb::new(0.0, 0.0, 0.0),
                        0,
                        vec![],
                    ),
                    TrianglePoint::<TwoDPoint, Rgb>::new(
                        TwoDPoint::new(0.0, 0.0),
                        Rgb::new(0.0, 0.0, 0.0),
                        0,
                        vec![],
                    ),
                    vec![],
                    0,
                ),
            )),
            scale,
            self.two_d_point_creator.create_two_d_point(0.0, 0.0),
            triangle_instance_2_point_1,
            triangle_instance_2_point_2,
            triangle_instance_2_point_3,
        );

        vertex_data.append(&mut triangle_instance_1.borrow().get_vertex_data().clone());
        vertex_data.append(&mut triangle_instance_2.borrow().get_vertex_data().clone());

        let number_of_vertices = triangle_instance_1.borrow().get_number_of_vertices()
            + triangle_instance_2.borrow().get_number_of_vertices();

        Rc::new(RefCell::new(RectangleInstance::new(
            name,
            rectangle,
            scale,
            position,
            point_1,
            point_2,
            point_3,
            point_4,
            number_of_vertices,
            vertex_data,
            triangle_instance_1,
            triangle_instance_2,
        )))
    }
}

pub struct RectangleInstanceScaler<TRectangleInstanceCreator, TTwoDPointCreator> {
    rectangle_instance_creator: Rc<TRectangleInstanceCreator>,
    two_d_point_creator: Rc<TTwoDPointCreator>,
}

impl<TRectangleInstanceCreator, TTwoDPointCreator>
    RectangleInstanceScaler<TRectangleInstanceCreator, TTwoDPointCreator>
{
    pub fn new(
        rectangle_instance_creator: Rc<TRectangleInstanceCreator>,
        two_d_point_creator: Rc<TTwoDPointCreator>,
    ) -> Self {
        Self {
            rectangle_instance_creator,
            two_d_point_creator,
        }
    }
}

impl<
        'ri,
        'r,
        TRectangleInstance: GetName
            + GetScale
            + GetPosition<TwoDPoint>
            + GetWidth
            + GetHeight
            + GetRgbValues
            + GetRectangle<Rectangle<Rgb>>,
        TRectangleInstanceCreator: CreateRectangleInstance<TwoDPoint, TRectangleInstance, Rectangle<Rgb>>,
        TTwoDPointCreator: CreateTwoDPoint<TwoDPoint>,
    > ScaleObjectInstance<TRectangleInstance>
    for RectangleInstanceScaler<TRectangleInstanceCreator, TTwoDPointCreator>
{
    fn scale_object_instance(
        &self,
        rectangle_instance: Rc<RefCell<TRectangleInstance>>,
        x: f32,
        y: f32,
    ) -> Rc<RefCell<TRectangleInstance>> {
        self.rectangle_instance_creator.create_rectangle_instance(
            rectangle_instance.borrow().get_name().to_string(),
            rectangle_instance.borrow().get_rectangle(),
            rectangle_instance.borrow().get_scale(),
            self.two_d_point_creator.create_two_d_point(
                rectangle_instance.borrow().get_position().get_x() / x,
                rectangle_instance.borrow().get_position().get_y() / y,
            ),
            rectangle_instance.borrow().get_width() / x,
            rectangle_instance.borrow().get_height() / y,
        )
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
        println!("LOOKING FOR CONTENT WITH NAME {}", content_name);

        for content in self.content.iter() {
            println!("CHECKING CONTENT WITH NAME {}", content.borrow().get_name());

            if content.borrow().get_name() == content_name {
                return Rc::clone(&content);
            }
        }

        todo!()
    }
}

impl<TContent> AddContent<TContent> for ContentProvider<TContent> {
    fn add_content(&mut self, content: Rc<RefCell<TContent>>) {
        self.content.push(content)
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;

    use garden::GetName;
    use mockall::mock;

    use crate::{AddContent, Get2DCoordiantes, GetB, GetG, GetR, GetX, GetY};

    use crate::rectangles::{Rectangle, RectangleInstance};

    use super::{CreateRectangle, RectangleCreator};

    #[test]
    fn when_a_rectangle_gets_its_name_then_the_name_is_returned() {
        let name = "RectangleName";

        let rgb = MockRectangleRgb::new();

        let rectangle = Rectangle::<MockRectangleRgb>::new(name.to_string(), 0.0, 0.0, rgb);

        let result = rectangle.get_name();

        assert_eq!(name, result);
    }

    #[test]
    fn when_a_rectangle_creator_creates_a_rectangle_then_the_rectangle_is_created() {
        let name = "RectangleName";

        let width = 1.23;

        let height = 4.56;

        let rgb = MockRectangleRgb::new();

        let rectangle_provider = Rc::new(RefCell::new(MockRectangleProvider::new()));
        rectangle_provider
            .borrow_mut()
            .expect_add_content()
            .times(1)
            .returning(move |_| {});

        let rectangle_creator = RectangleCreator::new(rectangle_provider);

        let rectangle = rectangle_creator.create_rectangle(name.to_string(), width, height, rgb);

        assert_eq!(name, rectangle.borrow().get_name());
    }

    #[test]
    fn when_a_rectangle_instance_gets_its_name_then_the_name_is_returned() {
        let name = "RectangleInstanceName";

        let rectangle = Rc::new(RefCell::new(Rectangle::new(
            "Rectangle".to_string(),
            1.0,
            2.0,
            MockRectangleRgb::new(),
        )));

        let scale = 0.0;

        let position = MockRectanglePosition::new();

        let point_1 = MockRectanglePoint::new();

        let point_2 = MockRectanglePoint::new();

        let point_3 = MockRectanglePoint::new();

        let point_4 = MockRectanglePoint::new();

        let number_of_vertices = 0;

        let vertex_data = vec![];

        let triangle_instance_1 = Rc::new(RefCell::new(MockRectangleTriangleInstance::new()));

        let triangle_instance_2 = Rc::new(RefCell::new(MockRectangleTriangleInstance::new()));

        let rectangle_instance = RectangleInstance::new(
            name.to_string(),
            rectangle,
            scale,
            position,
            point_1,
            point_2,
            point_3,
            point_4,
            number_of_vertices,
            vertex_data,
            triangle_instance_1,
            triangle_instance_2,
        );

        let result = rectangle_instance.get_name();

        assert_eq!(name, result);
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
        RectanglePoint {}
    }

    mock! {
        RectangleTriangleInstance {}
    }

    mock! {
        RectangleProvider {}
        impl AddContent<Rectangle<MockRectangleRgb>> for RectangleProvider {
            fn add_content(&mut self, content: Rc<RefCell<Rectangle<MockRectangleRgb>>>);
        }
    }
}
