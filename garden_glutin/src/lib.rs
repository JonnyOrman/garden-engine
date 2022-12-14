use std::{
    ffi::{c_void, CStr, CString},
    num::NonZeroU32,
    ops::Deref,
};

use garden_content::{GetNumberOfObjects, GetNumberOfVertices, GetVertexDataPtr};
use garden_games::{EndEngine, EndSystem, GameNameProvider, StartEngine, StartSystem};

use garden::{AddRun, Create, Run};
use garden_winit::{
    create_game_instance_builder, BuildGameInstance, CreateLoopSystem, Engine, GameInstance,
    RunGameInstance, RunLoopSystem,
};
use glutin::{
    config::{Config, ConfigTemplateBuilder},
    context::{
        ContextApi, ContextAttributes, ContextAttributesBuilder, NotCurrentContext,
        PossiblyCurrentContext,
    },
    display::{Display, GetGlDisplay},
    prelude::{GlConfig, GlDisplay, NotCurrentGlContextSurfaceAccessor},
    surface::{GlSurface, SurfaceAttributesBuilder, WindowSurface},
};
use glutin_winit::DisplayBuilder;
use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};
use winit::{
    dpi::PhysicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop, EventLoopBuilder, EventLoopWindowTarget},
    window::{Window, WindowBuilder},
};

use glutin::surface::{Surface, SwapInterval};

pub mod gl {
    #![allow(clippy::all)]
    include!(concat!(env!("OUT_DIR"), "/gl_bindings.rs"));

    pub use Gles2 as Gl;
}

pub fn start<
    'a,
    TContent: 'static + GetNumberOfObjects + GetNumberOfVertices + GetVertexDataPtr,
>(
    game_name: &'a str,
    content: TContent,
) {
    let event_loop = EventLoopBuilder::new().build().into();
    let game_instance = compose::<TContent>(game_name, &event_loop, content);

    game_instance.run_game_instance(event_loop);
}

fn compose<'a, TContent: 'static + GetNumberOfObjects + GetNumberOfVertices + GetVertexDataPtr>(
    name: &'a str,
    event_loop: &EventLoop<()>,
    content: TContent,
) -> GameInstance<
    'a,
    Engine<
        StartSystem<EngineStarter>,
        LoopSystem<
            EventRunner<
                ResumedEvent<GlWindowCreator>,
                WindowResizedEvent,
                WindowCloseRequestedEvent,
                RedrawEventsClearedEvent,
                TContent,
                Renderer,
            >,
        >,
        EndSystem<EngineEnder>,
        GameNameProvider<'a>,
    >,
> {
    let game_instance_builder = create_game_instance_builder::<
        EngineStarter,
        EngineStarterCreator,
        LoopSystem<
            EventRunner<
                ResumedEvent<GlWindowCreator>,
                WindowResizedEvent,
                WindowCloseRequestedEvent,
                RedrawEventsClearedEvent,
                TContent,
                Renderer,
            >,
        >,
        LoopSystemCreator<
            DisplayCreator,
            ContextAttributesCreator,
            FallbackContextAttributesCreator,
            NotCurrentGlContextCreator,
            ResumedEventCreator,
            WindowResizedEventCreator,
            WindowCloseRequestedEventCreator,
            RedrawEventsClearedEventCreator,
        >,
        EngineEnder,
        EngineEnderCreator,
        TContent,
    >(
        name,
        EngineStarterCreator::new(),
        LoopSystemCreator::new(
            DisplayCreator::new(),
            ContextAttributesCreator::new(),
            FallbackContextAttributesCreator::new(),
            NotCurrentGlContextCreator::new(),
            StateCreator::new(),
            ResumedEventCreator::new(),
            WindowResizedEventCreator::new(),
            WindowCloseRequestedEventCreator::new(),
            RedrawEventsClearedEventCreator::new(),
        ),
        EngineEnderCreator::new(),
        event_loop,
        content,
    );

    game_instance_builder.build_game_instance()
}

pub struct EngineStarter {}

impl EngineStarter {
    fn new() -> Self {
        Self {}
    }
}

impl StartEngine for EngineStarter {
    fn start_engine(self) {}
}

pub struct EngineStarterCreator {}

impl EngineStarterCreator {
    fn new() -> Self {
        Self {}
    }
}

impl Create<EngineStarter> for EngineStarterCreator {
    fn create(&self) -> EngineStarter {
        EngineStarter::new()
    }
}

pub trait RunEvent {
    fn run(&self, window_target: &EventLoopWindowTarget<()>);
}

pub trait RunLoop {
    fn run_loop(self, event_loop: EventLoop<()>);
}

pub trait CreateGlWindow {
    fn create_gl_window(&self, window: Window, gl_config: &Config) -> GlWindow;
}

pub struct GlWindowCreator {}

impl GlWindowCreator {
    fn new() -> Self {
        Self {}
    }
}

