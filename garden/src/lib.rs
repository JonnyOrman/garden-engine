use mockall::*;

#[automock]
pub trait GetName {
    fn get_name(&self) -> &str;
}

#[automock]
pub trait Initialise {
    fn initialise(&self);
}

pub trait GetInitialiser<TInitialise> {
    fn get_initialiser(&self) -> &TInitialise;
}

#[automock]
pub trait RunLoop {
    fn run_loop(&self);
}

#[automock]
pub trait Run {
    fn run(&self);
}

pub trait AddRun {
    fn add_run(&self, run: &dyn Run);
}

pub trait GetLoopRunner<TRunLoop> {
    fn get_loop_runner(&self) -> &TRunLoop;
}

pub trait GetEnder<TEnder> {
    fn get_ender(&self) -> &TEnder;
}

pub struct Component<'a> {
    name: &'a str,
}

impl<'a> Component<'a> {
    fn new(name: &'a str) -> Self {
        Self { name }
    }
}

impl<'a> GetName for Component<'a> {
    fn get_name(&self) -> &'a str {
        self.name
    }
}

pub struct InitialisationComponent<'a, TInitialise: Initialise> {
    name: &'a str,
    initialise: TInitialise,
}

impl<'a, TInitialise: Initialise> InitialisationComponent<'a, TInitialise> {
    fn new(name: &'a str, initialise: TInitialise) -> Self {
        Self { name, initialise }
    }
}

impl<'a, TInitialise: Initialise> GetName for InitialisationComponent<'a, TInitialise> {
    fn get_name(&self) -> &'a str {
        self.name
    }
}

impl<'a, TInitialise: Initialise> GetInitialiser<TInitialise>
    for InitialisationComponent<'a, TInitialise>
{
    fn get_initialiser(&self) -> &TInitialise {
        &self.initialise
    }
}

pub struct LoopComponent<'a, TRunLoop> {
    name: &'a str,
    run_loop: TRunLoop,
}

impl<'a, TRunLoop> LoopComponent<'a, TRunLoop> {
    fn new(name: &'a str, run_loop: TRunLoop) -> Self {
        Self { name, run_loop }
    }
}

impl<'a, TRunLoop> GetName for LoopComponent<'a, TRunLoop> {
    fn get_name(&self) -> &'a str {
        self.name
    }
}

impl<'a, TRunLoop> GetLoopRunner<TRunLoop> for LoopComponent<'a, TRunLoop> {
    fn get_loop_runner(&self) -> &TRunLoop {
        &self.run_loop
    }
}

pub struct EndComponent<'a, TEnder> {
    name: &'a str,
    end: TEnder,
}

impl<'a, TEnder> EndComponent<'a, TEnder> {
    fn new(name: &'a str, end: TEnder) -> Self {
        Self { name, end }
    }
}

impl<'a, TEnder> GetName for EndComponent<'a, TEnder> {
    fn get_name(&self) -> &'a str {
        self.name
    }
}

impl<'a, TEnder> GetEnder<TEnder> for EndComponent<'a, TEnder> {
    fn get_ender(&self) -> &TEnder {
        &self.end
    }
}

pub struct FullComponent<'a, TInitialise, TRunLoop, TEnd> {
    name: &'a str,
    initialise: TInitialise,
    run_loop: TRunLoop,
    end: TEnd,
}

impl<'a, TInitialise, TRunLoop, TEnd> FullComponent<'a, TInitialise, TRunLoop, TEnd> {
    fn new(name: &'a str, initialise: TInitialise, run_loop: TRunLoop, end: TEnd) -> Self {
        Self {
            name,
            initialise,
            run_loop,
            end,
        }
    }
}

impl<'a, TInitialise, TRunLoop, TEnd> GetName for FullComponent<'a, TInitialise, TRunLoop, TEnd> {
    fn get_name(&self) -> &'a str {
        self.name
    }
}

impl<'a, TInitialise, TRunLoop, TEnd> GetInitialiser<TInitialise>
    for FullComponent<'a, TInitialise, TRunLoop, TEnd>
{
    fn get_initialiser(&self) -> &TInitialise {
        &self.initialise
    }
}

