use garden::{Create, GetName, RunFullComponent};
// use garden_games::{
//     create_end_system, create_game_name_provider, create_start_system, End, EndEngine, EndSystem,
//     GameNameProvider, Start, StartEngine, StartSystem,
// };
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
