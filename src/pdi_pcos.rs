use super::{Pdf, PdfError};
use std::ffi;
use std::fmt;

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
        optlist: &str,
    ) -> Result<Document, PdfError> {
        let filename = ffi::CString::new(filename)?;
        let optlist = ffi::CString::new(optlist)?;
        unsafe {
            let handle = pdflib_sys::PDF_open_pdi_document(
                self.inner,
                filename.as_ptr(),
                0,
                optlist.as_ptr(),
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
        optlist: &str,
    ) -> Result<Page, PdfError> {
        let optlist = ffi::CString::new(optlist)?;
        unsafe {
            let handle =
                pdflib_sys::PDF_open_pdi_page(self.inner, doc.handle, pagenumber, optlist.as_ptr());
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
        optlist: &str,
    ) -> Result<(), PdfError> {
        let optlist = ffi::CString::new(optlist)?;
        unsafe_try_catch!(
            self.inner,
            pdflib_sys::PDF_fit_pdi_page(self.inner, page.handle, x, y, optlist.as_ptr())
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
        optlist: &str,
    ) -> Result<f64, PdfError> {
        let keyword = ffi::CString::new(keyword)?;
        let optlist = ffi::CString::new(optlist)?;
        let mut ret: f64 = 0.;
        unsafe_try_catch!(self.inner, {
            ret = pdflib_sys::PDF_info_pdi_page(
                self.inner,
                page.handle,
                keyword.as_ptr(),
                optlist.as_ptr(),
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
        optlist: &str,
    ) -> Result<(), PdfError> {
        let optlist = ffi::CString::new(optlist)?;
        unsafe {
            let ret = pdflib_sys::PDF_process_pdi(
                self.inner,
                doc.handle,
                page.map(|p| p.handle).unwrap_or(-1),
                optlist.as_ptr(),
            );
            if ret == -1 {
                return Err(self.get_error());
            }
        }
        Ok(())
    }
}
