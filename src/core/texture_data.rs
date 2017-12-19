use gl;
use errors::*;
use core::*;


#[derive(Debug, Clone, Copy)]
pub enum TextureCompression {
    DXT5 = 0x83F3,
}

#[derive(Debug, Clone, Copy)]
pub enum TextureFormat {
    RGBA = gl::RGBA as _,
    RED = gl::RED as _,
    ALPHA = gl::ALPHA as _,
}


#[derive(Debug)]
pub struct TextureData {
    pub data: Vec<u8>,
    pub size: (u16, u16),
    pub compression: Option<TextureCompression>,
    pub format: TextureFormat,
}

impl TextureData {
    pub fn new(
        width: u16,
        height: u16,
        data: Vec<u8>,
        compression: Option<TextureCompression>,
        format: TextureFormat,
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
        TextureData::new(width, height, data, None, TextureFormat::RGBA)
    }

    #[inline]
    pub fn new_alpha(width: u16, height: u16, data: Vec<u8>) -> TextureData {
        TextureData::new(width, height, data, None, TextureFormat::ALPHA)
    }

    pub fn upload(&self) -> Result<()> {
        // copy data into buffer memory
        unsafe {
            //  gl::PixelStorei(gl::UNPACK_SWAP_BYTES,0);
            //  gl::PixelStorei(gl::UNPACK_ALIGNMENT ,1);
            match self.compression {
                None => {
                    gl::TexImage2D(
                        gl::TEXTURE_2D,
                        0,
                        self.format as _,
                        self.size.0 as _,
                        self.size.1 as _,
                        0,
                        self.format as _,
                        gl::UNSIGNED_BYTE,
                        self.data.as_ptr() as _,
                    );
                    //          gl::GenerateMipmap(gl::TEXTURE_2D);
                }
                Some(kind) => {
                    println!("compressed");
                    gl::CompressedTexImage2D(
                        gl::TEXTURE_2D,
                        0,
                        kind as _,
                        self.size.0 as _,
                        self.size.1 as _,
                        0,
                        //gl::RGB,
                        (self.data.len() - 128) as _, //gl::UNSIGNED_BYTE as _,
                        &self.data[128] as *const u8 as _,
                    )
                }
            }
            // gl::GenerateMipmap(gl::TEXTURE_2D);
        }

        //check_gl_error()?;
        Ok(())
    }

    pub fn upload_at(&self, x: u16, y: u16) -> Result<()> {
        unsafe {
            gl::PixelStorei(gl::UNPACK_ALIGNMENT, 1);
            gl::TexSubImage2D(
                gl::TEXTURE_2D,
                0,
                x as _,
                y as _,
                self.size.0 as _,
                self.size.1 as _,
                self.format as _,
                gl::UNSIGNED_BYTE,
                self.data.as_ptr() as _,
            );
            //gl::GenerateMipmap(gl::TEXTURE_2D);
        }
        check_gl_error();
        Ok(())
    }
}
