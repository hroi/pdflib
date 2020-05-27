use super::{Pdf, PdfError};
use std::ffi;

impl Pdf {
    pub fn set_info(&mut self, k: &str, v: &str) -> Result<(), PdfError> {
        let k = ffi::CString::new(k)?;
        let v = ffi::CString::new(v)?;
        unsafe_try_catch!(
            self.inner,
            pdflib_sys::PDF_set_info(self.inner, k.as_ptr(), v.as_ptr(),)
        );
        Ok(())
    }
}
