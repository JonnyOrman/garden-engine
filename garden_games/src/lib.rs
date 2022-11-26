use std::rc::Rc;

use garden::{Check, Component, Create, End, GetName};

use mockall::*;

pub trait AddEndComponent {
    fn add_end_component(&self, end_component: &dyn End);
}

pub struct Ender<TEndEngine> {
    end_components: Vec<Box<dyn End>>,
    end_engine: TEndEngine,
}

impl<TEndEngine> Ender<TEndEngine> {
    fn new(end_components: Vec<Box<dyn End>>, end_engine: TEndEngine) -> Self {
        Self {
            end_components,
            end_engine,
        }
    }
}

impl<TEndEngine: End> End for Ender<TEndEngine> {
    fn end(&self) {
        for end_component in self.end_components.iter() {
            end_component.end()
        }

        self.end_engine.end()
    }
}

impl<TEndEngine: End> AddEndComponent for Ender<TEndEngine> {
    fn add_end_component(&self, end_component: &dyn End) {}
}

#[automock]
pub trait Run {
    fn run(&self);
}

pub trait AddRun {
    fn add_run(&self, run: &dyn Run);
}

pub struct GameInstance<'a, TEngine> {
    name: &'a str,
    engine: TEngine,
    components: Vec<Component<'a>>,
}

impl<'a, TEngine> GameInstance<'a, TEngine> {
    pub fn new(name: &'a str, engine: TEngine, components: Vec<Component<'a>>) -> Self {
        Self {
            name,
            engine,
            components,
        }
    }
}

impl<'a, TEngine: Run> Run for GameInstance<'a, TEngine> {
    fn run(&self) {
        self.engine.run()
    }
}

#[derive(Clone, Copy)]
pub struct Engine<TStart: Run, TLoop: Run, TEnd: Run, TGetName: GetName> {
    start_system: TStart,
    loop_system: TLoop,
    end_system: TEnd,
    game_name_provider: TGetName,
}

impl<TStart: Run + AddRun, TLoop: Run + AddRun, TEnd: Run + AddRun, TGetName: GetName>
    Engine<TStart, TLoop, TEnd, TGetName>
{
    fn new(
        start_system: TStart,
        loop_system: TLoop,
        end_system: TEnd,
        game_name_provider: TGetName,
    ) -> Self {
        Self {
            start_system,
            loop_system,
            end_system,
            game_name_provider,
        }
    }
}

impl<TStart: Run + AddRun, TLoop: Run + AddRun, TEnd: Run + AddRun, TGetName: GetName> Run
    for Engine<TStart, TLoop, TEnd, TGetName>
{
    fn run(&self) {
        self.start_system.run();

        self.loop_system.run();

        self.end_system.run();
    }
}

pub struct GameInstanceBuilder<'a, TEngine: Clone> {
    name: &'a str,
    engine: TEngine,
    components: Vec<Component<'a>>,
}

impl<'a, TEngine: Clone> GameInstanceBuilder<'a, TEngine> {
    fn new(name: &'a str, engine: TEngine, components: Vec<Component<'a>>) -> Self {
        Self {
            name,
            engine,
            components,
        }
    }

    pub fn build(&self) -> GameInstance<'a, TEngine> {
        GameInstance::<'a, TEngine>::new(self.name, self.engine.clone(), self.components.clone())
    }
}

pub fn create_game_instance_builder<
    'a,
    TEngineStarter: Run + Copy + Clone,
    TEngineStarterCreator: Create<TEngineStarter>,
    TLoopChecker: Check + Copy,
    TLoopCheckerCreator: Create<TLoopChecker>,
    TEngineEnder: Run + Copy + Clone,
    TEngineEnderCreator: Create<TEngineEnder>,
