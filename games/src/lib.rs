use garden::{AddComponent, Create, GetName, Run, RunFullComponent};
use garden_glutin::{
    ContextAttributesCreator, DisplayCreator, EventRunner, FallbackContextAttributesCreator,
    GlWindowCreator, LoopSystem, LoopSystemCreator, NotCurrentGlContextCreator,
    RedrawEventsClearedEvent, RedrawEventsClearedEventCreator, Renderer, ResumedEvent,
    ResumedEventCreator, StateCreator, WindowCloseRequestedEvent, WindowCloseRequestedEventCreator,
    WindowResizedEvent, WindowResizedEventCreator,
};
use garden_winit::{CreateLoopSystem, GameInstance, RunEngine, RunGameInstance, RunLoopSystem};

use winit::event_loop::{EventLoop, EventLoopBuilder, EventLoopWindowTarget};

pub struct EndSystem<TEngineEnder> {
    component_enders: Vec<Box<dyn Run>>,
    engine_ender: TEngineEnder,
}

impl<TEngineEnder: EndEngine> EndSystem<TEngineEnder> {
    fn new(component_enders: Vec<Box<dyn Run>>, engine_ender: TEngineEnder) -> Self {
        Self {
            component_enders,
            engine_ender,
        }
    }
}

impl<TEngineEnder: EndEngine> End for EndSystem<TEngineEnder> {
    fn end(self) {
        for end_component in self.component_enders.iter() {
            end_component.run()
        }

        self.engine_ender.end_engine()
    }
}

pub trait StartEngine {
    fn start_engine(self);
}

pub trait Start {
    fn start(self);
}

pub trait EndEngine {
    fn end_engine(self);
}

pub trait End {
    fn end(self);
}

pub struct StartSystem<TEngineStarter> {
    engine_starter: TEngineStarter,
}

impl<TEngineStarter: StartEngine> StartSystem<TEngineStarter> {
    fn new(engine_starter: TEngineStarter) -> Self {
        Self { engine_starter }
    }
}

impl<TEngineStarter: StartEngine> Start for StartSystem<TEngineStarter> {
    fn start(self) {
        self.engine_starter.start_engine();
    }
}

pub fn create_start_system<
    'a,
    TEngineStarter: StartEngine,
    TEngineStarterCreator: Create<TEngineStarter>,
>(
    engine_starter_creator: TEngineStarterCreator,
) -> StartSystem<TEngineStarter> {
    let engine_starter = engine_starter_creator.create();

    StartSystem::<TEngineStarter>::new(engine_starter)
}

pub fn create_end_system<'a, TEngineEnder: EndEngine, TEngineEnderCreator: Create<TEngineEnder>>(
    engine_ender_creator: TEngineEnderCreator,
) -> EndSystem<TEngineEnder> {
    let engine_ender = engine_ender_creator.create();

    let component_enders = Vec::<Box<dyn Run>>::new();

    EndSystem::<TEngineEnder>::new(component_enders, engine_ender)
}

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

pub fn run_game() {
    let event_loop = EventLoopBuilder::new().build();

    start_game(event_loop);

    // let game_instance_builder_and_event_loop =
    //     generate_game_instance_builder_and_event_loop("Garden: Glutin Example");

    // let game_instance = game_instance_builder_and_event_loop
    //     .0
    //     .build_game_instance(&game_instance_builder_and_event_loop.1);

    // game_instance.run_game_instance(game_instance_builder_and_event_loop.1)
}

fn start_game(event_loop: EventLoop<()>) {
    let game_instance_builder_and_event_loop =
        generate_game_instance_builder_and_event_loop("Garden: Glutin Example", &event_loop);

    let game_instance = game_instance_builder_and_event_loop
        //.0
        .build_game_instance(&event_loop);

    game_instance.run_game_instance(event_loop)
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

pub fn generate_game_instance_builder_and_event_loop<'a>(
    game_name: &'a str,
    event_loop: &EventLoop<()>,
) -> GameInstanceBuilder<
    'a,
    EngineStarterCreator,
    LoopSystemCreator<
        DisplayCreator,
        ContextAttributesCreator,
        FallbackContextAttributesCreator,
        NotCurrentGlContextCreator,
        ResumedEventCreator,
        WindowResizedEventCreator,
        WindowCloseRequestedEventCreator,
        RedrawEventsClearedEventCreator,
    >,
    EngineEnderCreator,
> {
    //let event_loop = EventLoopBuilder::new().build();
    let game_instance_builder = generate_game_instance_builder(game_name, &event_loop);

    game_instance_builder
}

