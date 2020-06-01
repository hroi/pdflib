use super::{OptionList, Pdf, PdfError};
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
        optlist: impl Into<OptionList>,
    ) -> Result<Font, PdfError> {
        let mut ret = Font { handle: 0 };
        let fontname = ffi::CString::new(fontname)?;
        let encoding = ffi::CString::new(encoding)?;
        unsafe {
            pdflib_sys::PDF_TRY!(self.inner, {
                ret.handle = pdflib_sys::PDF_load_font(
                    self.inner,
                    fontname.as_ptr(),
                    0,
                    encoding.as_ptr(),
                    optlist.into().as_ptr(),
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
