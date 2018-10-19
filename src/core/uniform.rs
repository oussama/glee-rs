use webgl::*;

pub trait SetUniform<T> {
    fn set_uniform(&self, _: WebGlUniformLocation, _: T) {
        unimplemented!();
    }
}

//impl<T> SetUniform<T> for Uniform {}

impl<'a> SetUniform<&'a mut [[f32; 4]; 4]> for WebGl2RenderingContext {
    fn set_uniform(&self, location: WebGlUniformLocation, value: &mut [[f32; 4]; 4]) {
        use std::mem;
        let array =
            unsafe { mem::transmute::<&mut [[f32; 4]; 4], &mut [f32; 16]>(value) as &mut [f32] };
        self.uniform_matrix4fv_with_f32_array_and_src_offset(Some(&location), false, array, 0);
    }
}

impl<'a> SetUniform<&'a mut [f32; 4]> for WebGl2RenderingContext {
    fn set_uniform(&self, location: WebGlUniformLocation, value: &mut [f32; 4]) {
        self.uniform4fv_with_f32_array(Some(&location), &mut *value);
    }
}

impl<'a> SetUniform<&'a mut [[f32; 3]; 3]> for WebGl2RenderingContext {
    fn set_uniform(&self, location: WebGlUniformLocation, value: &mut [[f32; 3]; 3]) {
        use std::mem;
        let array =
            unsafe { mem::transmute::<&mut [[f32; 3]; 3], &mut [f32; 9]>(value) as &mut [f32] };
        self.uniform_matrix3fv_with_f32_array(Some(&location), false, array);
    }
}

impl<'a> SetUniform<&'a mut [[f32; 2]; 2]> for WebGl2RenderingContext {
    fn set_uniform(&self, location: WebGlUniformLocation, value: &mut [[f32; 2]; 2]) {
        use std::mem;
        let array =
            unsafe { mem::transmute::<&mut [[f32; 2]; 2], &mut [f32; 4]>(value) as &mut [f32] };
        self.uniform_matrix2fv_with_f32_array(Some(&location), false, array);
    }
}

impl SetUniform<i32> for WebGl2RenderingContext {
    fn set_uniform(&self, location: WebGlUniformLocation, value: i32) {
        self.uniform1iv_with_i32_array(Some(&location), &mut [value]);
    }
}

impl SetUniform<f32> for WebGl2RenderingContext {
    fn set_uniform(&self, location: WebGlUniformLocation, value: f32) {
        self.uniform1fv_with_f32_array(Some(&location), &mut [value]);
    }
}

impl SetUniform<(f32, f32, f32, f32)> for WebGl2RenderingContext {
    fn set_uniform(&self, location: WebGlUniformLocation, value: (f32, f32, f32, f32)) {
        use std::mem;
        let mut array = unsafe { mem::transmute::<(f32, f32, f32, f32), [f32; 4]>(value) };
        self.uniform4fv_with_f32_array(Some(&location), &mut array);
    }
}
