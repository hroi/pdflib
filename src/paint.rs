use super::{Pdf, PdfError};

/// # Painting and Clipping
impl Pdf {
    pub fn stroke(&mut self) -> Result<(), PdfError> {
        unsafe_try_catch!(self.inner, pdflib_sys::PDF_stroke(self.inner));
        Ok(())
    }

    pub fn fill(&mut self) -> Result<(), PdfError> {
        unsafe_try_catch!(self.inner, pdflib_sys::PDF_fill(self.inner));
        Ok(())
    }
}
