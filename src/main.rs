#[macro_use]
extern crate glium;

use glium::{ DisplayBuild, Frame, Program, Surface, VertexBuffer };
use glium::backend::Facade;
use glium::index::{ NoIndices, PrimitiveType };
use glium::glutin::{WindowBuilder};
use glium::glutin::{Event, VirtualKeyCode};
use glium::uniforms::{EmptyUniforms, UniformValue, UniformsStorage};

use std::time::{Duration, Instant};

pub struct ScreenSettings {
    pub width: u32,
    pub height: u32,
    pub title: String
}

pub struct ProgramState {
    pub shall_continue: bool,
    pub start_time: Instant,
    pub counter: f32,
    pub frame_start: Instant
}

impl ProgramState {
    pub fn elapsed_millis(&self) -> u32 {
        let elapsed = self.start_time.elapsed();

        (elapsed.as_secs() * 1000) as u32 + (elapsed.subsec_nanos() / 1000000) as u32
    }
}

impl Default for ProgramState {
    fn default() -> Self {
        ProgramState {
            shall_continue: true,
            start_time: Instant::now(),
            counter: 0.0,
            frame_start: Instant::now()
        }
    }
}

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 3],
}


impl Vertex {
    pub fn new(x: f32, y: f32, z: f32) -> Vertex {
        Vertex { position: [ x, y, z ] }
    }
}

struct Shape {
    vertices: Vec<Vertex>,
    primitive_type: PrimitiveType
}

impl Shape {
    pub fn triangle() -> Shape {
        Shape { primitive_type: PrimitiveType::TrianglesList, vertices: vec![
            Vertex::new(0.0, -0.5, 0.0),
            Vertex::new(-0.5, 0.5, 0.0),
            Vertex::new(0.5, 0.5, 0.0)
        ]}
    }

    pub fn square() -> Shape {
        Shape { primitive_type: PrimitiveType::TriangleStrip, vertices: vec![
            Vertex::new( 0.5, -0.5, 0.0),
            Vertex::new( 0.5,  0.5, 0.0),
            Vertex::new(-0.5, -0.5, 0.0),
            Vertex::new(-0.5,  0.5, 0.0)
        ]}
    }

    pub fn five_point_star() -> Shape {
        Shape { primitive_type: PrimitiveType::TrianglesList, vertices: vec![
            Vertex::new( 1.0f32.sin(), 1.0f32.cos(), 0.0),
            Vertex::new( 0.5f32.sin(), 0.5f32.cos(), 0.0),
            Vertex::new( 0.0f32.sin(), 0.0f32.cos(), 0.0),

            Vertex::new(-1.0f32.sin(), -1.0f32.cos(), 0.0),
            Vertex::new(-0.5f32.sin(), -0.5f32.cos(), 0.0),
            Vertex::new(-0.0f32.sin(), -0.0f32.cos(), 0.0),
        ]}
    }

    pub fn cube() -> Shape {
        Shape { primitive_type: PrimitiveType::TrianglesList, vertices: vec![
            // Front face
            Vertex::new(-0.5, -0.5, -0.5),
            Vertex::new( 0.5, -0.5, -0.5),
            Vertex::new( 0.5,  0.5, -0.5),

            Vertex::new(-0.5, -0.5,  0.5),
            Vertex::new( 0.5,  0.5,  0.5),
            Vertex::new(-0.5,  0.5,  0.5),

            /*
            // Back face
            Vertex::new(-0.5, -0.5,  0.5),
            Vertex::new( 0.5, -0.5,  0.5),
            Vertex::new( 0.5,  0.5,  0.5),

            Vertex::new(-0.5, -0.5,  0.5),
            Vertex::new( 0.5,  0.5,  0.5),
            Vertex::new(-0.5,  0.5,  0.5),
            */
        ]}
    }
}

pub trait ShaderSource {
    fn vertex_shader(&self) -> String;
    fn fragment_shader(&self) -> String;
}


struct Scene {
    pub vertex_buffer: VertexBuffer<Vertex>,
    pub indices: NoIndices,
    pub shader_program: Program,
}

impl Scene {
    pub fn new(facade: &Facade, shape: &Shape, vertex_shader: &str, fragment_shader: &str) -> Scene {
        Scene {
            vertex_buffer: VertexBuffer::new(facade, &(shape.vertices)).unwrap(),
            indices: NoIndices(shape.primitive_type),
            shader_program: Program::from_source(facade, vertex_shader, fragment_shader, None).unwrap()
        }
    }
}

fn main() -> () {
    let settings = ScreenSettings { width: 1024, height: 768, title: format!("VisRS") };

    let display = WindowBuilder::new()
        .with_dimensions(settings.width, settings.height)
        .with_title(settings.title)
        .with_vsync()
        .build_glium()
        .unwrap();

    implement_vertex!(Vertex, position);

    let triangle = Shape::triangle();

    let mut program_state = ProgramState::default();

    let vertex_shader = include_str!("default.vert");
    let fragment_shader = include_str!("default.frag");

    let mut scene = Scene::new(&display, &triangle, vertex_shader, fragment_shader);

    while program_state.shall_continue {
        update_state(&mut scene, &mut program_state);

        draw_frame(display.draw(), &scene, &mut program_state);

        for event in display.poll_events() {
            handle_event(&event, &mut program_state);
        }

        //limit_rate(&program_state.frame_start);
    }
}

fn update_state(scene: &mut Scene, state: &mut ProgramState) -> () {
    state.counter += 0.01;
    state.frame_start = Instant::now();
}

fn draw_frame(mut frame: Frame, scene: &Scene, state: &mut ProgramState) -> () {
    frame.clear_color(0.0, 0.0, 0.0, 1.0);

    frame.draw(&scene.vertex_buffer, &scene.indices, &scene.shader_program, &uniform! {
        counter: state.counter,
        milliseconds: state.elapsed_millis(),
        matrix: [
           // [ 1.0, 0.0, 0.0, 0.0 ],
            [ state.counter.sin(), 0.0, 0.0, 0.0 ],
            [ 0.0, 1.0, 0.0, 0.0 ],
            [ 0.0, 0.0, 1.0, 0.0 ],
            [ 0.0, 0.0, 0.0, 1.0f32 ],
        ]
//        milliseconds: state.elapsed_millis()
    }, &Default::default()).unwrap();
    frame.finish().unwrap();
}

fn handle_event(event: &Event, state: &mut ProgramState) -> () {
    match event {
        &Event::Closed => { state.shall_continue = false; },
        &Event::Resized(width, height) => (),
        &Event::KeyboardInput(_, _, ref event) => match event.unwrap() {
            VirtualKeyCode::Escape => { state.shall_continue = false; },
            _ => ()
        },
        _ => ()
    }
}

fn limit_rate(frame_start: &Instant) -> () {
    let frame_ms = frame_start.elapsed().subsec_nanos() / 1_000_000;

    const FRAME_MS_LIMIT: u32 = 1000 / 60; // should correspond to about 25 frames/sec

    if frame_ms < FRAME_MS_LIMIT {
        let sleep_time = FRAME_MS_LIMIT - frame_ms;

        ::std::thread::sleep(Duration::from_millis(sleep_time as u64));
    }
}
