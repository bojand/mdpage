#![deny(rust_2018_idioms)]

use std::env;
use std::error::Error;

use std::fs;
use std::fs::File;

use std::io::BufReader;
use std::path::Path;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::content::Content;
use crate::utils::{build_title_for_dir, is_index_file, is_markdown};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Data {
    pub full_page: Option<bool>,
    pub title: Option<String>,
    pub subtitle: Option<String>,
    pub author: Option<String>,
    pub icon: Option<String>,
    pub main: Option<Content>,
    pub contents: Option<Vec<Content>>,
    pub script: Option<String>,
    pub style: Option<String>,
    pub links: Option<Vec<Link>>,
    pub header: Option<Content>,
    pub footer: Option<Content>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Link {
    pub link_type: Option<String>,
    pub src: Option<String>,
    pub integrity: Option<String>,
    pub crossorigin: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
enum ContentType {
    Normal,
    Main,
    Footer,
    Header,
}

impl Default for Data {
    fn default() -> Data {
        Data {
            full_page: Some(false),
            title: None,
            subtitle: None,
            author: None,
            icon: None,
            main: None,
            contents: None,
            script: None,
            style: None,
            links: None,
            footer: None,
            header: None,
        }
    }
}

impl Data {
    fn build(&mut self, root: &Path) -> Result<(), Box<dyn Error>> {
        self.init(root)?;

        self.build_contents(root)?;

        Ok(())
    }

    fn init(&mut self, root: &Path) -> Result<(), Box<dyn Error>> {
        if self.title.is_none() {
            self.title = Some(build_title_for_dir(root, fs::read_dir(root)?, true)?);
        }

        if self.main.is_some() {
            self.main.as_mut().unwrap().init_from_file(root);
        }

        if self.header.is_some() {
            self.header.as_mut().unwrap().init_from_file(root);
        }

        if self.footer.is_some() {
            self.footer.as_mut().unwrap().init_from_file(root);
        }

        let paths = fs::read_dir(root)?;

        let mut main = None;
        let mut header = None;
        let mut footer = None;
        let res = paths
            .filter_map(|p| {
                if let Ok(entry) = p {
                    if let Ok(file_type) = entry.file_type() {
                        if file_type.is_file() {
                            return init_entry_contents(root, entry, true).and_then(|(c, ct)| {
                                match ct {
                                    ContentType::Main => {
                                        main = Some(c);
                                        return None;
                                    }
                                    ContentType::Footer => {
                                        footer = Some(c);
                                        return None;
                                    }
                                    ContentType::Header => {
                                        header = Some(c);
                                        return None;
                                    }
                                    _ => return Some(vec![c]),
                                }
                            });
                        } else {
                            let entries =
                                fs::read_dir(entry.path().as_path()).expect("could not read dir");

                            let mut dirres = entries
                                .filter_map(|de| {
                                    if let Ok(dentry) = de {
                                        if let Ok(de_file_type) = dentry.file_type() {
                                            if de_file_type.is_file() {
                                                return init_entry_contents(root, dentry, false)
                                                    .and_then(|(c, _)| {
                                                        return Some(vec![c]);
                                                    });
                                            }
                                        }
                                    }
                                    return None;
                                })
                                .flatten()
                                .collect::<Vec<Content>>();

                            if dirres.len() > 0 {
                                let title = build_title_for_dir(
                                    entry.path().as_path(),
                                    fs::read_dir(entry.path()).expect("could not get title"),
                                    false,
                                )
                                .expect("could not get title");
                                let heading = Content::new_heading(title);

                                dirres.insert(0, heading);

                                let end = Content::new_break();

                                dirres.push(end);

                                return Some(dirres);
                            } else {
                                return None;
                            }
                        }
                    }
                }
                return None;
            })
            .flatten()
            .collect::<Vec<Content>>();

        if self.main.is_none() {
            self.main = main;
        }

        if self.footer.is_none() {
            self.footer = footer;
        }

        if self.header.is_none() {
            self.header = header;
        }

        if self.contents.is_none() {
            self.contents = Some(res);
        }

        return Ok(());
    }

    fn build_contents(&mut self, root: &Path) -> Result<(), Box<dyn Error>> {
        if self.contents.is_some() {
            let contents = self.contents.as_mut().unwrap();

            for c in contents {
                crate::content::fill_content(c, root)?;
            }
        }

        if self.main.is_some() {
            crate::content::fill_content(self.main.as_mut().unwrap(), root)?;
        }

        if self.header.is_some() {
            crate::content::fill_content(self.header.as_mut().unwrap(), root)?;
        }

        if self.footer.is_some() {
            crate::content::fill_content(self.footer.as_mut().unwrap(), root)?;
        }

        return Ok(());
    }
}

fn config_file(root: &Path) -> Option<PathBuf> {
    let json_config = root.join("mdpage.json");
    match json_config.as_path().exists() {
        true => Some(json_config),
        false => None,
    }
}

fn init_entry_contents(
    root: &Path,
    entry: std::fs::DirEntry,
    check_type: bool,
) -> Option<(Content, ContentType)> {
    if let Ok(file_type) = entry.file_type() {
        if file_type.is_file() {
            let entry_path = entry.path();

            if is_markdown(&entry_path) {
                let mut c = Content::default();
                c.file = Some(String::from(entry_path.as_path().to_str().unwrap()));
                c.init_from_file(root);
                let mut ct = ContentType::Normal;

                let is_index = check_type && is_index_file(&entry);
                let is_footer = check_type
                    && entry
                        .path()
                        .file_stem()
                        .and_then(|file_stem| file_stem.to_str())
                        .and_then(|file_name| Some(file_name.to_lowercase() == "footer"))
                        .unwrap_or_else(|| false);
                let is_header = check_type
                    && entry
                        .path()
                        .file_stem()
                        .and_then(|file_stem| file_stem.to_str())
                        .and_then(|file_name| Some(file_name.to_lowercase() == "header"))
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

    return None;
}

pub fn build(root: &Path) -> Result<Data, Box<dyn Error>> {
    let mut r = root;
    let current_dir = env::current_dir()?;
    let abs;
    if root.is_relative() {
        abs = current_dir.as_path().join(root).canonicalize()?;
        r = abs.as_path();
    }

    let path = config_file(r);
    let mut data: Data;
    if path.is_none() {
        data = Data::default();
    } else {
        let file = File::open(path.unwrap().as_path())?;
        let reader = BufReader::new(file);
        data = serde_json::from_reader(reader)?;
    }

    return match data.build(r) {
        Ok(()) => Ok(data),
        Err(e) => Err(e),
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init() {
        // empty
        let mut root = PathBuf::from("tests");
        let mut data = Data::default();
        assert!(data.init(&root).is_ok());
        let mut expected = Data::default();
        expected.title = Some(String::from("tests"));
        expected.contents = Some(vec![]); // initialized to empty
        assert_eq!(data, expected);

        // with subdirs
        root = PathBuf::from("tests/fixtures/data");
        data = Data::default();
        assert!(data.init(&root).is_ok());
        let mut expected_file =
            File::open("tests/fixtures/data/init_expected1.json").expect("could not open file");
        let mut reader = BufReader::new(expected_file);
        expected = serde_json::from_reader(reader).expect("could not read expected data");
        assert_eq!(data, expected);

        // with header and footer
        root = PathBuf::from("tests/fixtures/data/dir2");
        data = Data::default();
        assert!(data.init(&root).is_ok());
        expected_file =
            File::open("tests/fixtures/data/init_expected2.json").expect("could not open file");
        reader = BufReader::new(expected_file);
        expected = serde_json::from_reader(reader).expect("could not read expected data");
        assert_eq!(data, expected);

        // with pre-seeded data
        let seed_file =
            File::open("tests/fixtures/data/init_seed1.json").expect("could not open file");
        reader = BufReader::new(seed_file);
        data = serde_json::from_reader(reader).expect("could not read seed data");
        root = PathBuf::from("tests/fixtures/data/dir1");
        assert!(data.init(&root).is_ok());
        expected_file =
            File::open("tests/fixtures/data/init_expected3.json").expect("could not open file");
        reader = BufReader::new(expected_file);
        expected = serde_json::from_reader(reader).expect("could not read expected data");
        assert_eq!(data, expected);
    }
}