impl CreateGlWindow for GlWindowCreator {
    fn create_gl_window(&self, window: Window, gl_config: &Config) -> GlWindow {
        let (width, height): (u32, u32) = window.inner_size().into();
        let raw_window_handle = window.raw_window_handle();
        let attrs = SurfaceAttributesBuilder::<WindowSurface>::new().build(
            raw_window_handle,
            NonZeroU32::new(width).unwrap(),
            NonZeroU32::new(height).unwrap(),
        );

        unsafe {
            gl_config
                .display()
                .create_window_surface(gl_config, &attrs)
                .unwrap()
        };

        GlWindow::new(window, gl_config)
    }
}

pub trait RunResumedEvent<TContent, TRenderer> {
    fn run_resumed_event(
        &mut self,
        window: &mut Option<Window>,
        window_target: &EventLoopWindowTarget<()>,
        gl_config: &Config,
        not_current_gl_context: NotCurrentContext,
        gl_display: &Display,
        state: &mut Option<(PossiblyCurrentContext, GlWindow)>,
        renderer: &mut Option<TRenderer>,
        content: &TContent,
    );
}

pub struct ResumedEvent<TGlWindowCreator> {
    gl_window_creator: TGlWindowCreator,
}

impl<TGlWindowCreator: CreateGlWindow> ResumedEvent<TGlWindowCreator> {
    fn new(gl_window_creator: TGlWindowCreator) -> Self {
        Self { gl_window_creator }
    }
}

impl<TGlWindowCreator: CreateGlWindow, TContent: GetNumberOfVertices + GetVertexDataPtr>
    RunResumedEvent<TContent, Renderer> for ResumedEvent<TGlWindowCreator>
{
    fn run_resumed_event(
        &mut self,
        window: &mut Option<Window>,
        window_target: &EventLoopWindowTarget<()>,
        gl_config: &Config,
        not_current_gl_context: NotCurrentContext,
        gl_display: &Display,
        state: &mut Option<(PossiblyCurrentContext, GlWindow)>,
        renderer: &mut Option<Renderer>,
        content: &TContent,
    ) {
        #[cfg(target_os = "android")]
        println!("Android window available");

        let window = window.take().unwrap_or_else(|| {
            let window_builder = WindowBuilder::new().with_transparent(true);
            glutin_winit::finalize_window(window_target, window_builder, gl_config).unwrap()
        });

        let gl_window = self.gl_window_creator.create_gl_window(window, gl_config);

        let gl_context = not_current_gl_context
            .make_current(&gl_window.surface)
            .unwrap();

        renderer.get_or_insert_with(|| generate_renderer(gl_display, content));

        if let Err(res) = gl_window
            .surface
            .set_swap_interval(&gl_context, SwapInterval::Wait(NonZeroU32::new(1).unwrap()))
        {
            eprintln!("Error setting vsync: {:?}", res);
        }

        assert!(state.replace((gl_context, gl_window)).is_none());
    }
}

pub struct ResumedEventCreator {}

impl ResumedEventCreator {
    fn new() -> Self {
        Self {}
    }
}

impl Create<ResumedEvent<GlWindowCreator>> for ResumedEventCreator {
    fn create(&self) -> ResumedEvent<GlWindowCreator> {
        ResumedEvent::new(GlWindowCreator::new())
    }
}

pub trait RunWindowResizedEvent<TRenderer> {
    fn run_window_resized_event(
        &mut self,
        size: PhysicalSize<u32>,
        state: &mut Option<(PossiblyCurrentContext, GlWindow)>,
        renderer: &mut Option<TRenderer>,
    );
}

pub struct WindowResizedEvent {}

impl WindowResizedEvent {
    fn new() -> Self {
        Self {}
    }
}

impl<TRenderer: Resize> RunWindowResizedEvent<TRenderer> for WindowResizedEvent {
    fn run_window_resized_event(
        &mut self,
        size: PhysicalSize<u32>,
        state: &mut Option<(PossiblyCurrentContext, GlWindow)>,
        renderer: &mut Option<TRenderer>,
    ) {
        if size.width != 0 && size.height != 0 {
            if let Some((gl_context, gl_window)) = state {
                gl_window.surface.resize(
                    gl_context,
                    NonZeroU32::new(size.width).unwrap(),
                    NonZeroU32::new(size.height).unwrap(),
                );
                renderer
                    .as_ref()
                    .unwrap()
                    .resize(size.width as i32, size.height as i32);
            }
        }
    }
}

pub struct WindowResizedEventCreator {}

impl WindowResizedEventCreator {
    fn new() -> Self {
        Self {}
    }
}

impl Create<WindowResizedEvent> for WindowResizedEventCreator {
    fn create(&self) -> WindowResizedEvent {
        WindowResizedEvent::new()
    }
}

pub trait RunRedrawEventsClearedEvent<TContent, TRenderer> {
    fn run_redraw_events_cleared_event(
        &mut self,
        state: &mut Option<(PossiblyCurrentContext, GlWindow)>,
        renderer: &mut Option<TRenderer>,
        content: &TContent,
    );
}

pub struct RedrawEventsClearedEvent {}

