use gl;
use std::os::raw::c_void;
use self::gl::types::{GLfloat};
use std::{mem, ptr};

pub enum BufferSectionLength {
    Length1,
    Length2,
    Length3,
    Length4
}

impl BufferSectionLength {
    pub fn to_raw(&self) -> u32 {
        match *self {
            BufferSectionLength::Length1 => 1,
            BufferSectionLength::Length2 => 2,
            BufferSectionLength::Length3 => 3,
            BufferSectionLength::Length4 => 4
        }
    }
}

pub struct Buffer {
    vbo_id: u32,
    vao_id: u32,
    ebo_id: u32,
    buffer_data: Vec<f32>,
    indices: Option<Vec<u32>>,
}

impl Buffer {
    pub fn new(buffer_data: Vec<f32>, buffer_section_lengths: Vec<BufferSectionLength>, indices: Option<Vec<u32>>) -> Buffer {
        let mut buffer = Buffer {
            vbo_id: 0,
            vao_id: 0,
            ebo_id: 0,
            buffer_data,
            indices
        };

        unsafe {
            gl::GenVertexArrays(1, &mut buffer.vao_id);
            gl::GenBuffers(1, &mut buffer.vbo_id);

            gl::BindVertexArray(buffer.vao_id);

            gl::BindBuffer(gl::ARRAY_BUFFER, buffer.vbo_id);
            gl::BufferData(gl::ARRAY_BUFFER, (buffer.buffer_data.len() * mem::size_of::<GLfloat>()) as isize, buffer.buffer_data.as_ptr() as *const c_void, gl::STATIC_DRAW);  // TODO: This shouldn't always be FLOAT. Or STATIC_DRAW


            match buffer.indices {
                Some(ref indices) => {
                    gl::GenBuffers(1, &mut buffer.ebo_id);
                    gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, buffer.ebo_id);
                    gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, (indices.len() * mem::size_of::<u32>()) as isize, indices.as_ptr() as *const c_void, gl::STATIC_DRAW);
                },
                None => ()
            };

            let total_vertex_size: u32 = buffer_section_lengths.iter().map(|item| item.to_raw()).sum();
            let float_size = mem::size_of::<GLfloat>();
            let mut last_section_offset = 0;
            for (id, section_length) in buffer_section_lengths.iter().enumerate() {
                gl::VertexAttribPointer(id as u32, section_length.to_raw() as i32, gl::FLOAT, gl::FALSE, (total_vertex_size * float_size as u32) as i32, (last_section_offset * float_size as u32) as *const c_void);
                gl::EnableVertexAttribArray(id as u32);
                last_section_offset += section_length.to_raw()
            }

            gl::BindVertexArray(0);
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
        };
        buffer
    }

    pub fn draw(&self) {
        unsafe {
            gl::BindVertexArray(self.vao_id);
            match self.indices {
                Some(ref indices) => gl::DrawElements(gl::TRIANGLES, indices.len() as i32, gl::UNSIGNED_INT, ptr::null()),
                None => gl::DrawArrays(gl::TRIANGLES, 0, self.buffer_data.len() as i32)
            };
        }
    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &mut self.vao_id);
            gl::DeleteBuffers(1, &mut self.vbo_id);
            match self.indices {
                Some(_) => gl::DeleteBuffers(1, &mut self.ebo_id),
                None => ()
            };
        }
    }
}
