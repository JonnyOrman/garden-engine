use std::rc::Rc;

pub trait ConvertDegreesToRadians {
    fn convert_degrees_to_radians(&self, degrees: f64) -> f64;
}

pub struct DegreesToRadiansConverter {}

impl DegreesToRadiansConverter {
    pub fn new() -> Self {
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
    pub fn new() -> Self {
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
    pub fn new() -> Self {
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
    pub fn new(
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

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use crate::trigonometry::{
        AdjacentCalculator, CalculateAdjacent, CalculateOpposite, ConvertDegreesToRadians,
        DegreesToRadiansConverter, OppositeCalculator,
    };

    #[rstest]
    #[case(0.0, 0.0)]
    #[case(1.0, 0.017453292519943295)]
    #[case(15.0, 0.2617993877991494)]
    #[case(30.0, 0.5235987755982988)]
    #[case(45.0, 0.7853981633974483)]
    #[case(60.0, 1.0471975511965976)]
    #[case(90.0, 1.5707963267948966)]
    #[case(135.0, 2.356194490192345)]
    #[case(180.0, 3.141592653589793)]
    #[case(225.0, 3.9269908169872414)]
    #[case(270.0, 4.71238898038469)]
    #[case(315.0, 5.497787143782138)]
    #[case(360.0, 6.283185307179586)]
    fn when_a_degrees_to_radians_converter_converts_degrees_to_radians_then_they_are_converted_to_radians(
        #[case] degrees: f64,
        #[case] expected_radians: f64,
    ) {
        let degrees_to_radians_converter = DegreesToRadiansConverter::new();

        let result = degrees_to_radians_converter.convert_degrees_to_radians(degrees);

        assert_eq!(expected_radians, result);
    }

    #[rstest]
    #[case(5.0, 0.017453292519943295, 4.999238475781956)]
    #[case(5.0, 0.2617993877991494, 4.8296291314453415)]
    #[case(5.0, 0.5235987755982988, 4.330127018922194)]
    #[case(5.0, 0.7853981633974483, 3.5355339059327378)]
    #[case(5.0, 1.0471975511965976, 2.5000000000000004)]
    fn when_an_adjacent_calculator_calculates_the_adjacent_then_the_adjacent_is_calculated(
        #[case] hypotenuse: f64,
        #[case] angle_radians: f64,
        #[case] expected_adjacent: f64,
    ) {
        let adjacent_calculator = AdjacentCalculator::new();

        let result = adjacent_calculator.calculate_adjacent(hypotenuse, angle_radians);

        assert_eq!(expected_adjacent, result);
    }

    #[rstest]
    #[case(5.0, 0.017453292519943295, 0.08726203218641757)]
    #[case(5.0, 0.2617993877991494, 1.2940952255126037)]
    #[case(5.0, 0.5235987755982988, 2.4999999999999996)]
    #[case(5.0, 0.7853981633974483, 3.5355339059327373)]
    #[case(5.0, 1.0471975511965976, 4.330127018922193)]
    fn when_an_opposite_calculator_calculates_the_opposite_then_the_opposite_is_calculated(
        #[case] hypotenuse: f64,
        #[case] angle_radians: f64,
        #[case] expected_opposite: f64,
    ) {
        let opposite_calculator = OppositeCalculator::new();

        let result = opposite_calculator.calculate_opposite(hypotenuse, angle_radians);

        assert_eq!(expected_opposite, result);
    }
}