impl<'a, TInitialise, TRunLoop, TEnd> GetLoopRunner<TRunLoop>
    for FullComponent<'a, TInitialise, TRunLoop, TEnd>
{
    fn get_loop_runner(&self) -> &TRunLoop {
        &self.run_loop
    }
}

impl<'a, TInitialise, TRunLoop, TEnd> GetEnder<TEnd>
    for FullComponent<'a, TInitialise, TRunLoop, TEnd>
{
    fn get_ender(&self) -> &TEnd {
        &self.end
    }
}

pub struct InitialisationEndComponent<'a, TInitialise, TEnd> {
    name: &'a str,
    initialise: TInitialise,
    end: TEnd,
}

impl<'a, TInitialise, TEnd> InitialisationEndComponent<'a, TInitialise, TEnd> {
    fn new(name: &'a str, initialise: TInitialise, end: TEnd) -> Self {
        Self {
            name,
            initialise,
            end,
        }
    }
}

impl<'a, TInitialise, TEnd> GetName for InitialisationEndComponent<'a, TInitialise, TEnd> {
    fn get_name(&self) -> &'a str {
        self.name
    }
}

impl<'a, TInitialise, TEnd> GetInitialiser<TInitialise>
    for InitialisationEndComponent<'a, TInitialise, TEnd>
{
    fn get_initialiser(&self) -> &TInitialise {
        &self.initialise
    }
}

impl<'a, TInitialise, TEnd> GetEnder<TEnd> for InitialisationEndComponent<'a, TInitialise, TEnd> {
    fn get_ender(&self) -> &TEnd {
        &self.end
    }
}

pub struct LoopEndComponent<'a, TRunLoop, TEnd> {
    name: &'a str,
    run_loop: TRunLoop,
    end: TEnd,
}

impl<'a, TRunLoop, TEnd> LoopEndComponent<'a, TRunLoop, TEnd> {
    fn new(name: &'a str, run_loop: TRunLoop, end: TEnd) -> Self {
        Self {
            name,
            run_loop,
            end,
        }
    }
}

impl<'a, TRunLoop, TEnd> GetName for LoopEndComponent<'a, TRunLoop, TEnd> {
    fn get_name(&self) -> &'a str {
        self.name
    }
}

impl<'a, TRunLoop, TEnd> GetLoopRunner<TRunLoop> for LoopEndComponent<'a, TRunLoop, TEnd> {
    fn get_loop_runner(&self) -> &TRunLoop {
        &self.run_loop
    }
}

impl<'a, TRunLoop, TEnd> GetEnder<TEnd> for LoopEndComponent<'a, TRunLoop, TEnd> {
    fn get_ender(&self) -> &TEnd {
        &self.end
    }
}

pub trait Create<T> {
    fn create(&self) -> T;
}

#[cfg(test)]
mod tests {
    use crate::{Component, GetName, InitialisationComponent, Initialise};

    use super::*;

    #[test]
    fn when_a_component_gets_its_name_then_the_name_is_returned() {
        let name = "Test Component";
        let component = Component::new(name);

        let result = component.get_name();

        assert_eq!(name, result);
    }

    #[test]
    fn when_an_initialisation_component_gets_its_name_then_the_name_is_returned() {
        let name = "Test Component";

        let initialise = MockInitialise::new();

        let component = InitialisationComponent::new(name, initialise);

        let result = component.get_name();

        assert_eq!(name, result);
    }

    #[test]
    fn when_an_initialisation_component_gets_its_initialiser_then_the_initialiser_is_returned() {
        let mut initialise = MockInitialise::new();

        initialise.expect_initialise().times(1).returning(|| ());

        let component = InitialisationComponent::new("Test Component", initialise);

        component.get_initialiser().initialise();
    }

    #[test]
    fn when_a_loop_component_gets_its_name_then_the_name_is_returned() {
        let name = "Test Component";

        let run_loop = MockRunLoop::new();

        let component = LoopComponent::new(name, run_loop);

        let result = component.get_name();

        assert_eq!(name, result);
    }

    #[test]
    fn when_a_loop_component_gets_its_loop_runner_then_the_loop_runner_is_returned() {
        let mut run_loop = MockRunLoop::new();

        run_loop.expect_run_loop().times(1).returning(|| ());

        let component = LoopComponent::new("Test Component", run_loop);

        component.get_loop_runner().run_loop();
    }

