use garden::{
    gl, Initialise, OnCreateGlutinVbo, OnDraw, RunEndComponent, RunFullComponent, RunLoop,
};

use garden_loading::Load;
use garden_scenes::TwoDScene;
use garden_scenes_loading::compose_scene_loader;
use garden_winit::AddComponent;

pub fn add_scenes<TGameInstanceBuilder: AddComponent>(
    game_instance_builder: &mut TGameInstanceBuilder,
) {
    let component = compose_component();

    game_instance_builder.add(component);
}

fn compose_component() -> ScenesComponent<TwoDScene> {
    let scene_loader = compose_scene_loader();
    let scene = scene_loader.load();

    let content_component = ScenesComponent::new(scene);

    content_component
}

pub struct ScenesComponent<TScene> {
    scene: TScene,
}

impl<TScene> ScenesComponent<TScene> {
    fn new(scene: TScene) -> Self {
        Self { scene }
    }
}

impl<TScene> Initialise for ScenesComponent<TScene> {
    fn initialise(&self) {}
}

impl<TScene> RunLoop for ScenesComponent<TScene> {
    fn run_loop(&self) {}
}

impl<TScene> RunEndComponent for ScenesComponent<TScene> {
    fn run_end_component(self) {}
}

impl<TScene> OnDraw for ScenesComponent<TScene> {
    unsafe fn on_draw(&self, gl: &garden::gl::Gl) {}
}

impl<TScene> OnCreateGlutinVbo for ScenesComponent<TScene> {
    unsafe fn on_create_glutin_vbo(&self, gl: &gl::Gl) {}
}

impl<TScene> RunFullComponent for ScenesComponent<TScene> {}
