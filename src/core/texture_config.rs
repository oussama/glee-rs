use errors::*;
use webgl::*;

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

    pub fn apply(&self, ctx: &WebGl2RenderingContext) -> Result<()> {
        ctx.tex_parameteri(
            TextureBindPoint::Texture2d as u32,
            TextureParameter::TextureWrapS as u32,
            self.wrap.0 as i32,
        );
        ctx.tex_parameteri(
            TextureBindPoint::Texture2d as u32,
            TextureParameter::TextureWrapT as u32,
            self.wrap.1 as i32,
        );

        if self.wrap.0 == TextureWrap::ClampToEdge || self.wrap.1 == TextureWrap::ClampToEdge {
            ctx.tex_parameterf(
                TextureBindPoint::Texture2d as u32,
                TextureParameter::BorderColor as u32,
                self.border_color.0 as f32,
            );
        }

        ctx.tex_parameteri(
            TextureBindPoint::Texture2d as u32,
            TextureParameter::TextureMinFilter as u32,
            self.filtering.0 as i32,
        );
        ctx.tex_parameteri(
            TextureBindPoint::Texture2d as u32,
            TextureParameter::TextureMagFilter as u32,
            self.filtering.1 as i32,
        );

        Ok(())
    }
}
