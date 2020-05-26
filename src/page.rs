use super::{Pdf, PdfError};
use std::ffi;

/// # Page Functions
impl Pdf {
    pub fn begin_page_ext(
        &mut self,
        width: f64,
        height: f64,
        optlist: &str,
    ) -> Result<(), PdfError> {
        unsafe_try_catch!(
            self.inner,
            pdflib_sys::PDF_begin_page_ext(
                self.inner,
                width,
                height,
                ffi::CString::new(optlist).unwrap().as_ptr(),
            )
        );
        Ok(())
    }

    pub fn end_page_ext(&mut self, optlist: &str) -> Result<(), PdfError> {
        unsafe {
            pdflib_sys::PDF_TRY!(self.inner, {
                pdflib_sys::PDF_end_page_ext(
                    self.inner,
                    ffi::CString::new(optlist).unwrap().as_ptr(),
                );
            });
            pdflib_sys::PDF_CATCH!(self.inner, { return Err(self.get_error()) });
        }
        Ok(())
    }
}
