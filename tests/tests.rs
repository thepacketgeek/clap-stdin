use std::fs;

use assert_cmd::cargo::cargo_bin;
use assert_cmd::Command;
use predicates::prelude::*;

use clap_stdin::StdinError;

#[test]
fn test_maybe_stdin_positional_arg() {
    Command::new(cargo_bin("maybe_stdin_positional_arg"))
        .args(["FIRST", "--second", "SECOND"])
        .assert()
        .success()
        .stdout(predicate::str::starts_with(
            r#"Args { first: "FIRST", second: Some("SECOND") }"#,
        ));
    Command::new(cargo_bin("maybe_stdin_positional_arg"))
        .args(["-", "--second", "SECOND"])
        .write_stdin("TESTING")
        .assert()
        .success()
        .stdout(predicate::str::starts_with(
            r#"Args { first: "TESTING", second: Some("SECOND") }"#,
        ));
    Command::new(cargo_bin("maybe_stdin_positional_arg"))
        .args(["FIRST"])
        .write_stdin("TESTING")
        .assert()
        .success()
        .stdout(predicate::str::starts_with(
            r#"Args { first: "FIRST", second: None }"#,
        ));
}

#[test]
fn test_maybe_stdin_optional_arg() {
    Command::new(cargo_bin("maybe_stdin_optional_arg"))
        .args(["FIRST", "--second", "2"])
        .assert()
        .success()
        .stdout(predicate::str::starts_with(
            r#"Args { first: "FIRST", second: Some(2) }"#,
        ));
    Command::new(cargo_bin("maybe_stdin_optional_arg"))
        .write_stdin("2\n")
        .args(["FIRST", "--second", "-"])
        .assert()
        .success()
        .stdout(predicate::str::starts_with(
            r#"Args { first: "FIRST", second: Some(2) }"#,
        ));
    Command::new(cargo_bin("maybe_stdin_optional_arg"))
        .args(["FIRST"])
        .write_stdin("TESTING")
        .assert()
        .success()
        .stdout(predicate::str::starts_with(
            r#"Args { first: "FIRST", second: None }"#,
        ));
}

#[test]
fn test_maybe_stdin_twice() {
    Command::new(cargo_bin("maybe_stdin_twice"))
        .args(["FIRST", "2"])
        .assert()
        .success()
        .stdout(predicate::str::starts_with(
            r#"Args { first: "FIRST", second: 2 }"#,
        ));
    Command::new(cargo_bin("maybe_stdin_twice"))
        .write_stdin("2")
        .args(["FIRST", "-"])
        .assert()
        .success()
        .stdout(predicate::str::starts_with(
            r#"Args { first: "FIRST", second: 2 }"#,
        ));

    // Actually using stdin twice will fail because there's no value the second time
    Command::new(cargo_bin("maybe_stdin_twice"))
        .write_stdin("3")
        .args(["-", "-"])
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            StdinError::StdInRepeatedUse.to_string(),
        ));
}

