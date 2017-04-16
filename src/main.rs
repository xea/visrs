#[macro_use]
extern crate glium;

use glium::{ DisplayBuild, Frame, Program, Surface, VertexBuffer };
use glium::backend::Facade;
use glium::index::{ NoIndices, PrimitiveType };
use glium::glutin::{WindowBuilder};
use glium::glutin::{Event, VirtualKeyCode};
use glium::uniforms::Uniforms;

use std::time::{Duration, Instant};

pub struct ScreenSettings {
    pub width: u32,
    pub height: u32,
    pub title: String
}

pub struct ProgramState {
    pub shall_continue: bool,
    pub start_time: Instant,
}

impl ProgramState {
    pub fn elapsed_millis(&self) -> Duration {
        self.start_time.elapsed()
    }
}

impl Default for ProgramState {
    fn default() -> Self {
        ProgramState {
            shall_continue: true,
            start_time: Instant::now()
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
}

struct Scene<'a, T> where T: 'a + Uniforms {
    pub vertex_buffer: VertexBuffer<Vertex>,
    pub indices: NoIndices,
    pub shader_program: Program,
    pub uniforms: &'a T
}

impl<'a, T> Scene<'a, T> where T: 'a + Uniforms {
    pub fn new(facade: &Facade, vertices: &[Vertex], uniforms: &'a T, vertex_shader: &str, fragment_shader: &str) -> Scene<'a, T> {
        Scene {
            vertex_buffer: VertexBuffer::new(facade, vertices).unwrap(),
            indices: NoIndices(PrimitiveType::TrianglesList),
            shader_program: Program::from_source(facade, vertex_shader, fragment_shader, None).unwrap(),
            uniforms: uniforms
        }
    }
}

fn main() -> () {
    let settings = ScreenSettings { width: 1024, height: 768, title: format!("VisRS") };

    let display = WindowBuilder::new()
        .with_dimensions(settings.width, settings.height)
        .with_title(settings.title)
        .build_glium()
        .unwrap();

    implement_vertex!(Vertex, position);

    let triangle = Shape::triangle();

    let mut program_state = ProgramState::default();

    let vertex_shader = include_str!("default.vert");
    let fragment_shader = include_str!("default.frag");

    let mut uniform = uniform! {
        
    };

    let scene = Scene::new(&display, &triangle.vertices, &uniform, vertex_shader, fragment_shader);

    while program_state.shall_continue {
        draw_frame(display.draw(), &scene, &mut program_state);
        
        for event in display.poll_events() {
            handle_event(&event, &mut program_state);
        }
    }
}

fn draw_frame<T>(mut frame: Frame, scene: &Scene<T>, state: &mut ProgramState) -> () where T: Uniforms {
    frame.clear_color(0.0, 0.0, 0.0, 1.0);

    frame.draw(&scene.vertex_buffer, &scene.indices, &scene.shader_program, scene.uniforms, &Default::default()).unwrap();
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

