use webgl::*;

pub trait AsGlEnum {
    fn as_gl_enum() -> DataType;
}

impl AsGlEnum for u32 {
    fn as_gl_enum() -> DataType {
        DataType::U32
    }
}

impl AsGlEnum for u16 {
    fn as_gl_enum() -> DataType {
        DataType::U16
    }
}
