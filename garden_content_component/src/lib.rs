use std::rc::Rc;

use garden::{
    gl, Initialise, OnCreateGlutinVbo, OnDraw, RunEndComponent, RunFullComponent, RunLoop,
};
use garden_content::{Content, GetNumberOfObjects, GetNumberOfVertices, GetVertexDataPtr};
use garden_content_loading::compose_content_loader;
use garden_json::JsonToF32Converter;
use garden_loading::Load;
use garden_scenes::{GetHeight, GetWidth};
use garden_winit::AddComponent;

pub fn add_content<TGameInstanceBuilder: AddComponent, TScene: GetWidth + GetHeight>(
    game_instance_builder: &mut TGameInstanceBuilder,
    scene: &TScene,
    json_to_f32_converter: Rc<JsonToF32Converter>,
) {
    let component = compose_component(scene, json_to_f32_converter);

    game_instance_builder.add(component);
}

fn compose_component<TScene: GetWidth + GetHeight>(
    scene: &TScene,
    json_to_f32_converter: Rc<JsonToF32Converter>,
) -> ContentComponent<Content> {
    let content_loader = compose_content_loader(json_to_f32_converter);
    let mut content = content_loader.load();

    content.scale_object_instances(scene.get_width(), scene.get_height());

    let content_component = ContentComponent::new(content);

    content_component
}

pub struct ContentInitialiser {}

impl ContentInitialiser {
    fn new() -> Self {
        Self {}
    }
}

impl Initialise for ContentInitialiser {
    fn initialise(&self) {}
}

pub struct ContentLoopRunner {}

impl ContentLoopRunner {
    fn new() -> Self {
        Self {}
    }
}

impl RunLoop for ContentLoopRunner {
    fn run_loop(&self) {}
}

pub struct ContentComponent<TContent> {
    content: TContent,
}

impl<TContent> ContentComponent<TContent> {
    fn new(content: TContent) -> Self {
        Self { content }
    }
}

impl<TContent> Initialise for ContentComponent<TContent> {
    fn initialise(&self) {}
}

impl<TContent> RunLoop for ContentComponent<TContent> {
    fn run_loop(&self) {}
}

impl<TContent> RunEndComponent for ContentComponent<TContent> {
    fn run_end_component(self) {}
}

impl<TContent: GetNumberOfObjects> OnDraw for ContentComponent<TContent> {
    unsafe fn on_draw(&self, gl: &garden::gl::Gl) {
        for n in 0..self.content.get_number_of_objects() {
            gl.DrawArrays(
                gl::TRIANGLES,
                ((n as i32) * 5) - (2 * n as i32),
                ((n as i32) * 5) - (2 * n as i32) + 3,
            );
        }
    }
}

impl<TContent: GetNumberOfVertices + GetVertexDataPtr> OnCreateGlutinVbo
    for ContentComponent<TContent>
{
    unsafe fn on_create_glutin_vbo(&self, gl: &gl::Gl) {
        gl.BufferData(
            gl::ARRAY_BUFFER,
            (self.content.get_number_of_vertices() * std::mem::size_of::<f32>() as i32)
                as gl::types::GLsizeiptr,
            self.content.get_vertex_data_ptr() as *const _,
            gl::STATIC_DRAW,
        );
    }
}

impl<TContent: GetNumberOfVertices + GetVertexDataPtr + GetNumberOfObjects> RunFullComponent
    for ContentComponent<TContent>
{
}
