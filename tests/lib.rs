use assert_cmd::crate_name;
use assert_cmd::prelude::*;
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
