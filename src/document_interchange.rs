use super::{Pdf, PdfError};
use std::ffi;

impl Pdf {
    pub fn set_info(&mut self, k: &str, v: &str) -> Result<(), PdfError> {
        unsafe_try_catch!(
            self.inner,
            pdflib_sys::PDF_set_info(
                self.inner,
                ffi::CString::new(k).unwrap().as_ptr(),
                ffi::CString::new(v).unwrap().as_ptr(),
            )
        );
        Ok(())
    }
}
