use core::*;
use std::marker::PhantomData;
use gl;
use std::mem;


#[derive(Debug, Clone, Copy)]
pub enum BufferKind {
    Vertex = gl::ARRAY_BUFFER as _,
    Index = gl::ELEMENT_ARRAY_BUFFER as _,
}

#[derive(Debug, Clone, Copy)]
pub enum DrawMode {
    Static = 35044,
    Dynamic = 35048,
    Stream = 35040,
}


#[derive(Debug)]
pub struct GlBuffer<T: TypeInto<BufferKind>>(u32, PhantomData<T>);


impl<T: TypeInto<BufferKind>> GlBuffer<T>
where
    T: TypeInto<BufferKind>,
{
    pub fn new() -> GlBuffer<T> {
        let mut buffer = GlBuffer(0, PhantomData);
        unsafe {
            gl::GenBuffers(1, &mut buffer.0);
        }
        buffer
    }

    pub fn bind(&mut self) {
        unsafe {
            gl::BindBuffer(T::into() as _, self.0);
        }
    }

    /// copy data into buffer memory
    /// requires a bound buffer
    pub fn upload<V>(&mut self, data: &Vec<V>, draw: DrawMode) {
        let size = data.len() * mem::size_of::<V>();
        //println!("Buffer::upload.size {}",size);
        unsafe {
            gl::BufferData(
                T::into() as _,
                size as _,
                data.as_ptr() as *const _,
                draw as u32,
            );
            check_gl_error().expect("buffer/upload");
        }
    }
}

#[derive(Debug)]
pub struct IndexBufferKind;

#[derive(Debug)]
pub struct VertexBufferKind;

pub trait TypeInto<T> {
    fn into() -> T;
}

impl TypeInto<BufferKind> for VertexBufferKind {
    fn into() -> BufferKind {
        BufferKind::Vertex
    }
}

impl TypeInto<BufferKind> for IndexBufferKind {
    fn into() -> BufferKind {
        BufferKind::Index
    }
}


pub type GlVertexBuffer = GlBuffer<VertexBufferKind>;
pub type GlIndexBuffer = GlBuffer<IndexBufferKind>;

impl GlIndexBuffer {
    pub fn draw<T: AsGlEnum>(&self, len: usize) {
        unsafe {
            gl::DrawElements(gl::TRIANGLES, len as _, T::as_gl_enum(), 0 as _);
        }
    }
}

impl GlVertexBuffer {
    pub fn draw(&self, len: u32) {
        unsafe {
            gl::DrawArrays(gl::TRIANGLES, 0, len as _);
        }
    }
}

impl<T: TypeInto<BufferKind>> Drop for GlBuffer<T>
where
    T: TypeInto<BufferKind>,
{
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.0);
        }
    }
}