fn generate_game_instance_builder<'a>(
    game_name: &'a str,
    event_loop: &EventLoop<()>,
) -> GameInstanceBuilder<
    'a,
    EngineStarterCreator,
    LoopSystemCreator<
        DisplayCreator,
        ContextAttributesCreator,
        FallbackContextAttributesCreator,
        NotCurrentGlContextCreator,
        ResumedEventCreator,
        WindowResizedEventCreator,
        WindowCloseRequestedEventCreator,
        RedrawEventsClearedEventCreator,
    >,
    EngineEnderCreator,
> {
    create_game_instance_builder::<
        EngineStarter,
        EngineStarterCreator,
        LoopSystem<
            EventRunner<
                ResumedEvent<GlWindowCreator>,
                WindowResizedEvent,
                WindowCloseRequestedEvent,
                RedrawEventsClearedEvent,
                Renderer,
            >,
        >,
        LoopSystemCreator<
            DisplayCreator,
            ContextAttributesCreator,
            FallbackContextAttributesCreator,
            NotCurrentGlContextCreator,
            ResumedEventCreator,
            WindowResizedEventCreator,
            WindowCloseRequestedEventCreator,
            RedrawEventsClearedEventCreator,
        >,
        EngineEnder,
        EngineEnderCreator,
    >(
        game_name,
        EngineStarterCreator::new(),
        LoopSystemCreator::new(
            DisplayCreator::new(),
            ContextAttributesCreator::new(),
            FallbackContextAttributesCreator::new(),
            NotCurrentGlContextCreator::new(),
            StateCreator::new(),
            ResumedEventCreator::new(),
            WindowResizedEventCreator::new(),
            WindowCloseRequestedEventCreator::new(),
            RedrawEventsClearedEventCreator::new(),
        ),
        EngineEnderCreator::new(),
        &event_loop,
    )
}

pub struct EngineStarter {}

impl EngineStarter {
    fn new() -> Self {
        Self {}
    }
}

impl StartEngine for EngineStarter {
    fn start_engine(self) {}
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

pub trait RunEvent {
    fn run(&self, window_target: &EventLoopWindowTarget<()>);
}

pub trait RunLoop {
    fn run_loop(self, event_loop: EventLoop<()>);
}

pub struct EngineEnder {}

impl EngineEnder {
    fn new() -> Self {
        Self {}
    }
}

impl EndEngine for EngineEnder {
    fn end_engine(self) {}
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
    use crate::End;
    use mockall::{mock, Sequence};

    use super::*;

    mock! {
        Runner {}
        impl Run for Runner {
            fn run(&self);
        }
    }

    mock! {
        EngineEnder {}
        impl EndEngine for EngineEnder {
            fn end_engine(self);
        }
    }

    mock! {
        EngineStarter {}
        impl StartEngine for EngineStarter {
            fn start_engine(self);
        }
    }

    #[test]
    fn when_and_end_system_ends_a_game_then_it_ends_each_component_and_the_engine() {
        let mut component_ender_1 = MockRunner::new();
        let mut component_ender_2 = MockRunner::new();
        let mut component_ender_3 = MockRunner::new();

        let mut sequence = Sequence::new();

        component_ender_1
            .expect_run()
            .times(1)
            .in_sequence(&mut sequence)
            .returning(|| ());

        component_ender_2
            .expect_run()
            .times(1)
            .in_sequence(&mut sequence)
            .returning(|| ());

        component_ender_3
            .expect_run()
            .times(1)
            .in_sequence(&mut sequence)
            .returning(|| ());

        let mut component_enders = Vec::<Box<dyn Run>>::new();
        component_enders.push(Box::new(component_ender_1));
        component_enders.push(Box::new(component_ender_2));
        component_enders.push(Box::new(component_ender_3));

        let mut engine_ender = MockEngineEnder::new();

        engine_ender.expect_end_engine().times(1).returning(|| ());

        let end_system = EndSystem::new(component_enders, engine_ender);

        end_system.end();
    }

    #[test]
    fn when_a_start_system_runs_then_it_starts_the_engine_and_each_component() {
        let mut sequence = Sequence::new();

        let mut engine_starter = MockEngineStarter::new();

        engine_starter
            .expect_start_engine()
            .times(1)
            .in_sequence(&mut sequence)
            .returning(|| ());

        let start_system = StartSystem::<MockEngineStarter>::new(engine_starter);

        start_system.start();
    }

    #[test]
    fn when_a_game_name_provider_gets_the_name_then_it_provides_the_name() {
        let name = "Test Game";

        let game_name_provider = GameNameProvider::new(name);

        let result = game_name_provider.get_name();

        assert_eq!(name, result);
    }
}
