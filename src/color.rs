use super::{Pdf, PdfError};
use std::ffi;

/// # Color Functions
impl Pdf {
    pub fn setcolor(
        &mut self,
        fstype: &str,
        colorspace: &str,
        c1: f64,
        c2: f64,
        c3: f64,
        c4: f64,
    ) -> Result<(), PdfError> {
        unsafe_try_catch!(
            self.inner,
            pdflib_sys::PDF_setcolor(
                self.inner,
                ffi::CString::new(fstype).unwrap().as_c_str().as_ptr(),
                ffi::CString::new(colorspace).unwrap().as_c_str().as_ptr(),
                c1,
                c2,
                c3,
                c4,
            )
        );
        Ok(())
    }
}
