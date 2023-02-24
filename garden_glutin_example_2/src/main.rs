use garden_content_component::add_content;
use garden_glutin::generate_game_instance_builder_and_event_loop;
use garden_scenes_component::GetScene;
use garden_winit::{AddComponent, BuildGameInstance, RunGameInstance};

fn main() {
    let game_instance_builder_and_event_loop =
        generate_game_instance_builder_and_event_loop("Garden: Glutin Example");

    let mut game_instance_builder = game_instance_builder_and_event_loop.0;

    let json_component = garden_json_component::compose_component();

    let scene_component = garden_scenes_component::compose_component(&json_component);

    add_content(
        &mut game_instance_builder,
        scene_component.get_scene(),
        &json_component,
    );

    game_instance_builder.add(scene_component);

    let game_instance =
        game_instance_builder.build_game_instance(&game_instance_builder_and_event_loop.1);

    game_instance.run_game_instance(game_instance_builder_and_event_loop.1)
}
