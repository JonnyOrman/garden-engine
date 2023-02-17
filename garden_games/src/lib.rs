use garden::{Create, GetName, Run};

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
