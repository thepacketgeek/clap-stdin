use assert_cmd::Command;
use predicates::prelude::*;

use clap_stdin::StdInError;

#[test]
fn test_positional_arg() {
    Command::cargo_bin("positional_arg")
        .unwrap()
        .args(["FIRST", "--second", "SECOND"])
        .assert()
        .success()
        .stdout(predicate::str::starts_with(
            r#"Args { first: "FIRST", second: Some("SECOND") }"#,
        ));
    Command::cargo_bin("positional_arg")
        .unwrap()
        .args(["-", "--second", "SECOND"])
        .write_stdin("TESTING")
        .assert()
        .success()
        .stdout(predicate::str::starts_with(
            r#"Args { first: "TESTING", second: Some("SECOND") }"#,
        ));
    Command::cargo_bin("positional_arg")
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
fn test_optional_arg() {
    Command::cargo_bin("optional_arg")
        .unwrap()
        .args(["FIRST", "--second", "2"])
        .assert()
        .success()
        .stdout(predicate::str::starts_with(
            r#"Args { first: "FIRST", second: Some(2) }"#,
        ));
    Command::cargo_bin("optional_arg")
        .unwrap()
        .write_stdin("2\n")
        .args(["FIRST", "--second", "-"])
        .assert()
        .success()
        .stdout(predicate::str::starts_with(
            r#"Args { first: "FIRST", second: Some(2) }"#,
        ));
    Command::cargo_bin("optional_arg")
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
fn test_stdin_twice() {
    Command::cargo_bin("stdin_twice")
        .unwrap()
        .args(["FIRST", "2"])
        .assert()
        .success()
        .stdout(predicate::str::starts_with(
            r#"Args { first: "FIRST", second: 2 }"#,
        ));
    Command::cargo_bin("stdin_twice")
        .unwrap()
        .write_stdin("2")
        .args(["FIRST", "-"])
        .assert()
        .success()
        .stdout(predicate::str::starts_with(
            r#"Args { first: "FIRST", second: 2 }"#,
        ));

    // Actually using stdin twice will fail because there's no value the second time
    Command::cargo_bin("stdin_twice")
        .unwrap()
        .write_stdin("3")
        .args(["-", "-"])
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            StdInError::StdInRepeatedUse.to_string(),
        ));
}
