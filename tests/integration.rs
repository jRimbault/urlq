use std::path::PathBuf;

use predicates::prelude::*;

#[rstest::rstest]
#[case(&["https://curl.se", "set", "host=example.com"], "https://example.com/\n")]
#[case(&["https://curl.se/we/../are.html", "set", "port=8080"], "https://curl.se:8080/are.html\n")]
#[case(&["https://curl.se/we/are.html", "get", "path"], "/we/are.html\n")]
#[case(&["https://curl.se/we/are.html", "get", "port"], "443\n")]
#[case(&["https://example.com/hello.html", "get", "scheme", "port", "path"], "https 443 /hello.html\n")]
fn test(#[case] args: &[&str], #[case] stdout: &str) {
    assert_cmd::Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .args(args)
        .assert()
        .success()
        .stdout(predicate::eq(stdout));
}

#[rstest::rstest]
#[case(&["get", "host"], "cases/01")]
#[case(&["https://fake.host/hello#frag", "--json", "set", "user=::moo::"], "cases/02")]
#[case(&["https://fake.host/hello#frag", "set", "user=::moo::"], "cases/03")]
fn file(#[case] args: &[&str], #[case] path: &str) {
    let path = PathBuf::from(format!("tests/{path}"));
    let stdin = std::fs::read_to_string(path.join("stdin.txt")).unwrap_or_default();
    let stdout = std::fs::read_to_string(path.join("stdout.txt")).unwrap();
    assert_cmd::Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .args(args)
        .write_stdin(stdin)
        .assert()
        .success()
        .stdout(predicate::eq(stdout));
}
