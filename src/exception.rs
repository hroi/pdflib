use super::{Pdf, PdfError};
use std::ffi;

impl Pdf {
    pub(crate) fn get_error(&self) -> PdfError {
        unsafe {
            let code = pdflib_sys::PDF_get_errnum(self.inner);
            assert!(code != 0);
            let message = pdflib_sys::PDF_get_errmsg(self.inner);
            let message = ffi::CStr::from_ptr(message)
                .to_owned()
                .into_string()
                .unwrap();
            PdfError { code, message }
        }
    }
}
