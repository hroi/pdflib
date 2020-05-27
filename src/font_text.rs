use super::{Pdf, PdfError};
use std::ffi;
use std::fmt;

pub struct Font {
    pub(crate) handle: libc::c_int,
}

// Required for interpolation in optionlists
impl fmt::Display for Font {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.handle.fmt(f)
    }
}

/// # Font and Text Functions
/// ## Font Handling
impl Pdf {
    /// Search for a font and prepare it for later use.
    pub fn load_font(
        &mut self,
        fontname: &str,
        encoding: &str,
        optlist: &str,
    ) -> Result<Font, PdfError> {
        let mut ret = Font { handle: 0 };
        unsafe {
            pdflib_sys::PDF_TRY!(self.inner, {
                ret.handle = pdflib_sys::PDF_load_font(
                    self.inner,
                    ffi::CString::new(fontname)?.as_ptr(),
                    0,
                    ffi::CString::new(encoding)?.as_ptr(),
                    ffi::CString::new(optlist)?.as_ptr(),
                );
            });
            pdflib_sys::PDF_CATCH!(self.inner, {
                return Err(self.get_error());
            });
        }
        if ret.handle == -1 {
            return Err(self.get_error());
        }
        Ok(ret)
    }
}

/// ## Simple Text Output
impl Pdf {
    // pub unsafe extern "C" fn PDF_setfont(p: *mut PDF, font: c_int, fontsize: f64)
    pub fn setfont(&mut self, font: &Font, fontsize: f64) -> Result<(), PdfError> {
        unsafe_try_catch!(
            self.inner,
            pdflib_sys::PDF_setfont(self.inner, font.handle, fontsize)
        );
        Ok(())
    }
}
