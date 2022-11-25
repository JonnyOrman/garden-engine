use garden::{
    Component,
    End,
    GetName,
    Create,
    Check
};

pub trait AddEndComponent {
    fn add_end_component(&self, end_component: &dyn End);
}

pub struct Ender<TEndEngine> {
    end_components: Vec<Box<dyn End>>,
    end_engine: TEndEngine
}

impl<TEndEngine> Ender<TEndEngine> {
    fn new(
        end_components: Vec<Box<dyn End>>,
        end_engine: TEndEngine) -> Self{Self{
            end_components,
            end_engine}}
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
    fn add_end_component(&self, end_component: &dyn End) {
    }
}

pub trait Run {
    fn run(&self);
}

pub trait AddRun {
    fn add_run(&self, run: &dyn Run);
}

pub struct GameInstance<
    'a,
    TEngine
    > {
    name: &'a str,
    engine: TEngine,
    components: Vec<Component<'a>>
}

impl<
    'a,
    TEngine
    > GameInstance<'a,
        TEngine
        > {
    pub fn new(
        name: &'a str,
        engine: TEngine,
        components: Vec<Component<'a>>)
        -> Self{Self{
            name,
            engine,
            components}}
}

impl<
    'a,
    TEngine: Run
    > Run for GameInstance<'a,
    TEngine
    > {
    fn run(&self) {
        self.engine.run()
    }
}

#[derive(Clone, Copy)]
pub struct Engine<
    TStart: Run + AddRun,
    TLoop: Run + AddRun,
    TEnd: Run + AddRun,
    TGetName: GetName> {
    start_system: TStart,
    loop_system: TLoop,
    end_system: TEnd,
    game_name_provider: TGetName
}

impl<
TStart: Run + AddRun,
TLoop: Run + AddRun,
TEnd: Run + AddRun,
TGetName: GetName> Engine<
    TStart,
    TLoop,
    TEnd,
    TGetName> {
    fn new(
        start_system: TStart,
        loop_system: TLoop,
        end_system: TEnd,
        game_name_provider: TGetName) -> Self{Self{
            start_system,
            loop_system,
            end_system,
            game_name_provider
        }}
}

impl<
TStart: Run + AddRun,
TLoop: Run + AddRun,
TEnd: Run + AddRun,
TGetName: GetName> Run for Engine<
    TStart,
    TLoop,
    TEnd,
    TGetName> {
    fn run(&self) {
        self.start_system.run();

        self.loop_system.run();

        self.end_system.run();
    }
}

pub struct GameInstanceBuilder<
    'a,
    TEngine: Copy
    > {
    name: &'a str,
    engine: TEngine,
    components: Vec<Component<'a>>
}

impl<
    'a,
    TEngine: Copy
    > GameInstanceBuilder<
        'a,
        TEngine
        > {
    fn new(
        name: &'a str,
        engine: TEngine,
        components: Vec<Component<'a>>) -> Self{Self{
            name,
            engine,
            components}}
    
    pub fn build(&self) -> GameInstance::<
    'a,
    TEngine
    > {
        GameInstance::<'a, TEngine>::new(
            self.name,
            self.engine,
            self.components.clone()
        )
    }
}

pub fn create_game_instance_builder<
    'a,
    TEngineStarter: Run + Copy + Clone,
    TEngineStarterCreator: Create<TEngineStarter>,
    TLoopChecker: Check + Copy,
    TLoopCheckerCreator: Create<TLoopChecker>,
    TEngineEnder: Run + Copy + Clone,
    TEngineEnderCreator: Create<TEngineEnder>>(
        name: &'a str,
        engine_starter_creator: TEngineStarterCreator,
        loop_checker_creator: TLoopCheckerCreator,
        engine_ender_creator: TEngineEnderCreator) -> GameInstanceBuilder<
            'a,
            Engine<StartSystem<TEngineStarter>, LoopSystem<TLoopChecker, GameLoopRunner>, EndSystem<TEngineEnder>, GameNameProvider<'a>>> {
            let engine = create_engine::<
                TEngineStarter,
                TEngineStarterCreator,
                TLoopChecker,
                TLoopCheckerCreator,
                TEngineEnder,
                TEngineEnderCreator>(
                    name,
                    engine_starter_creator,
                    loop_checker_creator,
                    engine_ender_creator
                );

            let game_instance_buillder = GameInstanceBuilder::<
                'a,
                Engine<StartSystem<TEngineStarter>, LoopSystem<TLoopChecker, GameLoopRunner>, EndSystem<TEngineEnder>, GameNameProvider<'a>>>::new(
                name,
                engine,
                Vec::new()
            );

            game_instance_buillder
        }

