use std::env::set_current_dir;

use markdown::{CompileOptions, Constructs, Options, ParseOptions};

fn create_html_file(
    markdown_file: &str,
) -> Result<(), markdown::message::Message> {
    let html_file = &markdown_file.replace(".md", ".html");

    println!("{} -> {}", markdown_file, html_file);

    let markdown = std::fs::read_to_string(markdown_file).unwrap();
    let body = markdown::to_html_with_options(
        &markdown,
        &Options {
            compile: CompileOptions {
                allow_dangerous_html: true,
                allow_dangerous_protocol: true,
                ..CompileOptions::default()
            },
            parse: ParseOptions {
                constructs: Constructs {
                    math_text: true,
                    math_flow: true,
                    gfm_table: true,
                    ..Constructs::default()
                },
                math_text_single_dollar: false,
                ..ParseOptions::default()
            },
            ..Options::default()
        },
    )?;

    let title = &markdown_file
        .split('\\')
        .last()
        .unwrap()
        .replace(".md", "");

    let css_path = {
        let file_path_depth_count = html_file.split('\\').count();
        let mut css_path = String::new();
        for _ in 0..file_path_depth_count - 1 {
            css_path.push_str(r#"..\"#);
        }
        css_path.push_str(r#"css\style.css"#);
        css_path
    };

    let html = format!(
        r#"<!DOCTYPE html>
<HTML>
<HEAD>
<meta charset="UTF-8">
  <!-- code highlight -->
  <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.9.0/styles/school-book.css">
  <script src="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.9.0/highlight.min.js"></script>
  <script src="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.9.0/languages/go.min.js"></script>
  <script>hljs.highlightAll();</script>
  <script id="MathJax-script" async src="https://cdn.jsdelivr.net/npm/mathjax@3/es5/tex-mml-chtml.js"></script>
  <title>{title}</title>
</HEAD>

<BODY>
<h1 id="post-title">{title}</h1>

{body}

</BODY>

</HTML>"#);

    std::fs::write(html_file, html).unwrap();
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
    set_current_dir(r"..\").unwrap();
    create_html_file(r"index.md").unwrap();
    get_md_files(r"post").iter().for_each(|file| {
        create_html_file(file).unwrap();
    });
}