use core::*;
use webgl::*;
use std::rc::Rc;
use std::ops::Deref;

#[derive(Debug)]
pub struct GLTexture {
    ctx:Rc<GLContext>,
    handle:WebGLTexture
}

impl GLTexture {
    pub fn new(ctx:&Rc<GLContext>) -> GLTexture {
        GLTexture {
            ctx:ctx.clone(),
            handle: ctx.create_texture()
        }
    }

    pub fn bind(&self) {
        self.ctx.bind_texture(&self.handle);
    }

    pub fn unbind(&self) {
        self.ctx.unbind_texture();
    }
}

impl Drop for GLTexture {
    fn drop(&mut self){
        self.ctx.delete_texture(&self.handle);
    }
}

impl Deref for GLTexture {
    type Target = WebGLTexture;
    fn deref(&self) -> &Self::Target {
        &self.handle
    }
}