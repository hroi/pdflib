/*
 * pCOS starter:
 * Dump information from an existing PDF document
 *
 * Ported from start_pcos.c
 * Required software: PDFlib+PDI/PPS 9
 * Required data: PDF input file
 */

use pdflib::Pdf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut pdf = Pdf::new();
    /* This is where the data files are. Adjust as necessary. */
    pdf.set_option(concat!("SearchPath={{", env!("PDFLIB_DIR"), "/bind/data}}"))?;

    let pdfinput = "PLOP-datasheet.pdf";
    let docoptlist = "requiredmode=minimum";

    /* We do not create any output document, so no call to
     * begin_document() is required.
     */

    /* Open the input document */
    let doc = pdf.open_pdi_document(pdfinput, docoptlist)?;

    /* --------- general information (always available) */

    let pcosmode = pdf.pcos_get_number(&doc, "pcosmode")? as i32;

    println!("   File name: {}", pdf.pcos_get_string(&doc, "filename")?);

    println!(
        "   PDF version: {}",
        pdf.pcos_get_string(&doc, "pdfversionstring")?
    );

    println!(
        "   Encryption: {}",
        pdf.pcos_get_string(&doc, "encrypt/description")?
    );

    println!(
        "   Master pw: {}",
        if pdf.pcos_get_number(&doc, "encrypt/master")? as i32 != 0 {
            "yes"
        } else {
            "no"
        }
    );

    println!(
        "   User pw: {}",
        if pdf.pcos_get_number(&doc, "encrypt/user")? as i32 != 0 {
            "yes"
        } else {
            "no"
        }
    );

    println!(
        "Text copying: {}",
        if pdf.pcos_get_number(&doc, "encrypt/nocopy")? as i32 != 0 {
            "no"
        } else {
            "yes"
        }
    );

    println!(
        "  Linearized: {}",
        if pdf.pcos_get_number(&doc, "linearized")? as i32 != 0 {
            "yes"
        } else {
            "no"
        }
    );

    if pcosmode == 0 {
        println!("Minimum mode: no more information available\n\n");
        return Ok(());
    }

    println!("PDF/X status: {}", pdf.pcos_get_string(&doc, "pdfx")?);

    println!("PDF/A status: {}", pdf.pcos_get_string(&doc, "pdfa")?);

    let xfa_present =
        pdf.pcos_get_number(&doc, "type:/Root/AcroForm/XFA")? as u32 != pdflib::PCOS_OT_NULL;
    println!("    XFA data: {}", if xfa_present { "yes" } else { "no" });

    println!(
        "  Tagged PDF: {}",
        if pdf.pcos_get_number(&doc, "tagged")? as u32 != 0 {
            "yes"
        } else {
            "no"
        }
    );

    println!(
        "No. of pages: {}",
        pdf.pcos_get_number(&doc, "length:pages")?
    );

    println!(
        " Page 1 size: width={:.3}, height={:.3}",
        pdf.pcos_get_number(&doc, "pages[0]/width")?,
        pdf.pcos_get_number(&doc, "pages[0]/height")?
    );

    let count = pdf.pcos_get_number(&doc, "length:fonts")? as u32;
    println!("No. of fonts: {}", count);

    for i in 0..count {
        if pdf.pcos_get_number(&doc, &format!("fonts[{}]/embedded", i))? as u32 != 0 {
            print!("embedded ")
        } else {
            print!("unembedded ")
        }
        println!(
            "{} font ",
            pdf.pcos_get_string(&doc, &format!("fonts[{}]/type", i))?
        );
        println!(
            "{}",
            pdf.pcos_get_string(&doc, &format!("fonts[{}]/name", i))?
        );
    }

    println!();

    let plainmetadata = pdf.pcos_get_number(&doc, "encrypt/plainmetadata")? as i32 != 0;

    if pcosmode == 1 && !plainmetadata && pdf.pcos_get_number(&doc, "encrypt/nocopy")? as i32 != 0 {
        println!("Restricted mode: no more information available");
        return Ok(());
    }

    let count = pdf.pcos_get_number(&doc, "length:/Info")? as i32;

    for i in 0..count {
        let objtype = pdf.pcos_get_number(&doc, &format!("type:/Info[{}]", i))? as u32;
        println!(
            "{}: ",
            pdf.pcos_get_string(&doc, &format!("/Info[{}].key", i))?
        );

        /* Info entries can be stored as string or name objects */
        match objtype {
            pdflib::PCOS_OT_NAME | pdflib::PCOS_OT_STRING => {
                println!("'{}'", pdf.pcos_get_string(&doc, &format!("/Info[{}]", i))?);
            }
            _ => {
                println!(
                    "({} object)\n",
                    pdf.pcos_get_string(&doc, &format!("type:/Info[{}]", i))?
                );
            }
        }
    }

    print!("\nXMP metadata: ");

    let objtype = pdf.pcos_get_number(&doc, "type:/Root/Metadata")? as u32;

    if objtype == pdflib::PCOS_OT_STREAM {
        let contents = pdf.pcos_get_stream(&doc, "", "/Root/Metadata")?;
        println!("{} bytes ", contents.len());
        println!("contents:\n{}", String::from_utf8_lossy(&contents));
    } else {
        println!("not present");
    }
    pdf.close_pdi_document(doc)?;

    Ok(())
}
