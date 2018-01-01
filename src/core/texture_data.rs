use errors::*;
use core::*;

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

    pub fn upload(&self,ctx:GLContext) -> Result<()> {
        // copy data into buffer memory
            //  gl::PixelStorei(gl::UNPACK_SWAP_BYTES,0);
            //  gl::PixelStorei(gl::UNPACK_ALIGNMENT ,1);
            if let Some(compression) = self.compression {
                ctx.compressed_tex_image2d(
                        TextureBindPoint::Texture2d,
                        0,
                        compression,
                        self.size.0 as _,
                        self.size.1 as _,
                        &self.data,
                    );
            }else {
                ctx.tex_image2d(
                    TextureBindPoint::Texture2d,
                    0,
                    self.size.0 as _,
                    self.size.1 as _,
                    self.format as _,
                    DataType::U8,
                    &self.data,
                );
                    //          gl::GenerateMipmap(gl::TEXTURE_2D);
            }
            // gl::GenerateMipmap(gl::TEXTURE_2D);

        //check_gl_error()?;
        Ok(())
    }

    pub fn upload_at(&self,ctx:GLContext, x: u16, y: u16) -> Result<()> {
        ctx.pixel_storei(PixelStorageMode::UnpackAlignment,1);
        ctx.tex_sub_image2d(
            TextureBindPoint::Texture2d,
            0,
            x,
            y,
            self.size.0 as _,
            self.size.1 as _,
            self.format as _,
            DataType::U8,
            &self.data,
        );
        //unsafe {
            //gl::GenerateMipmap(gl::TEXTURE_2D);
        //}
        //check_gl_error();
        Ok(())
    }
}
