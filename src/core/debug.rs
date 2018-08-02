
//#[cfg(build = "debug")]
/*
pub fn check_gl_error() -> Result<()> {
    unsafe {
        use gl;
        let err = gl::GetError();
        if err != 0 {
            println!("gl_err {}", err);
            return Err(ErrorKind::GlLocationNotFound("err".into()).into());
        }
    }
    Ok(())
}*/
/*
#[cfg(not(build = "debug"))]
pub fn check_gl_error() -> Result<()> {
    Ok(())
}
*/
