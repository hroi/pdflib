use super::{Pdf, PdfError};

/// # Path Construction
impl Pdf {
    pub fn arc(&mut self, x: f64, y: f64, r: f64, alpha: f64, beta: f64) -> Result<(), PdfError> {
        unsafe_try_catch!(
            self.inner,
            pdflib_sys::PDF_arc(self.inner, x, y, r, alpha, beta)
        );
        Ok(())
    }

    pub fn rect(&mut self, x: f64, y: f64, width: f64, height: f64) -> Result<(), PdfError> {
        unsafe_try_catch!(
            self.inner,
            pdflib_sys::PDF_rect(self.inner, x, y, width, height)
        );
        Ok(())
    }

    /* pub unsafe extern "C" fn PDF_moveto(p: *mut PDF, x: f64, y: f64) */
    pub fn moveto(&mut self, x: f64, y: f64) -> Result<(), PdfError> {
        unsafe_try_catch!(self.inner, pdflib_sys::PDF_moveto(self.inner, x, y));
        Ok(())
    }

    /* pub unsafe extern "C" fn PDF_lineto(p: *mut PDF, x: f64, y: f64) */
    pub fn lineto(&mut self, x: f64, y: f64) -> Result<(), PdfError> {
        unsafe_try_catch!(self.inner, pdflib_sys::PDF_lineto(self.inner, x, y));
        Ok(())
    }
}
