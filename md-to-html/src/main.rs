use markdown::{CompileOptions, Constructs, Options, ParseOptions};
use std::env::set_current_dir;

fn main() {
    set_current_dir(r"..\").unwrap();

    println!("==> create home index...");
    convert_md_to_html("index.md").unwrap();

    let post_dir = "post";

    println!("==> create post index...");
    create_index_md(post_dir, "index.md");

    println!("==> create post...");
    get_md_files(post_dir).iter().for_each(|file| {
        convert_md_to_html(file).unwrap();
    });
}

fn create_index_md(
    cur_dir: &str,
    index_file_name: &str,
) {
    let mut content = String::new();
    for entry in std::fs::read_dir(cur_dir).expect("can't read dir") {
        let path = entry.expect("can't read dir path").path();
        if path.is_dir() {
            let dir_name = path
                .file_name().expect("path is empty")
                .to_str().expect("can't not convert from file name to str");
            let dir_path = path
                .to_str().expect("can't not convert from path to str");

            create_index_md(dir_path, index_file_name);

            content.push_str(&format!("- [{}]({}/index.html)\n", dir_name, dir_name));
        } else if path.extension().unwrap() == "md" {
            let file_name = path
                .file_name().expect("path is empty")
                .to_str().expect("can't not convert from file name to str");

            if file_name == index_file_name {
                continue;
            }

            content.push_str(&format!(
                "- [{}]({})\n",
                file_name.replace(".md", ""),
                file_name.replace(".md", ".html")
            ));
        }
    }

    let index_file_path = format!("{}/{}", cur_dir, index_file_name);
    println!("{}", index_file_path);
    std::fs::write(index_file_path, content).unwrap();
}

fn convert_md_to_html(
    markdown_file: &str,
) -> Result<(), markdown::message::Message> {
    let html_file = &markdown_file.replace(".md", ".html");

    println!("{} -> {}", markdown_file, html_file);

    let markdown = std::fs::read_to_string(markdown_file).unwrap();
    let markdown = replace_math_expr(&markdown);
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
                    character_escape: false,
                    ..Constructs::default()
                },
                // math_text_single_dollar: true,
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
            css_path.push_str(r#"../"#);
        }
        css_path.push_str(r#"css/style.css"#);
        css_path
    };

    let html = format!(
        r#"<!DOCTYPE html>
<HTML>
<HEAD>
<meta charset="UTF-8">
  <link rel="stylesheet" href="{css_path}">
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

fn replace_math_expr(body: &str) -> String {
    // $expr$ -> \\(expr)\\)
    let mut result = String::new();
    let mut in_math = false;
    for c in body.chars() {
        if c == '$' {
            if in_math {
                result.push_str(r#"\)"#);
            } else {
                result.push_str(r#"\("#);
            }
            in_math = !in_math;
        } else {
            result.push(c);
        }
    }
    result
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