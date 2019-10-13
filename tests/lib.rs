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
