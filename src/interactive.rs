use super::{OptionList, Pdf, PdfError};
use std::ffi;
use std::fmt;

/// # Interactive Features
/// ## Bookmarks
impl Pdf {}

/// ## Annotations
impl Pdf {
    /// Create an annotation on the current page.
    pub fn create_annotation(
        &mut self,
        llx: f64,
        lly: f64,
        urx: f64,
        ury: f64,
        type_: &str,
        optlist: impl Into<OptionList>,
    ) -> Result<(), PdfError> {
        let type_ = ffi::CString::new(type_)?;
        unsafe_try_catch!(
            self.inner,
            pdflib_sys::PDF_create_annotation(
                self.inner,
                llx,
                lly,
                urx,
                ury,
                type_.as_ptr(),
                optlist.into().as_ptr(),
            )
        );
        Ok(())
    }
}

/// ## Form Fields
impl Pdf {
    /// Create a form field on the current page subject to various options.
    #[allow(clippy::too_many_arguments)]
    pub fn create_field(
        &mut self,
        llx: f64,
        lly: f64,
        urx: f64,
        ury: f64,
        name: &str,
        type_: &str,
        optlist: impl Into<OptionList>,
    ) -> Result<(), PdfError> {
        let name = ffi::CString::new(name)?;
        let type_ = ffi::CString::new(type_)?;
        unsafe_try_catch!(
            self.inner,
            pdflib_sys::PDF_create_field(
                self.inner,
                llx,
                lly,
                urx,
                ury,
                name.as_ptr(),
                0,
                type_.as_ptr(),
                optlist.into().as_ptr(),
            )
        );
        Ok(())
    }

    /// Create a form field group subject to various options.
    pub fn create_fieldgroup(
        &mut self,
        name: &str,
        optlist: impl Into<OptionList>,
    ) -> Result<(), PdfError> {
        let name = ffi::CString::new(name)?;
        unsafe_try_catch!(
            self.inner,
            pdflib_sys::PDF_create_fieldgroup(
                self.inner,
                name.as_ptr(),
                0,
                optlist.into().as_ptr()
            )
        );
        Ok(())
    }
}
pub struct Action {
    pub(crate) handle: libc::c_int,
}

// Required for interpolation in optionlists
impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.handle.fmt(f)
    }
}

/// ## Actions
impl Pdf {
    pub fn create_action(
        &mut self,
        type_: &str,
        optlist: impl Into<OptionList>,
    ) -> Result<Action, PdfError> {
        let mut ret = Action { handle: 0 };
        let type_ = ffi::CString::new(type_)?;
        unsafe {
            pdflib_sys::PDF_TRY!(self.inner, {
                ret.handle = pdflib_sys::PDF_create_action(
                    self.inner,
                    type_.as_ptr(),
                    optlist.into().as_ptr(),
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
