use std::ops::Deref;
use std::rc::Rc;
use webgl::WebGlShader as GLShader;
use webgl::*;

use errors::*;

const NAME_SIZE: usize = 64;

/// Shader Source

#[derive(Debug)]
pub struct GLShaderSource {
    kind: ShaderKind,
    source: String,
}

impl GLShaderSource {
    pub fn from_bytes<T: Into<Vec<u8>>>(kind: ShaderKind, bytes: T) -> Result<GLShaderSource> {
        use std::str;
        let data: Vec<u8> = bytes.into();
        let source: &str =
            str::from_utf8(&data).chain_err(|| "failed to parse gl error to utf8")?;
        Ok(GLShaderSource {
            kind,
            source: source.into(),
        })
    }
}

impl GLShaderSource {
    pub fn compile(&self, ctx: &WebGl2RenderingContext) -> Result<GLShader> {
        // Create a vertex shader object
        let shader = ctx
            .create_shader(self.kind as u32)
            .expect("failed to create shader");
        // Attach vertex shader source code
        ctx.shader_source(&shader, &self.source);
        // Compile the vertex shader
        ctx.compile_shader(&shader);
        // check_gl_program_error(handle,GlProgramStatus::Compile)?;
        Ok(shader)
    }
}

#[derive(Debug)]
pub struct GLProgram {
    pub ctx: Rc<WebGl2RenderingContext>,
    handle: WebGlProgram,
}

impl Deref for GLProgram {
    type Target = WebGlProgram;
    fn deref(&self) -> &Self::Target {
        &self.handle
    }
}

impl GLProgram {
    pub fn new<'a>(
        ctx: &Rc<WebGl2RenderingContext>,
        shaders: &'a [&WebGlShader],
    ) -> Result<GLProgram> {
        // Create a shader program object to store
        // the combined shader program
        let shader_program = ctx.create_program().expect("failed to create shader");
        for shader in shaders {
            // Attach a vertex shader
            ctx.attach_shader(&shader_program, *shader);
        }
        // Link both the programs
        ctx.link_program(&shader_program);
        Ok(GLProgram {
            ctx: ctx.clone(),
            handle: shader_program,
        })
    }

    pub fn bind(&self) {
        self.ctx.use_program(Some(&self));
    }

    pub fn attribute_location(&self, name: &str) -> Option<u32> {
        let location = self.ctx.get_attrib_location(&self, name);
        if location < 0 {
            None
        } else {
            Some(location as u32)
        }
    }

    pub fn uniform_location(&self, name: &str) -> Option<WebGlUniformLocation> {
        self.ctx.get_uniform_location(&self, name)
    }

    pub fn parameter(&self, pname: ShaderParameter) -> i32 {
        self.ctx
            .get_program_parameter(&self, pname as u32)
            .as_f64()
            .expect("get_program_parameter") as i32
    }

    pub fn active_attributes_count(&self) -> usize {
        self.parameter(ShaderParameter::ActiveAttributes) as _
    }

    pub fn active_uniforms_count(&self) -> usize {
        self.parameter(ShaderParameter::ActiveUniforms) as _
    }

    /*
    pub fn uniform_at(&self,index:usize) -> ActiveInfo {
        use std::os::raw::c_char;
        let mut name:[c_char;NAME_SIZE] = [0;NAME_SIZE];
        let mut size = 0i32;
        let mut len = 0usize;
        let mut kind = 0u32;
    
        unsafe { gl::GetActiveUniform(self.0,index as _,NAME_SIZE as _,&mut (len as i32),&mut size,&mut kind,name.as_mut_ptr()) }
        let c_name = unsafe { CString::from_raw(name[0..len].as_mut_ptr())};
        ActiveInfo::new(c_name.into_string().unwrap() ,index as _,size as _,GlType::from(kind))
    }*/

    pub fn uniform_at(&self, location: u32) -> Option<WebGlActiveInfo> {
        self.ctx.get_active_uniform(&self, location)
    }

    pub fn attribute_at(&self, location: u32) -> Option<WebGlActiveInfo> {
        self.ctx.get_active_attrib(&self, location)
    }

    pub fn attributes(&self) -> Vec<WebGlActiveInfo> {
        let count = self.active_attributes_count();

        let mut attributes = Vec::with_capacity(count);
        for i in 0..count {
            if let Some(active_info) = self.attribute_at(i as u32) {
                attributes.push(active_info);
            } else {
                println!("attributes active_info not found at {}", i);
            }
        }
        attributes
    }

    pub fn uniforms(&self) -> Vec<WebGlActiveInfo> {
        let count = self.active_uniforms_count();
        println!("uniforms count {}", count);
        let mut uniforms = Vec::with_capacity(count);
        for i in 0..count {
            if let Some(active_info) = self.uniform_at(i as u32) {
                uniforms.push(active_info);
            } else {
                println!("uniforms active_info not found at {}", i);
            }
        }
        uniforms
    }
}
/*
fn check_gl_program_error(handle: &GLProgram, status: ShaderParameter) -> Result<()> {
    let mut success = gl::FALSE as gl::types::GLint;
    let mut info_log = Vec::with_capacity(512);
    unsafe {
        gl::GetProgramiv(handle.0, status as _, &mut success);
        if success != 0 {
            gl::GetProgramInfoLog(
                handle.0,
                512,
                ptr::null_mut(),
                info_log.as_mut_ptr() as *mut _,
            );
            let err = String::from_utf8(info_log).chain_err(|| "failed to parse gl error to utf8")?;
            return Err(ErrorKind::GlProgramError(err).into());
        }
    }
    Ok(())
}*/

#[derive(Debug)]
pub struct ActiveInfo {
    pub name: String,
    pub size: u8,
    pub kind: DataType,
    pub location: u32,
}

impl ActiveInfo {
    pub fn new<T: Into<String>>(name: T, location: u32, size: u8, kind: DataType) -> ActiveInfo {
        ActiveInfo {
            name: name.into(),
            size,
            kind,
            location,
        }
    }
}
