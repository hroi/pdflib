use super::{Pdf, PdfError};
use std::ffi;

/// # Text and Table Formatting
/// ## Single-Line Text with Textlines
impl Pdf {
    pub fn fit_textline(
        &mut self,
        text: &str,
        x: f64,
        y: f64,
        optlist: &str,
    ) -> Result<(), PdfError> {
        unsafe_try_catch!(
            self.inner,
            pdflib_sys::PDF_fit_textline(
                self.inner,
                ffi::CString::new(text).unwrap().as_ptr(),
                0,
                x,
                y,
                ffi::CString::new(optlist).unwrap().as_ptr(),
            )
        );
        Ok(())
    }
}
