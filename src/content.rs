use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::path::PathBuf;

use comrak::{markdown_to_html, ComrakOptions};
use derivative::Derivative;
use serde::{Deserialize, Serialize};

use crate::utils::{build_title_for_dir, get_title_from_file, is_ext, is_index_file};

#[derive(Serialize, Deserialize, Debug, Clone, Default, Derivative)]
#[derivative(PartialEq)]
pub struct Content {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_heading: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_break: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub markdown: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub html: Option<String>,

    #[derivative(PartialEq = "ignore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<PathBuf>,

    #[derivative(PartialEq = "ignore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<PathBuf>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Copy)]
pub enum ContentType {
    Normal,
    Main,
    Footer,
    Header,
}

impl Content {
    pub fn new(file: Option<PathBuf>) -> Content {
        Content {
            is_heading: Some(false),
            is_break: Some(false),
            label: None,
            markdown: None,
            html: None,
            url: None,
            dir: None,
            file,
        }
    }

    pub fn new_heading(label: String) -> Content {
        Content {
            is_heading: Some(true),
            is_break: Some(false),
            label: Some(label),
            file: None,
            dir: None,
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
            dir: None,
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

        if is_ext(&path, "md") || is_ext(&path, "html") || is_ext(&path, "htm") {
            info!("processing file: {}", path.display());

            if c.label.is_none() {
                if is_ext(&path, "md") {
                    let title = get_title_from_file(&path, true)?;
                    c.label = title;
                } else if let Some(name_str) =
                    path.file_stem().and_then(|name_str| name_str.to_str())
                {
                    c.label = Some(String::from(name_str));
                }
            }

            let mut file_contents = String::new();
            let mut file = File::open(path)?;
            file.read_to_string(&mut file_contents)?;
            let trimmed = file_contents.trim();

            if !trimmed.is_empty() {
                if is_ext(&path, "md") {
                    c.markdown = Some(trimmed.to_owned());
                } else {
                    c.html = Some(trimmed.to_owned());
                }
            }
        }
    }

    if c.html.is_none() && c.markdown.is_some() {
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

pub fn init_dir_sections(root: &Path) -> Result<std::vec::Vec<Content>, Box<dyn Error>> {
    let paths = fs::read_dir(root)?;

    let mut dirs = paths
        .filter_map(|p| {
            if let Ok(entry) = p {
                if let Ok(file_type) = entry.file_type() {
                    if file_type.is_dir() {
                        return Some(entry.path());
                    }
                }
            }

            None
        })
        .collect::<Vec<PathBuf>>();

    dirs.sort();

    let sections = dirs
        .into_iter()
        .filter_map(|path| init_dir_contents(root, &path))
        .flatten()
        .collect::<Vec<Content>>();

    Ok(sections)
}

pub fn init_dir_contents(root: &Path, path: &Path) -> Option<std::vec::Vec<Content>> {
    let entries = fs::read_dir(path).expect("could not read dir");

    let mut dirres = entries
        .filter_map(|de| {
            if let Ok(dentry) = de {
                if let Ok(de_file_type) = dentry.file_type() {
                    if de_file_type.is_file() {
                        return init_entry_contents(root, dentry, false).map(|(c, _)| vec![c]);
                    }
                }
            }
            None
        })
        .flatten()
        .collect::<Vec<Content>>();

    if !dirres.is_empty() {
        let title = build_title_for_dir(
            path,
            fs::read_dir(&path).expect("could not read dir"),
            false,
        )
        .unwrap_or_else(|_| panic!("could not get title from path: {}", path.display()));
        let heading = Content::new_heading(title);

        dirres.sort_by(|a, b| a.file.cmp(&b.file));

        dirres.insert(0, heading);

        let end = Content::new_break();

        dirres.push(end);

        return Some(dirres);
    }
    None
}

pub fn init_entry_contents(
    root: &Path,
    entry: std::fs::DirEntry,
    check_type: bool,
) -> Option<(Content, ContentType)> {
    if let Ok(file_type) = entry.file_type() {
        if file_type.is_file() {
            let entry_path = entry.path();

            if is_ext(&entry_path, "md") {
                let mut c = Content::default();
                c.file = Some(entry_path);
                c.init_from_file(root);
                let mut ct = ContentType::Normal;

                let is_index = check_type && is_index_file(&entry);
                let is_footer = check_type
                    && entry
                        .path()
                        .file_stem()
                        .and_then(|file_stem| file_stem.to_str())
                        .map(|file_name| file_name.to_lowercase() == "footer")
                        .unwrap_or_else(|| false);
                let is_header = check_type
                    && entry
                        .path()
                        .file_stem()
                        .and_then(|file_stem| file_stem.to_str())
                        .map(|file_name| file_name.to_lowercase() == "header")
                        .unwrap_or_else(|| false);
                if is_index {
                    ct = ContentType::Main;
                }
                if is_footer {
                    ct = ContentType::Footer;
                }
                if is_header {
                    ct = ContentType::Header;
                }

                return Some((c, ct));
            }
        }
    }

    None
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
