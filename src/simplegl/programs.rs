use gl;
use gl::types::{GLchar, GLint};
use simplegl::shaders::Shader;
use std::str;

fn check_for_errors(program: u32) -> Result<(), String> {
    let mut success = gl::FALSE as GLint;
    unsafe {
        gl::GetProgramiv(program, gl::LINK_STATUS, &mut success);
    };
    if success != gl::TRUE as GLint {
        let mut info_log = Vec::with_capacity(512);
        unsafe {
            let mut final_length = 0;
            gl::GetProgramInfoLog(program, 512, &mut final_length, info_log.as_mut_ptr() as *mut GLchar);
            info_log.set_len(final_length as usize);
        };
        Err(str::from_utf8(&info_log).unwrap().to_string())
    } else {
        Ok(())
    }
}

pub struct Program {
    program_id: u32
}

impl Program {
    pub fn new() -> Result<Program, String> {
        let program_id = unsafe {
            gl::CreateProgram()
        };
        if program_id == 0 {
            Err("Failed to create Program".to_string())
        } else {
            Ok(Program {
                program_id
            })
        }
    }

    pub fn link_shaders(&mut self, shaders: &[Shader]) -> Result<(), String> {
        for shader in shaders {
            unsafe {
                gl::AttachShader(self.program_id, shader.shader_id);
            }
        }
        unsafe {
            gl::LinkProgram(self.program_id);
            check_for_errors(self.program_id)
        }
    }

    pub fn program_id(&self) -> u32 {
        self.program_id
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.program_id);
        }
    }
}
