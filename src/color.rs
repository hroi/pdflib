use super::{Pdf, PdfError};
use std::ffi;

/// # Color Functions
/// ## Setting Color
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
        let fstype = ffi::CString::new(fstype)?;
        let colorspace = ffi::CString::new(colorspace)?;
        unsafe_try_catch!(
            self.inner,
            pdflib_sys::PDF_setcolor(
                self.inner,
                fstype.as_ptr(),
                colorspace.as_ptr(),
                c1,
                c2,
                c3,
                c4,
            )
        );
        Ok(())
    }
}
