/*
 *
 * formfields starter:
 * create a linearized PDF (for fast delivery over the Web, also known
 * as "fast Web view") which is encrypted and contains some form fields.
 * A few lines of JavaScript are inserted as "page open" action to
 * automatically populate the date field with the current date.
 *
 * Ported from starter_formfields.c
 * Required software: PDFlib/PDFlib+PDI/PPS 9
 * Required data: font file
 */

use pdflib::Pdf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    /* This is where the data files are. Adjust as necessary. */
    const SEARCH_PATH: &str = "./pdflib-sys/pdflib/bind/data";

    let mut pdf = Pdf::new();
    let (llx, mut lly, urx, mut ury) = (150., 550., 350., 575.);

    let js = r##"
        var d = util.printd("yyyy-mm-dd", new Date());
        var date = this.getField("date");
        date.value = d;;
    "##;

    pdf.set_option("SearchPath", &format!("{{{{{}}}}}", SEARCH_PATH))?;

    pdf.set_option("stringformat", "utf8")?;

    /* Prevent changes with a master password */
    pdf.begin_document(
        "starter_formfields.pdf",
        "linearize masterpassword=pdflib permissions={nomodify}",
    )?;

    pdf.set_info("Creator", "PDFlib starter sample")?;
    pdf.set_info("Title", "starter_formfields")?;

    let action = pdf.create_action("JavaScript", &format!("script[{}]={{{}}}", js.len(), js))?;

    let pageopt = format!(
        "width=a4.width height=a4.height action={{open={}}}",
        action.handle
    );
    pdf.begin_page_ext(0., 0., &pageopt)?;

    let font = pdf.load_font("NotoSerif-Regular", "winansi", "simplefont")?;

    pdf.setfont(&font, 24.)?;

    pdf.fit_textline("Date: ", 125., lly + 5., "position={right bottom}")?;

    let fieldopt = format!(
        "tooltip={{Date (will be filled automatically)}} bordercolor={{gray 0}} font={}",
        font.handle
    );
    pdf.create_field(llx, lly, urx, ury, "date", "textfield", &fieldopt)?;

    lly -= 100.;
    ury -= 100.;
    pdf.fit_textline("Name: ", 125., lly + 5.0, "position={right bottom}")?;

    let fieldopt = format!(
        "tooltip={{Enter your name here}} bordercolor={{gray 0}} font={}",
        font.handle
    );
    pdf.create_field(llx, lly, urx, ury, "name", "textfield", &fieldopt)?;
    pdf.end_page_ext("")?;
    pdf.end_document("")?;

    Ok(())
}
