/*
 * Basic starter:
 * Create some simple text, vector graphics and image output
 *
 * Required software: PDFlib/PDFlib+PDI/PPS 9
 * Required data: none
 */

// #include <stdio.h>
// #include <stdlib.h>

// #include "pdflib.h"

use pdflib_sys::*;
use std::ffi;
//use std::ptr;

fn main() -> Result<(), i32> {
    unsafe {
        /* This is where the data files are. Adjust as necessary. */
        //char* searchpath = "../data";
        //let searchpath = "../data\0".as_ptr() as *const libc::c_char;
        //const char* imagefile = "nesrin.jpg";
        let imagefile = "nesrin.jpg\0".as_ptr() as *const libc::c_char;

        //PDF * p;
        let p: *mut PDF; // = ptr::null_mut();
                         //int image;
        let image: libc::c_int;

        /* create a new PDFlib object */
        // if ((p = PDF_new()) == (PDF *) 0) {
        //     printf("Couldn't create PDFlib object (out of memory)!\n");
        //     return(2);
        // }
        p = PDF_new();
        if p.is_null() {
            println!("Couldn't create PDFlib object (out of memory)!");
            return Err(2);
        }

        // PDF_TRY(p) {
        //     char optlist[256];
        PDF_TRY!(p, {
            //let mut optlist = String::with_capacity(256);
            /* This means we must check return values of load_font() etc. */
            //PDF_set_option(p, "errorpolicy=return");
            PDF_set_option(p, b"errorpolicy=return\0".as_ptr() as _);

            //sprintf(optlist, "SearchPath={{%s}}", searchpath);
            //write!(&mut optlist, "SearchPath={{{{{}}}}}\0", std::str::from_utf8_unchecked(searchpath.to_bytes()));
            PDF_set_option(p, "SearchPath={{pdflib/bind/data}}\0".as_ptr() as _);
            //optlist.clear();

            if PDF_begin_document(
                p,
                b"starter_basic.pdf\0".as_ptr() as _,
                0,
                b"\0".as_ptr() as _,
            ) == -1
            {
                println!(
                    "Error: {}\n",
                    ffi::CStr::from_ptr(PDF_get_errmsg(p) as _).to_string_lossy()
                );

                PDF_delete(p);
                return Err(2);
            }

            PDF_set_info(
                p,
                b"Creator\0".as_ptr() as _,
                b"PDFlib starter sample\0".as_ptr() as _,
            );
            PDF_set_info(
                p,
                b"Title\0".as_ptr() as _,
                b"starter_basic\0".as_ptr() as _,
            );

            /* We load the image before the first page, and use it
             * on all pages
             */
            image = PDF_load_image(p, "auto\0".as_ptr() as _, imagefile, 0, b"\0".as_ptr() as _);

            if image == -1 {
                println!(
                    "Error: {}\n",
                    ffi::CStr::from_ptr(PDF_get_errmsg(p)).to_string_lossy()
                );
                PDF_delete(p);
                return Err(2);
            }

            /* Page 1 */
            PDF_begin_page_ext(
                p,
                0.,
                0.,
                b"width=a4.width height=a4.height\0".as_ptr() as _,
            );
            /* use NotoSerif-Regular font with text format UTF-8 for placing the text
             * and demonstrate various options how to pass the UTF-8 text to PDFlib
             */
            // sprintf(optlist, "fontname={NotoSerif-Regular} encoding=unicode embedding "
            //           "fontsize=24 textformat=utf8");

            /* using plain ASCII text */
            //PDF_fit_textline(p, b"en: Hello!".as_ptr() as _, 0, 50., 700., optlist.as_bytes().as_ptr() as _);
            let optlist = b"fontname={NotoSerif-Regular} encoding=unicode embedding fontsize=24 textformat=utf8\0".as_ptr() as _;
            PDF_fit_textline(p, b"en: Hello!\0".as_ptr() as _, 0, 50., 700., optlist);
            //optlist.clear();

            /* using hexadecimal character codes */
            PDF_fit_textline(
                p,
                b"\x67\x72\x3A\x20\xCE\x93\xCE\xB5\xCE\xB9\xCE\xAC\x21\0".as_ptr() as _,
                0,
                50.,
                650.,
                optlist,
            );

            PDF_fit_textline(
                p,
                b"ru: \xD0\x9F\xD1\x80\xD0\xB8\xD0\xB2\xD0\xB5\xD1\x82!\0".as_ptr() as _,
                0,
                50.,
                600.,
                optlist,
            );

            /* using PDFlib's character references */
            let optlist = b"fontname={NotoSerif-Regular} encoding=unicode embedding fontsize=24 textformat=utf8  charref=true\0".as_ptr() as _;
            PDF_fit_textline(
                p,
                b"es: &#xA1;Hola!\0".as_ptr() as _,
                0,
                50.0,
                550.0,
                optlist,
            );

            PDF_fit_image(p, image, 0.0, 0.0, b"scale=0.25\0".as_ptr() as _);

            PDF_end_page_ext(p, b"\0".as_ptr() as _);

            /* Page 2 */
            PDF_begin_page_ext(
                p,
                0.,
                0.,
                b"width=a4.width height=a4.height\0".as_ptr() as _,
            );

            /* red rectangle */
            PDF_setcolor(
                p,
                "fill\0".as_ptr() as _,
                "rgb\0".as_ptr() as _,
                1.0,
                0.0,
                0.0,
                0.0,
            );
            PDF_rect(p, 200., 200., 250., 150.);
            PDF_fill(p);

            /* blue circle */
            PDF_setcolor(
                p,
                "fill\0".as_ptr() as _,
                "rgb\0".as_ptr() as _,
                0.0,
                0.0,
                1.0,
                0.0,
            );
            PDF_arc(p, 400., 600., 100., 0., 360.);
            PDF_fill(p);

            /* thick gray line */
            PDF_setcolor(
                p,
                b"stroke\0".as_ptr() as _,
                b"gray\0".as_ptr() as _,
                0.5,
                0.0,
                0.0,
                0.0,
            );
            PDF_setlinewidth(p, 10.);
            PDF_moveto(p, 100., 500.);
            PDF_lineto(p, 300., 700.);
            PDF_stroke(p);

            /* Using the same image handle means the data will be copied
             * to the PDF only once, which saves space.
             */
            PDF_fit_image(p, image, 150.0, 25.0, "scale=0.25\0".as_ptr() as _);
            PDF_end_page_ext(p, "\0".as_ptr() as _);

            /* Page 3 */
            PDF_begin_page_ext(p, 0., 0., "width=a4.width height=a4.height\0".as_ptr() as _);

            /* Fit the image to a box of predefined size (without distortion) */
            let optlist = "boxsize={400 400} position={center} fitmethod=meet\0".as_ptr() as _;

            PDF_fit_image(p, image, 100., 200., optlist);

            PDF_end_page_ext(p, "\0".as_ptr() as _);

            PDF_close_image(p, image);
            PDF_end_document(p, "\0".as_ptr() as _);
        });

        PDF_CATCH!(p, {
            println!("PDFlib exception occurred:");
            println!(
                "[{}] {}: {}\n",
                PDF_get_errnum(p),
                ffi::CStr::from_ptr(PDF_get_apiname(p)).to_string_lossy(),
                ffi::CStr::from_ptr(PDF_get_errmsg(p)).to_string_lossy(),
            );
            PDF_delete(p);
            return Err(2);
        });

        PDF_delete(p);

        return Ok(());
    }
}
