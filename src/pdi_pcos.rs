use super::{OptionList, Pdf, PdfError};
use std::ffi;
use std::fmt;
use std::ptr;

pub struct Document {
    pub(crate) handle: libc::c_int,
}

impl fmt::Display for Document {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.handle.fmt(f)
    }
}

/// # PDF Import (PDI) and pCOS Functions
/// ## Document Functions
impl Pdf {
    /// Open a disk-based or virtual PDF document and prepare it for later use.
    pub fn open_pdi_document(
        &mut self,
        filename: &str,
        optlist: impl Into<OptionList>,
    ) -> Result<Document, PdfError> {
        let filename = ffi::CString::new(filename)?;
        unsafe {
            let handle = pdflib_sys::PDF_open_pdi_document(
                self.inner,
                filename.as_ptr(),
                0,
                optlist.into().as_ptr(),
            );
            if handle == -1 {
                Err(self.get_error())
            } else {
                Ok(Document { handle })
            }
        }
    }

    /// Close all open PDI page handles, and close the input PDF document.
    pub fn close_pdi_document(&mut self, doc: Document) -> Result<(), PdfError> {
        unsafe_try_catch!(
            self.inner,
            pdflib_sys::PDF_close_pdi_document(self.inner, doc.handle)
        );
        Ok(())
    }
}

pub struct Page {
    pub(crate) handle: libc::c_int,
}
impl fmt::Display for Page {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.handle.fmt(f)
    }
}

/// ## Page Functions
impl Pdf {
    /// Prepare a page for later use with PDF_fit_pdi_page( ).
    pub fn open_pdi_page(
        &mut self,
        doc: &Document,
        pagenumber: i32,
        optlist: impl Into<OptionList>,
    ) -> Result<Page, PdfError> {
        unsafe {
            let handle = pdflib_sys::PDF_open_pdi_page(
                self.inner,
                doc.handle,
                pagenumber,
                optlist.into().as_ptr(),
            );
            if handle == -1 {
                Err(self.get_error())
            } else {
                Ok(Page { handle })
            }
        }
    }

    /// Close the page handle and free all page-related resources.
    pub fn close_pdi_page(&mut self, page: Page) -> Result<(), PdfError> {
        unsafe_try_catch!(
            self.inner,
            pdflib_sys::PDF_close_pdi_page(self.inner, page.handle)
        );
        Ok(())
    }

    // void PDF_fit_pdi_page(PDF *p, int page, double x, double y, const char *optlist)
    /// Place an imported PDF page on the output page subject to various options.
    pub fn fit_pdi_page(
        &mut self,
        page: &Page,
        x: f64,
        y: f64,
        optlist: impl Into<OptionList>,
    ) -> Result<(), PdfError> {
        unsafe_try_catch!(
            self.inner,
            pdflib_sys::PDF_fit_pdi_page(self.inner, page.handle, x, y, optlist.into().as_ptr())
        );
        Ok(())
    }

    // XXX: weird sentinel return values in api:
    // The value of some page property as requested by keyword. If the requested property
    // is not available for the page, the function returns 0. If an object handle is
    // requested (e.g. clippingpath) this function will return a handle to the object,
    // or -1 (in PHP: 0) if the object is not available. If the requested keyword
    // produces text, a string index is returned, and the corresponding string must be
    // retrieved with PDF_get_string( ).
    /// Perform formatting calculations for a PDI page and query the resulting metrics.
    pub fn info_pdi_page(
        &mut self,
        page: &Page,
        keyword: &str,
        optlist: impl Into<OptionList>,
    ) -> Result<f64, PdfError> {
        let keyword = ffi::CString::new(keyword)?;
        let mut ret: f64 = 0.;
        unsafe_try_catch!(self.inner, {
            ret = pdflib_sys::PDF_info_pdi_page(
                self.inner,
                page.handle,
                keyword.as_ptr(),
                optlist.into().as_ptr(),
            )
        });
        Ok(ret)
    }
}

/// ## Other PDI Processing
impl Pdf {
    /// Process certain elements of an imported PDF document.
    pub fn process_pdi(
        &mut self,
        doc: &Document,
        page: Option<&Page>,
        optlist: impl Into<OptionList>,
    ) -> Result<(), PdfError> {
        unsafe {
            let ret = pdflib_sys::PDF_process_pdi(
                self.inner,
                doc.handle,
                page.map(|p| p.handle).unwrap_or(-1),
                optlist.into().as_ptr(),
            );
            if ret == -1 {
                return Err(self.get_error());
            }
        }
        Ok(())
    }
}

/// ## pCOS Functions
impl Pdf {
    /// Get the value of a pCOS path with type number or boolean.
    pub fn pcos_get_number(&mut self, doc: &Document, path: &str) -> Result<f64, PdfError> {
        let path = ffi::CString::new(path)?;
        let mut ret: f64 = 0.0;
        unsafe_try_catch!(
            self.inner,
            ret = pdflib_sys::PDF_pcos_get_number(self.inner, doc.handle, path.as_ptr())
        );
        Ok(ret)
    }

    /// Get the value of a pCOS path with type name, number, string, or boolean.
    pub fn pcos_get_string(&mut self, doc: &Document, path: &str) -> Result<String, PdfError> {
        let path = ffi::CString::new(path)?;
        let mut s: *const libc::c_char = ptr::null();
        unsafe_try_catch!(self.inner, {
            s = pdflib_sys::PDF_pcos_get_string(self.inner, doc.handle, path.as_ptr());
        });
        unsafe {
            let owned = ffi::CString::from(ffi::CStr::from_ptr(s));
            let ret = owned.into_string()?;
            Ok(ret)
        }
    }

    /// Get the contents of a pCOS path with type stream, fstream, or string.
    pub fn pcos_get_stream(
        &mut self,
        doc: &Document,
        optlist: impl Into<OptionList>,
        path: &str,
    ) -> Result<Vec<u8>, PdfError> {
        let path = ffi::CString::new(path)?;
        let mut stream_len = 0;
        let mut stream_ptr: *const u8 = ptr::null();
        unsafe_try_catch!(
            self.inner,
            stream_ptr = pdflib_sys::PDF_pcos_get_stream(
                self.inner,
                doc.handle,
                &mut stream_len,
                optlist.into().as_ptr(),
                path.as_ptr()
            )
        );
        assert!(!stream_ptr.is_null());
        let slice = unsafe { std::slice::from_raw_parts(stream_ptr, stream_len as usize) };
        Ok(slice.to_vec())
    }
}
