use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

pub fn title_string<R>(mut rdr: R) -> Option<String>
where
    R: BufRead,
{
    let mut line = String::new();

    while rdr.read_line(&mut line).expect("Unable to read line") > 0 {
        let mut trimmed = line.trim();
        if trimmed.starts_with('#') {
            trimmed = trimmed.trim_start_matches('#').trim_matches(' ');
            if !trimmed.is_empty() {
                return Some(trimmed.into());
            }
        }
    }

    None
}

pub fn build_title_for_dir(
    root: &Path,
    mut paths: fs::ReadDir,
    check_index: bool,
) -> Result<String, Box<dyn Error>> {
    let mut res = String::new();

    if let Some(file_name) = root.file_stem() {
        if let Some(title) = file_name.to_str() {
            res = String::from(title);
        }
    }

    if check_index {
        let index = paths.find(|p| {
            if let Ok(entry) = p {
                return is_index_file(entry);
            }
            false
        });

        if let Some(index_value) = index {
            let index_path = index_value.unwrap().path();

            if let Some(title) = get_title_from_file(&index_path, false)? {
                res = title;
            }
        }
    }

    Ok(res)
}

pub fn get_title_from_file(
    path: &Path,
    use_file_name: bool,
) -> Result<Option<String>, Box<dyn Error>> {
    let mut res = None;

    if use_file_name {
        if let Some(name_str) = path.file_stem().and_then(|name_str| name_str.to_str()) {
            res = Some(String::from(name_str));
        }
    }

    let file = File::open(path)?;

    let buffer = BufReader::new(file);

    let from_file = title_string(buffer);
    if from_file.is_some() {
        res = from_file;
    }

    Ok(res)
}

pub fn is_index_file(entry: &std::fs::DirEntry) -> bool {
    if let Ok(file_type) = entry.file_type() {
        if file_type.is_file() {
            return entry
                .path()
                .file_stem()
                .and_then(|file_stem| file_stem.to_str())
                .map(|file_name| file_name.to_lowercase())
                .map(|file_name| file_name == "index" || file_name == "readme")
                .unwrap_or_else(|| false);
        }
    }

    false
}

pub fn is_ext(path: &Path, ext: &str) -> bool {
    path.extension()
        .unwrap_or_default()
        .to_str()
        .unwrap_or_default()
        .to_lowercase()
        == ext
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::path::PathBuf;

    #[test]
    fn test_title_string() {
        assert_eq!(title_string("Hello world!".as_bytes()), None);
        assert_eq!(
            title_string("#Hello world!".as_bytes()).unwrap(),
            "Hello world!"
        );
        assert_eq!(
            title_string("# Hello world!".as_bytes()).unwrap(),
            "Hello world!"
        );
        assert_eq!(
            title_string("##Hello world!".as_bytes()).unwrap(),
            "Hello world!"
        );
        assert_eq!(
            title_string("## Hello world!".as_bytes()).unwrap(),
            "Hello world!"
        );
        assert_eq!(
            title_string("###   Hello world!   ".as_bytes()).unwrap(),
            "Hello world!"
        );
        assert_eq!(
            title_string("      ###   Hello world!   ".as_bytes()).unwrap(),
            "Hello world!"
        );
    }

    #[test]
    fn test_get_title_from_file() {
        let mut path = PathBuf::from("tests/fixtures/utils/title.md");

        assert_eq!(
            get_title_from_file(&path, false).unwrap().unwrap(),
            "Hello world!"
        );

        assert_eq!(
            get_title_from_file(&path, true).unwrap().unwrap(),
            "Hello world!"
        );

        path = PathBuf::from("tests/fixtures/utils/title2.md");

        assert_eq!(get_title_from_file(&path, false).unwrap(), None);

        assert_eq!(get_title_from_file(&path, true).unwrap().unwrap(), "title2");

        path = PathBuf::from("tests/fixtures/utils/unknown_file_test.md");

        assert!(get_title_from_file(&path, false).is_err());
    }

    #[test]
    fn test_build_title_for_dir() {
        let mut root = PathBuf::from("tests/fixtures");
        assert_eq!(
            build_title_for_dir(&root, fs::read_dir(&root).unwrap(), false).unwrap(),
            "fixtures"
        );
        assert_eq!(
            build_title_for_dir(&root, fs::read_dir(&root).unwrap(), true).unwrap(),
            "fixtures"
        );

        root = PathBuf::from("tests/fixtures/utils");
        assert_eq!(
            build_title_for_dir(&root, fs::read_dir(&root).unwrap(), false).unwrap(),
            "utils"
        );
        assert_eq!(
            build_title_for_dir(&root, fs::read_dir(&root).unwrap(), true).unwrap(),
            "utils"
        );

        root = PathBuf::from("tests/fixtures/utils2");
        assert_eq!(
            build_title_for_dir(&root, fs::read_dir(&root).unwrap(), false).unwrap(),
            "utils2"
        );
        assert_eq!(
            build_title_for_dir(&root, fs::read_dir(&root).unwrap(), true).unwrap(),
            "Main index"
        );

        root = PathBuf::from("tests/fixtures/utils3");
        assert_eq!(
            build_title_for_dir(&root, fs::read_dir(&root).unwrap(), false).unwrap(),
            "utils3"
        );
        assert_eq!(
            build_title_for_dir(&root, fs::read_dir(&root).unwrap(), true).unwrap(),
            "Main page"
        );
    }

    #[test]
    fn test_is_index_file() {
        let mut root = PathBuf::from("tests/fixtures");
        if let Ok(entries) = fs::read_dir(root) {
            for entry in entries {
                assert_eq!(is_index_file(&entry.unwrap()), false);
            }
        }

        root = PathBuf::from("tests/fixtures/utils");
        if let Ok(entries) = fs::read_dir(root) {
            for entry in entries {
                assert_eq!(is_index_file(&entry.unwrap()), false);
            }
        }

        // this is flaky

        root = PathBuf::from("tests/fixtures/utils2");
        let mut paths = fs::read_dir(root).unwrap();
        paths.next(); // consume 1
        assert_eq!(is_index_file(&paths.next().unwrap().unwrap()), true);

        root = PathBuf::from("tests/fixtures/utils3");
        paths = fs::read_dir(root).unwrap();
        paths.next(); // consume 1
        assert_eq!(is_index_file(&paths.next().unwrap().unwrap()), true);
    }
}
