use std::rc::Rc;

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
    fn calculate_opposite(&self, hypotenuse: f64, angle_radians: f64) -> f64;
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

pub trait CalculateTrigonometry:
    ConvertDegreesToRadians + CalculateAdjacent + CalculateOpposite
{
}

pub struct TrigonometryCalculator<
    TDegreesToRadiansConverter,
    TAdjacentCalculator,
    TOppositeCalculator,
> {
    degrees_to_radians_converter: Rc<TDegreesToRadiansConverter>,
    adjacent_calculator: Rc<TAdjacentCalculator>,
    opposite_calculator: Rc<TOppositeCalculator>,
}

impl<TDegreesToRadiansConverter, TAdjacentCalculator, TOppositeCalculator>
    TrigonometryCalculator<TDegreesToRadiansConverter, TAdjacentCalculator, TOppositeCalculator>
{
    fn new(
        degrees_to_radians_converter: Rc<TDegreesToRadiansConverter>,
        adjacent_calculator: Rc<TAdjacentCalculator>,
        opposite_calculator: Rc<TOppositeCalculator>,
    ) -> Self {
        Self {
            degrees_to_radians_converter,
            adjacent_calculator,
            opposite_calculator,
        }
    }
}

impl<
        TDegreesToRadiansConverter: ConvertDegreesToRadians,
        TAdjacentCalculator,
        TOppositeCalculator,
    > ConvertDegreesToRadians
    for TrigonometryCalculator<TDegreesToRadiansConverter, TAdjacentCalculator, TOppositeCalculator>
{
    fn convert_degrees_to_radians(&self, degrees: f64) -> f64 {
        self.degrees_to_radians_converter
            .convert_degrees_to_radians(degrees)
    }
}

impl<TDegreesToRadiansConverter, TAdjacentCalculator: CalculateAdjacent, TOppositeCalculator>
    CalculateAdjacent
    for TrigonometryCalculator<TDegreesToRadiansConverter, TAdjacentCalculator, TOppositeCalculator>
{
    fn calculate_adjacent(&self, hypotenuse: f64, angle_radians: f64) -> f64 {
        self.adjacent_calculator
            .calculate_adjacent(hypotenuse, angle_radians)
    }
}

impl<TDegreesToRadiansConverter, TAdjacentCalculator, TOppositeCalculator: CalculateOpposite>
    CalculateOpposite
    for TrigonometryCalculator<TDegreesToRadiansConverter, TAdjacentCalculator, TOppositeCalculator>
{
    fn calculate_opposite(&self, hypotenuse: f64, angle_radians: f64) -> f64 {
        self.opposite_calculator
            .calculate_opposite(hypotenuse, angle_radians)
    }
}

impl<
        TDegreesToRadiansConverter: ConvertDegreesToRadians,
        TAdjacentCalculator: CalculateAdjacent,
        TOppositeCalculator: CalculateOpposite,
    > CalculateTrigonometry
    for TrigonometryCalculator<TDegreesToRadiansConverter, TAdjacentCalculator, TOppositeCalculator>
{
}
