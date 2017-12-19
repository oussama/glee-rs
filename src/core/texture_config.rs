use gl;
use errors::*;
use core::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TextureWrap {
    Repeat = gl::REPEAT as _,
    MirroredRepeat = gl::MIRRORED_REPEAT as _,
    EdgeClamp = gl::CLAMP_TO_EDGE as _,
    BorderClamp = gl::CLAMP_TO_BORDER as _,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TextureFiltering {
    Linear = gl::LINEAR as _,
    Nearest = gl::NEAREST as _,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TextureMipmapFiltering {
    Linear = gl::LINEAR as _,
    Nearest = gl::NEAREST as _,
    LinearLinear = gl::LINEAR_MIPMAP_LINEAR as _,
    NearestLinear = gl::NEAREST_MIPMAP_LINEAR as _,
    LinearNearest = gl::LINEAR_MIPMAP_NEAREST as _,
    NearestNearest = gl::NEAREST_MIPMAP_NEAREST as _,
}

#[derive(Debug)]
pub struct TextureConfig {
    pub wrap: (TextureWrap, TextureWrap),
    pub border_color: (f32, f32, f32, f32),
    pub filtering: (TextureMipmapFiltering, TextureFiltering),
}

impl TextureConfig {
    pub fn new() -> TextureConfig {
        TextureConfig {
            wrap: (TextureWrap::Repeat, TextureWrap::Repeat),
            border_color: (1.0, 1.0, 1.0, 1.0),
            filtering: (TextureMipmapFiltering::Linear, TextureFiltering::Linear),
        }
    }

    pub fn apply(&self) -> Result<()> {
        unsafe {
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, self.wrap.0 as _);
            check_gl_error().chain_err(|| "gl/configure/TexParameteri")?;

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, self.wrap.1 as _);
            check_gl_error().chain_err(|| "gl/configure/TexParameteri")?;

            if self.wrap.0 == TextureWrap::BorderClamp || self.wrap.1 == TextureWrap::BorderClamp {
                gl::TexParameterfv(
                    gl::TEXTURE_2D,
                    gl::TEXTURE_BORDER_COLOR,
                    &self.border_color.0 as _,
                );
            }
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_MIN_FILTER,
                self.filtering.0 as _,
            );
            check_gl_error().chain_err(|| "gl/configure/TexParameteri")?;

            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_MAG_FILTER,
                self.filtering.1 as _,
            );
        }
        Ok(())
    }
}