impl RedrawEventsClearedEvent {
    fn new() -> Self {
        Self {}
    }
}

impl<TContent: GetNumberOfObjects, TRenderer: Render<TContent>>
    RunRedrawEventsClearedEvent<TContent, TRenderer> for RedrawEventsClearedEvent
{
    fn run_redraw_events_cleared_event(
        &mut self,
        state: &mut Option<(PossiblyCurrentContext, GlWindow)>,
        renderer: &mut Option<TRenderer>,
        content: &TContent,
    ) {
        if let Some((gl_context, gl_window)) = state {
            renderer.as_ref().unwrap().draw(content);
            gl_window.window.request_redraw();

            gl_window.surface.swap_buffers(gl_context).unwrap();
        }
    }
}

pub struct RedrawEventsClearedEventCreator {}

impl RedrawEventsClearedEventCreator {
    fn new() -> Self {
        Self {}
    }
}

impl Create<RedrawEventsClearedEvent> for RedrawEventsClearedEventCreator {
    fn create(&self) -> RedrawEventsClearedEvent {
        RedrawEventsClearedEvent::new()
    }
}

pub trait RunWindowCloseRequestedEvent {
    fn run_window_close_requested_event(&self, control_flow: &mut ControlFlow);
}

pub struct WindowCloseRequestedEvent {}

impl WindowCloseRequestedEvent {
    fn new() -> Self {
        Self {}
    }
}

impl RunWindowCloseRequestedEvent for WindowCloseRequestedEvent {
    fn run_window_close_requested_event(&self, control_flow: &mut ControlFlow) {
        control_flow.set_exit();
    }
}

pub struct WindowCloseRequestedEventCreator {}

impl WindowCloseRequestedEventCreator {
    fn new() -> Self {
        Self {}
    }
}

impl Create<WindowCloseRequestedEvent> for WindowCloseRequestedEventCreator {
    fn create(&self) -> WindowCloseRequestedEvent {
        WindowCloseRequestedEvent::new()
    }
}

pub trait RunEvents {
    fn run_resumed_event(&mut self, window_target: &EventLoopWindowTarget<()>);

    fn run_window_resized_event(&mut self, size: PhysicalSize<u32>);

    fn run_window_close_requested_event(&mut self, control_flow: &mut ControlFlow);

    fn run_redraw_events_cleared_event(&mut self);
}

pub struct EventRunner<
    TResumedEvent,
    TWindowResizedEvent,
    TWindowCloseRequestedEvent,
    TRedrawEventsClearedEvent,
    TContent,
    TRenderer,
> {
    window: Option<Window>,
    gl_config: Config,
    not_current_gl_context: Option<NotCurrentContext>,
    renderer: Option<TRenderer>,
    state: Option<(PossiblyCurrentContext, GlWindow)>,
    gl_display: Display,
    resumed_event: TResumedEvent,
    window_resized_event: TWindowResizedEvent,
    window_close_requested_event: TWindowCloseRequestedEvent,
    redraw_events_cleared_event: TRedrawEventsClearedEvent,
    content: TContent,
}

impl<
        TResumedEvent: RunResumedEvent<TContent, TRenderer>,
        TWindowResizedEvent: RunWindowResizedEvent<TRenderer>,
        TWindowCloseRequestedEvent: RunWindowCloseRequestedEvent,
        TRedrawEventsClearedEvent: RunRedrawEventsClearedEvent<TContent, TRenderer>,
        TContent,
        TRenderer,
    >
    EventRunner<
        TResumedEvent,
        TWindowResizedEvent,
        TWindowCloseRequestedEvent,
        TRedrawEventsClearedEvent,
        TContent,
        TRenderer,
    >
{
    fn new(
        window: Option<Window>,
        gl_config: Config,
        not_current_gl_context: Option<NotCurrentContext>,
        renderer: Option<TRenderer>,
        state: Option<(PossiblyCurrentContext, GlWindow)>,
        gl_display: Display,
        resumed_event: TResumedEvent,
        window_resized_event: TWindowResizedEvent,
        window_close_requested_event: TWindowCloseRequestedEvent,
        redraw_events_cleared_event: TRedrawEventsClearedEvent,
        content: TContent,
    ) -> Self {
        Self {
            window,
            gl_config,
            not_current_gl_context,
            renderer,
            state,
            gl_display,
            resumed_event,
            window_resized_event,
            window_close_requested_event,
            redraw_events_cleared_event,
            content,
        }
    }
}

