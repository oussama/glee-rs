use webgl::*;





/// This is Vertex Attribute
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attribute {
    // components_count
    pub len: u32,
    pub kind: DataType,
    pub normalize: bool,
    // how many bytes are between each position attribute in the array
    pub size: u32,
}

/// Vertex Attribute Implementation
impl Attribute {
    pub fn f32(len: u32) -> Attribute {
        Attribute {
            len: len as _,
            kind: DataType::Float,
            normalize: false,
            size: (4 * len) as _,
        }
    }

    pub fn u16(len: u32) -> Attribute {
        Attribute {
            len: len as _,
            kind: DataType::U16,
            normalize: false,
            size: (2 * len) as _,
        }
    }

    pub fn u8(len: u32) -> Attribute {
        Attribute {
            len: len as _,
            kind: DataType::U8,
            normalize: false,
            size: (1 * len) as _,
        }
    }

    pub fn new(kind: DataType, len: u32, size: u32) -> Attribute {
        Attribute {
            len,
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
    }

    pub fn enable(&self,ctx:&GLContext, location: u32) {
        ctx.enable_vertex_attrib_array(location);
    }
}
