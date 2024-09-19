use crate::{GetB, GetG, GetR, GetVertexData, Rgb, TrianglePoint, TwoDPoint};

pub trait CalculateEquilateralTrianglePoint<TTrianglePoint, TRgb> {
    fn calculate_equilateral_triangle_point(
        &self,
        size: f32,
        rgb: &TRgb,
        point: i32,
    ) -> TTrianglePoint;
}

pub struct EquilateralTrianglePointCalculator {}

impl EquilateralTrianglePointCalculator {
    pub fn new() -> Self {
        Self {}
    }
}

impl CalculateEquilateralTrianglePoint<TrianglePoint<TwoDPoint, Rgb>, Rgb>
    for EquilateralTrianglePointCalculator
{
    fn calculate_equilateral_triangle_point(
        &self,
        size: f32,
        rgb: &Rgb,
        point: i32,
    ) -> TrianglePoint<TwoDPoint, Rgb> {
        let halfSize = size / 2.0;

        let heightSquared = (size * size) - (halfSize * halfSize);

        let height = heightSquared.sqrt();

        let mut x = 0.0;
        let mut y = 0.0;

        if (point == 1) {
            y = height / 2.0;
        } else if (point == 2) {
            x = size / 2.0;
            y = (height / 2.0) * -1.0;
        } else if (point == 3) {
            x = halfSize * -1.0;
            y = (height / 2.0) * -1.0;
        }
        // else {
        //     throw
        // }

        let point = TwoDPoint::new(x, y);

        let this_rgb = Rgb::new(rgb.get_r(), rgb.get_g(), rgb.get_b());

        let mut vertex_data = vec![];
        vertex_data.append(&mut point.get_vertex_data());
        vertex_data.append(&mut rgb.get_vertex_data());

        TrianglePoint::<TwoDPoint, Rgb>::new(point, this_rgb, 3, vertex_data)
    }
}
