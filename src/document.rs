use super::{Pdf, PdfError};
use std::ffi;

/// # Document Functions
impl Pdf {
    pub fn begin_document(&mut self, filename: &str, optlist: &str) -> Result<(), PdfError> {
        unsafe {
            if pdflib_sys::PDF_begin_document(
                self.inner,
                ffi::CString::new(filename).unwrap().as_ptr(),
                0,
                ffi::CString::new(optlist).unwrap().as_ptr(),
            ) == -1
            {
                return Err(self.get_error());
            }
        }
        Ok(())
    }

    pub fn end_document(&mut self, optlist: &str) -> Result<(), PdfError> {
        unsafe_try_catch!(
            self.inner,
            pdflib_sys::PDF_end_document(self.inner, ffi::CString::new(optlist).unwrap().as_ptr(),)
        );
        Ok(())
    }
}
