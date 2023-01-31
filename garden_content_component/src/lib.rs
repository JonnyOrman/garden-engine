use garden::{
    gl, GetName, Initialise, OnCreateGlutinVbo, OnDraw, RunEndComponent, RunFullComponent, RunLoop,
};
use garden_content::{
    Content, GetB, GetG, GetNumberOfObjects, GetNumberOfVertices, GetPosition, GetR, GetRgb,
    GetScale, GetVertexDataPtr, GetX, GetY, Rgb, Triangle, TriangleInstance, TrianglePoint,
    TwoDPoint,
};
use garden_content_loading::compose_content_loader;
use garden_loading::Load;
use garden_scenes::{GetHeight, GetWidth};
use garden_winit::AddComponent;

pub fn add_content<TGameInstanceBuilder: AddComponent, TScene: GetWidth + GetHeight>(
    game_instance_builder: &mut TGameInstanceBuilder,
    scene: &TScene,
) {
    let component = compose_component(scene);

    game_instance_builder.add(component);
}

fn compose_component<TScene: GetWidth + GetHeight>(
    scene: &TScene,
) -> ContentComponent<
    Content<
        Triangle<TrianglePoint<TwoDPoint, Rgb>>,
        TriangleInstance<TwoDPoint, TrianglePoint<TwoDPoint, Rgb>>,
    >,
> {
    let content_loader = compose_content_loader();
    let content = content_loader.load();

    let scaled_content = scale_content_to_scene(content, scene);

    let content_component = ContentComponent::new(scaled_content);

    content_component
}

fn scale_content_to_scene<TScene: GetWidth + GetHeight>(
    content: Content<
        Triangle<TrianglePoint<TwoDPoint, Rgb>>,
        TriangleInstance<TwoDPoint, TrianglePoint<TwoDPoint, Rgb>>,
    >,
    scene: &TScene,
) -> Content<
    Triangle<TrianglePoint<TwoDPoint, Rgb>>,
    TriangleInstance<TwoDPoint, TrianglePoint<TwoDPoint, Rgb>>,
> {
    let mut scaled_triangles = vec![];

    let original_triangles = content.get_triangles();

    for triangle in original_triangles.as_ref().unwrap().iter() {
        let scaled_triangle = Triangle::<TrianglePoint<TwoDPoint, Rgb>>::new(
            triangle.get_name().to_owned(),
            TrianglePoint::<TwoDPoint, Rgb>::new(
                TwoDPoint::new(
                    triangle.get_point_1().get_x(),
                    triangle.get_point_1().get_y(),
                ),
                Rgb::new(
                    triangle.get_point_1().get_rgb().get_r(),
                    triangle.get_point_1().get_rgb().get_g(),
                    triangle.get_point_1().get_rgb().get_b(),
                ),
            ),
            TrianglePoint::<TwoDPoint, Rgb>::new(
                TwoDPoint::new(
                    triangle.get_point_1().get_x(),
                    triangle.get_point_1().get_y(),
                ),
                Rgb::new(
                    triangle.get_point_2().get_rgb().get_r(),
                    triangle.get_point_2().get_rgb().get_g(),
                    triangle.get_point_2().get_rgb().get_b(),
                ),
            ),
            TrianglePoint::<TwoDPoint, Rgb>::new(
                TwoDPoint::new(
                    triangle.get_point_1().get_x(),
                    triangle.get_point_1().get_y(),
                ),
                Rgb::new(
                    triangle.get_point_3().get_rgb().get_r(),
                    triangle.get_point_3().get_rgb().get_g(),
                    triangle.get_point_3().get_rgb().get_b(),
                ),
            ),
        );

        scaled_triangles.push(scaled_triangle);
    }

    let mut scaled_triangle_instances = vec![];

    let original_triangle_instances = content.get_triangle_instances();

    for triangle_instance in original_triangle_instances.as_ref().unwrap().iter() {
        let scaled_triangle_instance =
            TriangleInstance::<TwoDPoint, TrianglePoint<TwoDPoint, Rgb>>::new(
                triangle_instance.get_name().to_owned(),
                triangle_instance.get_content_name().to_owned(),
                triangle_instance.get_scale(),
                TwoDPoint::new(
                    triangle_instance.get_position().get_x() / scene.get_width(),
                    triangle_instance.get_position().get_y() / scene.get_height(),
                ),
                TrianglePoint::<TwoDPoint, Rgb>::new(
                    TwoDPoint::new(
                        triangle_instance.get_point_1().get_x() / scene.get_width(),
                        triangle_instance.get_point_1().get_y() / scene.get_height(),
                    ),
                    Rgb::new(
                        triangle_instance.get_point_1().get_rgb().get_r(),
                        triangle_instance.get_point_1().get_rgb().get_g(),
                        triangle_instance.get_point_1().get_rgb().get_b(),
                    ),
                ),
                TrianglePoint::<TwoDPoint, Rgb>::new(
                    TwoDPoint::new(
                        triangle_instance.get_point_2().get_x() / scene.get_width(),
                        triangle_instance.get_point_2().get_y() / scene.get_height(),
                    ),
                    Rgb::new(
                        triangle_instance.get_point_2().get_rgb().get_r(),
                        triangle_instance.get_point_2().get_rgb().get_g(),
                        triangle_instance.get_point_2().get_rgb().get_b(),
                    ),
                ),
                TrianglePoint::<TwoDPoint, Rgb>::new(
                    TwoDPoint::new(
                        triangle_instance.get_point_3().get_x() / scene.get_width(),
                        triangle_instance.get_point_3().get_y() / scene.get_height(),
                    ),
                    Rgb::new(
                        triangle_instance.get_point_3().get_rgb().get_r(),
                        triangle_instance.get_point_3().get_rgb().get_g(),
                        triangle_instance.get_point_3().get_rgb().get_b(),
                    ),
                ),
            );
        scaled_triangle_instances.push(scaled_triangle_instance);
    }

    Content::<
        Triangle<TrianglePoint<TwoDPoint, Rgb>>,
        TriangleInstance<TwoDPoint, TrianglePoint<TwoDPoint, Rgb>>,
    >::new(scaled_triangles, scaled_triangle_instances)
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
