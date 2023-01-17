use garden::{Create, GetName, RunFullComponent};
use garden_games::{
    create_end_system, create_game_name_provider, create_start_system, End, EndEngine, EndSystem,
    GameNameProvider, Start, StartEngine, StartSystem,
};
use winit::event_loop::EventLoop;

pub struct GameInstance<'a, TEngine> {
    name: &'a str,
    engine: TEngine,
}

impl<'a, TEngine: RunEngine> GameInstance<'a, TEngine> {
    pub fn new(name: &'a str, engine: TEngine) -> Self {
        Self { name, engine }
    }
}

impl<'a, TEngine: RunEngine> RunGameInstance for GameInstance<'a, TEngine> {
    fn run_game_instance(self, event_loop: EventLoop<()>) {
        self.engine.run_engine(event_loop)
    }
}

pub trait CreateLoopSystem<TLoopSystem> {
    fn create_loop_system(&self, event_loop: &EventLoop<()>) -> TLoopSystem;
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
    fn run_loop_system(self, event_loop: EventLoop<()>, components: Vec<Box<dyn RunFullComponent>>);
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
    components: Vec<Box<dyn RunFullComponent>>,
}

impl<TStartSystem: Start, TLoopSystem: RunLoopSystem, TEndSystem: End, TGetName: GetName>
    Engine<TStartSystem, TLoopSystem, TEndSystem, TGetName>
{
    fn new(
        start_system: TStartSystem,
        loop_system: TLoopSystem,
        end_system: TEndSystem,
        game_name_provider: TGetName,
        components: Vec<Box<dyn RunFullComponent>>,
    ) -> Self {
        Self {
            start_system,
            loop_system,
            end_system,
            game_name_provider,
            components,
        }
    }
}

impl<TStartSystem: Start, TLoopSystem: RunLoopSystem, TEndSystem: End, TGetName: GetName> RunEngine
    for Engine<TStartSystem, TLoopSystem, TEndSystem, TGetName>
{
    fn run_engine(self, event_loop: EventLoop<()>) {
        self.start_system.start();

        for component in self.components.iter() {
            component.initialise();
        }

        self.loop_system
            .run_loop_system(event_loop, self.components);

        self.end_system.end();
    }
}

pub trait BuildGameInstance<'a, TEngine> {
    fn build_game_instance(self, event_loop: &EventLoop<()>) -> GameInstance<'a, TEngine>;
}

pub trait AddComponent {
    fn add<T: RunFullComponent + 'static>(&mut self, t: T);
}

pub struct GameInstanceBuilder<'a, TEngineStarterCreator, TLoopSystemCreator, TEngineEnderCreator> {
    name: &'a str,
    full_components: Vec<Box<dyn RunFullComponent>>,
    engine_starter_creator: TEngineStarterCreator,
    loop_system_creator: TLoopSystemCreator,
    engine_ender_creator: TEngineEnderCreator,
}

impl<'a, TEngineStarterCreator, TLoopSystemCreator, TEngineEnderCreator>
    GameInstanceBuilder<'a, TEngineStarterCreator, TLoopSystemCreator, TEngineEnderCreator>
{
    fn new(
        name: &'a str,
        full_components: Vec<Box<dyn RunFullComponent>>,
        engine_starter_creator: TEngineStarterCreator,
        loop_system_creator: TLoopSystemCreator,
        engine_ender_creator: TEngineEnderCreator,
    ) -> Self {
        Self {
            name,
            full_components,
            engine_starter_creator,
            loop_system_creator,
            engine_ender_creator,
        }
    }
}

impl<'a, TEngineStarterCreator, TLoopSystemCreator, TEngineEnderCreator> AddComponent
    for GameInstanceBuilder<'a, TEngineStarterCreator, TLoopSystemCreator, TEngineEnderCreator>
{
    fn add<TComponent: RunFullComponent + 'static>(&mut self, component: TComponent) {
        let boxed_component = Box::new(component);

        self.full_components.push(boxed_component);
    }
}

