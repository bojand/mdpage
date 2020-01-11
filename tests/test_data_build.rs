use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

#[test]
fn test_data_build() -> Result<(), Box<dyn std::error::Error>> {
    // empty
    let mut root = PathBuf::from("tests");
    let mut data = mdpage::build(&root, None)?;
    let mut expected = mdpage::Data::default();
    expected.title = Some(String::from("tests"));
    expected.contents = Some(vec![]); // initialized to empty
    assert_eq!(data, expected);

    // with subdirs
    root = PathBuf::from("tests/fixtures/data");
    data = mdpage::build(&root, None)?;
    let mut expected_file = File::open("tests/build_expected1.json")?;
    let mut reader = BufReader::new(expected_file);
    expected = serde_json::from_reader(reader)?;
    assert_eq!(data, expected);

    // with header and footer
    root = PathBuf::from("tests/fixtures/data/dir2");
    data = mdpage::build(&root, None)?;
    expected_file = File::open("tests/build_expected2.json")?;
    reader = BufReader::new(expected_file);
    expected = serde_json::from_reader(reader)?;
    assert_eq!(data, expected);

    // with mdpage.json
    root = PathBuf::from("tests/fixtures/data/dir1");
    data = mdpage::build(&root, None)?;
    expected_file = File::open("tests/build_expected3.json")?;
    reader = BufReader::new(expected_file);
    expected = serde_json::from_reader(reader)?;
    assert_eq!(data, expected);

    // with subdirs and initial data
    root = PathBuf::from("tests/fixtures/data");
    data = mdpage::build(
        &root,
        Some(mdpage::Data {
            subtitle: Some(String::from("Custom subtitle")),
            full_page: Some(true),
            ..mdpage::Data::default()
        }),
    )?;
    expected_file = File::open("tests/build_expected4.json")?;
    reader = BufReader::new(expected_file);
    expected = serde_json::from_reader(reader)?;
    assert_eq!(data, expected);

    // single index
    root = PathBuf::from("docs/examples/single_index");
    data = mdpage::build(&root, None)?;
    expected_file = File::open("tests/build_expected_single.json")?;
    reader = BufReader::new(expected_file);
    expected = serde_json::from_reader(reader)?;
    assert_eq!(data, expected);

    // single page
    root = PathBuf::from("docs/examples/single_page");
    data = mdpage::build(&root, None)?;
    expected_file = File::open("tests/build_expected_single_page.json")?;
    reader = BufReader::new(expected_file);
    expected = serde_json::from_reader(reader)?;
    assert_eq!(data, expected);

    // HTML test
    root = PathBuf::from("tests/fixtures/data/html");
    data = mdpage::build(&root, None)?;
    expected_file = File::open("tests/build_expected_html.json")?;
    reader = BufReader::new(expected_file);
    expected = serde_json::from_reader(reader)?;
    assert_eq!(data, expected);

    // dir test
    root = PathBuf::from("docs/examples/config_dir");
    data = mdpage::build(&root, None)?;
    expected_file = File::open("tests/build_expected_dir.json")?;
    reader = BufReader::new(expected_file);
    expected = serde_json::from_reader(reader)?;
    assert_eq!(data, expected);

    Ok(())
}
