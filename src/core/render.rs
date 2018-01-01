use core::*;
use errors::*;
use webgl::*;

// ,camera:&Camera,model:*const f32
pub fn render(ctx:GLContext,shader: &WebGLShader, texture: &GLTexture, vao: &GLVertexArray) -> Result<()> {
    ctx.enable(Flag::Blend);
    ctx.blend_func(BlendMode::SrcAlpha,BlendMode::OneMinusSrcAlpha);

    texture.bind();
    //check_gl_error().chain_err(|| "mesh/bind")?;

    vao.bind();
    //vbo.render();
    //check_gl_error().chain_err(|| "mesh/render")?;

    Ok(())
}
