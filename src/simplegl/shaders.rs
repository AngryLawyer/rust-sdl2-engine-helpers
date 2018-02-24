use gl::types::{GLchar, GLint};
use gl;
use std::ffi::CString;
use std::ptr;
use std::str;

pub enum ShaderType {
    VertexShader,
    FragmentShader,
}

impl ShaderType {
    pub fn to_raw(&self) -> u32 {
        match *self {
            ShaderType::VertexShader => gl::VERTEX_SHADER,
            ShaderType::FragmentShader => gl::FRAGMENT_SHADER
        }
    }
}

pub struct Shader {
    pub shader_id: u32,
    pub shader_type: ShaderType,
}

fn check_for_errors(shader: u32) -> Result<(), String> {
    let mut success = gl::FALSE as GLint;
    unsafe {
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
    };
    if success != gl::TRUE as GLint {
        let mut info_log = Vec::with_capacity(512);
        unsafe {
            let mut final_length = 0;
            gl::GetShaderInfoLog(shader, 512, &mut final_length, info_log.as_mut_ptr() as *mut GLchar);
            info_log.set_len(final_length as usize);
        };
        Err(str::from_utf8(&info_log).unwrap().to_string())
    } else {
        Ok(())
    }
}

impl Shader {
    pub fn new(program: &str, shader_type: ShaderType) -> Result<Shader, String> {
        let shader_id = unsafe {
            gl::CreateShader(shader_type.to_raw())
        };
        if shader_id == 0 {
            return Err("Failed to CreateShader".to_string());
        }
        let program_c = CString::new(program.as_bytes()).unwrap();
        unsafe {
            gl::ShaderSource(shader_id, 1, &program_c.as_ptr(), ptr::null());
            gl::CompileShader(shader_id);
        }
        match check_for_errors(shader_id) {
            Ok(_) => {
                Ok(Shader {
                    shader_id,
                    shader_type
                })
            },
            Err(data) => {
                unsafe {
                    gl::DeleteShader(shader_id);
                }
                Err(data)
            }
        }
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.shader_id);
        }
    }
}
