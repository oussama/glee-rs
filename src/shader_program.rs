use core::*;
use errors::*;
use webgl::*;
use std::rc::Rc;

#[derive(Debug)]
pub struct ShaderProgram {
    vs_source: GLShaderSource,
    fs_source: GLShaderSource,
    uniforms: Vec<WebGLActiveInfo>,
    uniforms_locations:Vec<WebGLUniformLocation>,
    attributes: Vec<WebGLActiveInfo>,
    vs: WebGLShader,
    fs: WebGLShader,
    pub program: GLProgram,
}


impl ShaderProgram {
    pub fn from_bytes<T: Into<Vec<u8>>, U: Into<Vec<u8>>>(
        ctx:&Rc<GLContext>,
        vs_bytes: T,
        fs_bytes: U,
    ) -> Result<ShaderProgram> {
        let vs_source = GLShaderSource::from_bytes(ShaderKind::Vertex, vs_bytes)?;
        let fs_source = GLShaderSource::from_bytes(ShaderKind::Fragment, fs_bytes)?;
        let vs = vs_source.compile(&ctx)?;
        let fs = fs_source.compile(ctx)?;
        let program = GLProgram::new(&ctx,&[&vs, &fs])?;
        program.bind();
        let attributes = program.attributes();
        let uniforms = program.uniforms();
        Ok(ShaderProgram {
            uniforms_locations:Vec::new(),
            vs_source,
            fs_source,
            uniforms,
            attributes,
            program,
            fs,
            vs,
        })
    }

    pub fn prepare_uniform(&mut self,name:&str) {
        if let Some(location) = self.program.uniform_location(name) {
            self.uniforms_locations.push(location.clone());
        }
    }
    /// set uniform value by name
    pub fn set<T>(&self, name: &str, value: T) -> Option<u32>
    where
        GLContext: SetUniform<T>,
    {
//        let u = Uniform;
        
        use std::iter::Iterator;
        if let Some(location) = self.uniforms_locations.iter()
            .find(|it|it.name==name) {
            self.program.ctx.set_uniform(location.clone(), value);
            return None;
        }
        /*
        println!("Set Uniform failed {}",name);

        let n: String = name.into();
        if let Some(location) = self.program.uniform_location(&n) {
            self.uniforms_locations.push(location.clone());
            self.program.ctx.set_uniform(location.clone(), value);
        }*/
        None
    }

    pub fn bind(&self) {
        self.program.bind();
    }
}
