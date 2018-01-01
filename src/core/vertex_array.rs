use webgl::*;
use std::rc::Rc;

#[derive(Debug)]
pub struct GLVertexArray(WebGLVertexArray);


/// commonly referred to as VAO
/// stores list of VBO & attributes mapping
impl GLVertexArray {
    
    pub fn new(ctx:&GLContext) -> GLVertexArray {
        GLVertexArray(ctx.create_vertex_array())
    }

    pub fn bind(&self) {
        
       // #[cfg(not(target_os = "emscripten"))]
        //unsafe {
           // gl::BindVertexArray(self.0);
        //}
    }

    pub fn unbind(&self) {
       // #[cfg(not(target_os = "emscripten"))]
       // unsafe {
           // gl::BindVertexArray(0);
       // }
    }
}

impl Drop for GLVertexArray {
    fn drop(&mut self) {
        //#[cfg(not(target_os = "emscripten"))]
       // unsafe {
           // gl::DeleteVertexArrays(1, &self.0);
       // }
    }
}
