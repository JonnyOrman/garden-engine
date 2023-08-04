pub trait GetName {
    fn get_name(&self) -> &str;
}

pub trait Initialise {
    fn initialise(&self);
}

pub trait RunEndComponent {
    fn run_end_component(self);
}

pub trait OnDraw {
    unsafe fn on_draw(&self, gl: &gl::Gl);
}

pub trait OnCreateGlutinVbo {
    unsafe fn on_create_glutin_vbo(&self, gl: &gl::Gl);
}

pub trait GetInitialiser<TInitialise> {
    fn get_initialiser(self) -> TInitialise;
}

pub trait RunLoop {
    fn run_loop(&self);
}

pub trait Run {
    fn run(&self);
}

pub trait GetLoopRunner<TRunLoop> {
    fn get_loop_runner(&self) -> &TRunLoop;
}

pub trait GetEnder<TEnder> {
    fn get_ender(&self) -> &TEnder;
}

pub trait RunFullComponent:
    Initialise + RunLoop + RunEndComponent + OnDraw + OnCreateGlutinVbo
{
}

pub trait AddComponent {
    fn add<T: RunFullComponent + 'static>(&mut self, t: T);
}

pub trait Create<T> {
    fn create(&self) -> T;
}

pub trait GetWidth {
    fn get_width(&self) -> f32;
}

pub trait GetHeight {
    fn get_height(&self) -> f32;
}

pub mod gl {
    #![allow(clippy::all)]
    include!(concat!(env!("OUT_DIR"), "/gl_bindings.rs"));

    pub use Gles2 as Gl;
}
