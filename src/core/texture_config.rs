use errors::*;
use webgl::{TextureMagFilter,TextureMinFilter,TextureWrap,GLContext,TextureParameter};


#[derive(Debug)]
pub struct TextureConfig {
    pub wrap: (TextureWrap, TextureWrap),
    pub border_color: (f32, f32, f32, f32),
    pub filtering: (TextureMinFilter, TextureMagFilter),
}

impl TextureConfig {
    pub fn new() -> TextureConfig {
        TextureConfig {
            wrap: (TextureWrap::Repeat, TextureWrap::Repeat),
            border_color: (1.0, 1.0, 1.0, 1.0),
            filtering: (TextureMinFilter::Linear, TextureMagFilter::Linear),
        }
    }

    pub fn apply(&self,ctx:&GLContext) -> Result<()> {

        ctx.tex_parameteri(TextureParameter::TextureWrapS, self.wrap.0 as _);
        ctx.tex_parameteri(TextureParameter::TextureWrapT, self.wrap.1 as _);

        if self.wrap.0 == TextureWrap::ClampToEdge || self.wrap.1 == TextureWrap::ClampToEdge {
            ctx.tex_parameterfv(TextureParameter::BorderColor, self.border_color.0 as _);
        }
        
        ctx.tex_parameteri(TextureParameter::TextureMinFilter, self.filtering.0 as _);
        ctx.tex_parameteri(TextureParameter::TextureMagFilter, self.filtering.1 as _);

        Ok(())
    }
}
