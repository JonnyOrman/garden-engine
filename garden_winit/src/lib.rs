use garden::{AddRun, Component, Create, GetName};
use garden_games::{
    create_end_system, create_game_name_provider, create_start_system, End, EndEngine, EndSystem,
    GameNameProvider, Start, StartEngine, StartSystem,
};
use winit::event_loop::EventLoop;

pub struct GameInstance<'a, TEngine> {
    name: &'a str,
    engine: TEngine,
    components: Vec<Component<'a>>,
}

impl<'a, TEngine: RunEngine> GameInstance<'a, TEngine> {
    pub fn new(name: &'a str, engine: TEngine, components: Vec<Component<'a>>) -> Self {
        Self {
            name,
            engine,
            components,
        }
    }
}

impl<'a, TEngine: RunEngine> RunGameInstance for GameInstance<'a, TEngine> {
    fn run_game_instance(self, event_loop: EventLoop<()>) {
        self.engine.run_engine(event_loop)
    }
}

pub trait CreateLoopSystem<TLoopSystem, TContent> {
    fn create_loop_system(&self, event_loop: &EventLoop<()>, content: TContent) -> TLoopSystem;
}

pub trait RunEngine {
    fn run_engine(self, event_loop: EventLoop<()>);
}

pub trait RunLoop {
    fn run_loop(self, event_loop: EventLoop<()>);
}

pub trait RunGameInstance {
    fn run_game_instance(self, event_loop: EventLoop<()>);
}

pub trait RunLoopSystem {
    fn run_loop_system(self, event_loop: EventLoop<()>);
}

pub struct Engine<
    TStartSystem: Start,
    TLoopSystem: RunLoopSystem,
    TEndSystem: End,
    TGetName: GetName,
> {
    start_system: TStartSystem,
    loop_system: TLoopSystem,
    end_system: TEndSystem,
    game_name_provider: TGetName,
}

impl<
        TStartSystem: Start + AddRun,
        TLoopSystem: RunLoopSystem + AddRun,
        TEndSystem: End + AddRun,
        TGetName: GetName,
    > Engine<TStartSystem, TLoopSystem, TEndSystem, TGetName>
{
    fn new(
        start_system: TStartSystem,
        loop_system: TLoopSystem,
        end_system: TEndSystem,
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

impl<
        TStartSystem: Start + AddRun,
        TLoopSystem: RunLoopSystem + AddRun,
        TEndSystem: End + AddRun,
        TGetName: GetName,
    > RunEngine for Engine<TStartSystem, TLoopSystem, TEndSystem, TGetName>
{
    fn run_engine(self, event_loop: EventLoop<()>) {
        self.start_system.start();

        self.loop_system.run_loop_system(event_loop);

        self.end_system.end();
    }
}

pub trait BuildGameInstance<'a, TEngine> {
    fn build_game_instance(self) -> GameInstance<'a, TEngine>;
}

pub struct GameInstanceBuilder<'a, TEngine> {
    name: &'a str,
    engine: TEngine,
    components: Vec<Component<'a>>,
}

impl<'a, TEngine: RunEngine> GameInstanceBuilder<'a, TEngine> {
    fn new(name: &'a str, engine: TEngine, components: Vec<Component<'a>>) -> Self {
        Self {
            name,
            engine,
            components,
        }
    }
}

impl<'a, TEngine: RunEngine> BuildGameInstance<'a, TEngine> for GameInstanceBuilder<'a, TEngine> {
    fn build_game_instance(self) -> GameInstance<'a, TEngine> {
        GameInstance::<'a, TEngine>::new(self.name, self.engine, self.components)
    }
}

pub fn create_game_instance_builder<
    'a,
    TEngineStarter: StartEngine,
    TEngineStarterCreator: Create<TEngineStarter>,
    TLoopSystem: RunLoopSystem + AddRun,
    TLoopSystemCreator: CreateLoopSystem<TLoopSystem, TContent>,
    TEngineEnder: EndEngine,
    TEngineEnderCreator: Create<TEngineEnder>,
    TContent,
