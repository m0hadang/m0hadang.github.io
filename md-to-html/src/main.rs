fn create_html_file(
    markdown_file: &str,
    file_path: &str,
) -> Result<(), markdown::message::Message> {

    println!("{} -> {}", markdown_file, file_path);

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

    std::fs::write(file_path, html).unwrap();
    Ok(())
}

fn get_md_files(dir: &str) -> Vec<String> {
    let mut file_list = Vec::new();
    for entry in std::fs::read_dir(dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_dir() {
            file_list.append(&mut get_md_files(path.to_str().unwrap()));
        } else if path.extension().unwrap() == "md" {
            file_list.push(path.to_str().unwrap().to_string());
        }
    }
    file_list
}

fn main() {
    create_html_file(r"..\index.md", r"..\index.html").unwrap();
    get_md_files(r"..\post").iter().for_each(|file| {
        create_html_file(file, &file.replace(".md", ".html")).unwrap();
    });
}