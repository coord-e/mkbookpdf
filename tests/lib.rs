use assert_cmd::crate_name;
use assert_cmd::prelude::*;
use assert_fs::prelude::*;
use assert_fs::NamedTempFile;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn test_empty_args() {
    Command::cargo_bin(crate_name!())
        .unwrap()
        .assert()
        .failure();
}

#[test]
fn test_no_output() {
    Command::cargo_bin(crate_name!())
        .unwrap()
        .arg("tests/data/sample.pdf")
        .assert()
        .failure();
}

#[test]
fn test_output() {
    let temp = NamedTempFile::new("tmp.pdf").unwrap();
    let path = temp.path();

    Command::cargo_bin(crate_name!())
        .unwrap()
        .arg("tests/data/sample.pdf")
        .args(&["-o", path.to_str().unwrap()])
        .assert()
        .success();

    temp.assert(predicate::function(|x: &[u8]| !x.is_empty()).from_file_path());
}

#[test]
fn test_print() {
    Command::cargo_bin(crate_name!())
        .unwrap()
        .env("MKBL_LP", "echo")
        .arg("tests/data/sample.pdf")
        .arg("--print")
        .assert()
        .success()
        .stdout(predicate::str::contains("-o sides=two-sided-short-edge"))
        .stdout(predicate::str::contains("-o number-up=2"))
        .stdout(predicate::str::contains("-o number-up-layout=lrtb"));
}

#[test]
fn test_print_output() {
    let temp = NamedTempFile::new("tmp.pdf").unwrap();
    let path = temp.path();

    Command::cargo_bin(crate_name!())
        .unwrap()
        .env("MKBL_LP", "echo")
        .arg("tests/data/sample.pdf")
        .args(&["-o", path.to_str().unwrap()])
        .arg("--print")
        .assert()
        .success()
        .stdout(predicate::str::contains(path.to_str().unwrap()));

    temp.assert(predicate::function(|x: &[u8]| !x.is_empty()).from_file_path());
}

#[test]
fn test_print_name() {
    let name: &str = "MY_PRINTER";

    Command::cargo_bin(crate_name!())
        .unwrap()
        .env("MKBL_LP", "echo")
        .arg("tests/data/sample.pdf")
        .args(&["--print", name])
        .assert()
        .success()
        .stdout(predicate::str::contains(name));
}
