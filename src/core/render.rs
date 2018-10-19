use core::*;
use errors::*;
use webgl::*;

// ,camera:&Camera,model:*const f32
pub fn render(
    ctx: WebGl2RenderingContext,
    _shader: &WebGlShader,
    texture: &GLTexture,
    vao: &GLVertexArray,
) -> Result<()> {
    ctx.enable(Flag::Blend as u32);
    ctx.blend_func(
        BlendMode::SrcAlpha as u32,
        BlendMode::OneMinusSrcAlpha as u32,
    );

    texture.bind();
    //check_gl_error().chain_err(|| "mesh/bind")?;

    vao.bind();
    //vbo.render();
    //check_gl_error().chain_err(|| "mesh/render")?;

    Ok(())
}
