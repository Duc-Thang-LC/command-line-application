use assert_cmd::prelude::*; // Add methods on command
use predicates::prelude::*; // Used for writing assertion
use std::process::Command; // Run programs
use assert_fs::prelude::*;

#[test]
fn file_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("command-line-application")?;

    cmd.arg("foobar").arg("test/file/doesnt/exist");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("could not read file"));

    Ok(())
}

#[test]
fn find_content_in_file() -> Result<(), Box<dyn std::error::Error>> {
    let file = assert_fs::NamedTempFile::new("sample.txt")?;
    file.write_str("Actual test content\nMore content\nAnother test")?;

    let mut cmd = Command::cargo_bin("command-line-application")?;
    cmd.arg("test").arg(file.path());
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("content\nAnother test"));

    Ok(())
}

#[test]
fn empty_pattern() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("command-line-application")?;
    //cmd.arg("").arg("");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("not provided"));

    Ok(())
}