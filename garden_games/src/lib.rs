use garden::{AddRun, Create, GetName, Run};
use mockall::automock;

pub trait AddComponentEnder {
    fn add_component_ender(&self, component_ender: &dyn Run);
}

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

impl<TEngineEnder> AddComponentEnder for EndSystem<TEngineEnder> {
    fn add_component_ender(&self, component_ender: &dyn Run) {}
}

#[automock]
pub trait StartEngine {
    fn start_engine(self);
}

pub trait Start {
    fn start(self);
}

#[automock]
pub trait EndEngine {
    fn end_engine(self);
}

pub trait End {
    fn end(self);
}

pub struct StartSystem<TEngineStarter> {
    engine_starter: TEngineStarter,
    component_starters: Vec<Box<dyn Run>>,
}

impl<TEngineStarter: StartEngine> StartSystem<TEngineStarter> {
    fn new(engine_starter: TEngineStarter, component_starters: Vec<Box<dyn Run>>) -> Self {
        Self {
            engine_starter,
            component_starters,
        }
    }
}

impl<TEngineStarter: StartEngine> Start for StartSystem<TEngineStarter> {
    fn start(self) {
        self.engine_starter.start_engine();

        for component_starter in self.component_starters.iter() {
            component_starter.run()
        }
    }
}

impl<'a, TEngineStarter: StartEngine> AddRun for StartSystem<TEngineStarter> {
    fn add_run(&self, run: &dyn Run) {
        panic!("garden_games::StartSystem::add_run not implemented")
        //self.component_initialisers.push(Rc::new(run))
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

    let component_starters = Vec::<Box<dyn Run>>::new();

    StartSystem::<TEngineStarter>::new(engine_starter, component_starters)
}

impl<'a, TEngineEnder: EndEngine> AddRun for EndSystem<TEngineEnder> {
    fn add_run(&self, run: &dyn Run) {
        panic!("garden_games::EndSystem::add_run not implemented")
    }
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
    use garden::MockRun;
    use mockall::Sequence;

    use super::*;

    #[test]
    fn when_and_end_system_ends_a_game_then_it_ends_each_component_and_the_engine() {
        let mut component_ender_1 = MockRun::new();
        let mut component_ender_2 = MockRun::new();
        let mut component_ender_3 = MockRun::new();

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

        let mut engine_ender = MockEndEngine::new();

        engine_ender.expect_end_engine().times(1).returning(|| ());

        let end_system = EndSystem::new(component_enders, engine_ender);

        end_system.end();
    }

    #[test]
    fn when_a_start_system_runs_then_it_starts_the_engine_and_each_component() {
        let mut component_starter_1 = MockRun::new();
        let mut component_starter_2 = MockRun::new();
        let mut component_starter_3 = MockRun::new();

        let mut sequence = Sequence::new();

        let mut engine_starter = MockStartEngine::new();

        engine_starter
            .expect_start_engine()
            .times(1)
            .in_sequence(&mut sequence)
            .returning(|| ());

        component_starter_1
            .expect_run()
            .times(1)
            .in_sequence(&mut sequence)
            .returning(|| ());

        component_starter_2
            .expect_run()
            .times(1)
            .in_sequence(&mut sequence)
            .returning(|| ());

        component_starter_3
            .expect_run()
            .times(1)
            .in_sequence(&mut sequence)
            .returning(|| ());

        let mut component_starters = Vec::<Box<dyn Run>>::new();
        component_starters.push(Box::new(component_starter_1));
        component_starters.push(Box::new(component_starter_2));
        component_starters.push(Box::new(component_starter_3));

        let start_system = StartSystem::new(engine_starter, component_starters);

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
