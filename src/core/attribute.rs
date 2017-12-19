use gl;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum GlBool {
    True = gl::TRUE as _,
    False = gl::FALSE as _,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum AttributeKind {
    Float = gl::FLOAT as _,
    HalfFloat = gl::HALF_FLOAT as _,
    U8 = gl::UNSIGNED_BYTE as _,
    U16 = gl::UNSIGNED_SHORT as _,
}


/// This is Vertex Attribute
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attribute {
    // components_count
    pub len: u32,
    pub kind: AttributeKind,
    pub normalize: GlBool,
    // how many bytes are between each position attribute in the array
    pub size: u32,
}

/// Vertex Attribute Implementation
impl Attribute {
    pub fn f32(len: u32) -> Attribute {
        Attribute {
            len: len as _,
            kind: AttributeKind::Float,
            normalize: GlBool::False,
            size: (4 * len) as _,
        }
    }

    pub fn u16(len: u32) -> Attribute {
        Attribute {
            len: len as _,
            kind: AttributeKind::U16,
            normalize: GlBool::False,
            size: (2 * len) as _,
        }
    }

    pub fn u8(len: u32) -> Attribute {
        Attribute {
            len: len as _,
            kind: AttributeKind::U8,
            normalize: GlBool::False,
            size: (1 * len) as _,
        }
    }

    pub fn new(kind: AttributeKind, len: u32, size: u32) -> Attribute {
        Attribute {
            len,
            kind,
            normalize: GlBool::False,
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
    pub fn map(&self, location: u32, stride: u16, offset: u16) {
        unsafe {
            gl::VertexAttribPointer(
                location,
                self.len as _,
                self.kind as u32,
                self.normalize as u8,
                stride as _,
                offset as _,
            );
        }
    }

    pub fn enable(&self, location: u32) {
        unsafe {
            gl::EnableVertexAttribArray(location);
        }
    }
}
