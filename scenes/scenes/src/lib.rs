use garden::{GetHeight, GetWidth};

pub struct TwoDScene {
    width: f32,
    height: f32,
}

impl TwoDScene {
    pub fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }
}

impl GetWidth for TwoDScene {
    fn get_width(&self) -> f32 {
        self.width
    }
}

impl GetHeight for TwoDScene {
    fn get_height(&self) -> f32 {
        self.height
    }
}

#[cfg(test)]
mod tests {
    use crate::TwoDScene;
    use garden::{GetHeight, GetWidth};

    #[test]
    fn when_a_two_d_scene_gets_its_width_then_the_width_is_returned() {
        let width = 123.45;
        let height = 0.0;

        let two_d_point = TwoDScene::new(width, height);

        let result = two_d_point.get_width();

        assert_eq!(result, width);
    }

    #[test]
    fn when_a_two_d_scene_gets_its_height_then_the_height_is_returned() {
        let width = 0.0;
        let height = 123.45;

        let two_d_point = TwoDScene::new(width, height);

        let result = two_d_point.get_height();

        assert_eq!(result, height);
    }
}
