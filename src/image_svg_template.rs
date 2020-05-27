use super::{Pdf, PdfError};
use std::ffi;
pub struct Image {
    pub(crate) inner: libc::c_int,
}

/// # Images
impl Pdf {
    pub fn load_image(
        &mut self,
        imagetype: &str,
        filename: &str,
        optlist: &str,
    ) -> Result<Image, PdfError> {
        unsafe {
            let res = pdflib_sys::PDF_load_image(
                self.inner,
                ffi::CString::new(imagetype).unwrap().as_ptr(),
                ffi::CString::new(filename).unwrap().as_ptr(),
                0,
                ffi::CString::new(optlist).unwrap().as_ptr(),
            );
            if res != -1 {
                Ok(Image { inner: res })
            } else {
                Err(self.get_error())
            }
        }
    }

    pub fn fit_image(
        &mut self,
        image: &Image,
        x: f64,
        y: f64,
        optlist: &str,
    ) -> Result<(), PdfError> {
        unsafe {
            pdflib_sys::PDF_TRY!(self.inner, {
                pdflib_sys::PDF_fit_image(
                    self.inner,
                    image.inner,
                    x,
                    y,
                    ffi::CString::new(optlist).unwrap().as_ptr(),
                );
            });
            pdflib_sys::PDF_CATCH!(self.inner, { return Err(self.get_error()) });
        }
        Ok(())
    }

    pub fn close_image(&mut self, image: Image) -> Result<(), PdfError> {
        unsafe {
            pdflib_sys::PDF_TRY!(self.inner, {
                pdflib_sys::PDF_close_image(self.inner, image.inner);
            });
            pdflib_sys::PDF_CATCH!(self.inner, {
                pdflib_sys::PDF_CATCH!(self.inner, { return Err(self.get_error()) });
            });
        }
        Ok(())
    }
}
