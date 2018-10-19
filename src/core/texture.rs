use std::ops::Deref;
use std::rc::Rc;
use webgl::*;

#[derive(Debug)]
pub struct GLTexture {
    ctx: Rc<WebGl2RenderingContext>,
    handle: WebGlTexture,
}

impl GLTexture {
    pub fn new(ctx: &Rc<WebGl2RenderingContext>) -> GLTexture {
        GLTexture {
            ctx: ctx.clone(),
            handle: ctx.create_texture().expect("failed to create texture"),
        }
    }

    pub fn bind(&self) {
        self.ctx
            .bind_texture(TextureBindPoint::Texture2d as u32, Some(&self.handle));
    }

    pub fn unbind(&self) {
        self.ctx
            .bind_texture(TextureBindPoint::Texture2d as u32, None);
    }
}

impl Drop for GLTexture {
    fn drop(&mut self) {
        self.ctx.delete_texture(Some(&self.handle));
    }
}

impl Deref for GLTexture {
    type Target = WebGlTexture;
    fn deref(&self) -> &Self::Target {
        &self.handle
    }
}
