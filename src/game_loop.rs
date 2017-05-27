use std::time::{Duration, Instant};
use std::thread::sleep;

pub struct GameLoop {
    // TODO: Think about including time information, separate think and render steps
    frame_number: u64,
    fps: u32
}

impl GameLoop {
    pub fn new(fps: u32) -> GameLoop {
        GameLoop {
            frame_number: 0,
            fps: fps
        }
    }

    pub fn run<F>(&mut self, mut fun: F)
        where F: FnMut(u64) -> bool {  //FIXME: Could be an enum or something
        let ns_per_frame : Duration = Duration::new(0, 1_000_000_000 / self.fps);
        'running: loop {
            let start = Instant::now();

            if fun(self.frame_number) {
                break 'running;
            }
            // Framerate cap
            let next_render_step = start + ns_per_frame;
            let now = Instant::now();
            if next_render_step >= now {
                sleep(next_render_step - now);
            }
            self.frame_number += 1;
        }
    }
}
