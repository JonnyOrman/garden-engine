pub trait ConvertDegreesToRadians {
    fn convert_degrees_to_radians(&self, degrees: f64) -> f64;
}

pub struct DegreesToRadiansConverter {}

impl DegreesToRadiansConverter {
    fn new() -> Self {
        Self {}
    }
}

impl ConvertDegreesToRadians for DegreesToRadiansConverter {
    fn convert_degrees_to_radians(&self, degrees: f64) -> f64 {
        degrees * std::f64::consts::PI / 180.0
    }
}

pub trait CalculateAdjacent {
    fn calculate_adjacent(&self, hypotenuse: f64, angle_radians: f64) -> f64;
}

pub struct AdjacentCalculator {}

impl AdjacentCalculator {
    fn new() -> Self {
        Self {}
    }
}

impl CalculateAdjacent for AdjacentCalculator {
    fn calculate_adjacent(&self, hypotenuse: f64, angle_radians: f64) -> f64 {
        angle_radians.cos() * hypotenuse
    }
}

pub trait CalculateOpposite {
    fn calculate_adjacent(&self, hypotenuse: f64, angle_radians: f64) -> f64;
}

pub struct OppositeCalculator {}

impl OppositeCalculator {
    fn new() -> Self {
        Self {}
    }
}

impl CalculateOpposite for OppositeCalculator {
    fn calculate_opposite(&self, hypotenuse: f64, angle_radians: f64) -> f64 {
        angle_radians.sin() * hypotenuse
    }
}
