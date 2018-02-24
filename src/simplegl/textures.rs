use gl;
use image;
use std::os::raw::c_void;

pub struct Texture {
    texture_id: u32,
    active_texture_id: u8,
}

impl Texture {
    // TODO: active_texture_id should be an enum
    pub fn new(image: &image::RgbaImage, active_texture_id: u8) -> Texture {
        let mut texture = Texture {
            texture_id: 0,
            active_texture_id: active_texture_id
        };
        unsafe {
            gl::GenTextures(1, &mut texture.texture_id);
            gl::ActiveTexture(gl::TEXTURE0 + texture.active_texture_id as u32);
            gl::BindTexture(gl::TEXTURE_2D, texture.texture_id);
            // TODO: Accept the following as params
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA as i32, image.width() as i32, image.height() as i32, 0, gl::RGBA, gl::UNSIGNED_BYTE, image.as_ptr() as *const c_void);
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
        texture
    }

    pub fn texture_id(&self) ->u32 {
        self.texture_id
    }

    pub fn active_texture_id(&self) -> u8 {
        self.active_texture_id
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, &mut self.texture_id);
        }
    }
}
