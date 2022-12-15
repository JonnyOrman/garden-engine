use garden_content::{Content, Rgb, Triangle, TrianglePoint, TwoDPoint};

pub trait LoadContent<TContent> {
    fn load_content(self) -> TContent;
}

pub struct ContentLoader {}

impl ContentLoader {
    pub fn new() -> Self {
        Self {}
    }
}

impl LoadContent<Content<Triangle<TrianglePoint<TwoDPoint, Rgb>>>> for ContentLoader {
    fn load_content(self) -> Content<Triangle<TrianglePoint<TwoDPoint, Rgb>>> {
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
        ])
    }
}
