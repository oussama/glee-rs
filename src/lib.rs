// error chain
#![recursion_limit = "1024"]
#[macro_use]
extern crate error_chain;

extern crate byteorder;
extern crate webgl;

extern crate bincode;
extern crate serde;
#[macro_use]
extern crate serde_derive;

extern crate arrayvec;

extern crate glenum;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}


pub mod core;
pub use self::core::*;

pub mod shader_program;
pub use self::shader_program::*;

pub mod vertex_attribute;
pub use self::vertex_attribute::*;

pub mod vertex_format;
pub use self::vertex_format::*;

pub use self::utils::*;

pub mod parsers;
pub use self::parsers::*;

pub use webgl::*;


pub mod errors {

    error_chain!{

        types {
            Error, ErrorKind, ResultExt, Result;
        }

        links {
            //Another(other_error::Error, other_error::ErrorKind) #[cfg(unix)];
        }

        foreign_links {
            Fmt(::std::fmt::Error);
            Io(::std::io::Error) #[cfg(unix)];
        }

        errors {

            GlProgramError(t: String) {
                description("GlProgramError")
                display("GlProgramError: '{}'", t)
            }

            GlError
            GlLocationNotFound(t: String) {
                description("GlLocationNotFound")
                display("GlLocationNotFound: '{}'", t)
            }

            GlAttributeLocationNotFound(t: String) {
                description("GlAttributeLocationNotFound")
                display("GlAttributeLocationNotFound: '{}'", t)
            }

            InvalidDDSFile

            InvalidToolchainName(t: String) {
                description("invalid toolchain name")
                display("invalid toolchain name: '{}'", t)
            }
        }
    }

}



pub mod utils {
    use std::mem::size_of;

    pub trait IntoBytes {
        fn into_bytes(self) -> Vec<u8>;
    }

    impl<T> IntoBytes for Vec<T> {
        fn into_bytes(self) -> Vec<u8> {
            let len = size_of::<T>() * self.len();
            unsafe {
                let mut slice = self.into_boxed_slice();

                let out = Vec::<u8>::from_raw_parts(Box::into_raw(slice) as _, len, len);
                out
            }
        }
    }


    pub trait AsBytes<'a> {
        fn as_bytes(self) -> &'a[u8];
    }

    impl<'a,T> AsBytes<'a> for &'a[T] {
        fn as_bytes(self) -> &'a[u8] {
            use std::slice;
            let len = size_of::<T>() * self.len();
            unsafe {
                slice::from_raw_parts(self.as_ptr() as _, len)
            }
        }
    }

}
