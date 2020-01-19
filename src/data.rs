//! Data serves both as the configuration data for mdPage
//! as well as the actual template data for generating content.

use std::env;
use std::error::Error;

use std::fs;
use std::fs::File;

use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::content::{
    init_dir_contents, init_dir_sections, init_entry_contents, Content, ContentType,
};
use crate::utils::{build_title_for_dir, is_ext};

/// Data serves both as the configuration data for mdPage
/// as well as the actual template data for generating content.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Data {
    /// Whether to do full page or not.
    pub full_page: Option<bool>,
    /// Title used in header and title of the document.
    pub title: Option<String>,
    /// Subtitle used in header.
    pub subtitle: Option<String>,
    /// Author used in metadata.
    pub author: Option<String>,
    /// The favicon link.
    pub icon: Option<String>,
    /// The main content used for the front page.
    pub main: Option<Content>,
    /// The content of the document.
    pub contents: Option<Vec<Content>>,
    /// The custom JavaScript to be added in the `script` tag.
    pub script: Option<String>,
    /// The custom CSS to be added in the `style` tag.
    pub style: Option<String>,
    /// The custom style and script links.
    pub links: Option<Vec<Link>>,
    /// Custom header content.
    pub header: Option<Content>,
    /// Custom footer content.
    pub footer: Option<Content>,
}

/// Link represents a link we can insert into the head of the generated document.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Link {
    /// Link type.
    pub link_type: Option<LinkType>,
    /// The link source.
    pub src: Option<String>,
    /// Optional integrity for SRI.
    pub integrity: Option<String>,
    /// Optional crossorigin for SRI.
    pub crossorigin: Option<String>,
}

/// Link Type enum.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum LinkType {
    Style,
    Script,
}

impl LinkType {
    pub fn from_str(s: &str) -> Option<LinkType> {
        match s {
            "style" => Some(LinkType::Style),
            "stylesheet" => Some(LinkType::Style),
            "script" => Some(LinkType::Script),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            LinkType::Style => "stylesheet",
            LinkType::Script => "script",
        }
    }
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
            self.title = Some(build_title_for_dir(
                root,
                fs::read_dir(root).map_err(|err| {
                    format!("Error reading dir: {}. {}", root.display(), err.to_string())
                })?,
                true,
            )?);
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

        let mut main = None;
        let mut header = None;
        let mut footer = None;

        let paths = fs::read_dir(root)
            .map_err(|err| format!("Error reading dir: {}. {}", root.display(), err.to_string()))?;

        let mut res = paths
            .filter_map(|p| {
                if let Ok(entry) = p {
                    if let Ok(file_type) = entry.file_type() {
                        if file_type.is_file() {
                            return init_entry_contents(root, entry, true).and_then(|(c, ct)| {
                                match ct {
                                    ContentType::Main => {
                                        main = Some(c);
                                        None
                                    }
                                    ContentType::Footer => {
                                        footer = Some(c);
                                        None
                                    }
                                    ContentType::Header => {
                                        header = Some(c);
                                        None
                                    }
                                    _ => Some(vec![c]),
                                }
                            });
                        }
                    }
                }
                None
            })
            .flatten()
            .collect::<Vec<Content>>();

        res.sort_by(|a, b| a.file.cmp(&b.file));

        let mut sections = init_dir_sections(root)?;

        if !res.is_empty() {
            res.push(Content::new_break());
        }
        res.append(&mut sections);

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

        Ok(())
    }

    fn build_contents(&mut self, root: &Path) -> Result<(), Box<dyn Error>> {
        if self.contents.is_some() {
            let mut contents = self.contents.as_mut().unwrap();

            let has_dir = contents.iter().any(|c| c.dir.is_some());
            if has_dir {
                let mut expanded_contents = Vec::new();
                let mut index = 0;
                while index < contents.len() {
                    // fix dir entries
                    if contents[index].dir.is_some() {
                        let mut pathbuf = contents[index].dir.clone().unwrap();

                        if root.has_root() && pathbuf.is_relative() {
                            pathbuf = root.join(&pathbuf).canonicalize().unwrap_or_else(|_| {
                                panic!(
                                    "could not resolve path. root: {} path: {}",
                                    root.display(),
                                    pathbuf.display()
                                )
                            });
                        }

                        if pathbuf.is_dir() {
                            let mut dir_contents = Vec::new();

                            // get the base files
                            if let Some(mut root_dir_contents) = init_dir_contents(root, &pathbuf) {
                                dir_contents.append(&mut root_dir_contents);
                            }

                            // do subdirs
                            let mut sub_dir_contents = init_dir_sections(&pathbuf)?;
                            dir_contents.append(&mut sub_dir_contents);

                            // add into the overall
                            let mut di = 0;
                            while di < dir_contents.len() {
                                expanded_contents.push(dir_contents[di].clone());
                                di += 1;
                            }
                        }
                    } else {
                        expanded_contents.push(contents[index].clone());
                    }

                    index += 1;
                }

                self.contents = Some(expanded_contents);
            }

            contents = self.contents.as_mut().unwrap();

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

        Ok(())
    }
}

fn config_file(root: &Path) -> Option<PathBuf> {
    let mut r = Path::new(root);
    let json_config = r.join("mdpage.json");
    if json_config.as_path().exists() {
        Some(json_config)
    } else {
        r = Path::new(root);
        let toml_config = r.join("mdpage.toml");
        if toml_config.as_path().exists() {
            Some(toml_config)
        } else {
            None
        }
    }
}

/// Build the content data from a root directory path and optional initial value.
pub fn build(root: &Path, initial_value: Option<Data>) -> Result<Data, Box<dyn Error>> {
    let mut r = root;
    let current_dir = env::current_dir()?;
    let abs;
    if root.is_relative() {
        abs = current_dir
            .as_path()
            .join(root)
            .canonicalize()
            .map_err(|err| {
                format!(
                    "could not join current dir {} with path: {}. {}",
                    current_dir.display(),
                    root.display(),
                    err.to_string()
                )
            })?;
        r = abs.as_path();
    }

    let path = config_file(r);
    let mut data = initial_value.unwrap_or_default();

    if let Some(file_path) = path {
        info!("reading config: {}", file_path.display());
        let mut file = File::open(file_path.as_path()).map_err(|err| {
            format!(
                "Error reading file: {}. {}",
                file_path.display(),
                err.to_string()
            )
        })?;
        if is_ext(&file_path, "json") {
            let reader = BufReader::new(file);
            data = serde_json::from_reader(reader).map_err(|err| {
                format!(
                    "Error reading json: {}. {}",
                    file_path.display(),
                    err.to_string()
                )
            })?;
        } else if is_ext(&file_path, "toml") {
            let mut content = String::new();
            file.read_to_string(&mut content).map_err(|err| {
                format!(
                    "Error reading file: {}. {}",
                    file_path.display(),
                    err.to_string()
                )
            })?;
            data = toml::from_str(&content).map_err(|err| {
                format!(
                    "Error reading toml: {}. {}",
                    file_path.display(),
                    err.to_string()
                )
            })?;
        }
    }

    match data.build(r) {
        Ok(()) => Ok(data),
        Err(e) => Err(e),
    }
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

        // just single index
        root = PathBuf::from("docs/examples/single_index");
        data = Data::default();
        assert!(data.init(&root).is_ok());
        expected_file = File::open("tests/fixtures/data/init_expected_single.json")
            .expect("could not open file");
        reader = BufReader::new(expected_file);
        expected = serde_json::from_reader(reader).expect("could not read expected data");
        assert_eq!(data, expected);
    }
}
