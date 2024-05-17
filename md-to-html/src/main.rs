fn create_html_file(markdown_file: &str, html_file: &str) -> Result<(), markdown::message::Message> {
    let markdown = std::fs::read_to_string(markdown_file).unwrap();
    let body = markdown::to_html_with_options(
        &markdown,
        &markdown::Options::gfm()
    )?;

    let html = format!(
r#"<!DOCTYPE html>
<HTML>
<HEAD>
<meta charset="UTF-8">
  <link rel="stylesheet" href="css\style.css">
  <title>Document</title>
</HEAD>

<BODY>

{body}

</BODY>

</HTML>"#);

    std::fs::write(html_file, html).unwrap();
    Ok(())
}

fn main() {
    create_html_file(r"..\index.md", r"..\index.html").unwrap();
    // create_html_file(r"..\apple.md", r"..\apple.html").unwrap();
}