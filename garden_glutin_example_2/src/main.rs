use garden_content_component::add_content;
use garden_glutin::generate_game_instance_builder_and_event_loop;
use garden_scenes_component::add_scenes;
use garden_winit::{BuildGameInstance, RunGameInstance};

fn main() {
    let game_instance_builder_and_event_loop =
        generate_game_instance_builder_and_event_loop("Garden: Glutin Example");

    let mut game_instance_builder = game_instance_builder_and_event_loop.0;

    add_content(&mut game_instance_builder);
    add_scenes(&mut game_instance_builder);

    let game_instance =
        game_instance_builder.build_game_instance(&game_instance_builder_and_event_loop.1);

    game_instance.run_game_instance(game_instance_builder_and_event_loop.1)
}
