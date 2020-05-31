use std::fmt;

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

mod consts;
pub use consts::*;

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

pub struct Pdf {
    inner: *mut pdflib_sys::PDF,
}

impl Pdf {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Pdf {
        let p = unsafe {
            let p = pdflib_sys::PDF_new();
            assert!(
                !p.is_null(),
                "Couldn't create PDFlib object (out of memory)!\n"
            );
            p
        };
        let mut ret = Pdf { inner: p };
        ret.set_option("errorpolicy=return").unwrap();
        ret.set_option("stringformat=utf8").unwrap();
        ret
    }
}

impl Drop for Pdf {
    fn drop(&mut self) {
        unsafe {
            pdflib_sys::PDF_delete(self.inner);
        }
    }
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
    fn from(err: std::ffi::NulError) -> PdfError {
        PdfError {
            code: -1,
            apiname: std::ffi::CString::new("std::ffi::CString").unwrap(),
            message: std::ffi::CString::new(err.to_string()).unwrap(),
        }
    }
}

impl std::convert::From<std::ffi::IntoStringError> for PdfError {
    fn from(err: std::ffi::IntoStringError) -> PdfError {
        PdfError {
            code: -1,
            apiname: std::ffi::CString::new("std::ffi::CString").unwrap(),
            message: std::ffi::CString::new(err.to_string()).unwrap(),
        }
    }
}

impl std::error::Error for PdfError {}

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
        pdf.set_option("filenamehandling=unicode").unwrap();
    }

    #[test]
    #[should_panic]
    fn nullbyte_arg() {
        let mut pdf = Pdf::new();
        pdf.set_option("filename\0handling=contains null").unwrap();
    }

    #[test]
    #[should_panic]
    fn set_option_invalid() {
        let mut pdf = Pdf::new();
        pdf.set_option("invalid=option").unwrap();
    }
}