impl<
        TResumedEvent: RunResumedEvent<TContent, TRenderer>,
        TWindowResizedEvent: RunWindowResizedEvent<TRenderer>,
        TWindowCloseRequestedEvent: RunWindowCloseRequestedEvent,
        TRedrawEventsClearedEvent: RunRedrawEventsClearedEvent<TContent, TRenderer>,
        TContent,
        TRenderer,
    > RunEvents
    for EventRunner<
        TResumedEvent,
        TWindowResizedEvent,
        TWindowCloseRequestedEvent,
        TRedrawEventsClearedEvent,
        TContent,
        TRenderer,
    >
{
    fn run_resumed_event(&mut self, window_target: &EventLoopWindowTarget<()>) {
        self.resumed_event.run_resumed_event(
            &mut self.window,
            window_target,
            &self.gl_config,
            self.not_current_gl_context.take().unwrap(),
            &self.gl_display,
            &mut self.state,
            &mut self.renderer,
            &mut self.content,
        )
    }

    fn run_window_resized_event(&mut self, size: PhysicalSize<u32>) {
        self.window_resized_event.run_window_resized_event(
            size,
            &mut self.state,
            &mut self.renderer,
        )
    }

    fn run_window_close_requested_event(&mut self, control_flow: &mut ControlFlow) {
        self.window_close_requested_event
            .run_window_close_requested_event(control_flow)
    }

    fn run_redraw_events_cleared_event(&mut self) {
        self.redraw_events_cleared_event
            .run_redraw_events_cleared_event(&mut self.state, &mut self.renderer, &self.content)
    }
}

pub struct LoopSystem<TEventRunner> {
    event_runner: TEventRunner,
}

impl<TEventRunner> LoopSystem<TEventRunner> {
    fn new(event_runner: TEventRunner) -> Self {
        Self { event_runner }
    }
}

impl<TEventRunner> AddRun for LoopSystem<TEventRunner> {
    fn add_run(&self, run: &dyn Run) {}
}

impl<TEventRunner: 'static + RunEvents> RunLoopSystem for LoopSystem<TEventRunner> {
    fn run_loop_system(mut self, event_loop: EventLoop<()>) {
        event_loop.run(move |event, window_target, control_flow| {
            control_flow.set_wait();
            match event {
                Event::Resumed => {
                    self.event_runner.run_resumed_event(window_target);
                }
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::Resized(size) => self.event_runner.run_window_resized_event(size),
                    WindowEvent::CloseRequested => self
                        .event_runner
                        .run_window_close_requested_event(control_flow),
                    _ => (),
                },
                Event::RedrawEventsCleared => self.event_runner.run_redraw_events_cleared_event(),
                _ => (),
            }
        })
    }
}

pub struct GlWindow {
    pub surface: Surface<WindowSurface>,
    pub window: Window,
}

impl GlWindow {
    pub fn new(window: Window, config: &Config) -> Self {
        let (width, height): (u32, u32) = window.inner_size().into();
        let raw_window_handle = window.raw_window_handle();
        let attrs = SurfaceAttributesBuilder::<WindowSurface>::new().build(
            raw_window_handle,
            NonZeroU32::new(width).unwrap(),
            NonZeroU32::new(height).unwrap(),
        );

        let surface = unsafe {
            config
                .display()
                .create_window_surface(config, &attrs)
                .unwrap()
        };

        Self { window, surface }
    }
}

pub trait Render<TContent> {
    fn draw(&self, content: &TContent);
}

pub trait Resize {
    fn resize(&self, width: i32, height: i32);
}

pub struct Renderer {
    program: gl::types::GLuint,
    vao: gl::types::GLuint,
    vbo: gl::types::GLuint,
    gl: gl::Gl,
}

impl Renderer {
    pub fn new(
        program: gl::types::GLuint,
        vao: gl::types::GLuint,
        vbo: gl::types::GLuint,
        gl: gl::Gl,
    ) -> Self {
        Self {
            program,
            vao,
            vbo,
            gl,
        }
    }
}

impl<TContent: GetNumberOfObjects> Render<TContent> for Renderer {
    fn draw(&self, content: &TContent) {
        unsafe {
            self.gl.UseProgram(self.program);

            self.gl.BindVertexArray(self.vao);
            self.gl.BindBuffer(gl::ARRAY_BUFFER, self.vbo);

            self.gl.ClearColor(0.1, 0.1, 0.1, 0.9);
            self.gl.Clear(gl::COLOR_BUFFER_BIT);

            for n in 0..content.get_number_of_objects() {
                self.gl.DrawArrays(
                    gl::TRIANGLES,
                    ((n as i32) * 5) - (2 * n as i32),
                    ((n as i32) * 5) - (2 * n as i32) + 3,
                );
            }
        }
    }
}

impl Resize for Renderer {
    fn resize(&self, width: i32, height: i32) {
        unsafe {
            self.gl.Viewport(0, 0, width, height);
        }
    }
}

impl Deref for Renderer {
    type Target = gl::Gl;

    fn deref(&self) -> &Self::Target {
        &self.gl
    }
}

impl Drop for Renderer {
    fn drop(&mut self) {
        unsafe {
            self.gl.DeleteProgram(self.program);
            self.gl.DeleteBuffers(1, &self.vbo);
            self.gl.DeleteVertexArrays(1, &self.vao);
        }
    }
}

pub trait CreateGl {
    fn create_gl(self, display: &Display) -> gl::Gl;
}

pub struct GlCreator {}

