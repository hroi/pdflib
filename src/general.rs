use super::{Pdf, PdfError};
use std::ffi;

impl Pdf {
    pub(crate) fn get_error(&self) -> PdfError {
        unsafe {
            let code = pdflib_sys::PDF_get_errnum(self.inner);
            assert!(code != 0);
            let message = pdflib_sys::PDF_get_errmsg(self.inner);
            let message = ffi::CStr::from_ptr(message).to_owned();
            PdfError { code, message }
        }
    }
}

/// # General Functions
/// ## Exception Handling
impl Pdf {}
/// ## Unicode Conversion
impl Pdf {}
/// ## Global Options
impl Pdf {
    fn format_option(k: &str, v: &str) -> ffi::CString {
        ffi::CString::new(format!("{}={}", k, v)).unwrap()
    }

    pub fn set_option(&mut self, k: &str, v: &str) -> Result<(), PdfError> {
        unsafe_try_catch!(
            self.inner,
            pdflib_sys::PDF_set_option(self.inner, Self::format_option(k, v).as_c_str().as_ptr())
        );
        Ok(())
    }
}
