use core::*;
use errors::*;

#[derive(Debug)]
pub struct ShaderProgram {
    vs_source: GlShaderSource,
    fs_source: GlShaderSource,
    uniforms: Vec<ActiveInfo>,
    attributes: Vec<ActiveInfo>,
    vs: GlShader,
    fs: GlShader,
    pub program: GlProgram,
}


impl ShaderProgram {
    pub fn from_bytes<T: Into<Vec<u8>>, U: Into<Vec<u8>>>(
        vs_bytes: T,
        fs_bytes: U,
    ) -> Result<ShaderProgram> {
        let vs_source = GlShaderSource::from_bytes(ShaderKind::Vertex, vs_bytes)?;
        let fs_source = GlShaderSource::from_bytes(ShaderKind::Fragment, fs_bytes)?;
        let vs = vs_source.compile()?;
        let fs = fs_source.compile()?;
        let program = GlProgram::new(&[&vs, &fs])?;
        program.bind();
        let attributes = program.attributes();
        let uniforms = program.uniforms();
        println!("Attributes {:?}", attributes);
        println!("Uniforms {:?}", uniforms);
        Ok(ShaderProgram {
            vs_source,
            fs_source,
            uniforms,
            attributes,
            program,
            fs,
            vs,
        })
    }

    /// set uniform value by name
    pub fn set<T>(&self, name: &str, value: T) -> Option<u32>
    where
        Uniform: SetUniform<T>,
    {
        let u = Uniform;
        /*
        use std::iter::Iterator;
        if let Some(location) = self.uniforms.iter()
            .find(|it|it.name==name)
            .map(|it| it.location) {
            u.set(location,value);
            return Some(location)
        }
        println!("Set Uniform failed {}",name);

        */
        let n: String = name.into();
        if let Ok(location) = self.program.uniform_location(&n) {
            u.set(location, value);
        } else {
            println!("not found");
        }
        None
    }

    pub fn bind(&self) {
        self.program.bind();
    }
}
