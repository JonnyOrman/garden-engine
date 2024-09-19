use garden::AddComponent;
use garden_content_component::add_content;
use garden_games::{generate_game_instance_builder_and_event_loop, BuildGameInstance};
use garden_scenes_component::GetScene;
use garden_winit::RunGameInstance;
use winit::event_loop::EventLoopBuilder;

fn main() {
    let event_loop = EventLoopBuilder::new().build();

    let mut game_instance_builder_and_event_loop =
        generate_game_instance_builder_and_event_loop("Garden: Glutin Example", &event_loop);

    // let game_instance = game_instance_builder_and_event_loop
    //     //.0
    //     .build_game_instance(&event_loop);

    //let mut game_instance_builder = game_instance_builder_and_event_loop.0;

    let json_component = garden_json_component::compose_component();

    let scene_component = garden_scenes_component::compose_component(&json_component);

    let maths_component = garden_maths_component::compose_component();

    add_content(
        &mut game_instance_builder_and_event_loop,
        scene_component.get_scene(),
        &json_component,
        &maths_component,
    );

    game_instance_builder_and_event_loop.add(scene_component);

    let game_instance = game_instance_builder_and_event_loop.build_game_instance(&event_loop);

    game_instance.run_game_instance(event_loop)
}
