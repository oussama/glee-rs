
use webgl::*;



pub trait SetUniform<T> {
    fn set_uniform(&self, _: WebGLUniformLocation, _: T) {
        unimplemented!();
    }
}

//impl<T> SetUniform<T> for Uniform {}

impl<'a> SetUniform<&'a [[f32; 4]; 4]> for GLContext {
    fn set_uniform(&self, location: WebGLUniformLocation, value: &[[f32; 4]; 4]) {
        self.uniform_matrix_4fv(location,value);
    }
}

impl<'a> SetUniform<&'a [f32; 4]> for GLContext {
    fn set_uniform(&self, location: WebGLUniformLocation, value: &[f32; 4]) {
        self.uniform_4fv(location,value);
    }
}


impl<'a> SetUniform<&'a [[f32; 3]; 3]> for GLContext {
    fn set_uniform(&self, location: WebGLUniformLocation, value:  &[[f32; 3]; 3]) {
        self.uniform_matrix_3fv(location,value);
    }
}

impl<'a> SetUniform<&'a [[f32; 2]; 2]> for GLContext {
    fn set_uniform(&self, location: WebGLUniformLocation, value: &[[f32; 2]; 2]) {
        self.uniform_matrix_2fv(location,value);
    }
}

impl SetUniform<i32> for GLContext {
    fn set_uniform(&self, location: WebGLUniformLocation, value: i32) {
        self.uniform_1i(location,value);
    }
}

impl SetUniform<f32> for GLContext {
    fn set_uniform(&self, location: WebGLUniformLocation, value: f32) {
        self.uniform_1f(location,value);
    }
}

impl SetUniform<(f32, f32, f32, f32)> for GLContext {
    fn set_uniform(&self, location: WebGLUniformLocation, value: (f32, f32, f32, f32)) {
        self.uniform_4f(location,value);
    }
}