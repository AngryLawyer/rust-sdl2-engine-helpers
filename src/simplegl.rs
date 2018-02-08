use gl;
use sdl2;
use sdl2::{VideoSubsystem};
use sdl2::render::Canvas;
use sdl2::video::Window;

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
}
