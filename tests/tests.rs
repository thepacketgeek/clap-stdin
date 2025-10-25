use std::fs;

use assert_cmd::Command;
use predicates::prelude::*;

use clap_stdin::StdinError;

#[test]
fn test_maybe_stdin_positional_arg() {
    Command::cargo_bin("maybe_stdin_positional_arg")
        .unwrap()
        .args(["FIRST", "--second", "SECOND"])
        .assert()
        .success()
        .stdout(predicate::str::starts_with(
            r#"Args { first: "FIRST", second: Some("SECOND") }"#,
        ));
    Command::cargo_bin("maybe_stdin_positional_arg")
        .unwrap()
        .args(["-", "--second", "SECOND"])
        .write_stdin("TESTING")
        .assert()
        .success()
        .stdout(predicate::str::starts_with(
            r#"Args { first: "TESTING", second: Some("SECOND") }"#,
        ));
    Command::cargo_bin("maybe_stdin_positional_arg")
        .unwrap()
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
    Command::cargo_bin("maybe_stdin_optional_arg")
        .unwrap()
        .args(["FIRST", "--second", "2"])
        .assert()
        .success()
        .stdout(predicate::str::starts_with(
            r#"Args { first: "FIRST", second: Some(2) }"#,
        ));
    Command::cargo_bin("maybe_stdin_optional_arg")
        .unwrap()
        .write_stdin("2\n")
        .args(["FIRST", "--second", "-"])
        .assert()
        .success()
        .stdout(predicate::str::starts_with(
            r#"Args { first: "FIRST", second: Some(2) }"#,
        ));
    Command::cargo_bin("maybe_stdin_optional_arg")
        .unwrap()
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
    Command::cargo_bin("maybe_stdin_twice")
        .unwrap()
        .args(["FIRST", "2"])
        .assert()
        .success()
        .stdout(predicate::str::starts_with(
            r#"Args { first: "FIRST", second: 2 }"#,
        ));
    Command::cargo_bin("maybe_stdin_twice")
        .unwrap()
        .write_stdin("2")
        .args(["FIRST", "-"])
        .assert()
        .success()
        .stdout(predicate::str::starts_with(
            r#"Args { first: "FIRST", second: 2 }"#,
        ));

    // Actually using stdin twice will fail because there's no value the second time
    Command::cargo_bin("maybe_stdin_twice")
        .unwrap()
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

    Command::cargo_bin("file_or_stdin_positional_arg")
        .unwrap()
        .args([&tmp_path, "--second", "SECOND"])
        .assert()
        .success()
        .stdout(predicate::str::starts_with(
            r#"FIRST: FILE; SECOND: Some("SECOND")"#,
        ));
    Command::cargo_bin("file_or_stdin_positional_arg")
        .unwrap()
        .args(["--second", "SECOND"])
        .write_stdin("STDIN")
        .assert()
        .success()
        .stdout(predicate::str::starts_with(
            r#"FIRST: STDIN; SECOND: Some("SECOND")"#,
        ));
    Command::cargo_bin("file_or_stdin_positional_arg")
        .unwrap()
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

    Command::cargo_bin("file_or_stdin_optional_arg")
        .unwrap()
        .args(["FIRST", "--second", &tmp_path])
        .assert()
        .success()
        .stdout(predicate::str::starts_with(
            r#"FIRST: FIRST, SECOND: Some(2)"#,
        ));
    Command::cargo_bin("file_or_stdin_optional_arg")
        .unwrap()
        .write_stdin("2\n")
        .args(["FIRST", "--second", "-"])
        .assert()
        .success()
        .stdout(predicate::str::starts_with(
            r#"FIRST: FIRST, SECOND: Some(2)"#,
        ));
    Command::cargo_bin("file_or_stdin_optional_arg")
        .unwrap()
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

    Command::cargo_bin("file_or_stdin_twice")
        .unwrap()
        .args([&tmp_path, "2"])
        .assert()
        .success()
        .stdout(predicate::str::starts_with(r#"FIRST: FILE; SECOND: 2"#));
    Command::cargo_bin("file_or_stdin_twice")
        .unwrap()
        .write_stdin("2")
        .args([&tmp_path, "-"])
        .assert()
        .success()
        .stdout(predicate::str::starts_with(r#"FIRST: FILE; SECOND: 2"#));

    // Actually using stdin twice will fail because there's no value the second time
    Command::cargo_bin("file_or_stdin_twice")
        .unwrap()
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

    Command::cargo_bin("is_stdin")
        .unwrap()
        .args([&tmp_path, "2"])
        .assert()
        .success()
        .stdout(predicate::str::contains(
            r#"FIRST is_stdin: false; SECOND is_stdin: false"#,
        ));
    Command::cargo_bin("is_stdin")
        .unwrap()
        .write_stdin("2")
        .args([&tmp_path, "-"])
        .assert()
        .success()
        .stdout(predicate::str::contains(
            r#"FIRST is_stdin: false; SECOND is_stdin: true"#,
        ));
    Command::cargo_bin("is_stdin")
        .unwrap()
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

    Command::cargo_bin("file_or_stdout_positional_arg")
        .unwrap()
        .args(["-v", "FILE", tmp_path])
        .assert()
        .success();
    let output = String::from_utf8_lossy(&std::fs::read(&tmp_path).unwrap()).to_string();
    assert_eq!(&output, "FILE\n");

    Command::cargo_bin("file_or_stdout_positional_arg")
        .unwrap()
        .args(["-v", "FILE", "-"])
        .assert()
        .success()
        .stdout(predicate::str::starts_with(r#"FILE"#));

    Command::cargo_bin("file_or_stdout_positional_arg")
        .unwrap()
        .args(["-v", "FILE"])
        .assert()
        .success()
        .stdout(predicate::str::starts_with(r#"FILE"#));
}

#[test]
fn test_file_or_stdout_optional_args() {
    let tmp = tempfile::NamedTempFile::new().expect("couldn't create temp file");
    let tmp_path = tmp.path().to_str().unwrap();

    Command::cargo_bin("file_or_stdout_optional_arg")
        .unwrap()
        .args(["-v", "FILE", "--output", tmp_path])
        .assert()
        .success();
    let output = String::from_utf8_lossy(&std::fs::read(&tmp_path).unwrap()).to_string();
    assert_eq!(&output, "FILE\n");

    Command::cargo_bin("file_or_stdout_optional_arg")
        .unwrap()
        .args(["-v", "FILE", "--output", "-"])
        .assert()
        .success()
        .stdout(predicate::str::starts_with(r#"FILE"#));

    Command::cargo_bin("file_or_stdout_optional_arg")
        .unwrap()
        .args(["-v", "FILE"])
        .assert()
        .success()
        .stdout(predicate::str::starts_with(r#"FILE"#));
}
