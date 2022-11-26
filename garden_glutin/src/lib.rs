use garden_games::{
    create_game_instance_builder, AddRun, EndSystem, Engine, GameInstance, GameLoopRunner,
    GameNameProvider, LoopSystem, Run, StartSystem,
};

use garden::{Check, Create, GetName};

pub fn start(game_name: &str) {
    let game_instance = compose::<
        StartSystem<EngineStarter>,
        LoopSystem<EngineLoopChecker, GameLoopRunner>,
        EndSystem<EngineEnder>,
        GameNameProvider,
    >(game_name);

    game_instance.run();
}

fn compose<'a, TStart: Run + AddRun, TLoop: Run + AddRun, TEnd: Run + AddRun, TGetName: GetName>(
    name: &'a str,
) -> GameInstance<
    'a,
    Engine<
        StartSystem<EngineStarter>,
        LoopSystem<EngineLoopChecker, GameLoopRunner>,
        EndSystem<EngineEnder>,
        GameNameProvider,
    >,
> {
    let game_instance_builder = create_game_instance_builder::<
        EngineStarter,
        EngineStarterCreator,
        EngineLoopChecker,
        EngineLoopCheckerCreator,
        EngineEnder,
        EngineEnderCreator,
    >(
        name,
        EngineStarterCreator::new(),
        EngineLoopCheckerCreator::new(),
        EngineEnderCreator::new(),
    );

    game_instance_builder.build()
}

#[derive(Copy, Clone)]
pub struct EngineStarter {}

impl EngineStarter {
    fn new() -> Self {
        Self {}
    }
}

impl Run for EngineStarter {
    fn run(&self) {}
}

pub struct EngineStarterCreator {}

impl EngineStarterCreator {
    fn new() -> Self {
        Self {}
    }
}

impl Create<EngineStarter> for EngineStarterCreator {
    fn create(&self) -> EngineStarter {
        EngineStarter::new()
    }
}

#[derive(Copy, Clone)]
pub struct EngineLoopChecker {}

impl EngineLoopChecker {
    fn new() -> Self {
        Self {}
    }
}

impl Check for EngineLoopChecker {
    fn check(&self) -> bool {
        true
    }
}

pub struct EngineLoopCheckerCreator {}

impl EngineLoopCheckerCreator {
    fn new() -> Self {
        Self {}
    }
}

impl Create<EngineLoopChecker> for EngineLoopCheckerCreator {
    fn create(&self) -> EngineLoopChecker {
        EngineLoopChecker::new()
    }
}

#[derive(Copy, Clone)]
pub struct EngineEnder {}

impl EngineEnder {
    fn new() -> Self {
        Self {}
    }
}

impl Run for EngineEnder {
    fn run(&self) {}
}

pub struct EngineEnderCreator {}

impl EngineEnderCreator {
    fn new() -> Self {
        Self {}
    }
}

impl Create<EngineEnder> for EngineEnderCreator {
    fn create(&self) -> EngineEnder {
        EngineEnder::new()
    }
}

#[cfg(test)]
mod tests {
    use garden::{Check, Create};
    use garden_games::Run;

    use crate::{
        EngineEnder, EngineLoopChecker, EngineLoopCheckerCreator, EngineStarter,
        EngineStarterCreator,
    };

    #[test]
    #[ignore]
    fn when_an_engine_starter_runs_then_the_engine_starts() {
        let engine_starter = EngineStarter::new();

        engine_starter.run();
    }

    #[test]
    #[ignore]
    fn when_an_engine_starter_creator_creates_an_engine_starter_then_an_engine_starter_is_created()
    {
        let engine_starter_creator = EngineStarterCreator::new();

        let engine_starter = engine_starter_creator.create();
    }

    #[test]
    #[ignore]
    fn when_an_engine_loop_checker_checks_if_the_loop_can_run_then_it_checks_if_it_can_run() {
        let engine_loop_checker = EngineLoopChecker::new();

        let result = engine_loop_checker.check();

        assert!(result);
    }

    #[test]
    #[ignore]
    fn when_an_engine_loop_checker_creates_an_engine_loop_checker_then_an_engine_loop_checker_is_created(
    ) {
        let engine_loop_checker_creator = EngineLoopCheckerCreator::new();

        let engine_loop_checker = engine_loop_checker_creator.create();
    }

    #[test]
    #[ignore]
    fn when_an_engine_ender_runs_then_the_engine_ends() {
        let engine_ender = EngineEnder::new();

        engine_ender.run();
    }

    #[test]
    #[ignore]
    fn when_an_engine_ender_creator_creates_an_engine_ender_then_an_engine_ender_is_created() {
        let engine_ender_creator = EngineStarterCreator::new();

        let engine_ender = engine_ender_creator.create();
    }
}
