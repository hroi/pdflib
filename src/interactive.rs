use super::{Pdf, PdfError};
use std::ffi;

/// # Interactive Features
/// ## Bookmarks
impl Pdf {}
/// ## Annotations
impl Pdf {}
/// ## Form Fields
impl Pdf {
    /// Create a form field on the current page subject to various options.
    pub fn create_field(
        &mut self,
        llx: f64,
        lly: f64,
        urx: f64,
        ury: f64,
        name: &str,
        type_: &str,
        optlist: &str,
    ) -> Result<(), PdfError> {
        unsafe_try_catch!(
            self.inner,
            pdflib_sys::PDF_create_field(
                self.inner,
                llx,
                lly,
                urx,
                ury,
                ffi::CString::new(name)?.as_ptr(),
                0,
                ffi::CString::new(type_,)?.as_ptr(),
                ffi::CString::new(optlist)?.as_ptr(),
            )
        );
        Ok(())
    }

    /// Create a form field group subject to various options.
    pub fn create_fieldgroup(&mut self, name: &str, optlist: &str) -> Result<(), PdfError> {
        unsafe_try_catch!(
            self.inner,
            pdflib_sys::PDF_create_fieldgroup(
                self.inner,
                ffi::CString::new(name)?.as_ptr(),
                0,
                ffi::CString::new(optlist)?.as_ptr(),
            )
        );
        Ok(())
    }
}
pub struct Action {
    pub handle: libc::c_int,
}
/// ## Actions
impl Pdf {
    // pub unsafe extern "C" fn PDF_create_action(
    //     p: *mut PDF,
    //     type_: *const c_char,
    //     optlist: *const c_char
    // ) -> c_int
    pub fn create_action(&mut self, type_: &str, optlist: &str) -> Result<Action, PdfError> {
        let mut ret = Action { handle: 0 };
        unsafe {
            pdflib_sys::PDF_TRY!(self.inner, {
                ret.handle = pdflib_sys::PDF_create_action(
                    self.inner,
                    ffi::CString::new(type_)?.as_ptr(),
                    ffi::CString::new(optlist)?.as_ptr(),
                );
            });
            pdflib_sys::PDF_CATCH!(
                self.inner,
                pdflib_sys::PDF_CATCH!(self.inner, { return Err(self.get_error()) })
            )
        }
        Ok(ret)
    }
}
/// ## Named Destinations
impl Pdf {}
/// ## PDF Packages and Portfolios
impl Pdf {}
/// ## Geospatial Features
impl Pdf {}
