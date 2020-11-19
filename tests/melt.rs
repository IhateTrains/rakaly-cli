mod utils;

use assert_cmd::Command;
use std::io::BufReader;
use std::{fs::File, io::Cursor};

#[test]
fn test_eu4_melt() {
    let file = utils::request("eu4saves-test-cases", "kandy2.bin.eu4");
    let mut cmd = Command::cargo_bin("rakaly").unwrap();
    let assert = cmd.arg("melt").arg(&file).assert();

    assert.success();

    let melted_path = file.with_file_name("kandy2.bin_melted.eu4");
    assert!(melted_path.exists());

    let melted_file = File::open(&melted_path).unwrap();
    let (_, enc) = eu4save::Eu4Extractor::extract_save(BufReader::new(melted_file)).unwrap();
    assert_eq!(enc, eu4save::Encoding::Text)
}

#[test]
fn test_eu4_melt_stdout() {
    let file = utils::request("eu4saves-test-cases", "kandy2.bin.eu4");
    let mut cmd = Command::cargo_bin("rakaly").unwrap();
    let assert = cmd.arg("melt").arg("--to-stdout").arg(&file).assert();

    let out = assert.get_output();
    let stdout = &out.stdout;
    let (_, enc) = eu4save::Eu4Extractor::extract_save(Cursor::new(stdout)).unwrap();
    assert_eq!(enc, eu4save::Encoding::Text)
}
