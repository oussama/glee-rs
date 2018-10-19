use errors::*;

use webgl::*;

#[derive(Debug)]
pub struct TextureData {
    pub data: Vec<u8>,
    pub size: (u16, u16),
    pub compression: Option<TextureCompression>,
    pub format: PixelFormat,
}

impl TextureData {
    pub fn new(
        width: u16,
        height: u16,
        data: Vec<u8>,
        compression: Option<TextureCompression>,
        format: PixelFormat,
    ) -> TextureData {
        TextureData {
            size: (width, height),
            data,
            compression,
            format,
        }
    }

    #[inline]
    pub fn new_rgba(width: u16, height: u16, data: Vec<u8>) -> TextureData {
        TextureData::new(width, height, data, None, PixelFormat::Rgba)
    }

    #[inline]
    pub fn new_alpha(width: u16, height: u16, data: Vec<u8>) -> TextureData {
        TextureData::new(width, height, data, None, PixelFormat::Alpha)
    }

    pub fn upload(&mut self, ctx: &WebGl2RenderingContext) -> Result<()> {
        // copy data into buffer memory
        //  gl::PixelStorei(gl::UNPACK_SWAP_BYTES,0);
        //  gl::PixelStorei(gl::UNPACK_ALIGNMENT ,1);
        if let Some(compression) = self.compression {
            ctx.compressed_tex_image_2d_with_u8_array(
                TextureBindPoint::Texture2d as u32,
                0,
                compression as u32,
                self.size.0 as i32,
                self.size.1 as i32,
                0, // border
                &mut *self.data,
            );
        } else {
            ctx.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_u8_array_and_src_offset(
                TextureBindPoint::Texture2d as u32,
                0,
                self.format as i32,
                self.size.0 as i32,
                self.size.1 as i32,
                0, // border
                self.format as u32,
                DataType::U8 as u32,
                &mut *self.data,
                0, // src_offset
            );
            //          gl::GenerateMipmap(gl::TEXTURE_2D);
        }
        // gl::GenerateMipmap(gl::TEXTURE_2D);

        //check_gl_error()?;
        Ok(())
    }

    pub fn upload_at(&mut self, ctx: &WebGl2RenderingContext, x: u16, y: u16) -> Result<()> {
        ctx.pixel_storei(PixelStorageMode::UnpackAlignment as u32, 1);
        ctx.tex_sub_image_2d_with_i32_and_i32_and_u32_and_type_and_u8_array_and_src_offset(
            TextureBindPoint::Texture2d as u32,
            0,
            x as i32,
            y as i32,
            self.size.0 as i32,
            self.size.1 as i32,
            self.format as u32,
            DataType::U8 as u32,
            &mut *self.data,
            0,
        );
        //unsafe {
        //gl::GenerateMipmap(gl::TEXTURE_2D);
        //}
        //check_gl_error();
        Ok(())
    }
}
