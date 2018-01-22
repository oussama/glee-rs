use webgl::*;





/// This is Vertex Attribute
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attribute {
    
    pub kind: DataType,
    pub normalize: bool,
    // components_count
    pub size: AttributeSize,
    
}

/// Vertex Attribute Implementation
impl Attribute {

    pub fn len(&self) -> usize{
        match self.kind {
            DataType::Float | DataType::I32 | DataType::U32 => 4 * self.size as usize,
            DataType::I16 | DataType::U16 => 2 * self.size as usize,
            DataType::I8 | DataType::U8 => self.size as usize,
        }
    }

    pub fn f32(size:AttributeSize) -> Attribute {
        Attribute {
           // len: len as _,
            kind: DataType::Float,
            normalize: false,
            size,
        }
    }

    pub fn u16(size: AttributeSize) -> Attribute {
        Attribute {
          //  len: len as _,
            kind: DataType::U16,
            normalize: false,
            size,
        }
    }

    pub fn u8(size: AttributeSize) -> Attribute {
        Attribute {
            //len: len as _,
            kind: DataType::U8,
            normalize: false,
            size,
        }
    }

    pub fn new(kind: DataType, size: AttributeSize) -> Attribute {
        Attribute {
            //len,
            kind,
            normalize: false,
            size,
        }
    }

    /// needs a bound VBO
    /// It is important to know that this function will
    /// store not only the stride and the offset,
    /// but also the VBO that is currently bound to GL_ARRAY_BUFFER.
    /// That means that you don't have to explicitly bind the correct
    /// VBO when the actual drawing functions are called.
    /// This also implies that you can use a different VBO for each attribute.
    pub fn map(&self,ctx:&GLContext,  location: u32, stride: u16, offset: u16) {
        ctx.vertex_attrib_pointer(location,self.size,self.kind,self.normalize,stride as _,offset as _);
        //ctx.log(format!("location {} stride {} offset {}", location,stride,offset ));
    }

    pub fn enable(&self,ctx:&GLContext, location: u32) {
        ctx.enable_vertex_attrib_array(location);
    }
}
