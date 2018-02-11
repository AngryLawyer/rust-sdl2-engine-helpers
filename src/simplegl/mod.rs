use gl;
use sdl2;
use sdl2::{VideoSubsystem};
use sdl2::render::Canvas;
use sdl2::video::Window;

bitflags! {
    pub struct ClearFlags: u32 {
        const COLOR_BUFFER = gl::COLOR_BUFFER_BIT;
        const DEPTH_BUFFER = gl::DEPTH_BUFFER_BIT;
        const STENCIL_BUFFER = gl::STENCIL_BUFFER_BIT;
    }
}

pub fn find_sdl_gl_driver() -> Option<u32> {
    for (index, item) in sdl2::render::drivers().enumerate() {
        if item.name == "opengl" {
            return Some(index as u32);
        }
    }
    None
}

pub struct SimpleGl {
}

impl SimpleGl {
    // TODO: we need to embody the call to .opengl in the window constructor
    pub fn initialize(canvas: &Canvas<Window>, video: &VideoSubsystem) -> Result<SimpleGl, String> {
        gl::load_with(|name| video.gl_get_proc_address(name) as *const _);
        try!(canvas.window().gl_set_context_to_current());

        Ok(SimpleGl {})
    }

    pub fn clear_color(&self, r: f32, g: f32, b: f32, a: f32) {
        unsafe {
            gl::ClearColor(r, g, b, a)
        }
    }

    pub fn clear(&self, flags: ClearFlags) {
        unsafe {
            gl::Clear(flags.bits())
        }
    }
}
