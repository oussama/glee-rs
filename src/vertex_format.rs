use core::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VertexAttribute {
    name: String,
    attribute: Attribute,
}

use std::ops::DerefMut;
use std::ops::Deref;

impl DerefMut for VertexAttribute {
    fn deref_mut(&mut self) -> &mut Attribute {
        &mut self.attribute
    }
}

impl Deref for VertexAttribute {
    type Target = Attribute;

    fn deref(&self) -> &Attribute {
        &self.attribute
    }
}

impl VertexAttribute {
    pub fn f32<T: Into<String>>(name: T, len: u32) -> VertexAttribute {
        VertexAttribute {
            name: name.into(),
            attribute: Attribute::f32(len),
        }
    }

    pub fn u16<T: Into<String>>(name: T, len: u32) -> VertexAttribute {
        VertexAttribute {
            name: name.into(),
            attribute: Attribute::u16(len),
        }
    }

    pub fn u8<T: Into<String>>(name: T, len: u32) -> VertexAttribute {
        VertexAttribute {
            name: name.into(),
            attribute: Attribute::u8(len),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VertexFormat {
    pub attributes: Vec<VertexAttribute>,
}

impl VertexFormat {
    pub fn new(attributes: Vec<VertexAttribute>) -> VertexFormat {
        VertexFormat { attributes }
    }
}

impl VertexFormat {
    pub fn stride(&self) -> usize {
        let v: u32 = self.attributes.iter().map(|it| it.attribute.size).sum();
        (v as usize)
    }

    pub fn map(&mut self, program: &GlProgram) {
        let stride = self.stride();
        let mut offset = 0;

        for attribute in &self.attributes {
            if let Ok(location) = program.attribute_location(&attribute.name) {
                attribute.attribute.map(location, stride as _, offset);
                attribute.attribute.enable(location);
            } else {
                // println!("Attribute not found {}",attribute.name);
            }
            offset += attribute.attribute.size as u16;
        }
    }
}
