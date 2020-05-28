use super::{Pdf, PdfError};
use std::ffi;
use std::fmt;

pub struct Asset {
    pub(crate) handle: libc::c_int,
}

impl fmt::Display for Asset {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.handle.fmt(f)
    }
}
/// # Multimedia features
impl Pdf {}

/// ## Asset and Rich Media Features
impl Pdf {
    /// Load a rich media asset or file attachment from a disk-based or virtual file.
    pub fn load_asset(
        &mut self,
        type_: &str,
        filename: &str,
        optlist: &str,
    ) -> Result<Asset, PdfError> {
        let type_ = ffi::CString::new(type_)?;
        let filename = ffi::CString::new(filename)?;
        let optlist = ffi::CString::new(optlist)?;
        let handle: libc::c_int;
        unsafe {
            handle = pdflib_sys::PDF_load_asset(
                self.inner,
                type_.as_ptr(),
                filename.as_ptr(),
                0,
                optlist.as_ptr(),
            );
            if handle == -1 {
                return Err(self.get_error());
            }
        }
        Ok(Asset { handle })
    }
}
