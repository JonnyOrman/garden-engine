use garden::GetName;

use crate::{
    triangles::TriangleInstance, Get2DCoordiantes, GetB, GetContentInstanceData, GetG,
    GetNumberOfObjects, GetNumberOfVertices, GetR, GetRgb, GetVertexData, GetX, GetY, Rgb, Scale,
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

pub trait CreateRectangle<TRgb> {
    fn create_rectangle(&self, name: String, width: f32, height: f32, rgb: TRgb)
        -> Rectangle<TRgb>;
}

pub struct RectangleCreator {}

impl RectangleCreator {
    pub fn new() -> Self {
        Self {}
    }
}

impl<TRgb> CreateRectangle<TRgb> for RectangleCreator {
    fn create_rectangle(
        &self,
        name: String,
        width: f32,
        height: f32,
        rgb: TRgb,
    ) -> Rectangle<TRgb> {
        Rectangle::new(name, width, height, rgb)
    }
}

pub struct RectangleInstance<TPosition, TPoint, TTriangleInstance, TRgb> {
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
    triangle_instance_1: TTriangleInstance,
    triangle_instance_2: TTriangleInstance,
    rgb: TRgb,
}

impl<TPosition, TRgb, TPoint, TTriangleInstance>
    RectangleInstance<TPosition, TPoint, TTriangleInstance, TRgb>
{
    pub fn new(
        name: String,
        content_name: String,
        scale: f32,
        position: TPosition,
        width: f32,
        height: f32,
        rgb: TRgb,
        point_1: TPoint,
        point_2: TPoint,
        point_3: TPoint,
        point_4: TPoint,
        number_of_vertices: i32,
        vertex_data: Vec<f32>,
        triangle_instance_1: TTriangleInstance,
        triangle_instance_2: TTriangleInstance,
    ) -> Self {
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

impl<TPosition, TPoint, TTrianglePoint, TRgb> GetName
    for RectangleInstance<TPosition, TPoint, TTrianglePoint, TRgb>
{
    fn get_name(&self) -> &str {
        &self.name
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

        let new_triangle_instance_1_point_1 = TrianglePoint::new(
            TwoDPoint::new(new_point_1.get_x(), new_point_1.get_y()),
            Rgb::new(
                new_point_1.get_rgb().get_r(),
                new_point_1.get_rgb().get_g(),
                new_point_1.get_rgb().get_b(),
            ),
        );

        let new_triangle_instance_1_point_2 = TrianglePoint::new(
            TwoDPoint::new(new_point_2.get_x(), new_point_2.get_y()),
            Rgb::new(
                new_point_2.get_rgb().get_r(),
                new_point_2.get_rgb().get_g(),
                new_point_2.get_rgb().get_b(),
            ),
        );

        let new_triangle_instance_1_point_3 = TrianglePoint::new(
            TwoDPoint::new(new_point_3.get_x(), new_point_3.get_y()),
            Rgb::new(
                new_point_3.get_rgb().get_r(),
                new_point_3.get_rgb().get_g(),
                new_point_3.get_rgb().get_b(),
            ),
        );

        let mut new_triangle_instance_1_vertex_data = vec![];

        new_triangle_instance_1_vertex_data
            .append(&mut new_triangle_instance_1_point_1.get_vertex_data().clone());
        new_triangle_instance_1_vertex_data
            .append(&mut new_triangle_instance_1_point_2.get_vertex_data().clone());
        new_triangle_instance_1_vertex_data
            .append(&mut new_triangle_instance_1_point_3.get_vertex_data().clone());

        let new_triangle_instance_1_number_of_vertices = new_triangle_instance_1_point_1
            .get_number_of_vertices()
            + new_triangle_instance_1_point_2.get_number_of_vertices()
            + new_triangle_instance_1_point_3.get_number_of_vertices();

        let new_triangle_instance_1 = TriangleInstance::new(
            self.name.clone() + "-triangle-1",
            "".to_string(),
            self.scale,
            TwoDPoint::new(0.0, 0.0),
            new_triangle_instance_1_point_1,
            new_triangle_instance_1_point_2,
            new_triangle_instance_1_point_3,
            new_triangle_instance_1_number_of_vertices,
            new_triangle_instance_1_vertex_data,
        );

        let new_triangle_instance_2_point_1 = TrianglePoint::new(
            TwoDPoint::new(new_point_1.get_x(), new_point_1.get_y()),
            Rgb::new(
                new_point_1.get_rgb().get_r(),
                new_point_1.get_rgb().get_g(),
                new_point_1.get_rgb().get_b(),
            ),
        );

        let new_triangle_instance_2_point_2 = TrianglePoint::new(
            TwoDPoint::new(new_point_3.get_x(), new_point_3.get_y()),
            Rgb::new(
                new_point_3.get_rgb().get_r(),
                new_point_3.get_rgb().get_g(),
                new_point_3.get_rgb().get_b(),
            ),
        );

        let new_triangle_instance_2_point_3 = TrianglePoint::new(
            TwoDPoint::new(new_point_4.get_x(), new_point_4.get_y()),
            Rgb::new(
                new_point_4.get_rgb().get_r(),
                new_point_4.get_rgb().get_g(),
                new_point_4.get_rgb().get_b(),
            ),
        );

        let mut new_triangle_instance_2_vertex_data = vec![];

        new_triangle_instance_2_vertex_data
            .append(&mut new_triangle_instance_2_point_1.get_vertex_data().clone());
        new_triangle_instance_2_vertex_data
            .append(&mut new_triangle_instance_2_point_2.get_vertex_data().clone());
        new_triangle_instance_2_vertex_data
            .append(&mut new_triangle_instance_2_point_3.get_vertex_data().clone());

        let new_triangle_instance_2_number_of_vertices = new_triangle_instance_2_point_1
            .get_number_of_vertices()
            + new_triangle_instance_2_point_2.get_number_of_vertices()
            + new_triangle_instance_2_point_3.get_number_of_vertices();

        let new_triangle_instance_2 = TriangleInstance::new(
            self.name.clone() + "-triangle-2",
            "".to_string(),
            self.scale,
            TwoDPoint::new(0.0, 0.0),
            new_triangle_instance_2_point_1,
            new_triangle_instance_2_point_2,
            new_triangle_instance_2_point_3,
            new_triangle_instance_2_number_of_vertices,
            new_triangle_instance_2_vertex_data,
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

pub trait CreateRectangleInstance<TPosition, TPoint, TTriangle, TRgb> {
    fn create_rectangle_instance(
        &self,
        name: String,
        content_name: String,
        scale: f32,
        position: TPosition,
        width: f32,
        height: f32,
        rgb: TRgb,
    ) -> RectangleInstance<TPosition, TPoint, TTriangle, TRgb>;
}

pub struct RectangleInstanceCreator {}

impl RectangleInstanceCreator {
    pub fn new() -> Self {
        Self {}
    }
}

impl<TRgb: GetR + GetG + GetB, TPosition: Get2DCoordiantes>
    CreateRectangleInstance<
        TPosition,
        TrianglePoint<TwoDPoint, Rgb>,
        TriangleInstance<TwoDPoint, TrianglePoint<TwoDPoint, Rgb>>,
        TRgb,
    > for RectangleInstanceCreator
{
    fn create_rectangle_instance(
        &self,
        name: String,
        content_name: String,
        scale: f32,
        position: TPosition,
        width: f32,
        height: f32,
        rgb: TRgb,
    ) -> RectangleInstance<
        TPosition,
        TrianglePoint<TwoDPoint, Rgb>,
        TriangleInstance<TwoDPoint, TrianglePoint<TwoDPoint, Rgb>>,
        TRgb,
    > {
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

        let triangle_instance_1_point_1 = TrianglePoint::new(
            TwoDPoint::new(point_1.get_x(), point_1.get_y()),
            Rgb::new(
                point_1.get_rgb().get_r(),
                point_1.get_rgb().get_g(),
                point_1.get_rgb().get_b(),
            ),
        );

        let triangle_instance_1_point_2 = TrianglePoint::new(
            TwoDPoint::new(point_2.get_x(), point_2.get_y()),
            Rgb::new(
                point_2.get_rgb().get_r(),
                point_2.get_rgb().get_g(),
                point_2.get_rgb().get_b(),
            ),
        );

        let triangle_instance_1_point_3 = TrianglePoint::new(
            TwoDPoint::new(point_3.get_x(), point_3.get_y()),
            Rgb::new(
                point_3.get_rgb().get_r(),
                point_3.get_rgb().get_g(),
                point_3.get_rgb().get_b(),
            ),
        );

        let mut triangle_instance_1_vertex_data = vec![];

        triangle_instance_1_vertex_data
            .append(&mut triangle_instance_1_point_1.get_vertex_data().clone());
        triangle_instance_1_vertex_data
            .append(&mut triangle_instance_1_point_2.get_vertex_data().clone());
        triangle_instance_1_vertex_data
            .append(&mut triangle_instance_1_point_3.get_vertex_data().clone());

        let triangle_instance_1_number_of_vertices = triangle_instance_1_point_1
            .get_number_of_vertices()
            + triangle_instance_1_point_2.get_number_of_vertices()
            + triangle_instance_1_point_3.get_number_of_vertices();

        let triangle_instance_1 = TriangleInstance::new(
            name.clone() + "-triangle-1",
            "".to_string(),
            scale,
            TwoDPoint::new(0.0, 0.0),
            triangle_instance_1_point_1,
            triangle_instance_1_point_2,
            triangle_instance_1_point_3,
            triangle_instance_1_number_of_vertices,
            triangle_instance_1_vertex_data,
        );

        let triangle_instance_2_point_1 = TrianglePoint::new(
            TwoDPoint::new(point_1.get_x(), point_1.get_y()),
            Rgb::new(
                point_1.get_rgb().get_r(),
                point_1.get_rgb().get_g(),
                point_1.get_rgb().get_b(),
            ),
        );

        let triangle_instance_2_point_2 = TrianglePoint::new(
            TwoDPoint::new(point_3.get_x(), point_3.get_y()),
            Rgb::new(
                point_3.get_rgb().get_r(),
                point_3.get_rgb().get_g(),
                point_3.get_rgb().get_b(),
            ),
        );

        let triangle_instance_2_point_3 = TrianglePoint::new(
            TwoDPoint::new(point_4.get_x(), point_4.get_y()),
            Rgb::new(
                point_4.get_rgb().get_r(),
                point_4.get_rgb().get_g(),
                point_4.get_rgb().get_b(),
            ),
        );

        let mut triangle_instance_2_vertex_data = vec![];

        triangle_instance_2_vertex_data
            .append(&mut triangle_instance_2_point_1.get_vertex_data().clone());
        triangle_instance_2_vertex_data
            .append(&mut triangle_instance_2_point_2.get_vertex_data().clone());
        triangle_instance_2_vertex_data
            .append(&mut triangle_instance_2_point_3.get_vertex_data().clone());

        let triangle_instance_2_number_of_vertices = triangle_instance_2_point_1
            .get_number_of_vertices()
            + triangle_instance_2_point_2.get_number_of_vertices()
            + triangle_instance_2_point_3.get_number_of_vertices();

        let triangle_instance_2 = TriangleInstance::new(
            name.clone() + "-triangle-2",
            "".to_string(),
            scale,
            TwoDPoint::new(0.0, 0.0),
            triangle_instance_2_point_1,
            triangle_instance_2_point_2,
            triangle_instance_2_point_3,
            triangle_instance_2_number_of_vertices,
            triangle_instance_2_vertex_data,
        );

        vertex_data.append(&mut triangle_instance_1.get_vertex_data().clone());
        vertex_data.append(&mut triangle_instance_2.get_vertex_data().clone());

        let number_of_vertices = triangle_instance_1.get_number_of_vertices()
            + triangle_instance_2.get_number_of_vertices();

        RectangleInstance::new(
            name,
            content_name,
            scale,
            position,
            width,
            height,
            rgb,
            point_1,
            point_2,
            point_3,
            point_4,
            number_of_vertices,
            vertex_data,
            triangle_instance_1,
            triangle_instance_2,
        )
    }
}

#[cfg(test)]
mod tests {
    use garden::GetName;
    use mockall::mock;

    use crate::{
        Get2DCoordiantes, GetB, GetG, GetNumberOfObjects, GetNumberOfVertices, GetR, GetX, GetY,
    };

    use crate::rectangles::{
        CreateRectangleInstance, Rectangle, RectangleInstance, RectangleInstanceCreator,
    };

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

        let rectangle_creator = RectangleCreator::new();

        let rectangle = rectangle_creator.create_rectangle(name.to_string(), width, height, rgb);

        assert_eq!(name, rectangle.get_name());
    }

    #[test]
    fn when_a_rectangle_instance_gets_its_name_then_the_name_is_returned() {
        let name = "RectangleInstanceName";

        let content_name = "";

        let scale = 0.0;

        let position = MockRectanglePosition::new();

        let width = 0.0;

        let height = 0.0;

        let rgb = MockRectangleRgb::new();

        let point_1 = MockRectanglePoint::new();

        let point_2 = MockRectanglePoint::new();

        let point_3 = MockRectanglePoint::new();

        let point_4 = MockRectanglePoint::new();

        let number_of_vertices = 0;

        let vertex_data = vec![];

        let triangle_instance_1 = MockRectangleTriangleInstance::new();

        let triangle_instance_2 = MockRectangleTriangleInstance::new();

        let rectangle_instance = RectangleInstance::new(
            name.to_string(),
            content_name.to_string(),
            scale,
            position,
            width,
            height,
            rgb,
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

    #[test]
    fn when_a_rectangle_instance_creator_creates_a_rectangle_instance_then_the_rectangle_instance_is_created(
    ) {
        let name = "RectangleInstanceName";

        let content_name = "";

        let scale = 0.0;

        let mut position = MockRectanglePosition::new();
        position.expect_get_x().times(4).returning(move || 1.23);
        position.expect_get_y().times(4).returning(move || 4.56);

        let width = 1.23;

        let height = 4.56;

        let mut rgb = MockRectangleRgb::new();
        rgb.expect_get_r().times(4).returning(move || 1.0);
        rgb.expect_get_g().times(4).returning(move || 0.0);
        rgb.expect_get_b().times(4).returning(move || 0.0);

        let rectangle_instance_creator = RectangleInstanceCreator::new();

        let rectangle_instance = rectangle_instance_creator.create_rectangle_instance(
            name.to_string(),
            content_name.to_string(),
            scale,
            position,
            width,
            height,
            rgb,
        );

        assert_eq!(name, rectangle_instance.get_name());
        assert_eq!(30, rectangle_instance.get_number_of_vertices());
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
        RectangleTriangleInstance{}
    }
}
