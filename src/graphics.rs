use super::{Pdf, PdfError};

/// # Graphics State
impl Pdf {
    pub fn setlinewidth(&mut self, width: f64) -> Result<(), PdfError> {
        unsafe_try_catch!(self.inner, pdflib_sys::PDF_setlinewidth(self.inner, width));
        Ok(())
    }
}
