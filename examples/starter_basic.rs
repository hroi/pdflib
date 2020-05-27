/*
 * Basic starter:
 * Create some simple text, vector graphics and image output
 *
 * Ported from starter_basic.c
 * Required software: PDFlib/PDFlib+PDI/PPS 9
 * Required data: none
 */

use pdflib::Pdf;

const IMAGEFILE: &str = "nesrin.jpg";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut p = Pdf::new();

    p.set_option("errorpolicy", "return")?;
    p.set_option("SearchPath", "{{./pdflib-sys/pdflib/bind/data}}")?;

    p.begin_document("starter_basic.pdf", "")?;

    p.set_info("Creator", "PDFlib starter sample")?;
    p.set_info("Title", "starter_basic")?;

    /* We load the image before the first page, and use it
     * on all pages
     */
    let image = p.load_image("auto", IMAGEFILE, "")?;

    /* Page 1 */
    p.begin_page_ext(0., 0., "width=a4.width height=a4.height")?;
    /* use NotoSerif-Regular font with text format UTF-8 for placing the text
     * and demonstrate various options how to pass the UTF-8 text to PDFlib
     */
    let optlist = "fontname={NotoSerif-Regular} encoding=unicode embedding fontsize=24";

    /* using plain ASCII text */
    p.fit_textline("en: Hello!", 50., 700., optlist)?;

    /* using greek */
    p.fit_textline("gr: Γειά!", 50., 650., optlist)?;

    /* using cyrillic */
    p.fit_textline("ru: Привет!", 50., 600., optlist)?;

    /* using PDFlib's character references */
    let optlist =
        "fontname={NotoSerif-Regular} encoding=unicode embedding fontsize=24 charref=true";
    p.fit_textline("es: &#xA1;Hola!", 50.0, 550.0, optlist)?;

    p.fit_image(&image, 0.0, 0.0, "scale=0.25")?;

    p.end_page_ext("")?;

    /* Page 2 */
    p.begin_page_ext(0., 0., "width=a4.width height=a4.height")?;

    /* red rectangle */
    p.setcolor("fill", "rgb", 1.0, 0.0, 0.0, 0.0)?;
    p.rect(200., 200., 250., 150.)?;
    p.fill()?;

    /* blue circle */
    p.setcolor("fill", "rgb", 0.0, 0.0, 1.0, 0.0)?;
    p.arc(400., 600., 100., 0., 360.)?;
    p.fill()?;

    /* thick gray line */
    p.setcolor("stroke", "gray", 0.5, 0.0, 0.0, 0.0)?;
    p.setlinewidth(10.)?;
    p.moveto(100., 500.)?;
    p.lineto(300., 700.)?;
    p.stroke()?;

    /* Using the same image handle means the data will be copied
     * to the PDF only once, which saves space.
     */
    p.fit_image(&image, 150.0, 25.0, "scale=0.25")?;
    p.end_page_ext("")?;

    /* Page 3 */
    p.begin_page_ext(0., 0., "width=a4.width height=a4.height")?;

    /* Fit the image to a box of predefined size (without distortion) */

    p.fit_image(
        &image,
        100.,
        200.,
        "boxsize={400 400} position={center} fitmethod=meet",
    )?;

    p.end_page_ext("")?;

    p.close_image(image)?;
    p.end_document("")?;

    Ok(())
}
