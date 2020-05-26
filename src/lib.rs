use std::fmt;

pub struct Pdf {
    inner: *mut pdflib_sys::PDF,
}

pub struct PdfError {
    pub code: i32,
    pub message: String,
}

impl fmt::Display for PdfError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.message, self.code)
    }
}

impl fmt::Debug for PdfError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "pdf error {}: {}", self.code, self.message)
    }
}

impl<'a> std::error::Error for PdfError {}

impl Pdf {
    pub fn new() -> Pdf {
        unsafe {
            let p = pdflib_sys::PDF_new();
            assert!(!p.is_null());
            pdflib_sys::PDF_set_option(p, "errorpolicy=return\0".as_ptr() as _);
            let mut ret = Pdf { inner: p };
            ret.set_option("errorpolicy", "return").unwrap();
            ret
        }
    }
}

macro_rules! unsafe_try_catch {
    ($p:expr, $body:expr) => {
        unsafe {
            pdflib_sys::PDF_TRY!($p, $body);
            pdflib_sys::PDF_CATCH!($p, {
                let code = pdflib_sys::PDF_get_errnum($p);
                assert!(code != 0);
                let message = pdflib_sys::PDF_get_errmsg($p);
                let message = std::ffi::CStr::from_ptr(message)
                    .to_owned()
                    .into_string()
                    .unwrap();
                return Err(PdfError { code, message });
            });
        }
    };
}

mod document;
pub use document::*;
mod path;
pub use path::*;
mod paint;
pub use paint::*;
mod color;
pub use color::*;
mod image;
pub use image::*;
mod graphics;
pub use graphics::*;
mod page;
pub use page::*;
mod text;
pub use text::*;
mod exception;
pub use exception::*;
mod option;
pub use option::*;
mod interchange;
pub use interchange::*;

impl Drop for Pdf {
    fn drop(&mut self) {
        unsafe {
            pdflib_sys::PDF_delete(self.inner);
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn new() {
        let _ = Pdf::new();
    }

    #[test]
    fn set_option() {
        let mut pdf = Pdf::new();
        pdf.set_option("filenamehandling", "unicode").unwrap();
    }

    #[test]
    #[should_panic]
    fn set_option_invalid() {
        let mut pdf = Pdf::new();
        pdf.set_option("invalid", "option").unwrap();
    }
}