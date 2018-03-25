use gl;
use sdl2;
use sdl2::{VideoSubsystem};
use sdl2::render::Canvas;
use sdl2::video::{Window, WindowBuilder, GLContext};
use sdl2::video::GLProfile;

pub mod buffers;
pub mod shaders;
pub mod programs;
pub mod textures;

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

pub trait SimpleGlBuilder {
    fn simple_gl(&mut self, video: &VideoSubsystem) -> SimpleGl;
}

impl SimpleGlBuilder for WindowBuilder {
    fn simple_gl(&mut self, video_subsystem: &VideoSubsystem) -> SimpleGl {

        let gl_attr = video_subsystem.gl_attr();
        gl_attr.set_context_profile(GLProfile::Core);
        gl_attr.set_context_version(3, 3);

        let window = self
            .opengl()
            .build()
            .unwrap();

        let ctx = window.gl_create_context().unwrap();
        gl::load_with(|name| video_subsystem.gl_get_proc_address(name) as *const _);

        debug_assert_eq!(gl_attr.context_profile(), GLProfile::Core);
        debug_assert_eq!(gl_attr.context_version(), (3, 3));

        SimpleGl {
            window,
            ctx
        }
    }
}

pub struct SimpleGl {
    window: Window,
    ctx: GLContext,
}

impl SimpleGl {

    pub fn window(&mut self) -> &mut Window {
        &mut self.window
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
