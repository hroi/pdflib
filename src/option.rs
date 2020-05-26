use super::{Pdf, PdfError};
use std::ffi;

fn format_option(k: &str, v: &str) -> ffi::CString {
    ffi::CString::new(format!("{}={}", k, v)).unwrap()
}

/// # Global Options
impl Pdf {
    pub fn set_option(&mut self, k: &str, v: &str) -> Result<(), PdfError> {
        unsafe_try_catch!(
            self.inner,
            pdflib_sys::PDF_set_option(self.inner, format_option(k, v).as_c_str().as_ptr())
        );
        Ok(())
    }
}
