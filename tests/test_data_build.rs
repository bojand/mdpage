use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

#[test]
fn test_data_build() -> Result<(), Box<dyn std::error::Error>> {
    // empty
    let mut root = PathBuf::from("tests");
    let mut data = mdpage::build(&root)?;
    let mut expected = mdpage::Data::default();
    expected.title = Some(String::from("tests"));
    expected.contents = Some(vec![]); // initialized to empty
    assert_eq!(data, expected);

    // with subdirs
    root = PathBuf::from("tests/fixtures/data");
    data = mdpage::build(&root)?;
    let mut expected_file = File::open("tests/build_expected1.json")?;
    let mut reader = BufReader::new(expected_file);
    expected = serde_json::from_reader(reader)?;
    assert_eq!(data, expected);

    // with header and footer
    root = PathBuf::from("tests/fixtures/data/dir2");
    data = mdpage::build(&root)?;
    expected_file = File::open("tests/build_expected2.json")?;
    reader = BufReader::new(expected_file);
    expected = serde_json::from_reader(reader)?;
    assert_eq!(data, expected);

    // with mdpage.json
    root = PathBuf::from("tests/fixtures/data/dir1");
    data = mdpage::build(&root)?;
    expected_file = File::open("tests/build_expected3.json")?;
    reader = BufReader::new(expected_file);
    expected = serde_json::from_reader(reader)?;
    assert_eq!(data, expected);

    Ok(())
}
