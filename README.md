# [m0hadang.github.io](https://m0hadang.github.io/)

Personal blog/documentation site hosted on GitHub Pages.

## Structure
- `md-to-html.exe` : html file generator using markdown file
  - https://github.com/m0hadang/md-to-html
- `src-post/` : Markdown source files
  - you should write post(*.md) in this directory
  - you can push your source post but it's not necessary
- `post/` : Generated HTML files
- `index.md` : Home page source
- `css/` : Stylesheets

## Build

Convert Markdown to HTML by running the build tool from the project root:

```ps1
md-to-html.exe
```

## Flow

1. Edit `.md` files in `src-post/`
2. Run `md-to-html.exe`
3. HTML is generated in `post/`