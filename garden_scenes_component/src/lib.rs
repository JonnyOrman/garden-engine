use std::rc::Rc;

use garden::{
    gl, Initialise, OnCreateGlutinVbo, OnDraw, RunEndComponent, RunFullComponent, RunLoop,
};

use garden_json::JsonToF32Converter;
use garden_loading::Load;
use garden_scenes::TwoDScene;
use garden_scenes_loading::compose_scene_loader;

pub fn compose_component(
    json_to_f32_converter: Rc<JsonToF32Converter>,
) -> ScenesComponent<TwoDScene> {
    let scene_loader = compose_scene_loader(json_to_f32_converter);
    let scene = scene_loader.load();

    let content_component = ScenesComponent::new(scene);

    content_component
}

pub trait GetScene<TScene> {
    fn get_scene(&self) -> &TScene;
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

impl<TScene> GetScene<TScene> for ScenesComponent<TScene> {
    fn get_scene(&self) -> &TScene {
        &self.scene
    }
}
