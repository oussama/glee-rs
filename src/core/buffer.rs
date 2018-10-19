use core::*;
use std::marker::PhantomData;

use webgl::{BufferKind, DrawMode, Primitives, WebGl2RenderingContext, WebGlBuffer};

use std::rc::Rc;

use utils::AsMutBytes;

#[derive(Debug)]
pub struct GLBuffer<T: TypeInto<BufferKind>> {
    ctx: Rc<WebGl2RenderingContext>,
    handle: WebGlBuffer,
    p: PhantomData<T>,
}

impl<T: TypeInto<BufferKind>> GLBuffer<T>
where
    T: TypeInto<BufferKind>,
{
    pub fn new(ctx: &Rc<WebGl2RenderingContext>) -> GLBuffer<T> {
        ctx.create_buffer()
            .map(|handle| GLBuffer {
                ctx: ctx.clone(),
                handle,
                p: PhantomData,
            })
            .expect("failed to created gl buffer")
    }

    pub fn bind(&mut self) {
        self.ctx.bind_buffer(T::into() as _, Some(&self.handle));
    }

    /// copy data into buffer memory
    /// requires a bound buffer
    pub fn upload<V>(&mut self, data: &mut [V], draw: DrawMode) {
        self.ctx
            .buffer_data_with_u8_array(T::into() as _, data.as_mut_bytes(), draw as _);
        //check_gl_error().expect("buffer/upload");
    }

    pub fn update<V>(&mut self, offset: u32, data: &mut [V]) {
        self.ctx.buffer_sub_data_with_i32_and_u8_array(
            T::into() as _,
            offset as _,
            data.as_mut_bytes(),
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
        self.ctx.draw_elements_with_i32(
            Primitives::Triangles as u32,
            len as i32,
            T::as_gl_enum() as u32,
            0,
        );
    }
}

impl GLVertexBuffer {
    pub fn draw(&self, len: usize) {
        self.ctx
            .draw_arrays(Primitives::Triangles as u32, 0, len as i32);
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
