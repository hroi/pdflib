use super::{OptionList, Pdf, PdfError};
use std::ffi;

impl Pdf {
    pub(crate) fn get_error(&self) -> PdfError {
        unsafe {
            let code = pdflib_sys::PDF_get_errnum(self.inner);
            assert!(code != 0);
            let apiname = pdflib_sys::PDF_get_apiname(self.inner);
            let apiname = ffi::CStr::from_ptr(apiname).to_owned();
            let message = pdflib_sys::PDF_get_errmsg(self.inner);
            let message = ffi::CStr::from_ptr(message).to_owned();
            PdfError {
                code,
                apiname,
                message,
            }
        }
    }
}

/// # General Functions
/// ## Exception Handling
impl Pdf {}
/// ## Unicode Conversion
impl Pdf {}
/// ## Global Options
impl Pdf {
    pub fn set_option(&mut self, optlist: impl Into<OptionList>) -> Result<(), PdfError> {
        unsafe_try_catch!(
            self.inner,
            pdflib_sys::PDF_set_option(self.inner, optlist.into().as_ptr())
        );
        Ok(())
    }
}
