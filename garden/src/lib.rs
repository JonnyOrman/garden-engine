pub trait GetName {
    fn get_name(&self) -> &str;
}

pub trait Initialise {
    fn initialise(&self);
}

pub trait GetInitialiser<TInitialise> {
    fn get_initialiser(&self) -> &TInitialise;
}

pub trait RunLoop {
    fn run_loop(&self);
}

pub trait GetLoopRunner<TRunLoop> {
    fn get_loop_runner(&self) -> &TRunLoop;
}

pub trait End {
    fn end(&self);
}

pub trait GetEnder<TEnd> {
    fn get_ender(&self) -> &TEnd;
}

pub struct Component<'a> {
    name: &'a str
}

impl<'a> Component<'a> {
    fn new(name: &'a str) -> Self{Self{name}}
}

impl<'a> GetName for Component<'a> {
    fn get_name(&self) -> &'a str {
        self.name
    }
}

pub struct InitialisationComponent<'a, TInitialise> {
    name: &'a str,
    initialise: TInitialise
}

impl<'a, TInitialise> InitialisationComponent<'a, TInitialise> {
    fn new(
        name: &'a str,
        initialise: TInitialise) -> Self{Self{
            name,
            initialise}}
}

impl<'a, TInitialise> GetName for InitialisationComponent<'a, TInitialise> {
    fn get_name(&self) -> &'a str {
        self.name
    }
}

impl<'a, TInitialise> GetEnder<TInitialise> for InitialisationComponent<'a, TInitialise> {
    fn get_ender(&self) -> &TInitialise {
        &self.initialise
    }
}

pub struct LoopComponent<'a, TRunLoop> {
    name: &'a str,
    run_loop: TRunLoop
}

impl<'a, TRunLoop> LoopComponent<'a, TRunLoop> {
    fn new(
        name: &'a str,
        run_loop: TRunLoop) -> Self{Self{
            name,
            run_loop}}
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

pub struct EndComponent<'a, TEnd> {
    name: &'a str,
    end: TEnd
}

impl<'a, TEnd> EndComponent<'a, TEnd> {
    fn new(
        name: &'a str,
        end: TEnd) -> Self{Self{
            name,
            end}}
}

impl<'a, TEnd> GetName for EndComponent<'a, TEnd> {
    fn get_name(&self) -> &'a str {
        self.name
    }
}

impl<'a, TEnd> GetEnder<TEnd> for EndComponent<'a, TEnd> {
    fn get_ender(&self) -> &TEnd {
        &self.end
    }
}

pub struct FullComponent<'a, TInitialise, TRunLoop, TEnd> {
    name: &'a str,
    initialise: TInitialise,
    run_loop: TRunLoop,
    end: TEnd
}

impl<'a, TInitialise, TRunLoop, TEnd> FullComponent<'a, TInitialise, TRunLoop, TEnd> {
    fn new(
        name: &'a str,
        initialise: TInitialise,
        run_loop: TRunLoop,
        end: TEnd) -> Self{Self{
            name,
            initialise,
            run_loop,
            end}}
}

impl<'a, TInitialise, TRunLoop, TEnd> GetName for FullComponent<'a, TInitialise, TRunLoop, TEnd> {
    fn get_name(&self) -> &'a str {
        self.name
    }
}

impl<'a, TInitialise, TRunLoop, TEnd> GetInitialiser<TInitialise> for FullComponent<'a, TInitialise, TRunLoop, TEnd> {
    fn get_initialiser(&self) -> &TInitialise {
        &self.initialise
    }
}

impl<'a, TInitialise, TRunLoop, TEnd> GetLoopRunner<TRunLoop> for FullComponent<'a, TInitialise, TRunLoop, TEnd> {
    fn get_loop_runner(&self) -> &TRunLoop {
        &self.run_loop
    }
}

impl<'a, TInitialise, TRunLoop, TEnd> GetEnder<TEnd> for FullComponent<'a, TInitialise, TRunLoop, TEnd> {
    fn get_ender(&self) -> &TEnd {
        &self.end
    }
}

pub struct InitialisationEndComponent<'a, TInitialise, TEnd> {
    name: &'a str,
    initialise: TInitialise,
    end: TEnd
}

impl<'a, TInitialise, TEnd> InitialisationEndComponent<'a, TInitialise, TEnd> {
    fn new(
        name: &'a str,
        initialise: TInitialise,
        end: TEnd) -> Self{Self{
            name,
            initialise,
            end}}
}

impl<'a, TInitialise, TEnd> GetName for InitialisationEndComponent<'a, TInitialise, TEnd> {
    fn get_name(&self) -> &'a str {
        self.name
    }
}

impl<'a, TInitialise, TEnd> GetInitialiser<TInitialise> for InitialisationEndComponent<'a, TInitialise, TEnd> {
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
    end: TEnd
}

impl<'a, TRunLoop, TEnd> LoopEndComponent<'a, TRunLoop, TEnd> {
    fn new(
        name: &'a str,
        run_loop: TRunLoop,
        end: TEnd) -> Self{Self{
            name,
            run_loop,
            end}}
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

pub trait GetLoopRunningStatus {
    fn get_loop_running_status(&self) -> bool;
}

pub trait CanRun {
    fn can_run(&self) -> bool;
}

pub struct GameLoopChecker<TGetLoopRunningStatus> {
    get_loop_running_status: TGetLoopRunningStatus
}

impl<TGetLoopRunningStatus> GameLoopChecker<TGetLoopRunningStatus> {
    fn new(
        get_loop_running_status: TGetLoopRunningStatus) -> Self{Self{
            get_loop_running_status}}
}

impl<TGetLoopRunningStatus: GetLoopRunningStatus> CanRun for GameLoopChecker<TGetLoopRunningStatus> {
    fn can_run(&self) -> bool {
        self.get_loop_running_status.get_loop_running_status()
    }
}

#[cfg(test)]
mod tests {
    use crate::{Component, GetName};

    #[test]
    fn when_component_is_constructed_then_it_has_the_correct_name() {
        let component = Component::new("Test Component");

        let result = component.get_name();

        assert_eq!("Test Component", result);
    }
}
