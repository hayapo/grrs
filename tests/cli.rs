use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn file_does_not_exist() -> Result<(), Box<dyn std::error::Error>> {
  let mut cmd = Command::cargo_bin("grrs")?;

  cmd.arg("foobar").arg("test/file/does/not/exist");
  cmd.assert()
    .failure()
    .stderr(predicate::str::contains("No such file or directory"));

  Ok(())
}

use assert_fs::prelude::*;

#[test]
fn find_content_in_file() -> Result<(), Box<dyn std::error::Error>> {
  let file = assert_fs::NamedTempFile::new("sample.txt")?;
  file.write_str("A test\nActual content\nMore content\nStill more content")?;

  let mut cmd = Command::cargo_bin("grrs")?;
  cmd.arg("content").arg(file.path());
  cmd.assert()
    .success()
    .stdout(predicate::str::contains("Actual content\nMore content\nStill more content"));
  
  Ok(())
}

#[test]
fn empty_string_pattern() -> Result<(), Box<dyn std::error::Error>> {
  let file = assert_fs::NamedTempFile::new("sample.txt")?;
  file.write_str("A test\nActual content\nMore content\nStill more content")?;

  let mut cmd = Command::cargo_bin("grrs")?;
  cmd.arg("").arg(file.path());
  cmd.assert()
    .success()
    .stdout(predicate::str::contains("A test\nActual content\nMore content\nStill more content"));
  
  Ok(())
}