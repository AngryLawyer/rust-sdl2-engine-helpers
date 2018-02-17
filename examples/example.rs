extern crate sdl2;
extern crate sdl2_engine_helpers;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

use sdl2_engine_helpers::simplegl::{SimpleGlBuilder, ClearFlags};

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let (simplegl, mut canvas) = video_subsystem.window("Example", 800, 600)
        .position_centered()
        .simple_gl(&video_subsystem);

    simplegl.clear_color(1.0, 1.0, 1.0, 1.0);
    simplegl.clear(ClearFlags::COLOR_BUFFER);

    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();

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