>(
    name: &'a str,
    engine_starter_creator: TEngineStarterCreator,
    loop_checker_creator: TLoopCheckerCreator,
    engine_ender_creator: TEngineEnderCreator,
) -> GameInstanceBuilder<
    'a,
    Engine<
        StartSystem<TEngineStarter>,
        LoopSystem<TLoopChecker, GameLoopRunner>,
        EndSystem<TEngineEnder>,
        GameNameProvider<'a>,
    >,
> {
    let engine = create_engine::<
        TEngineStarter,
        TEngineStarterCreator,
        TLoopChecker,
        TLoopCheckerCreator,
        TEngineEnder,
        TEngineEnderCreator,
    >(
        name,
        engine_starter_creator,
        loop_checker_creator,
        engine_ender_creator,
    );

    let game_instance_buillder = GameInstanceBuilder::<
        'a,
        Engine<
            StartSystem<TEngineStarter>,
            LoopSystem<TLoopChecker, GameLoopRunner>,
            EndSystem<TEngineEnder>,
            GameNameProvider<'a>,
        >,
    >::new(name, engine, Vec::new());

    game_instance_buillder
}

pub fn create_engine<
    'a,
    TEngineStarter: Run + Clone,
    TEngineStarterCreator: Create<TEngineStarter>,
    TLoopChecker: Check + Clone,
    TLoopCheckerCreator: Create<TLoopChecker>,
    TEngineEnder: Run + Copy + Clone,
    TEngineEnderCreator: Create<TEngineEnder>,
>(
    name: &'a str,
    engine_starter_creator: TEngineStarterCreator,
    loop_checker_creator: TLoopCheckerCreator,
    engine_ender_creator: TEngineEnderCreator,
) -> Engine<
    StartSystem<TEngineStarter>,
    LoopSystem<TLoopChecker, GameLoopRunner>,
    EndSystem<TEngineEnder>,
    GameNameProvider,
> {
    Engine::<
        StartSystem<TEngineStarter>,
        LoopSystem<TLoopChecker, GameLoopRunner>,
        EndSystem<TEngineEnder>,
        GameNameProvider,
    >::new(
        create_start_system::<TEngineStarter, TEngineStarterCreator>(engine_starter_creator),
        create_loop_system(loop_checker_creator),
        create_end_system::<TEngineEnder, TEngineEnderCreator>(engine_ender_creator),
        create_game_name_provider(name),
    )
}

#[derive(Clone)]
pub struct StartSystem<TEngineStarter: Run> {
    engine_starter: TEngineStarter,
    component_initialisers: Vec<Rc<dyn Run>>,
}

impl<TEngineStarter: Run> StartSystem<TEngineStarter> {
    fn new(engine_starter: TEngineStarter, component_initialisers: Vec<Rc<dyn Run>>) -> Self {
        Self {
            engine_starter,
            component_initialisers,
        }
    }
}

pub fn create_start_system<
    'a,
    TEngineStarter: Run + Clone,
    TEngineStarterCreator: Create<TEngineStarter>,
>(
    engine_starter_creator: TEngineStarterCreator,
) -> StartSystem<TEngineStarter> {
    let engine_starter = engine_starter_creator.create();

    let component_initialisers = Vec::<Rc<dyn Run>>::new();

    StartSystem::<TEngineStarter>::new(engine_starter, component_initialisers)
}

impl<TEngineStarter: Run> Run for StartSystem<TEngineStarter> {
    fn run(&self) {
        self.engine_starter.run();

        for component_initialiser in self.component_initialisers.iter() {
            component_initialiser.run()
        }
    }
}

impl<'a, TEngineStarter: Run> AddRun for StartSystem<TEngineStarter> {
    fn add_run(&self, run: &dyn Run) {
        //self.component_initialisers.push(Rc::new(run))
    }
}

#[derive(Copy, Clone)]
pub struct LoopSystem<TChecker: Check, TRunner: Run> {
    checker: TChecker,
    runner: TRunner,
}

impl<TChecker: Check, TRunner: Run> LoopSystem<TChecker, TRunner> {
    fn new(checker: TChecker, runner: TRunner) -> Self {
        Self { checker, runner }
    }
}

