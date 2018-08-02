use core::*;
use std::marker::PhantomData;

use webgl::{BufferKind,DrawMode,GLContext,WebGLBuffer,Primitives};

use std::rc::Rc;

use utils::AsBytes;


#[derive(Debug)]
pub struct GLBuffer<T: TypeInto<BufferKind>>{
    ctx:Rc<GLContext>,
    handle:WebGLBuffer,
    p:PhantomData<T>,
}


impl<T: TypeInto<BufferKind>> GLBuffer<T>
where
    T: TypeInto<BufferKind>,
{
    pub fn new(ctx:&Rc<GLContext>) -> GLBuffer<T> {
        let handle = ctx.create_buffer();
        GLBuffer{ctx:ctx.clone(),handle,p:PhantomData}
    }

    pub fn bind(&mut self) {
        self.ctx.bind_buffer(T::into() as _, &self.handle);
    }

    /// copy data into buffer memory
    /// requires a bound buffer
    pub fn upload<V>(&mut self, data: &[V], draw: DrawMode) {
        self.ctx.buffer_data(
                T::into() as _,
                &data.as_bytes(),
                draw,
            );
            //check_gl_error().expect("buffer/upload");
    }

    pub fn update<V>(&mut self,offset:u32, data: &[V]) {
        self.ctx.buffer_sub_data(
                T::into() as _,
                offset,
                &data.as_bytes(),
            );
            //check_gl_error().expect("buffer/upload");
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
        BufferKind::Array
    }
}

impl TypeInto<BufferKind> for IndexBufferKind {
    fn into() -> BufferKind {
        BufferKind::ElementArray
    }
}


pub type GLVertexBuffer = GLBuffer<VertexBufferKind>;
pub type GLIndexBuffer = GLBuffer<IndexBufferKind>;

impl GLIndexBuffer {
    pub fn draw<T: AsGlEnum>(&self, len: usize) {
        self.ctx.draw_elements(Primitives::Triangles,len,T::as_gl_enum(),0);
    }
}

impl GLVertexBuffer {
    pub fn draw(&self, len: usize) {
        self.ctx.draw_arrays(Primitives::Triangles,len);
    }
}

impl<T: TypeInto<BufferKind>> Drop for GLBuffer<T>
where
    T: TypeInto<BufferKind>,
{
    fn drop(&mut self) {
      //  self.delete_buffer(self.0);
    }
}
