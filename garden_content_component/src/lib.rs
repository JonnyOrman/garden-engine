use garden::{
    gl, Initialise, OnCreateGlutinVbo, OnDraw, RunEndComponent, RunFullComponent, RunLoop,
};
use garden_content::{
    Content, GetNumberOfObjects, GetNumberOfVertices, GetVertexDataPtr, Rgb, Triangle,
    TriangleInstance, TrianglePoint, TwoDPoint,
};
use garden_content_loading::compose_content_loader;
use garden_loading::Load;
use garden_winit::AddComponent;

pub fn add_content<TGameInstanceBuilder: AddComponent>(
    game_instance_builder: &mut TGameInstanceBuilder,
) {
    let component = compose_component();

    game_instance_builder.add(component);
}

fn compose_component() -> ContentComponent<
    Content<
        Triangle<TrianglePoint<TwoDPoint, Rgb>>,
        TriangleInstance<TwoDPoint, TrianglePoint<TwoDPoint, Rgb>>,
    >,
> {
    let content_loader = compose_content_loader();
    let content = content_loader.load();

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
