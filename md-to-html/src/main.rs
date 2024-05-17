use markdown::ParseOptions;

fn create_html_file(markdown_file: &str, html_file: &str) -> Result<(), markdown::message::Message> {
    let markdown = std::fs::read_to_string(markdown_file).unwrap();
    let html = markdown::to_html_with_options(
        &markdown,
        &markdown::Options::gfm()
    )?;

    let mut html_data = String::new();
    html_data.push_str("<HTML>");
    html_data.push_str( r#"<HEAD>
<meta charset="UTF-8">
    <link rel="stylesheet" href="css\style.css">
    <title>Document</title>
</HEAD>"#);

    html_data.push_str("\n");
    html_data.push_str("<BODY>\n");
    html_data.push_str(&html);
    html_data.push_str("\n");
    html_data.push_str("</BODY>\n");

    html_data.push_str("</HTML>\n");
    std::fs::write(html_file, html_data).unwrap();
    Ok(())
}

fn main() {
    create_html_file(r"..\index.md", r"..\index.html").unwrap();
    // create_html_file(r"..\apple.md", r"..\apple.html").unwrap();
}