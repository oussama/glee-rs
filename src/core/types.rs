use gl;
use gl::types::GLenum;

pub trait AsGlEnum {
    fn as_gl_enum() -> GLenum;
}

impl AsGlEnum for u32 {
    fn as_gl_enum() -> GLenum {
        gl::UNSIGNED_INT
    }
}

impl AsGlEnum for u16 {
    fn as_gl_enum() -> GLenum {
        gl::UNSIGNED_SHORT
    }
}
