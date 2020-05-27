use std::fmt;

pub struct Pdf {
    inner: *mut pdflib_sys::PDF,
}

pub struct PdfError {
    code: i32,
    apiname: std::ffi::CString,
    message: std::ffi::CString,
}

impl fmt::Display for PdfError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.message.to_string_lossy(), self.code)
    }
}

impl fmt::Debug for PdfError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} error {}: {}",
            self.apiname.to_string_lossy(),
            self.code,
            self.message.to_string_lossy()
        )
    }
}

impl std::convert::From<std::ffi::NulError> for PdfError {
    fn from(_: std::ffi::NulError) -> PdfError {
        PdfError {
            code: -1,
            apiname: std::ffi::CString::new("").unwrap(),
            message: std::ffi::CString::new("null bytes in input string").unwrap(),
        }
    }
}

impl Drop for Pdf {
    fn drop(&mut self) {
        unsafe {
            pdflib_sys::PDF_delete(self.inner);
        }
    }
}

impl std::error::Error for PdfError {}

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
                let apiname = std::ffi::CStr::from_ptr(pdflib_sys::PDF_get_apiname($p)).to_owned();
                let message = std::ffi::CStr::from_ptr(pdflib_sys::PDF_get_errmsg($p)).to_owned();
                return Err(PdfError {
                    code,
                    apiname,
                    message,
                });
            });
        }
    };
}

mod general;
pub use general::*;
mod document_page;
pub use document_page::*;
mod font_text;
pub use font_text::*;
mod text_table;
pub use text_table::*;
mod object_fitting;
pub use object_fitting::*;
mod graphics;
pub use graphics::*;
mod color;
pub use color::*;
mod image_svg_template;
pub use image_svg_template::*;
mod pdi_pcos;
pub use pdi_pcos::*;
mod block_filling;
pub use block_filling::*;
mod interactive;
pub use interactive::*;
mod multimedia;
pub use multimedia::*;
mod document_interchange;
pub use document_interchange::*;
// mod document;
// pub use document::*;
// mod path;
// pub use path::*;
// mod paint;
// pub use paint::*;
// mod page;
// pub use page::*;
// mod text;
// pub use text::*;
// mod exception;
// pub use exception::*;
// mod option;
// pub use option::*;

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