impl GlCreator {
    fn new() -> Self {
        Self {}
    }
}

impl CreateGl for GlCreator {
    fn create_gl(self, display: &Display) -> gl::Gl {
        let gl = gl::Gl::load_with(|symbol| {
            let symbol = CString::new(symbol).unwrap();
            display.get_proc_address(symbol.as_c_str()).cast()
        });

        if let Some(renderer) = get_gl_string(&gl, gl::RENDERER) {
            println!("Running on {}", renderer.to_string_lossy());
        }
        if let Some(version) = get_gl_string(&gl, gl::VERSION) {
            println!("OpenGL Version {}", version.to_string_lossy());
        }

        if let Some(shaders_version) = get_gl_string(&gl, gl::SHADING_LANGUAGE_VERSION) {
            println!("Shaders version on {}", shaders_version.to_string_lossy());
        }

        return gl;
    }
}

pub trait CreateShaders {
    unsafe fn create_vertex_shader(&self, gl: &gl::Gl) -> gl::types::GLuint;
    unsafe fn create_fragment_shader(&self, gl: &gl::Gl) -> gl::types::GLuint;
}

pub struct ShaderCreator {}

impl ShaderCreator {
    fn new() -> Self {
        Self {}
    }

    unsafe fn create(
        &self,
        gl: &gl::Gl,
        shader: gl::types::GLenum,
        source: &[u8],
    ) -> gl::types::GLuint {
        let shader = gl.CreateShader(shader);
        gl.ShaderSource(
            shader,
            1,
            [source.as_ptr().cast()].as_ptr(),
            std::ptr::null(),
        );
        gl.CompileShader(shader);
        shader
    }
}

impl CreateShaders for ShaderCreator {
    unsafe fn create_vertex_shader(&self, gl: &gl::Gl) -> gl::types::GLuint {
        self.create(gl, gl::VERTEX_SHADER, VERTEX_SHADER_SOURCE)
    }

    unsafe fn create_fragment_shader(&self, gl: &gl::Gl) -> gl::types::GLuint {
        self.create(gl, gl::FRAGMENT_SHADER, FRAGMENT_SHADER_SOURCE)
    }
}

pub trait CreateProgram {
    unsafe fn create_program(&self, gl: &gl::Gl) -> gl::types::GLuint;
}

pub struct ProgramCreator {}

impl ProgramCreator {
    fn new() -> Self {
        Self {}
    }
}

impl CreateProgram for ProgramCreator {
    unsafe fn create_program(&self, gl: &gl::Gl) -> gl::types::GLuint {
        gl.CreateProgram()
    }
}

pub trait SetupVertexAttrib {
    unsafe fn setup(self, gl: &gl::Gl, program: gl::types::GLuint);
}

pub struct VertexAttribSetup {}

impl VertexAttribSetup {
    fn new() -> Self {
        Self {}
    }

    unsafe fn setup_attrib(
        &self,
        gl: &gl::Gl,
        program: gl::types::GLuint,
        num: i32,
        str: &[u8],
        ptr: *const c_void,
    ) {
        let attrib = self.get_attrib_location(gl, program, str);
        self.setup_pointer(gl, attrib as gl::types::GLuint, num, ptr);
        gl.EnableVertexAttribArray(attrib as gl::types::GLuint);
    }

    unsafe fn get_attrib_location(
        &self,
        gl: &gl::Gl,
        program: gl::types::GLuint,
        str: &[u8],
    ) -> i32 {
        gl.GetAttribLocation(program, str.as_ptr() as *const _)
    }

    unsafe fn setup_pointer(
        &self,
        gl: &gl::Gl,
        gluint: gl::types::GLuint,
        num: i32,
        ptr: *const c_void,
    ) {
        gl.VertexAttribPointer(
            gluint,
            num,
            gl::FLOAT,
            0,
            5 * std::mem::size_of::<f32>() as gl::types::GLsizei,
            ptr,
        );
    }
}

impl SetupVertexAttrib for VertexAttribSetup {
    unsafe fn setup(self, gl: &gl::Gl, program: gl::types::GLuint) {
        self.setup_attrib(gl, program, 2, b"position\0", std::ptr::null());

        self.setup_attrib(
            gl,
            program,
            3,
            b"color\0",
            (2 * std::mem::size_of::<f32>()) as *const () as *const _,
        );
    }
}

pub trait CreateGLutin<TContent> {
    unsafe fn create_glutin(self, gl: &gl::Gl, content: &TContent) -> gl::types::GLuint;
}

pub struct VaoCreator {}

impl VaoCreator {
    fn new() -> Self {
        Self {}
    }
}

impl<TContent> CreateGLutin<TContent> for VaoCreator {
    unsafe fn create_glutin(self, gl: &gl::Gl, content: &TContent) -> gl::types::GLuint {
        let mut vao = std::mem::zeroed();
        gl.GenVertexArrays(1, &mut vao);
        gl.BindVertexArray(vao);

        vao
    }
}

pub struct VboCreator {}

