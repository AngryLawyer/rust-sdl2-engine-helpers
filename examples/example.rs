extern crate sdl2;
extern crate sdl2_engine_helpers;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

use sdl2_engine_helpers::simplegl::{SimpleGlBuilder, ClearFlags};
use sdl2_engine_helpers::simplegl::buffers::{Buffer, BufferSectionLength};
use sdl2_engine_helpers::simplegl::shaders::{Shader, ShaderType};

pub fn main() {
    let vertex_shader_raw = r#"#version 330 core
layout (location = 0) in vec3 aPos;

void main()
{
    gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
}"#;

    let fragment_shader_raw = r#"#version 330 core
out vec4 FragColor;

void main()
{
    FragColor = vec4(1.0f, 0.5f, 0.2f, 1.0f);
}"#;

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let mut simplegl = video_subsystem.window("Example", 800, 600)
        .position_centered()
        .simple_gl(&video_subsystem);

    simplegl.clear_color(1.0, 1.0, 1.0, 1.0);
    simplegl.clear(ClearFlags::COLOR_BUFFER);

    simplegl.canvas().present();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let vertex_shader = Shader::new(vertex_shader_raw, ShaderType::VertexShader).unwrap();
    let fragment_shader = Shader::new(fragment_shader_raw, ShaderType::FragmentShader).unwrap();

    let buffer = Buffer::new(vec![
        -0.5, -0.5, 0.0,
        0.5, -0.5, 0.0,
        0.0, 0.5, 0.0
    ], vec![BufferSectionLength::Length3], None);

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        // The rest of the game loop goes here...
    }
}
