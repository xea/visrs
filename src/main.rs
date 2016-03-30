#[macro_use]
extern crate glium;
extern crate libc;
extern crate time;
extern crate filetime;

use filetime::FileTime;
use glium::{DisplayBuild, Surface, VertexBuffer};
use glium::glutin::WindowBuilder;
use std::collections::HashMap;
use std::fs::{File, metadata};
use std::io::{Cursor, Read};
use std::thread;
use std::sync::mpsc::channel;
use std::sync::mpsc::Sender;
use std::time::Duration;
use time::now;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}

#[derive(Copy, Clone, Debug)]
enum ShaderType {
    Vertex,
    Fragment,
    Geometry
}

#[derive(Default)]
struct ShaderProgramSource {
    vertex: Option<String>,
    fragment: Option<String>,
    geometry: Option<String>
}


fn update_shaders(tx: Sender<ShaderProgramSource>, sources: Vec<(&str, ShaderType)>) {
    let mut mtimes: HashMap<String, u64> = HashMap::new();

    loop {
        let mut shader = ShaderProgramSource { vertex: None, fragment: None, geometry: None };

        let mut changed = false;

        for source in sources.iter() {
            match File::open(source.0) {
                Ok(mut file) => {
                    //let mtime = file.metadata().unwrap().mtime();
                    let metadata = metadata(source.0).unwrap();
                    let mtime = FileTime::from_last_modification_time(&metadata).seconds();


                    if !mtimes.contains_key(source.0) || *mtimes.get(source.0).unwrap() < mtime {
                        mtimes.insert(source.0.to_string(), mtime);
                        changed = true;
                    }

                    let mut file_buffer = String::new();

                    match file.read_to_string(&mut file_buffer) {
                        Err(_) => { println!("Couldn't read file {}", source.0); ()},
                        _ => match source.1 {
                                ShaderType::Vertex => shader.vertex = Some(file_buffer.clone().to_string()),
                                ShaderType::Fragment => shader.fragment = Some(file_buffer.clone().to_string()),
                                ShaderType::Geometry => shader.geometry = Some(file_buffer.clone().to_string()),
                        }

                    }

                },
                _ => ()
            };
        }

        if changed {
            tx.clone().send(shader).unwrap();
        }

        thread::sleep(Duration::from_millis(500));
    }

}

fn main() {
    let display = WindowBuilder::new().build_glium().unwrap();
    //let version = display.get_opengl_version_string();

    let (tx, rx) = channel();

    let sources = vec![ ("default.vs", ShaderType::Vertex), ("default.fs", ShaderType::Fragment), ("default.gs", ShaderType::Geometry) ];

    thread::spawn(move || {
        update_shaders(tx, sources);
    });

    implement_vertex!(Vertex, position);

/*
    let vertex1 = Vertex { position: [ -1.0,  1.0 ] };
    let vertex2 = Vertex { position: [ -1.0, -1.0 ] };
    let vertex3 = Vertex { position: [  1.0,  1.0 ] };
    */

    let vertex4 = Vertex { position: [ 0.5,  0.5] };
    let vertex5 = Vertex { position: [-0.0, -0.5] };
    let vertex6 = Vertex { position: [-0.5,  0.25] };
    /*
    let vertex4 = Vertex { position: [ -1.0, -1.0 ] };
    let vertex5 = Vertex { position: [  1.0, -1.0 ] };
    let vertex6 = Vertex { position: [  1.0,  1.0 ] };
    */

    //let shape = vec![vertex1, vertex2, vertex3, vertex4, vertex5, vertex6];
    let shape = vec![vertex4, vertex5, vertex6];

    let vertex_buffer = VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

//    let program = glium::Program::from_source(&display, &vs_src, &fs_src, None).unwrap();

    let program_start_time = now();
    let sample_rate = 44100;
    let mut resolution = display.get_framebuffer_dimensions();
    let mut mouse_position = (0, 0);

    let init_shader = rx.recv().unwrap();
    let mut frame_count = 0;
    let mut program = glium::Program::from_source(&display, &init_shader.vertex.unwrap(), &init_shader.fragment.unwrap(), None).unwrap();
    let mut last_frame_render_time = 0.0;
    let mut average_render_time = 0.0;
    let mut fps = 0.0;

    loop {
        match rx.try_recv() {
            Ok(shader) => {
                println!("Shaders updated");
                match glium::Program::from_source(&display, &shader.vertex.unwrap(), &shader.fragment.unwrap(), None) {
                    Ok(prg) => program = prg,
                    Err(error) => println!("Couldn't compile shader: {}", error)
                }

            },
            _ => ()
        };

        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);

        let frame_time = now();
        frame_count += 1;

        let t = (frame_count as f32 / 500.0).sin();

/*
uniform float     iChannelTime[4];       // channel playback time (in seconds)
uniform vec3      iChannelResolution[4]; // channel resolution (in pixels)
uniform vec4      iMouse;                // mouse pixel coords. xy: current (if MLB down), zw: click
uniform samplerXX iChannel0..3;          // input channel. XX = 2D/Cube
*/

        let uniforms = uniform! {
            iTimeDelta: last_frame_render_time,
            iGlobalTime: (frame_time - program_start_time).num_milliseconds() as f32 / 1000.0,
            iSampleRate: sample_rate as f32,
            iResolution: [ resolution.0 as f32, resolution.1 as f32, 0.0 as f32 ],
            iMouse: [ mouse_position.0 as f32, mouse_position.1 as f32, 0.0 as f32, 0.0 as f32 ],
            iFrame: frame_count,
            iDate: [ (frame_time.tm_year + 1900) as f32, frame_time.tm_mon as f32, frame_time.tm_mday as f32, (frame_time.tm_hour * 3600 + frame_time.tm_min * 60 + frame_time.tm_sec) as f32 ],
            iFPS: fps,
            matrix: [
                [t.cos(), t.sin(), 0.0, 0.0],
                [-t.sin(), t.cos(), 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [t, 0.0, 0.0, 1.0f32],
            ]
        };

        // textures
//        let image = image::load(Cursor::new(&include_bytes!("texture1.png")[..]), image::PNG).unwrap().to_rgba();

        target.draw(&vertex_buffer, &indices, &program, &uniforms, &Default::default()).unwrap();
        target.finish().unwrap();

        last_frame_render_time = (now() - frame_time).num_milliseconds() as f32 / 1000.0;
        average_render_time = (average_render_time * frame_count as f32 + last_frame_render_time) / (frame_count + 1) as f32;
        fps = 1.0 / average_render_time;

        for ev in display.poll_events() {
            match ev {
                glium::glutin::Event::Closed => return,
                glium::glutin::Event::MouseMoved(position) => mouse_position = position,
                glium::glutin::Event::Resized(x, y) => {println!("Resized to {} {}", x, y); resolution = (x, y)},
                glium::glutin::Event::KeyboardInput(_, _, event) => {
                    match event.unwrap() {
                        glium::glutin::VirtualKeyCode::Escape => return,
                        _ => ()
                    }
                },
                _ => {()}
            }
        }

        // FPS limiting
    }
}