impl VboCreator {
    fn new() -> Self {
        Self {}
    }
}

impl<TContent: GetNumberOfVertices + GetVertexDataPtr> CreateGLutin<TContent> for VboCreator {
    unsafe fn create_glutin(self, gl: &gl::Gl, content: &TContent) -> gl::types::GLuint {
        let mut vbo = std::mem::zeroed();
        gl.GenBuffers(1, &mut vbo);
        gl.BindBuffer(gl::ARRAY_BUFFER, vbo);

        let ptr_to_use = content.get_vertex_data_ptr() as *const _;
        let message3 = format!("get_vertex_data_ptr ptr to use {:p}", ptr_to_use);
        println!("{}", message3);

        gl.BufferData(
            gl::ARRAY_BUFFER,
            (content.get_number_of_vertices() * std::mem::size_of::<f32>() as i32)
                as gl::types::GLsizeiptr,
            ptr_to_use,
            gl::STATIC_DRAW,
        );

        vbo
    }
}

pub trait CreateRenderer<TRenderer, TContent> {
    fn create_renderer(self, display: &Display, content: &TContent) -> TRenderer;
}

pub struct RendererCreator<
    TGlCreator,
    TShaderCreator,
    TProgramCreator,
    TVertexAttribSetup,
    TVaoCreator,
    TVboCreator,
> {
    gl_creator: TGlCreator,
    shader_creator: TShaderCreator,
    program_creator: TProgramCreator,
    vertex_attrib_setup: TVertexAttribSetup,
    vao_creator: TVaoCreator,
    vbo_creator: TVboCreator,
}

impl<TGlCreator, TShaderCreator, TProgramCreator, TVertexAttribSetup, TVaoCreator, TVboCreator>
    RendererCreator<
        TGlCreator,
        TShaderCreator,
        TProgramCreator,
        TVertexAttribSetup,
        TVaoCreator,
        TVboCreator,
    >
{
    fn new(
        gl_creator: TGlCreator,
        shader_creator: TShaderCreator,
        program_creator: TProgramCreator,
        vertex_attrib_setup: TVertexAttribSetup,
        vao_creator: TVaoCreator,
        vbo_creator: TVboCreator,
    ) -> Self {
        Self {
            gl_creator,
            shader_creator,
            program_creator,
            vertex_attrib_setup,
            vao_creator,
            vbo_creator,
        }
    }
}

impl<
        TGlCreator: CreateGl,
        TShaderCreator: CreateShaders,
        TProgramCreator: CreateProgram,
        TVertexAttribSetup: SetupVertexAttrib,
        TVaoCreator: CreateGLutin<TContent>,
        TVboCreator: CreateGLutin<TContent>,
        TContent,
    > CreateRenderer<Renderer, TContent>
    for RendererCreator<
        TGlCreator,
        TShaderCreator,
        TProgramCreator,
        TVertexAttribSetup,
        TVaoCreator,
        TVboCreator,
    >
{
    fn create_renderer(self, display: &Display, content: &TContent) -> Renderer {
        unsafe {
            let gl = self.gl_creator.create_gl(display);

            let program = self.program_creator.create_program(&gl);

            let vertex_shader = self.shader_creator.create_vertex_shader(&gl);
            gl.AttachShader(program, vertex_shader);

            let fragment_shader = self.shader_creator.create_fragment_shader(&gl);
            gl.AttachShader(program, fragment_shader);

            gl.LinkProgram(program);

            gl.UseProgram(program);

            gl.DeleteShader(vertex_shader);
            gl.DeleteShader(fragment_shader);

            let vao = self.vao_creator.create_glutin(&gl, content);

            let vbo = self.vbo_creator.create_glutin(&gl, content);

            self.vertex_attrib_setup.setup(&gl, program);

            Renderer::new(program, vao, vbo, gl)
        }
    }
}

fn compose_renderer_creator() -> RendererCreator<
    GlCreator,
    ShaderCreator,
    ProgramCreator,
    VertexAttribSetup,
    VaoCreator,
    VboCreator,
> {
    RendererCreator::new(
        GlCreator::new(),
        ShaderCreator::new(),
        ProgramCreator::new(),
        VertexAttribSetup::new(),
        VaoCreator::new(),
        VboCreator::new(),
    )
}

fn generate_renderer<TContent: GetNumberOfVertices + GetVertexDataPtr>(
    display: &Display,
    content: &TContent,
) -> Renderer {
    compose_renderer_creator().create_renderer(display, content)
}

fn get_gl_string(gl: &gl::Gl, variant: gl::types::GLenum) -> Option<&'static CStr> {
    unsafe {
        let s = gl.GetString(variant);
        (!s.is_null()).then(|| CStr::from_ptr(s.cast()))
    }
}

pub trait CreateDisplay {
    fn create_display(&self, event_loop: &EventLoop<()>) -> (Option<Window>, Config);
}

pub struct DisplayCreator {}

impl<'a> DisplayCreator {
    fn new() -> Self {
        Self {}
    }
}

