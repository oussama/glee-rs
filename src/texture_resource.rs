use core::{GLTexture, TextureConfig, TextureData};
use webgl::WebGl2RenderingContext;

use std::rc::Rc;

#[derive(Debug)]
pub struct TextureResource {
    pub handle: GLTexture,
    pub config: TextureConfig,
    pub data: TextureData,
}

impl TextureResource {
    pub fn new(
        ctx: &Rc<WebGl2RenderingContext>,
        config: TextureConfig,
        mut data: TextureData,
    ) -> TextureResource {
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