>(
    name: &'a str,
    engine_starter_creator: TEngineStarterCreator,
    loop_system_creator: TLoopSystemCreator,
    engine_ender_creator: TEngineEnderCreator,
    event_loop: &EventLoop<()>,
    content: TContent,
) -> GameInstanceBuilder<
    'a,
    Engine<StartSystem<TEngineStarter>, TLoopSystem, EndSystem<TEngineEnder>, GameNameProvider<'a>>,
> {
    let engine = create_engine::<
        TEngineStarter,
        TEngineStarterCreator,
        TLoopSystem,
        TLoopSystemCreator,
        TEngineEnder,
        TEngineEnderCreator,
        TContent,
    >(
        name,
        engine_starter_creator,
        loop_system_creator,
        engine_ender_creator,
        event_loop,
        content,
    );

    let game_instance_buillder = GameInstanceBuilder::<
        'a,
        Engine<
            StartSystem<TEngineStarter>,
            TLoopSystem,
            EndSystem<TEngineEnder>,
            GameNameProvider<'a>,
        >,
    >::new(name, engine, Vec::new());

    game_instance_buillder
}

pub fn create_engine<
    'a,
    TEngineStarter: StartEngine,
    TEngineStarterCreator: Create<TEngineStarter>,
    TLoopSystem: RunLoopSystem + AddRun,
    TLoopSystemCreator: CreateLoopSystem<TLoopSystem, TContent>,
    TEngineEnder: EndEngine,
    TEngineEnderCreator: Create<TEngineEnder>,
    TContent,
>(
    name: &'a str,
    engine_starter_creator: TEngineStarterCreator,
    loop_system_creator: TLoopSystemCreator,
    engine_ender_creator: TEngineEnderCreator,
    event_loop: &EventLoop<()>,
    content: TContent,
) -> Engine<StartSystem<TEngineStarter>, TLoopSystem, EndSystem<TEngineEnder>, GameNameProvider<'a>>
{
    Engine::<
        StartSystem<TEngineStarter>,
        TLoopSystem,
        EndSystem<TEngineEnder>,
        GameNameProvider,
    >::new(
        create_start_system::<TEngineStarter, TEngineStarterCreator>(engine_starter_creator),
        loop_system_creator.create_loop_system(event_loop, content),
        create_end_system::<TEngineEnder, TEngineEnderCreator>(engine_ender_creator),
        create_game_name_provider(name)
    )
}

#[cfg(test)]
mod tests {
    use mockall::mock;

    #[test]
    #[ignore]
    fn when_a_game_instance_runs_then_it_runs_the_engine() {
        // let mut engine = MockRun::new();

        // engine.expect_run().times(1).returning(|| ());

        // let game_instance =
        //     GameInstance::<MockRun>::new("Game Name", engine, Vec::<Component>::new());

        // game_instance.run();
    }

    #[test]
    #[ignore]
    fn when_an_engine_runs_then_it_runs_the_start_loop_and_end() {
        // mock! {
        //     EngineStage {}
        //     impl RunSelf for EngineStage {
        //         fn run(self);
        //     }
        //     impl AddRun for EngineStage {
        //         fn add_run(&self, run: &dyn Run);
        //     }
        // }

        // let mut start = MockEngineStage::new();

        // let mut loop_runner = MockEngineStage::new();

        // let mut end = MockEngineStage::new();

        // let get_name = MockGetName::new();

        // let mut sequence = Sequence::new();

        // start
        //     .expect_run()
        //     .times(1)
        //     .in_sequence(&mut sequence)
        //     .returning(|| ());

        // loop_runner
        //     .expect_run()
        //     .times(1)
        //     .in_sequence(&mut sequence)
        //     .returning(|| ());

        // end.expect_run()
        //     .times(1)
        //     .in_sequence(&mut sequence)
        //     .returning(|| ());

        // let game_instance =
        //     Engine::<MockEngineStage, MockEngineStage, MockEngineStage, MockGetName>::new(
        //         start,
        //         loop_runner,
        //         end,
        //         get_name,
        //     );

        // game_instance.run();
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
}
