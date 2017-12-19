mod shader;
pub use self::shader::*;


mod uniform;
pub use self::uniform::*;

mod texture;
pub use self::texture::*;

mod texture_config;
pub use self::texture_config::*;

mod texture_data;
pub use self::texture_data::*;

mod render;
pub use self::render::*;

mod debug;
pub use self::debug::*;

mod buffer;
pub use self::buffer::*;

mod vertex_array;
pub use self::vertex_array::*;

mod attribute;
pub use self::attribute::*;

mod types;
pub use self::types::*;
use gl;

#[derive(Debug)]
pub enum GlType {
    Float = gl::FLOAT as _,
    Float2 = gl::FLOAT_VEC2 as _,
    Float3 = gl::FLOAT_VEC3 as _,
    Float4 = gl::FLOAT_VEC4 as _,
    Int = gl::INT as _,
    Int2 = gl::INT_VEC2 as _,
    Int3 = gl::INT_VEC3 as _,
    Int4 = gl::INT_VEC4 as _,
    Bool = gl::BOOL as _,
    Bool2 = gl::BOOL_VEC2 as _,
    Bool3 = gl::BOOL_VEC3 as _,
    Bool4 = gl::BOOL_VEC4 as _,
    FloatMat2 = gl::FLOAT_MAT2 as _,
    FloatMat3 = gl::FLOAT_MAT3 as _,
    FloatMat4 = gl::FLOAT_MAT4 as _,
    Sampler2d = gl::SAMPLER_2D as _,
    SamplerCube = gl::SAMPLER_CUBE as _,
}


impl From<u32> for GlType {
    fn from(src: u32) -> GlType {
        match src {
            gl::FLOAT => GlType::Float,
            gl::FLOAT_VEC2 => GlType::Float2,
            gl::FLOAT_VEC3 => GlType::Float3,
            gl::FLOAT_VEC4 => GlType::Float4,
            gl::INT => GlType::Int,
            gl::INT_VEC2 => GlType::Int2,
            gl::INT_VEC3 => GlType::Int3,
            gl::INT_VEC4 => GlType::Int4,
            gl::BOOL => GlType::Bool,
            gl::BOOL_VEC2 => GlType::Bool2,
            gl::BOOL_VEC3 => GlType::Bool3,
            gl::BOOL_VEC4 => GlType::Bool4,
            gl::FLOAT_MAT2 => GlType::FloatMat2,
            gl::FLOAT_MAT3 => GlType::FloatMat3,
            gl::FLOAT_MAT4 => GlType::FloatMat4,
            gl::SAMPLER_2D => GlType::Sampler2d,
            gl::SAMPLER_CUBE => GlType::SamplerCube,
            _ => unimplemented!(),
        }
    }
}
