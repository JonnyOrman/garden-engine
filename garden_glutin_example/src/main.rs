use garden_content::{Content, Rgb, Triangle, TrianglePoint, TwoDPoint};

use garden_glutin::start;

fn main() {
    start(
        "Garden: Glutin Example",
        Content::new(vec![
            Triangle::new(
                TrianglePoint::new(TwoDPoint::new(-1.0, -1.0), Rgb::new(1.0, 0.0, 0.0)),
                TrianglePoint::new(TwoDPoint::new(-0.5, 0.0), Rgb::new(0.0, 1.0, 0.0)),
                TrianglePoint::new(TwoDPoint::new(-0.0, -1.0), Rgb::new(0.0, 0.0, 1.0)),
            ),
            Triangle::new(
                TrianglePoint::new(TwoDPoint::new(0.0, 0.0), Rgb::new(1.0, 0.0, 0.0)),
                TrianglePoint::new(TwoDPoint::new(0.5, 1.0), Rgb::new(0.0, 1.0, 0.0)),
                TrianglePoint::new(TwoDPoint::new(1.0, 0.0), Rgb::new(0.0, 0.0, 1.0)),
            ),
        ]),
    )
}