#[test]
fn test_file_or_stdin_positional_arg() {
    let tmp = tempfile::NamedTempFile::new().expect("couldn't create temp file");
    fs::write(&tmp, "FILE").expect("couldn't write to temp file");
    let tmp_path = tmp.path().to_str().unwrap();

    Command::new(cargo_bin("file_or_stdin_positional_arg"))
        .args([&tmp_path, "--second", "SECOND"])
        .assert()
        .success()
        .stdout(predicate::str::starts_with(
            r#"FIRST: FILE; SECOND: Some("SECOND")"#,
        ));
    Command::new(cargo_bin("file_or_stdin_positional_arg"))
        .args(["--second", "SECOND"])
        .write_stdin("STDIN")
        .assert()
        .success()
        .stdout(predicate::str::starts_with(
            r#"FIRST: STDIN; SECOND: Some("SECOND")"#,
        ));
    Command::new(cargo_bin("file_or_stdin_positional_arg"))
        .args([&tmp_path])
        .write_stdin("TESTING")
        .assert()
        .success()
        .stdout(predicate::str::starts_with(r#"FIRST: FILE; SECOND: None"#));
}

#[test]
fn test_file_or_stdin_optional_arg() {
    let tmp = tempfile::NamedTempFile::new().expect("couldn't create temp file");
    // In this case, --second is `Option<FileOrStdin<u32>>` so we'll have a number in the file
    fs::write(&tmp, "2").expect("couldn't write to temp file");
    let tmp_path = tmp.path().to_str().unwrap();

    Command::new(cargo_bin("file_or_stdin_optional_arg"))
        .args(["FIRST", "--second", &tmp_path])
        .assert()
        .success()
        .stdout(predicate::str::starts_with(
            r#"FIRST: FIRST, SECOND: Some(2)"#,
        ));
    Command::new(cargo_bin("file_or_stdin_optional_arg"))
        .write_stdin("2\n")
        .args(["FIRST", "--second", "-"])
        .assert()
        .success()
        .stdout(predicate::str::starts_with(
            r#"FIRST: FIRST, SECOND: Some(2)"#,
        ));
    Command::new(cargo_bin("file_or_stdin_optional_arg"))
        .args(["FIRST"])
        .write_stdin("TESTING")
        .assert()
        .success()
        .stdout(predicate::str::starts_with(r#"FIRST: FIRST, SECOND: None"#));
}

#[test]
fn test_file_or_stdin_twice() {
    let tmp = tempfile::NamedTempFile::new().expect("couldn't create temp file");
    fs::write(&tmp, "FILE").expect("couldn't write to temp file");
    let tmp_path = tmp.path().to_str().unwrap();

    Command::new(cargo_bin("file_or_stdin_twice"))
        .args([&tmp_path, "2"])
        .assert()
        .success()
        .stdout(predicate::str::starts_with(r#"FIRST: FILE; SECOND: 2"#));
    Command::new(cargo_bin("file_or_stdin_twice"))
        .write_stdin("2")
        .args([&tmp_path, "-"])
        .assert()
        .success()
        .stdout(predicate::str::starts_with(r#"FIRST: FILE; SECOND: 2"#));

    // Actually using stdin twice will fail because there's no value the second time
    Command::new(cargo_bin("file_or_stdin_twice"))
        .write_stdin("3")
        .args(["-", "-"])
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            StdinError::StdInRepeatedUse.to_string(),
        ));
}

#[test]
fn test_is_stdin() {
    let tmp = tempfile::NamedTempFile::new().expect("couldn't create temp file");
    fs::write(&tmp, "FILE").expect("couldn't write to temp file");
    let tmp_path = tmp.path().to_str().unwrap();

    Command::new(cargo_bin("is_stdin"))
        .args([&tmp_path, "2"])
        .assert()
        .success()
        .stdout(predicate::str::contains(
            r#"FIRST is_stdin: false; SECOND is_stdin: false"#,
        ));
    Command::new(cargo_bin("is_stdin"))
        .write_stdin("2")
        .args([&tmp_path, "-"])
        .assert()
        .success()
        .stdout(predicate::str::contains(
            r#"FIRST is_stdin: false; SECOND is_stdin: true"#,
        ));
    Command::new(cargo_bin("is_stdin"))
        .write_stdin("testing")
        .args(["-", "2"])
        .assert()
        .success()
        .stdout(predicate::str::contains(
            r#"FIRST is_stdin: true; SECOND is_stdin: false"#,
        ));
}

#[test]
fn test_file_or_stdout_positional_args() {
    let tmp = tempfile::NamedTempFile::new().expect("couldn't create temp file");
    let tmp_path = tmp.path().to_str().unwrap();

    Command::new(cargo_bin("file_or_stdout_positional_arg"))
        .args(["-v", "FILE", tmp_path])
        .assert()
        .success();
    let output = String::from_utf8_lossy(&std::fs::read(&tmp_path).unwrap()).to_string();
    assert_eq!(&output, "FILE\n");

    Command::new(cargo_bin("file_or_stdout_positional_arg"))
        .args(["-v", "FILE", "-"])
        .assert()
        .success()
        .stdout(predicate::str::starts_with(r#"FILE"#));

    Command::new(cargo_bin("file_or_stdout_positional_arg"))
        .args(["-v", "FILE"])
        .assert()
        .success()
        .stdout(predicate::str::starts_with(r#"FILE"#));
}

#[test]
fn test_file_or_stdout_optional_args() {
    let tmp = tempfile::NamedTempFile::new().expect("couldn't create temp file");
    let tmp_path = tmp.path().to_str().unwrap();

    Command::new(cargo_bin("file_or_stdout_optional_arg"))
        .args(["-v", "FILE", "--output", tmp_path])
        .assert()
        .success();
    let output = String::from_utf8_lossy(&std::fs::read(&tmp_path).unwrap()).to_string();
    assert_eq!(&output, "FILE\n");

    Command::new(cargo_bin("file_or_stdout_optional_arg"))
        .args(["-v", "FILE", "--output", "-"])
        .assert()
        .success()
        .stdout(predicate::str::starts_with(r#"FILE"#));

    Command::new(cargo_bin("file_or_stdout_optional_arg"))
        .args(["-v", "FILE"])
        .assert()
        .success()
        .stdout(predicate::str::starts_with(r#"FILE"#));
}

#[test]
fn test_file_or_stdout_truncate_overwrites() {
    let tmp = tempfile::NamedTempFile::new().expect("couldn't create temp file");
    let tmp_path = tmp.path().to_str().unwrap();

    // Write initial content
    fs::write(&tmp, "EXISTING CONTENT").expect("couldn't write to temp file");

    // Write with default (truncate) mode
    Command::new(cargo_bin("file_or_stdout_positional_arg"))
        .args(["-v", "NEW", tmp_path])
        .assert()
        .success();
    let output = String::from_utf8_lossy(&fs::read(&tmp_path).unwrap()).to_string();
    assert_eq!(&output, "NEW\n");
}

#[test]
fn test_file_or_stdout_append_positional_args() {
    let tmp = tempfile::NamedTempFile::new().expect("couldn't create temp file");
    let tmp_path = tmp.path().to_str().unwrap();

    // Write initial content
    fs::write(&tmp, "EXISTING\n").expect("couldn't write to temp file");

    // Append mode should add to existing content
    Command::new(cargo_bin("file_or_stdout_append_positional_arg"))
        .args(["-v", "APPENDED", tmp_path])
        .assert()
        .success();
    let output = String::from_utf8_lossy(&fs::read(&tmp_path).unwrap()).to_string();
    assert_eq!(&output, "EXISTING\nAPPENDED\n");

    // Stdout mode should still work
    Command::new(cargo_bin("file_or_stdout_append_positional_arg"))
        .args(["-v", "FILE", "-"])
        .assert()
        .success()
        .stdout(predicate::str::starts_with(r#"FILE"#));
}

#[test]
fn test_file_or_stdout_append_optional_args() {
    let tmp = tempfile::NamedTempFile::new().expect("couldn't create temp file");
    let tmp_path = tmp.path().to_str().unwrap();

    // Write initial content
    fs::write(&tmp, "EXISTING\n").expect("couldn't write to temp file");

    // Append mode should add to existing content
    Command::new(cargo_bin("file_or_stdout_append_optional_arg"))
        .args(["-v", "APPENDED", "--output", tmp_path])
        .assert()
        .success();
    let output = String::from_utf8_lossy(&fs::read(&tmp_path).unwrap()).to_string();
    assert_eq!(&output, "EXISTING\nAPPENDED\n");

    // Stdout mode should still work
    Command::new(cargo_bin("file_or_stdout_append_optional_arg"))
        .args(["-v", "FILE", "--output", "-"])
        .assert()
        .success()
        .stdout(predicate::str::starts_with(r#"FILE"#));

    // Default to stdout when not specified
    Command::new(cargo_bin("file_or_stdout_append_optional_arg"))
        .args(["-v", "FILE"])
        .assert()
        .success()
        .stdout(predicate::str::starts_with(r#"FILE"#));
}
