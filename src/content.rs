use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::path::PathBuf;

use comrak::{markdown_to_html, ComrakOptions};
use derivative::Derivative;
use serde::{Deserialize, Serialize};

use crate::utils::{get_title_from_file, is_ext};

#[derive(Serialize, Deserialize, Debug, Clone, Default, Derivative)]
#[derivative(PartialEq)]
pub struct Content {
    pub is_heading: Option<bool>,
    pub is_break: Option<bool>,
    pub label: Option<String>,
    pub url: Option<String>,
    pub markdown: Option<String>,
    pub html: Option<String>,

    #[derivative(PartialEq = "ignore")]
    pub file: Option<PathBuf>,
}

impl Content {
    pub fn new(file: Option<PathBuf>) -> Content {
        Content {
            is_heading: Some(false),
            is_break: Some(false),
            label: None,
            file,
            markdown: None,
            html: None,
            url: None,
        }
    }

    pub fn new_heading(label: String) -> Content {
        Content {
            is_heading: Some(true),
            is_break: Some(false),
            label: Some(label),
            file: None,
            markdown: None,
            html: None,
            url: None,
        }
    }

    pub fn new_break() -> Content {
        Content {
            is_heading: Some(false),
            is_break: Some(true),
            label: None,
            file: None,
            markdown: None,
            html: None,
            url: None,
        }
    }

    pub fn init_from_file(&mut self, root: &Path) {
        if self.file.is_some() {
            let mut pathbuf = self.file.clone().unwrap();

            if root.has_root() && pathbuf.is_relative() {
                pathbuf = root.join(&pathbuf).canonicalize().unwrap_or_else(|_| {
                    panic!(
                        "could not resolve path. root: {} path: {}",
                        root.display(),
                        pathbuf.display()
                    )
                });
            }

            if is_ext(&pathbuf, "md") {
                self.label = get_title_from_file(&pathbuf, true).unwrap_or_else(|_| {
                    panic!("could not get title from path: {}", pathbuf.display())
                });
            }
        }
    }
}

pub fn fill_content(c: &mut Content, root: &Path) -> Result<(), Box<dyn Error>> {
    if c.url.is_some() || c.html.is_some() || (c.file.is_none() && c.markdown.is_none()) {
        return Ok(());
    }

    if c.file.is_some() {
        let mut pathbuf = c.file.clone().unwrap();

        if root.has_root() && pathbuf.is_relative() {
            pathbuf = root.join(&pathbuf).canonicalize().unwrap_or_else(|_| {
                panic!(
                    "could not resolve path. root: {} path: {}",
                    root.display(),
                    pathbuf.display()
                )
            });
        }

        let path = pathbuf.as_path();

        info!("processing file: {}", path.display());

        if c.label.is_none() {
            let title = get_title_from_file(&pathbuf, true)?;
            c.label = title;
        }

        let mut file_contents = String::new();
        let mut file = File::open(path)?;
        file.read_to_string(&mut file_contents)?;
        let markdown = file_contents.trim();
        if !markdown.is_empty() {
            c.markdown = Some(markdown.to_owned());
        }
    }

    if c.markdown.is_some() {
        let options = ComrakOptions {
            ext_strikethrough: true,
            ext_autolink: true,
            ext_table: true,
            unsafe_: true,
            ext_tagfilter: true,
            ext_tasklist: true,
            ..ComrakOptions::default()
        };
        let html = Some(markdown_to_html(
            c.markdown.as_ref().unwrap().as_str(),
            &options,
        ));
        c.html = html;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fill_content() {
        // empty
        let mut c = Content::new(None);
        let root = Path::new(".");
        assert!(fill_content(&mut c, &root).is_ok());
        assert_eq!(c, Content::new(None));

        // just markdown
        c = Content::new(None);
        c.markdown = Some(String::from("# Hello world!"));
        assert!(fill_content(&mut c, &root).is_ok());
        let mut expected = Content::new(None);
        expected.markdown = Some(String::from("# Hello world!"));
        expected.html = Some(String::from("<h1>Hello world!</h1>\n"));
        assert_eq!(c, expected);

        // with file
        c = Content::new(Some(PathBuf::from("tests/fixtures/utils3/README.md")));
        assert!(fill_content(&mut c, &root).is_ok());
        let mut expected = Content::new(Some(PathBuf::from("tests/fixtures/utils3/README.md")));
        expected.label = Some(String::from("Main page"));
        expected.markdown = Some(String::from("# Main page\n\nSome content."));
        expected.html = Some(String::from("<h1>Main page</h1>\n<p>Some content.</p>\n"));
        assert_eq!(c, expected);

        // with unknown file
        c = Content::new(Some(PathBuf::from(
            "tests/fixtures/utils3/readme_unknown.md",
        )));
        assert!(fill_content(&mut c, &root).is_err());

        // with url
        c = Content::new(Some(PathBuf::from("tests/fixtures/utils3/README.md")));
        c.url = Some(String::from("https://github.com"));
        assert!(fill_content(&mut c, &root).is_ok());
        let mut expected = Content::new(Some(PathBuf::from("tests/fixtures/utils3/README.md")));
        expected.markdown = None;
        expected.html = None;
        expected.url = Some(String::from("https://github.com"));
        assert_eq!(c, expected);

        // with html
        c = Content::new(Some(PathBuf::from("tests/fixtures/utils3/README.md")));
        c.html = Some(String::from("<h1>Some title</h1>"));
        assert!(fill_content(&mut c, &root).is_ok());
        let mut expected = Content::new(Some(PathBuf::from("tests/fixtures/utils3/README.md")));
        expected.markdown = None;
        expected.html = Some(String::from("<h1>Some title</h1>"));
        assert_eq!(c, expected);
    }
}