impl<TChecker: Check, TRunner: Run> Run for LoopSystem<TChecker, TRunner> {
    fn run(&self) {
        while self.checker.check() {
            self.runner.run()
        }
    }
}

impl<TChecker: Check, TRunner: Run> AddRun for LoopSystem<TChecker, TRunner> {
    fn add_run(&self, run: &dyn Run) {}
}

pub fn create_loop_system<TChecker: Check, TCheckerCreator: Create<TChecker>>(
    checker_creator: TCheckerCreator,
) -> LoopSystem<TChecker, GameLoopRunner> {
    LoopSystem::new(checker_creator.create(), create_loop_runner())
}

#[derive(Copy, Clone)]
pub struct GameLoopRunner {}

impl GameLoopRunner {
    fn new() -> Self {
        Self {}
    }
}

impl Run for GameLoopRunner {
    fn run(&self) {}
}

fn create_loop_runner() -> GameLoopRunner {
    GameLoopRunner::new()
}

#[derive(Clone)]
pub struct EndSystem<TEngineEnder: Run> {
    engine_ender: TEngineEnder,
    component_enders: Vec<Rc<dyn Run>>,
}

impl<TEngineEnder: Run> EndSystem<TEngineEnder> {
    fn new(engine_ender: TEngineEnder, component_enders: Vec<Rc<dyn Run>>) -> Self {
        Self {
            engine_ender,
            component_enders,
        }
    }
}

impl<TEngineEnder: Run> Run for EndSystem<TEngineEnder> {
    fn run(&self) {
        for component_ender in self.component_enders.iter() {
            component_ender.run()
        }

        self.engine_ender.run();
    }
}

impl<'a, TEngineEnder: Run> AddRun for EndSystem<TEngineEnder> {
    fn add_run(&self, run: &dyn Run) {
        //self.component_enders.push(Rc::new(run))
    }
}

pub fn create_end_system<
    'a,
    TEngineEnder: Run + Clone,
    TEngineEnderCreator: Create<TEngineEnder>,
>(
    engine_ender_creator: TEngineEnderCreator,
) -> EndSystem<TEngineEnder> {
    let engine_ender = engine_ender_creator.create();

    let component_enders = Vec::<Rc<dyn Run>>::new();

    EndSystem::<TEngineEnder>::new(engine_ender, component_enders)
}

#[derive(Copy, Clone)]
pub struct GameNameProvider<'a> {
    name: &'a str,
}

impl<'a> GameNameProvider<'a> {
    fn new(name: &'a str) -> Self {
        Self { name }
    }
}

impl<'a> GetName for GameNameProvider<'a> {
    fn get_name(&self) -> &str {
        self.name
    }
}

pub fn create_game_name_provider<'a>(name: &'a str) -> GameNameProvider<'a> {
    GameNameProvider::<'a>::new(name)
}

#[cfg(test)]
mod tests {
    use garden::{Component, End, MockCheck, MockEnd, MockGetName};

    use crate::{Ender, GameInstance, Run};

    use super::*;

    #[test]
    fn when_and_ender_ends_a_game_then_it_ends_each_component_and_the_engine() {
        let mut component_ender_1 = MockEnd::new();
        let mut component_ender_2 = MockEnd::new();
        let mut component_ender_3 = MockEnd::new();

        let mut sequence = Sequence::new();

        component_ender_1
            .expect_end()
            .times(1)
            .in_sequence(&mut sequence)
            .returning(|| ());

        component_ender_2
            .expect_end()
            .times(1)
            .in_sequence(&mut sequence)
            .returning(|| ());

        component_ender_3
            .expect_end()
            .times(1)
            .in_sequence(&mut sequence)
            .returning(|| ());

        let mut component_enders = Vec::<Box<dyn End>>::new();
        component_enders.push(Box::new(component_ender_1));
        component_enders.push(Box::new(component_ender_2));
        component_enders.push(Box::new(component_ender_3));

        let mut engine_ender = MockEnd::new();

        engine_ender.expect_end().times(1).returning(|| ());

        let ender = Ender::new(component_enders, engine_ender);

        ender.end();
    }

