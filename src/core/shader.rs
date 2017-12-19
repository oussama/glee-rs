use std::ptr;
use std::ffi::CString;

use gl;
use gl::types::*;

use errors::*;

const NAME_SIZE: usize = 64;

pub enum GlProgramStatus {
    Validate = gl::VALIDATE_STATUS as _,
    Compile = gl::COMPILE_STATUS as _,
    Link = gl::LINK_STATUS as _,
}

#[derive(Debug, Clone, Copy)]
pub enum ShaderKind {
    Vertex = 35633,
    Fragment = 35632,
}

/// Shader Source

#[derive(Debug)]
pub struct GlShaderSource {
    kind: ShaderKind,
    source: CString,
}

impl GlShaderSource {
    pub fn from_bytes<T: Into<Vec<u8>>>(kind: ShaderKind, bytes: T) -> Result<GlShaderSource> {
        let source = CString::new(bytes).chain_err(|| "cannot parse shader source into c string")?;
        Ok(GlShaderSource { kind, source })
    }
}

impl GlShaderSource {
    pub fn compile(&self) -> Result<GlShader> {
        let mut handle = GlShader(0);
        unsafe {
            handle.0 = gl::CreateShader(self.kind as _);

            gl::ShaderSource(handle.0, 1, &self.source.as_ptr(), ptr::null());
            gl::CompileShader(handle.0);
        }
        // check_gl_program_error(handle,GlProgramStatus::Compile)?;

        Ok(handle)
    }
}


/// Shader

#[derive(Debug)]
pub struct GlShader(u32);

impl GlShader {
    pub fn id(&self) -> u32 {
        self.0
    }
}
impl Drop for GlShader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.0);
        }
    }
}


#[derive(Debug)]
pub struct GlProgram(u32);


impl GlProgram {
    pub fn new<'a>(shaders: &'a [&GlShader]) -> Result<GlProgram> {
        unsafe {
            let id = gl::CreateProgram();
            for shader in shaders {
                gl::AttachShader(id, shader.id());
            }
            let program = GlProgram(id);
            program.link()?;
            Ok(program)
        }
    }

    pub fn id(&self) -> u32 {
        self.0
    }

    fn link(&self) -> Result<()> {
        unsafe {
            gl::LinkProgram(self.0);
            check_gl_program_error(self, GlProgramStatus::Link)
        }
    }


    pub fn bind(&self) {
        unsafe {
            gl::UseProgram(self.0);
        }
    }

    pub fn attribute_location(&self, name: &str) -> Result<u32> {
        use std::ffi::CString;
        let c_name = CString::new(name).chain_err(|| "cstring parse failed")?;
        unsafe {
            let location = gl::GetAttribLocation(self.0, c_name.as_ptr() as _);
            if location == -1 {
                return Err(ErrorKind::GlAttributeLocationNotFound(name.into()).into());
            } else {
                return Ok(location as _);
            }
        }
    }

    pub fn uniform_location(&self, name: &str) -> Result<u32> {
        use std::ffi::CString;
        let c_name = CString::new(name).chain_err(|| "cstring parse failed")?;
        unsafe {
            let location = gl::GetUniformLocation(self.0, c_name.as_ptr() as _);
            if location == -1 {
                return Err(ErrorKind::GlAttributeLocationNotFound(name.into()).into());
            } else {
                return Ok(location as _);
            }
        }
    }

    pub fn parameter(&self, parameter: ProgramParameter) -> i32 {
        let mut res = 0;
        unsafe {
            gl::GetProgramiv(self.0, parameter as _, &mut res);
        }
        res
    }

    pub fn active_attributes_count(&self) -> usize {
        self.parameter(ProgramParameter::ActiveAttributes) as _
    }

    pub fn active_uniforms_count(&self) -> usize {
        self.parameter(ProgramParameter::ActiveUniforms) as _
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

    pub fn uniform_at(&self, index: u32) -> ActiveInfo {

        let mut name: Vec<u8> = Vec::with_capacity(NAME_SIZE);
        let mut size = 0i32;
        let mut len = 0i32;
        let mut kind = 0u32;

        unsafe {
            gl::GetActiveUniform(
                self.0,
                index as _,
                NAME_SIZE as _,
                &mut len,
                &mut size,
                &mut kind,
                name.as_mut_ptr() as _,
            );
            name.set_len(len as _);
        }

        //let c_name = unsafe { CString::from_raw(name[0..(len+1)].as_mut_ptr())};
        ActiveInfo::new(
            String::from_utf8(name).unwrap(),
            index as _,
            size as _,
            GlType::from(kind),
        )
    }

    pub fn attribute_at(&self, index: u32) -> ActiveInfo {

        let mut name: Vec<u8> = Vec::with_capacity(NAME_SIZE);
        let mut size = 0i32;
        let mut len = 0i32;
        let mut kind = 0u32;

        unsafe {
            gl::GetActiveAttrib(
                self.0,
                index as _,
                NAME_SIZE as _,
                &mut len,
                &mut size,
                &mut kind,
                name.as_mut_ptr() as _,
            );
            name.set_len(len as _);
        }

        //let c_name = unsafe { CString::from_raw(name[0..(len+1)].as_mut_ptr())};
        ActiveInfo::new(
            String::from_utf8(name).unwrap(),
            index,
            size as _,
            GlType::from(kind),
        )
    }


    pub fn attributes(&self) -> Vec<ActiveInfo> {
        let count = self.active_attributes_count();
        let mut attributes = Vec::with_capacity(count);
        for i in 0..count {
            attributes.push(self.attribute_at(i as u32));
        }
        attributes
    }

    pub fn uniforms(&self) -> Vec<ActiveInfo> {
        let count = self.active_uniforms_count();
        let mut uniforms = Vec::with_capacity(count);
        for i in 0..count {
            uniforms.push(self.uniform_at(i as u32));
        }
        uniforms
    }
}

impl Drop for GlProgram {
    fn drop(&mut self) {}
}


fn check_gl_program_error(handle: &GlProgram, status: GlProgramStatus) -> Result<()> {
    let mut success = gl::FALSE as gl::types::GLint;
    let mut info_log = Vec::with_capacity(512);
    unsafe {
        gl::GetProgramiv(handle.0, status as _, &mut success);
        if success != gl::TRUE as GLint {
            gl::GetProgramInfoLog(
                handle.0,
                512,
                ptr::null_mut(),
                info_log.as_mut_ptr() as *mut GLchar,
            );
            let err = String::from_utf8(info_log).chain_err(|| "failed to parse gl error to utf8")?;
            return Err(ErrorKind::GlProgramError(err).into());
        }
    }
    Ok(())
}


pub enum ProgramParameter {
    DeleteStatus = gl::DELETE_STATUS as _,
    ActiveAttributes = gl::ACTIVE_ATTRIBUTES as _,
    ActiveUniforms = gl::ACTIVE_UNIFORMS as _,
}

use core::*;

#[derive(Debug)]
pub struct ActiveInfo {
    pub name: String,
    pub size: u8,
    pub kind: GlType,
    pub location: u32,
}

impl ActiveInfo {
    pub fn new<T: Into<String>>(name: T, location: u32, size: u8, kind: GlType) -> ActiveInfo {
        ActiveInfo {
            name: name.into(),
            size,
            kind,
            location,
        }
    }
}