    #[test]
    fn when_an_end_component_gets_its_name_then_the_name_is_returned() {
        let name = "Test Component";

        let end = MockRun::new();

        let component = EndComponent::new(name, end);

        let result = component.get_name();

        assert_eq!(name, result);
    }

    #[test]
    fn when_an_end_component_gets_its_ender_then_the_ender_is_returned() {
        let mut end = MockRun::new();

        end.expect_run().times(1).returning(|| ());

        let component = EndComponent::new("Test Component", end);

        component.get_ender().run();
    }

    #[test]
    fn when_a_full_component_gets_its_name_then_the_name_is_returned() {
        let name = "Test Component";

        let initialise = MockInitialise::new();

        let run_loop = MockRunLoop::new();

        let end = MockRun::new();

        let component = FullComponent::new(name, initialise, run_loop, end);

        let result = component.get_name();

        assert_eq!(name, result);
    }

    #[test]
    fn when_a_full_component_gets_its_initialiser_then_the_initialiser_is_returned() {
        let mut initialise = MockInitialise::new();

        let run_loop = MockRunLoop::new();

        let end = MockRun::new();

        initialise.expect_initialise().times(1).returning(|| ());

        let component = FullComponent::new("Test Component", initialise, run_loop, end);

        component.get_initialiser().initialise();
    }

    #[test]
    fn when_a_full_component_gets_its_loop_runner_then_the_loop_runner_is_returned() {
        let initialise = MockInitialise::new();

        let mut run_loop = MockRunLoop::new();

        let end = MockRun::new();

        run_loop.expect_run_loop().times(1).returning(|| ());

        let component = FullComponent::new("Test Component", initialise, run_loop, end);

        component.get_loop_runner().run_loop();
    }

    #[test]
    fn when_a_full_component_gets_its_ender_then_the_ender_is_returned() {
        let initialise = MockInitialise::new();

        let run_loop = MockRunLoop::new();

        let mut end = MockRun::new();

        end.expect_run().times(1).returning(|| ());

        let component = FullComponent::new("Test Component", initialise, run_loop, end);

        component.get_ender().run();
    }

    #[test]
    fn when_an_initialisation_end_component_gets_its_name_then_the_name_is_returned() {
        let name = "Test Component";

        let initialise = MockInitialise::new();

        let end = MockRun::new();

        let component = InitialisationEndComponent::new(name, initialise, end);

        let result = component.get_name();

        assert_eq!(name, result);
    }

    #[test]
    fn when_an_initialisation_end_component_gets_its_initialiser_then_the_initialiser_is_returned()
    {
        let mut initialise = MockInitialise::new();

        let end = MockRun::new();

        initialise.expect_initialise().times(1).returning(|| ());

        let component = InitialisationEndComponent::new("Test Component", initialise, end);

        component.get_initialiser().initialise();
    }

    #[test]
    fn when_an_initialisation_end_component_gets_its_ender_then_the_ender_is_returned() {
        let initialise = MockInitialise::new();

        let mut end = MockRun::new();

        end.expect_run().times(1).returning(|| ());

        let component = InitialisationEndComponent::new("Test Component", initialise, end);

        component.get_ender().run();
    }

    #[test]
    fn when_a_loop_end_component_gets_its_name_then_the_name_is_returned() {
        let name = "Test Component";

        let run_loop = MockRunLoop::new();

        let end = MockRun::new();

        let component = LoopEndComponent::new(name, run_loop, end);

        let result = component.get_name();

        assert_eq!(name, result);
    }

    #[test]
    fn when_a_loop_end_component_gets_its_loop_runner_then_the_loop_runner_is_returned() {
        let mut run_loop = MockRunLoop::new();

        let end = MockRun::new();

        run_loop.expect_run_loop().times(1).returning(|| ());

        let component = LoopEndComponent::new("Test Component", run_loop, end);

        component.get_loop_runner().run_loop();
    }

    #[test]
    fn when_a_loop_end_component_gets_its_ender_then_the_ender_is_returned() {
        let run_loop = MockRunLoop::new();

        let mut end = MockRun::new();

        end.expect_run().times(1).returning(|| ());

        let component = LoopEndComponent::new("Test Component", run_loop, end);

        component.get_ender().run();
    }
}
