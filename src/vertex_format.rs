use webgl::AttributeSize;
use core::*;
use std::rc::Rc;
use shader_program::ShaderProgram;

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
    pub fn f32<T: Into<String>>(name: T, size: AttributeSize) -> VertexAttribute {
        VertexAttribute {
            name: name.into(),
            attribute: Attribute::f32(size),
        }
    }

    pub fn u16<T: Into<String>>(name: T, size: AttributeSize) -> VertexAttribute {
        VertexAttribute {
            name: name.into(),
            attribute: Attribute::u16(size),
        }
    }

    pub fn u8<T: Into<String>>(name: T, size: AttributeSize) -> VertexAttribute {
        VertexAttribute {
            name: name.into(),
            attribute: Attribute::u8(size),
        }
    }
}

#[derive(Default,Debug, Clone, Serialize, Deserialize)]
pub struct VertexFormat {
    pub attributes: Vec<VertexAttribute>,
    pub mapped:bool,

}

impl VertexFormat {
    pub fn new(attributes: Vec<VertexAttribute>) -> VertexFormat {
        VertexFormat { attributes, mapped:false }
    }
}

impl VertexFormat {

    pub fn stride(&self) -> usize {
        self.attributes.iter().map(|it| it.attribute.len() ).sum()
    }

    pub fn map(&mut self,p: &Rc<ShaderProgram>) {
        if self.mapped {
            return;
        }
        let program = &p.program; 
        let stride = self.stride();
        let mut offset = 0;

        for attribute in &self.attributes {
            if let Some(location) = program.attribute_location(&attribute.name) {
                attribute.attribute.map(&program.ctx,location, stride as _, offset as _);
                attribute.attribute.enable(&program.ctx,location);
            } else {
                println!("Attribute not found {}",attribute.name);
            }
            offset += attribute.attribute.len();
        }

        self.mapped = true;
    }
}
