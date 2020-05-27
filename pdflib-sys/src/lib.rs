#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(clippy::all)] // no clippy for bindgen'ed code, please

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

// #define PDF_TRY(p)		if (p) { if (setjmp(pdf_jbuf(p)->jbuf) == 0)
#[macro_export]
macro_rules! PDF_TRY {
    ($p:expr, $body:expr) => {
        if !$p.is_null() {
            let jb = pdflib_sys::pdf_jbuf($p);
            if (pdflib_sys::setjmp((*jb).jbuf.as_mut_ptr()) == 0) {
                $body
            }
        }
    };
}

// #define PDF_CATCH(p)		} if (pdf_catch(p))
#[macro_export]
macro_rules! PDF_CATCH {
    ($p:expr, $body:expr) => {
        if pdflib_sys::pdf_catch($p) > 0 {
            $body
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi;
    use std::ptr;
    #[test]
    fn pdf_new() {
        unsafe {
            let p = PDF_new();
            println!("PDF pointer: {:p}", p);
            super::PDF_delete(p);
        }
    }

    #[test]
    fn version() {
        unsafe {
            println!(
                "pdflib version {}.{}",
                PDF_get_majorversion(),
                PDF_get_minorversion()
            )
        }
    }

    #[test]
    fn document() {
        unsafe {
            let p = PDF_new();
            assert!(!p.is_null());
            // let opts = ffi::CString::new("stringformat=utf8 filenamehandling=unicode").unwrap();
            // PDF_set_option(p, opts.as_ptr());
            //PDF_set_option(p, "stringformat=utf8\0".as_ptr() as _);
            let opts = "filenamehandling=legacy\0".as_ptr() as _;
            PDF_set_option(p, opts);
            //let filename = b"\0" as *const u8 as *const i8;
            let filename = ffi::CString::new("\u{feff}test-output.pdf").unwrap();
            PDF_begin_document(
                p,
                filename.as_ptr(),
                filename.as_bytes().len() as libc::c_int,
                ptr::null(),
            );
            // println!("began document");
            // let errno = PDF_get_errnum(p);
            // assert_eq!(errno, 0);
            let (width, height) = (595.0f64, 842.0f64);
            PDF_begin_page_ext(p, width, height, ptr::null());
            PDF_end_page_ext(p, ptr::null());
            PDF_end_document(p, ptr::null());
            //println!("ended document");
        }
    }

    // #[test]
    // fn api() {
    //     unsafe {
    //         let api = PDF_get_api().as_ref().unwrap();
    //         println!("api version {}.{}", api.major, api.minor);
    //     }
    // }
}