impl CreateDisplay for DisplayCreator {
    fn create_display(&self, event_loop: &EventLoop<()>) -> (Option<Window>, Config) {
        let window_builder = if cfg!(wgl_backend) {
            Some(WindowBuilder::new().with_transparent(true))
        } else {
            None
        };

        let display_builder = DisplayBuilder::new().with_window_builder(window_builder);

        let template = ConfigTemplateBuilder::new().with_alpha_size(8);

        display_builder
            .build(event_loop, template, |configs| {
                configs
                    .reduce(|accum, config| {
                        let transparency_check = config.supports_transparency().unwrap_or(false)
                            & !accum.supports_transparency().unwrap_or(false);

                        if transparency_check || config.num_samples() > accum.num_samples() {
                            config
                        } else {
                            accum
                        }
                    })
                    .unwrap()
            })
            .unwrap()
    }
}

pub struct StateCreator {}

impl StateCreator {
    fn new() -> Self {
        Self {}
    }
}

impl Create<Option<(PossiblyCurrentContext, GlWindow)>> for StateCreator {
    fn create(&self) -> Option<(PossiblyCurrentContext, GlWindow)> {
        None
    }
}

pub trait CreateContextAttributes {
    fn create_context_attributes(
        &self,
        raw_window_handle: Option<RawWindowHandle>,
    ) -> ContextAttributes;
}

pub struct ContextAttributesCreator {}

impl ContextAttributesCreator {
    fn new() -> Self {
        Self {}
    }
}

impl CreateContextAttributes for ContextAttributesCreator {
    fn create_context_attributes(
        &self,
        raw_window_handle: Option<RawWindowHandle>,
    ) -> ContextAttributes {
        ContextAttributesBuilder::new().build(raw_window_handle)
    }
}

pub trait CreateFallbackContextAttributes {
    fn create_fallback_context_attributes(
        &self,
        raw_window_handle: Option<RawWindowHandle>,
    ) -> ContextAttributes;
}

pub struct FallbackContextAttributesCreator {}

impl FallbackContextAttributesCreator {
    fn new() -> Self {
        Self {}
    }
}

impl CreateFallbackContextAttributes for FallbackContextAttributesCreator {
    fn create_fallback_context_attributes(
        &self,
        raw_window_handle: Option<RawWindowHandle>,
    ) -> ContextAttributes {
        ContextAttributesBuilder::new()
            .with_context_api(ContextApi::Gles(None))
            .build(raw_window_handle)
    }
}

pub struct LoopSystemCreator<
    TDisplayCreator,
    TContextAttributesCreator,
    TFallbackContextAttributesCreator,
    TNotCurrentGlContextCreator,
    TResumedEventCreator,
    TWindowResizedEventCreator,
    TWindowCloseRequestedEventCreator,
    TRedrawEventsClearedEventCreator,
> {
    display_creator: TDisplayCreator,
    context_attributes_creator: TContextAttributesCreator,
    fallback_context_attributes_creator: TFallbackContextAttributesCreator,
    not_current_gl_context_creator: TNotCurrentGlContextCreator,
    state_creator: StateCreator,
    resumed_event_creator: TResumedEventCreator,
    window_resized_event_creator: TWindowResizedEventCreator,
    window_close_requested_event_creator: TWindowCloseRequestedEventCreator,
    redraw_events_cleared_event_creator: TRedrawEventsClearedEventCreator,
}

impl<
        TDisplayCreator,
        TContextAttributesCreator,
        TFallbackContextAttributesCreator,
        TNotCurrentGlContextCreator,
        TResumedEventCreator,
        TWindowResizedEventCreator,
        TWindowCloseRequestedEventCreator,
        TRedrawEventsClearedEventCreator,
    >
    LoopSystemCreator<
        TDisplayCreator,
        TContextAttributesCreator,
        TFallbackContextAttributesCreator,
        TNotCurrentGlContextCreator,
        TResumedEventCreator,
        TWindowResizedEventCreator,
        TWindowCloseRequestedEventCreator,
        TRedrawEventsClearedEventCreator,
    >
{
    fn new(
        display_creator: TDisplayCreator,
        context_attributes_creator: TContextAttributesCreator,
        fallback_context_attributes_creator: TFallbackContextAttributesCreator,
        not_current_gl_context_creator: TNotCurrentGlContextCreator,
        state_creator: StateCreator,
        resumed_event_creator: TResumedEventCreator,
        window_resized_event_creator: TWindowResizedEventCreator,
        window_close_requested_event_creator: TWindowCloseRequestedEventCreator,
        redraw_events_cleared_event_creator: TRedrawEventsClearedEventCreator,
    ) -> Self {
        Self {
            display_creator,
            context_attributes_creator,
            fallback_context_attributes_creator,
            not_current_gl_context_creator,
            state_creator,
            resumed_event_creator,
            window_resized_event_creator,
            window_close_requested_event_creator,
            redraw_events_cleared_event_creator,
        }
    }
}

