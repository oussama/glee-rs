use webgl::GLContext;
use core::{TextureData, TextureConfig, GLTexture};

use std::rc::Rc;

#[derive(Debug)]
pub struct TextureResource {
    pub handle: GLTexture,
    pub config: TextureConfig,
    pub data: TextureData,
}


impl TextureResource {
    pub fn new(ctx:&Rc<GLContext>, config: TextureConfig, data: TextureData) -> TextureResource {
        let handle = GLTexture::new(ctx);
        handle.bind();
        //check_gl_error();
        let _ = data.upload(ctx);
        //check_gl_error();
        let _ = config.apply(ctx);
        handle.unbind();
        TextureResource {
            handle,
            config,
            data,
        }
    }
}
