use core::*;
use errors::*;

// ,camera:&Camera,model:*const f32
pub fn render(shader: &GlShader, texture: &GlTexture, vao: &GlVertexArray) -> Result<()> {
    unsafe {
        use gl;
        gl::Enable(gl::BLEND);
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
    }

    texture.bind();
    check_gl_error().chain_err(|| "mesh/bind")?;

    vao.bind();
    //vbo.render();
    check_gl_error().chain_err(|| "mesh/render")?;

    Ok(())
}
