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

#[cfg(test)]
mod tests {
    use garden::GetName;
    use mockall::mock;

    use crate::rectangles::Rectangle;

    #[test]
    fn when_a_rectangle_gets_its_name_then_the_name_is_returned() {
        let name = "SomeRectangle";

        let rgb = MockRectangleRgb::new();

        let rectangle = Rectangle::<MockRectangleRgb>::new(name.to_string(), 0.0, 0.0, rgb);

        let result = rectangle.get_name();

        assert_eq!(name, result);
    }

    mock! {
        RectangleRgb {}
    }
}
