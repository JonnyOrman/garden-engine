use std::rc::Rc;

use garden_maths::trigonometry::{
    AdjacentCalculator, DegreesToRadiansConverter, OppositeCalculator, TrigonometryCalculator,
};

pub struct MathsComponent {
    degrees_to_radians_converter: Rc<DegreesToRadiansConverter>,
    adjacent_calculator: Rc<AdjacentCalculator>,
    opposite_calculator: Rc<OppositeCalculator>,
    trigonometry_calculator: Rc<
        TrigonometryCalculator<DegreesToRadiansConverter, AdjacentCalculator, OppositeCalculator>,
    >,
}

impl MathsComponent {
    fn new(
        degrees_to_radians_converter: Rc<DegreesToRadiansConverter>,
        adjacent_calculator: Rc<AdjacentCalculator>,
        opposite_calculator: Rc<OppositeCalculator>,
        trigonometry_calculator: Rc<
            TrigonometryCalculator<
                DegreesToRadiansConverter,
                AdjacentCalculator,
                OppositeCalculator,
            >,
        >,
    ) -> Self {
        Self {
            degrees_to_radians_converter,
            adjacent_calculator,
            opposite_calculator,
            trigonometry_calculator,
        }
    }

    pub fn get_trigonometry_calculator(
        &self,
    ) -> Rc<TrigonometryCalculator<DegreesToRadiansConverter, AdjacentCalculator, OppositeCalculator>>
    {
        Rc::clone(&self.trigonometry_calculator)
    }
}

pub fn compose_component() -> MathsComponent {
    let degrees_to_radians_converter = Rc::new(DegreesToRadiansConverter::new());

    let adjacent_calculator = Rc::new(AdjacentCalculator::new());

    let opposite_calculator = Rc::new(OppositeCalculator::new());

    let trigonometry_calculator = Rc::new(TrigonometryCalculator::new(
        Rc::clone(&degrees_to_radians_converter),
        Rc::clone(&adjacent_calculator),
        Rc::clone(&opposite_calculator),
    ));

    let content_component = MathsComponent::new(
        degrees_to_radians_converter,
        adjacent_calculator,
        opposite_calculator,
        trigonometry_calculator,
    );

    content_component
}
