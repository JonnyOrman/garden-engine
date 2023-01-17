use garden_glutin::generate_game_instance_builder_and_event_loop;
use garden_winit::{BuildGameInstance, RunGameInstance};

fn main() {
    let game_instance_builder_and_event_loop =
        generate_game_instance_builder_and_event_loop("Garden: Glutin Example");

    let game_instance = game_instance_builder_and_event_loop
        .0
        .build_game_instance(&game_instance_builder_and_event_loop.1);

    game_instance.run_game_instance(game_instance_builder_and_event_loop.1)
}