impl<
        TDisplayCreator: CreateDisplay,
        TContextAttributesCreator: CreateContextAttributes,
        TFallbackContextAttributesCreator: CreateFallbackContextAttributes,
        TNotCurrentGlContextCreator: CreateNotCurrentGlContext,
        TResumedEventCreator: Create<ResumedEvent<GlWindowCreator>>,
        TWindowResizedEventCreator: Create<WindowResizedEvent>,
        TWindowCloseRequestedEventCreator: Create<WindowCloseRequestedEvent>,
        TRedrawEventsClearedEventCreator: Create<RedrawEventsClearedEvent>,
        TContent: GetNumberOfObjects + GetNumberOfVertices + GetVertexDataPtr,
    >
    CreateLoopSystem<
        LoopSystem<
            EventRunner<
                ResumedEvent<GlWindowCreator>,
                WindowResizedEvent,
                WindowCloseRequestedEvent,
                RedrawEventsClearedEvent,
                TContent,
                Renderer,
            >,
        >,
        TContent,
    >
    for LoopSystemCreator<
        TDisplayCreator,
        TContextAttributesCreator,
        TFallbackContextAttributesCreator,
        TNotCurrentGlContextCreator,
        TResumedEventCreator,
        TWindowResizedEventCreator,
        TWindowCloseRequestedEventCreator,
        TRedrawEventsClearedEventCreator,
    >
{
    fn create_loop_system(
        &self,
        event_loop: &EventLoop<()>,
        content: TContent,
    ) -> LoopSystem<
        EventRunner<
            ResumedEvent<GlWindowCreator>,
            WindowResizedEvent,
            WindowCloseRequestedEvent,
            RedrawEventsClearedEvent,
            TContent,
            Renderer,
        >,
    > {
        let display = self.display_creator.create_display(event_loop);
        let window = display.0;
        let gl_config = display.1;

        let gl_display = gl_config.display();

        let raw_window_handle = window.as_ref().map(|window| window.raw_window_handle());

        let context_attributes = self
            .context_attributes_creator
            .create_context_attributes(raw_window_handle);

        let fallback_context_attributes = self
            .fallback_context_attributes_creator
            .create_fallback_context_attributes(raw_window_handle);

        let not_current_gl_context = self
            .not_current_gl_context_creator
            .create_not_current_gl_context(
                &gl_display,
                &gl_config,
                &context_attributes,
                &fallback_context_attributes,
            );

        let state = self.state_creator.create();

        let resumed_event = self.resumed_event_creator.create();

        let window_resized_event = self.window_resized_event_creator.create();

        let window_close_requested_event = self.window_close_requested_event_creator.create();

        let redraw_events_cleared_event = self.redraw_events_cleared_event_creator.create();

        let event_runner = EventRunner::new(
            window,
            gl_config,
            not_current_gl_context,
            None,
            state,
            gl_display,
            resumed_event,
            window_resized_event,
            window_close_requested_event,
            redraw_events_cleared_event,
            content,
        );

        LoopSystem::new(event_runner)
    }
}

pub trait CreateNotCurrentGlContext {
    fn create_not_current_gl_context(
        &self,
        gl_display: &Display,
        gl_config: &Config,
        context_attributes: &ContextAttributes,
        fallback_context_attributes: &ContextAttributes,
    ) -> Option<NotCurrentContext>;
}

pub struct NotCurrentGlContextCreator {}

impl<'a> NotCurrentGlContextCreator {
    fn new() -> Self {
        Self {}
    }
}

impl CreateNotCurrentGlContext for NotCurrentGlContextCreator {
    fn create_not_current_gl_context(
        &self,
        gl_display: &Display,
        gl_config: &Config,
        context_attributes: &ContextAttributes,
        fallback_context_attributes: &ContextAttributes,
    ) -> Option<NotCurrentContext> {
        Some(unsafe {
            gl_display
                .create_context(gl_config, context_attributes)
                .unwrap_or_else(|_| {
                    gl_display
                        .create_context(gl_config, fallback_context_attributes)
                        .expect("failed to create context")
                })
        })
    }
}

pub struct EngineEnder {}

impl EngineEnder {
    fn new() -> Self {
        Self {}
    }
}

impl EndEngine for EngineEnder {
    fn end_engine(self) {}
}

pub struct EngineEnderCreator {}

impl EngineEnderCreator {
    fn new() -> Self {
        Self {}
    }
}

impl Create<EngineEnder> for EngineEnderCreator {
    fn create(&self) -> EngineEnder {
        EngineEnder::new()
    }
}

const VERTEX_SHADER_SOURCE: &[u8] = b"
#version 100
precision mediump float;

attribute vec2 position;
attribute vec3 color;

varying vec3 v_color;

void main() {
    gl_Position = vec4(position, 0.0, 1.0);
    v_color = color;
}
\0";

const FRAGMENT_SHADER_SOURCE: &[u8] = b"
#version 100
precision mediump float;

varying vec3 v_color;

void main() {
    gl_FragColor = vec4(v_color, 1.0);
}
\0";
