use std::{fs, path::PathBuf};

use assert_cmd::Command;
use tempfile::tempdir;

#[test]
fn unpack_missing_args() {
    let mut cmd = Command::cargo_bin("unitypkg-cli").unwrap();
    let assert = cmd.args(&["unpack"]).assert();

    assert.failure().code(2);
}

#[test]
fn unpack() {
    let output = tempdir().unwrap();

    let mut cmd = Command::cargo_bin("unitypkg-cli").unwrap();
    let assert = cmd
        .args(&[
            "unpack",
            "--input",
            fs::canonicalize(PathBuf::from("./tests/fixtures/simple-cube.unitypackage"))
                .unwrap()
                .to_str()
                .unwrap(),
            output.into_path().to_str().unwrap(),
        ])
        .assert();

    assert.success();
}
