use assert_cmd::crate_name;
use assert_cmd::Command;
use assert_fs::prelude::*;
use assert_fs::NamedTempFile;
use predicates::prelude::*;

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

    temp.assert(predicate::path::exists());
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

    temp.assert(predicate::path::exists());
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

#[test]
fn test_not_found_input() {
    Command::cargo_bin(crate_name!())
        .unwrap()
        .arg("tests/data/no_such_file.pdf")
        .args(&["-o", "tests/data/no_such_file.pdf"])
        .assert()
        .failure()
        .stderr(
            predicate::str::contains("No such file")
                .or(predicate::str::contains("cannot find the file")),
        );
}

#[test]
fn test_not_found_lp() {
    Command::cargo_bin(crate_name!())
        .unwrap()
        .env("MKBL_LP", "mkbl_no_such_command")
        .arg("tests/data/sample.pdf")
        .arg("--print")
        .assert()
        .failure()
        .stderr(predicate::str::contains("could not be found"));
}

#[test]
fn test_fail_lp() {
    Command::cargo_bin(crate_name!())
        .unwrap()
        .env("MKBL_LP", "false")
        .arg("tests/data/sample.pdf")
        .arg("--print")
        .assert()
        .failure()
        .stderr(predicate::str::contains("returned non-zero exit code"));
}

#[test]
fn test_invalid_pdf() {
    let temp = NamedTempFile::new("tmp.pdf").unwrap();
    let path = temp.path();

    temp.touch().unwrap(); // empty

    Command::cargo_bin(crate_name!())
        .unwrap()
        .arg(path.to_str().unwrap())
        .args(&["-o", path.to_str().unwrap()])
        .assert()
        .failure()
        .stderr(predicate::str::contains("Invalid file header"));

    temp.assert(predicate::function(|x: &[u8]| x.is_empty()).from_file_path());
}

#[test]
fn test_confirm_cancel() {
    Command::cargo_bin(crate_name!())
        .unwrap()
        .env("MKBL_LP", "true")
        .arg("tests/data/sample.pdf")
        .arg("--print")
        .arg("-i")
        .write_stdin("n")
        .assert()
        .failure()
        .stderr(predicate::str::contains("cancelled"));
}

#[test]
fn test_confirm_cancel_default() {
    Command::cargo_bin(crate_name!())
        .unwrap()
        .env("MKBL_LP", "true")
        .arg("tests/data/sample.pdf")
        .arg("--print")
        .arg("-i")
        .write_stdin("\n")
        .assert()
        .failure()
        .stderr(predicate::str::contains("cancelled"));
}

#[test]
fn test_confirm_accept() {
    Command::cargo_bin(crate_name!())
        .unwrap()
        .env("MKBL_LP", "true")
        .arg("tests/data/sample.pdf")
        .arg("--print")
        .arg("-i")
        .write_stdin("y")
        .assert()
        .success();
}

#[test]
fn test_confirm_when() {
    Command::cargo_bin(crate_name!())
        .unwrap()
        .env("MKBL_LP", "true")
        .arg("tests/data/sample.pdf")
        .arg("--print")
        .arg("-I")
        .assert()
        .success();
}

#[test]
fn test_confirm_when_large() {
    Command::cargo_bin(crate_name!())
        .unwrap()
        .env("MKBL_LP", "true")
        .env("MKBL_CONFIRM_WHEN", "5")
        .arg("tests/data/sample.pdf")
        .arg("--print")
        .arg("-I")
        .write_stdin("y")
        .assert()
        .success();
}