impl<
        'a,
        TEngineStarter: StartEngine,
        TEngineStarterCreator: Create<TEngineStarter>,
        TLoopSystem: RunLoopSystem,
        TLoopSystemCreator: CreateLoopSystem<TLoopSystem>,
        TEngineEnder: EndEngine,
        TEngineEnderCreator: Create<TEngineEnder>,
    >
    BuildGameInstance<
        'a,
        Engine<
            StartSystem<TEngineStarter>,
            TLoopSystem,
            EndSystem<TEngineEnder>,
            GameNameProvider<'a>,
        >,
    > for GameInstanceBuilder<'a, TEngineStarterCreator, TLoopSystemCreator, TEngineEnderCreator>
{
    fn build_game_instance(
        self,
        event_loop: &EventLoop<()>,
    ) -> GameInstance<
        'a,
        Engine<
            StartSystem<TEngineStarter>,
            TLoopSystem,
            EndSystem<TEngineEnder>,
            GameNameProvider<'a>,
        >,
    > {
        let engine = create_engine::<
            TEngineStarter,
            TEngineStarterCreator,
            TLoopSystem,
            TLoopSystemCreator,
            TEngineEnder,
            TEngineEnderCreator,
        >(
            self.name,
            self.engine_starter_creator,
            self.loop_system_creator,
            self.engine_ender_creator,
            event_loop,
            self.full_components,
        );

        GameInstance::<
            'a,
            Engine<
                StartSystem<TEngineStarter>,
                TLoopSystem,
                EndSystem<TEngineEnder>,
                GameNameProvider<'a>,
            >,
        >::new(self.name, engine)
    }
}

pub fn create_game_instance_builder<
    'a,
    TEngineStarter: StartEngine,
    TEngineStarterCreator: Create<TEngineStarter>,
    TLoopSystem: RunLoopSystem,
    TLoopSystemCreator: CreateLoopSystem<TLoopSystem>,
    TEngineEnder: EndEngine,
    TEngineEnderCreator: Create<TEngineEnder>,
>(
    name: &'a str,
    engine_starter_creator: TEngineStarterCreator,
    loop_system_creator: TLoopSystemCreator,
    engine_ender_creator: TEngineEnderCreator,
    event_loop: &EventLoop<()>,
) -> GameInstanceBuilder<'a, TEngineStarterCreator, TLoopSystemCreator, TEngineEnderCreator> {
    let game_instance_buillder = GameInstanceBuilder::<
        'a,
        TEngineStarterCreator,
        TLoopSystemCreator,
        TEngineEnderCreator,
    >::new(
        name,
        Vec::new(),
        engine_starter_creator,
        loop_system_creator,
        engine_ender_creator,
    );

    game_instance_buillder
}

pub fn create_engine<
    'a,
    TEngineStarter: StartEngine,
    TEngineStarterCreator: Create<TEngineStarter>,
    TLoopSystem: RunLoopSystem,
    TLoopSystemCreator: CreateLoopSystem<TLoopSystem>,
    TEngineEnder: EndEngine,
    TEngineEnderCreator: Create<TEngineEnder>,
>(
    name: &'a str,
    engine_starter_creator: TEngineStarterCreator,
    loop_system_creator: TLoopSystemCreator,
    engine_ender_creator: TEngineEnderCreator,
    event_loop: &EventLoop<()>,
    components: Vec<Box<dyn RunFullComponent>>,
) -> Engine<StartSystem<TEngineStarter>, TLoopSystem, EndSystem<TEngineEnder>, GameNameProvider<'a>>
{
    Engine::<
        StartSystem<TEngineStarter>,
        TLoopSystem,
        EndSystem<TEngineEnder>,
        GameNameProvider
    >::new(
        create_start_system::<TEngineStarter, TEngineStarterCreator>(engine_starter_creator),
        loop_system_creator.create_loop_system(event_loop),
        create_end_system::<TEngineEnder, TEngineEnderCreator>(engine_ender_creator),
        create_game_name_provider(name),
        components
    )
}