pub fn create_engine<
    'a,
    TEngineStarter: Run  + Clone,
    TEngineStarterCreator: Create<TEngineStarter>,
    TLoopChecker: Check + Clone,
    TLoopCheckerCreator: Create<TLoopChecker>,
    TEngineEnder: Run + Copy + Clone,
    TEngineEnderCreator: Create<TEngineEnder>>(
        name: &'a str,
        engine_starter_creator: TEngineStarterCreator,
        loop_checker_creator: TLoopCheckerCreator,
        engine_ender_creator: TEngineEnderCreator
        ) -> Engine<StartSystem<TEngineStarter>, LoopSystem<TLoopChecker, GameLoopRunner>, EndSystem<TEngineEnder>, GameNameProvider> {
        Engine::<StartSystem<TEngineStarter>, LoopSystem<TLoopChecker, GameLoopRunner>, EndSystem<TEngineEnder>, GameNameProvider>::new(
            create_start_system::<TEngineStarter, TEngineStarterCreator>(engine_starter_creator),
            create_loop_system(loop_checker_creator),
            create_end_system::<TEngineEnder, TEngineEnderCreator>(engine_ender_creator),
            create_game_name_provider(name)
        )
}

#[derive(Copy, Clone)]
pub struct StartSystem<TEngineStarter: Run> {
    engine_starter: TEngineStarter//,
    //component_initialisers: Vec<Box<dyn Run>>
}

impl<TEngineStarter: Run> StartSystem<TEngineStarter> {
    fn new(
        engine_starter: TEngineStarter
    ) -> Self{Self{
        engine_starter
    }}
}

pub fn create_start_system<
    'a,
    TEngineStarter: Run + Clone,
    TEngineStarterCreator: Create<TEngineStarter>>(
    engine_starter_creator: TEngineStarterCreator
) -> StartSystem<TEngineStarter> {
    let engine_starter = engine_starter_creator.create();

    StartSystem::<TEngineStarter>::new(
        engine_starter
    )
}

impl<TEngineStarter: Run> Run for StartSystem<TEngineStarter> {
    fn run(&self) {
        self.engine_starter.run();

        // for component_initialiser in self.component_initialisers.iter() {
        //     component_initialiser.run()
        // }
    }
}

impl<
    'a,
    TEngineStarter: Run
    > AddRun for StartSystem<TEngineStarter> {
    fn add_run(&self, run: &dyn Run) {
        //self.initialisation_components.push(Box::new(run))
    }
}

pub struct EngineStarter {}

impl EngineStarter {
    fn new() -> Self{Self{}}
}

impl Run for EngineStarter {
    fn run(&self) {}
}

#[derive(Copy, Clone)]
pub struct LoopSystem<
    TChecker: Check,
    TRunner: Run> {
    checker: TChecker,
    runner: TRunner
}

impl<
    TChecker: Check,
    TRunner: Run> LoopSystem<
        TChecker,
        TRunner> {
    fn new(
        checker: TChecker,
        runner: TRunner
    ) -> Self{Self{
        checker,
        runner
    }}
}

impl<
    TChecker: Check,
    TRunner: Run> Run for LoopSystem<
        TChecker,
        TRunner> {
    fn run(&self) {
        while self.checker.check() {
            self.runner.run()
        }
    }
}

impl<
    TChecker: Check,
    TRunner: Run> AddRun for LoopSystem <
        TChecker,
        TRunner> {
    fn add_run(&self, run: &dyn Run) {}
}

pub fn create_loop_system<
    TChecker: Check,
    TCheckerCreator: Create<TChecker>>(checker_creator: TCheckerCreator) -> LoopSystem<
    TChecker,
    GameLoopRunner> {
    LoopSystem::new(
        checker_creator.create(),
        create_loop_runner()
    )
}

#[derive(Copy, Clone)]
pub struct GameLoopRunner {}

impl GameLoopRunner {
    fn new() -> Self{Self{}}
}

impl Run for GameLoopRunner {
    fn run(&self) {}
}

fn create_loop_runner() -> GameLoopRunner {
    GameLoopRunner::new()
}

#[derive(Copy, Clone)]
pub struct EndSystem<TEngineEnder: Run> {
    engine_ender: TEngineEnder//,
    //component_initialisers: Vec<Box<dyn Run>>
}

impl<TEngineEnder: Run> EndSystem<TEngineEnder> {
    fn new(engine_ender: TEngineEnder) -> Self{Self{engine_ender}}
}

pub fn create_end_system<
    'a,
    TEngineEnder: Run + Clone,
    TEngineEnderCreator: Create<TEngineEnder>>(
    engine_ender_creator: TEngineEnderCreator
) -> EndSystem<TEngineEnder> {
    let engine_ender = engine_ender_creator.create();

    EndSystem::<TEngineEnder>::new(
        engine_ender
    )
}

impl<TEngineEnder: Run> Run for EndSystem<TEngineEnder> {
    fn run(&self) {
        // for component_ender in self.component_enders.iter() {
        //     component_ender.run()
        // }

        self.engine_ender.run();
    }
}

impl<
    'a,
    TEngineEnder: Run> AddRun for EndSystem<TEngineEnder> {
    fn add_run(&self, run: &dyn Run) {
        //self.component_enders.push(Box::new(run))
    }
}

#[derive(Copy, Clone)]
pub struct GameNameProvider<'a> {
    name: &'a str
}

impl<'a> GameNameProvider<'a> {
    fn new(name: &'a str) -> Self{Self{name}}
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
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
