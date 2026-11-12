//! The page that lists a directory. (The one of `ntex-files` does not escape a quote in a file name, so a name could end the
//! link and add attributes to it.)

use crate::confine::InsideRoot;
use ntex::{
    http::Response,
    web::{HttpRequest, WebResponse},
};
use ntex_files::Directory;
use percent_encoding::{percent_decode_str, utf8_percent_encode, AsciiSet, NON_ALPHANUMERIC};
use std::{cmp::Ordering, fmt::Write, io};

/// Everything but letters, digits and `- _ . ~ /` is written as %XX in a link (a quote or a `#` in a name must not end the link).
const PATH: &AsciiSet = &NON_ALPHANUMERIC
    .remove(b'-')
    .remove(b'_')
    .remove(b'.')
    .remove(b'~')
    .remove(b'/');

/// Text for the inside of an element or of a quoted attribute.
pub fn escape(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    for c in text.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&#39;"),
            c => out.push(c),
        }
    }
    out
}

pub fn encode_path(path: &str) -> String {
    utf8_percent_encode(path, PATH).to_string()
}

/// One line of the page.
#[derive(Debug, PartialEq)]
pub struct Row {
    pub name: String,
    pub href: String,
    pub is_dir: bool,
}

/// The page for the directory `dir` (`/a/b/`). The text is escaped here and nowhere else.
pub fn render(dir: &str, rows: &[Row]) -> String {
    let title = escape(&format!("Index of {dir}"));
    let mut items = String::new();
    for row in rows {
        let slash = if row.is_dir { "/" } else { "" };
        let _ = write!(
            items,
            "<li><a href=\"{}\">{}{slash}</a></li>",
            escape(&row.href),
            escape(&row.name)
        );
    }
    format!(
        "<!doctype html><html><head><meta charset=\"utf-8\"><title>{title}</title></head>\
         <body><h1>{title}</h1><ul>{items}</ul></body></html>"
    )
}

/// Folders first, then by name.
fn order(a: &Row, b: &Row) -> Ordering {
    b.is_dir.cmp(&a.is_dir).then_with(|| a.name.cmp(&b.name))
}

/// `/a/b` -> `/a/b/`, whatever the request looked like.
fn directory_of(request_path: &str) -> String {
    let decoded = percent_decode_str(request_path).decode_utf8_lossy();
    let mut dir = String::from("/");
    for segment in decoded
        .split('/')
        .filter(|s| !s.is_empty() && *s != "." && *s != "..")
    {
        dir.push_str(segment);
        dir.push('/');
    }
    dir
}

/// The renderer for `ntex_files::Files::files_listing_renderer`.
pub fn directory_listing(
    root: &InsideRoot,
    dir: &Directory,
    req: &HttpRequest,
) -> Result<WebResponse, io::Error> {
    let base = directory_of(req.path());

    let mut rows = Vec::new();
    for entry in dir.path.read_dir()? {
        if !dir.is_visible(&entry) {
            continue;
        }
        let Ok(entry) = entry else { continue };
        // a link that leaves the root is not offered (and the file server would refuse it)
        if entry.file_type().is_ok_and(|t| t.is_symlink()) && !root.contains(&entry.path()) {
            continue;
        }
        let Ok(meta) = entry.metadata() else { continue };
        let name = entry.file_name().to_string_lossy().into_owned();
        let is_dir = meta.is_dir();
        let href = encode_path(&format!("{base}{name}{}", if is_dir { "/" } else { "" }));
        rows.push(Row { name, href, is_dir });
    }
    rows.sort_by(order);

    Ok(WebResponse::new(
        Response::Ok()
            .content_type("text/html; charset=utf-8")
            // the page has no script and loads nothing
            .header(
                "content-security-policy",
                "default-src 'none'; base-uri 'none'",
            )
            .body(render(&base, &rows)),
        req.to_owned(),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn row(name: &str, href: &str, is_dir: bool) -> Row {
        Row {
            name: name.into(),
            href: href.into(),
            is_dir,
        }
    }

    #[test]
    fn links_survive_odd_characters() {
        assert_eq!(encode_path("/a b/c#d?e\"f.txt"), "/a%20b/c%23d%3Fe%22f.txt");
        assert_eq!(
            encode_path("/日本/a-b_c.d~e"),
            "/%E6%97%A5%E6%9C%AC/a-b_c.d~e"
        );
    }

    #[test]
    fn names_and_links_are_escaped_in_the_page() {
        let html = render(
            "/<b>/",
            &[row(
                "<script>x</script>",
                "/\"onmouseover=\"alert(1)",
                false,
            )],
        );
        assert!(!html.contains("<script>"));
        assert!(!html.contains("\"onmouseover"));
        assert!(html.contains("&lt;script&gt;"));
        assert!(html.contains("Index of /&lt;b&gt;/"));
    }

    #[test]
    fn folders_end_with_a_slash_and_come_first() {
        let mut rows = vec![
            row("b.txt", "/b.txt", false),
            row("sub", "/sub/", true),
            row("a.txt", "/a.txt", false),
        ];
        rows.sort_by(order);
        let names: Vec<_> = rows.iter().map(|r| r.name.as_str()).collect();
        assert_eq!(names, ["sub", "a.txt", "b.txt"]);
        assert!(render("/", &rows).contains("<li><a href=\"/sub/\">sub/</a></li>"));
    }

    #[test]
    fn the_directory_of_a_request() {
        assert_eq!(directory_of("/"), "/");
        assert_eq!(directory_of("/a/b"), "/a/b/");
        assert_eq!(directory_of("/a%20b/"), "/a b/");
        assert_eq!(directory_of("/../a"), "/a/");
    }
}
