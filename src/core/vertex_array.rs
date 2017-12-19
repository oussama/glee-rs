use gl;

#[derive(Debug)]
pub struct GlVertexArray(u32);


/// commonly referred to as VAO
/// stores list of VBO & attributes mapping
impl GlVertexArray {
    pub fn new() -> GlVertexArray {
        let mut vao = GlVertexArray(0);

        #[cfg(not(target_os = "emscripten"))]
        unsafe {
            gl::GenVertexArrays(1, &mut vao.0);
        }
        vao
    }

    pub fn bind(&self) {
        #[cfg(not(target_os = "emscripten"))]
        unsafe {
            gl::BindVertexArray(self.0);
        }
    }

    pub fn unbind(&self) {
        #[cfg(not(target_os = "emscripten"))]
        unsafe {
            gl::BindVertexArray(0);
        }
    }
}

impl Drop for GlVertexArray {
    fn drop(&mut self) {
        #[cfg(not(target_os = "emscripten"))]
        unsafe {
            gl::DeleteVertexArrays(1, &self.0);
        }
    }
}
