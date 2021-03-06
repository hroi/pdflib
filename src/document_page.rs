use super::{OptionList, Pdf, PdfError};
use std::ffi;

/// # Document and Page Functions
/// ## Document Functions
impl Pdf {
    pub fn begin_document(
        &mut self,
        filename: &str,
        optlist: impl Into<OptionList>,
    ) -> Result<(), PdfError> {
        let filename = ffi::CString::new(filename)?;
        unsafe {
            if pdflib_sys::PDF_begin_document(
                self.inner,
                filename.as_ptr(),
                0,
                optlist.into().as_ptr(),
            ) == -1
            {
                return Err(self.get_error());
            }
        }
        Ok(())
    }

    pub fn end_document<O: Into<OptionList>>(&mut self, optlist: O) -> Result<(), PdfError> {
        unsafe_try_catch!(
            self.inner,
            pdflib_sys::PDF_end_document(self.inner, optlist.into().as_ptr(),)
        );
        Ok(())
    }
}

/// ## Page Functions
impl Pdf {
    pub fn begin_page_ext(
        &mut self,
        width: f64,
        height: f64,
        optlist: impl Into<OptionList>,
    ) -> Result<(), PdfError> {
        unsafe_try_catch!(
            self.inner,
            pdflib_sys::PDF_begin_page_ext(self.inner, width, height, optlist.into().as_ptr(),)
        );
        Ok(())
    }

    pub fn end_page_ext(&mut self, optlist: impl Into<OptionList>) -> Result<(), PdfError> {
        unsafe {
            pdflib_sys::PDF_TRY!(self.inner, {
                pdflib_sys::PDF_end_page_ext(self.inner, optlist.into().as_ptr());
            });
            pdflib_sys::PDF_CATCH!(self.inner, { return Err(self.get_error()) });
        }
        Ok(())
    }
}
