use garden_games::{
    GameInstance,
    Run,
    AddRun,
    create_game_instance_builder, StartSystem, LoopSystem, EndSystem, GameNameProvider, Engine, GameLoopRunner
};

use garden::{
    GetName,
    Create,
    Check
};

pub fn start(game_name: &str) {
    let game_instance = compose::<StartSystem<EngineStarter>, LoopSystem<EngineLoopChecker, GameLoopRunner>, EndSystem<EngineEnder>, GameNameProvider>(game_name);

    game_instance.run();
}

fn compose<
    'a,
    TStart: Run + AddRun,
    TLoop: Run + AddRun,
    TEnd: Run + AddRun,
    TGetName: GetName>(name: &'a str) -> GameInstance<'a, Engine<StartSystem<EngineStarter>, LoopSystem<EngineLoopChecker, GameLoopRunner>, EndSystem<EngineEnder>, GameNameProvider>> {
    let game_instance_builder = create_game_instance_builder::<
        EngineStarter,
        EngineStarterCreator,
        EngineLoopChecker,
        EngineLoopCheckerCreator,
        EngineEnder,
        EngineEnderCreator>(
        name,
        EngineStarterCreator::new(),
        EngineLoopCheckerCreator::new(),
        EngineEnderCreator::new()
    );

    game_instance_builder.build()
}

#[derive(Copy, Clone)]
pub struct EngineStarter {}

impl EngineStarter {
    fn new() -> Self{Self{}}
}

impl Run for EngineStarter {
    fn run(&self) {

    }
}

pub struct EngineStarterCreator {}

impl EngineStarterCreator {
    fn new() -> Self{Self{}}
}

impl Create<EngineStarter> for EngineStarterCreator {
    fn create(&self) -> EngineStarter {
        EngineStarter::new()
    }
}

#[derive(Copy, Clone)]
pub struct EngineLoopChecker {}

impl EngineLoopChecker {
    fn new() -> Self{Self{}}
}

impl Check for EngineLoopChecker {
    fn check(&self) -> bool {
        true
    }
}

pub struct EngineLoopCheckerCreator {}

impl EngineLoopCheckerCreator {
    fn new() -> Self{Self{}}
}

impl Create<EngineLoopChecker> for EngineLoopCheckerCreator {
    fn create(&self) -> EngineLoopChecker {
        EngineLoopChecker::new()
    }
}

#[derive(Copy, Clone)]
pub struct EngineEnder {}

impl EngineEnder {
    fn new() -> Self{Self{}}
}

impl Run for EngineEnder {
    fn run(&self) {}
}

pub struct EngineEnderCreator {}

impl EngineEnderCreator {
    fn new() -> Self{Self{}}
}

impl Create<EngineEnder> for EngineEnderCreator {
    fn create(&self) -> EngineEnder {
        EngineEnder::new()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
