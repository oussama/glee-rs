use std::ffi::CString;
use errors::*;
use gl;

use core::*;

#[derive(Debug)]
pub struct Uniform;

impl Uniform {
    pub fn get_location(shader: &GlProgram, name: &str) -> Result<u32> {
        let c_name = CString::new(name).chain_err(|| "Error::NulError")?;
        unsafe {
            let location = gl::GetUniformLocation(shader.id(), c_name.as_ptr());
            if location > -1 && location < 17 {
                Ok(location as u32)
            } else {
                Err(ErrorKind::GlLocationNotFound(name.into()).into())
            }
        }
    }
}



pub trait SetUniform<T> {
    fn set(&self, _: u32, _: T) {
        unimplemented!();
    }
}

//impl<T> SetUniform<T> for Uniform {}

impl<'a> SetUniform<&'a [[f32; 4]; 4]> for Uniform {
    fn set(&self, location: u32, value: &[[f32; 4]; 4]) {
        unsafe {
            gl::UniformMatrix4fv(location as i32, 1, gl::FALSE, &value[0] as _);
        }
        check_gl_error();
    }
}

impl SetUniform<Mat3<f32>> for Uniform {
    fn set(&self, location: u32, value: Mat3<f32>) {
        unsafe {
            gl::UniformMatrix3fv(location as i32, 1, gl::FALSE, value.0);
        }
    }
}

impl SetUniform<Mat2<f32>> for Uniform {
    fn set(&self, location: u32, value: Mat2<f32>) {
        unsafe {
            gl::UniformMatrix2fv(location as i32, 1, gl::FALSE, value.0);
        }
    }
}

impl SetUniform<i32> for Uniform {
    fn set(&self, location: u32, value: i32) {
        unsafe {
            gl::Uniform1i(location as i32, value as _);
        }
    }
}

impl SetUniform<f32> for Uniform {
    fn set(&self, location: u32, value: f32) {
        unsafe {
            gl::Uniform1f(location as i32, value as _);
        }
    }
}

impl SetUniform<(f32, f32, f32, f32)> for Uniform {
    fn set(&self, location: u32, value: (f32, f32, f32, f32)) {
        unsafe {
            gl::Uniform4f(location as i32, value.0, value.1, value.2, value.3);
        }
    }
}

pub struct Mat4<T>(pub *const T);
pub struct Mat3<T>(pub *const T);
pub struct Mat2<T>(pub *const T);