    #[test]
    fn when_a_game_instance_runs_then_it_runs_the_engine() {
        let mut engine = MockRun::new();

        engine.expect_run().times(1).returning(|| ());

        let game_instance =
            GameInstance::<MockRun>::new("Game Name", engine, Vec::<Component>::new());

        game_instance.run();
    }

    #[test]
    fn when_an_engine_runs_then_it_runs_the_start_loop_and_end() {
        mock! {
            EngineStage {}
            impl Run for EngineStage {
                fn run(&self);
            }
            impl AddRun for EngineStage {
                fn add_run(&self, run: &dyn Run);
            }
        }

        let mut start = MockEngineStage::new();

        let mut loop_runner = MockEngineStage::new();

        let mut end = MockEngineStage::new();

        let get_name = MockGetName::new();

        let mut sequence = Sequence::new();

        start
            .expect_run()
            .times(1)
            .in_sequence(&mut sequence)
            .returning(|| ());

        loop_runner
            .expect_run()
            .times(1)
            .in_sequence(&mut sequence)
            .returning(|| ());

        end.expect_run()
            .times(1)
            .in_sequence(&mut sequence)
            .returning(|| ());

        let game_instance =
            Engine::<MockEngineStage, MockEngineStage, MockEngineStage, MockGetName>::new(
                start,
                loop_runner,
                end,
                get_name,
            );

        game_instance.run();
    }

    #[test]
    #[ignore]
    fn when_a_game_instance_builder_builds_a_game_instance_then_it_builds_the_game_instance_with_the_name_engine_and_components(
    ) {
        // let name = "Test Game";

        // mock! {
        //     MyStruct {}
        //     impl Clone for MyStruct {
        //         fn clone(&self) -> Self;
        //     }
        // }

        // let mut engine = MockMyStruct::new();

        // let components = Vec::<Component>::new();

        // let game_instance_builder = GameInstanceBuilder::new(name, engine, components);

        // let game_instance = game_instance_builder.build();
    }

    #[test]
    #[ignore]
    fn when_a_start_system_runs_then_the_engine_starter_runs() {
        let mut engine_starter = MockRun::new();

        engine_starter.expect_run().times(1).returning(|| ());

        let component_initialisers = Vec::<Rc<dyn Run>>::new();

        let start_system = StartSystem::new(engine_starter, component_initialisers);

        start_system.run();
    }

    #[test]
    fn when_a_loop_system_runs_then_the_loop_runs_while_it_can() {
        let mut checker = MockCheck::new();

        let mut sequence = Sequence::new();

        checker
            .expect_check()
            .times(2)
            .in_sequence(&mut sequence)
            .returning(|| true);

        checker
            .expect_check()
            .times(1)
            .in_sequence(&mut sequence)
            .returning(|| false);

        let mut runner = MockRun::new();

        runner.expect_run().times(2).return_const(());

        let loop_system = LoopSystem::new(checker, runner);

        loop_system.run();
    }

    #[test]
    #[ignore]
    fn when_a_game_loop_runner_runs_then_the_loop_runs() {
        let game_loop_runner = GameLoopRunner::new();

        game_loop_runner.run();
    }

    #[test]
    #[ignore]
    fn when_an_end_system_runs_then_engine_ender_runs() {
        let mut engine_ender = MockRun::new();

        engine_ender.expect_run().times(1).returning(|| ());

        let component_enders = Vec::<Rc<dyn Run>>::new();

        let end_system = EndSystem::new(engine_ender, component_enders);

        end_system.run();
    }

    #[test]
    fn when_a_game_name_provider_gets_the_name_then_it_provides_the_name() {
        let name = "Test Game";

        let game_name_provider = GameNameProvider::new(name);

        let result = game_name_provider.get_name();

        assert_eq!(name, result);
    }
}
