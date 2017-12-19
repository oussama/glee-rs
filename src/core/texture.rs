use gl;
use core::*;

#[derive(Debug)]
pub struct GlTexture(u32);

impl GlTexture {
    pub fn new() -> GlTexture {
        let mut handle = GlTexture(0);
        unsafe {
            gl::GenTextures(1, &mut handle.0);
        }
        handle
    }

    pub fn id(&self) -> u32 {
        self.0
    }

    pub fn bind(&self) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, self.0);
        }
        check_gl_error().expect("texture/bind");
    }

    pub fn unbind(&self) {
        println!("tex.unbind");
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
    }
}
