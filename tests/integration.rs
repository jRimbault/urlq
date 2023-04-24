use std::path::PathBuf;

use predicates::prelude::*;

#[rstest::rstest]
#[case(&["get", "host"], "cases/01")]
#[case(&["https://fake.host/hello#frag", "--json", "set", "user=::moo::"], "cases/02")]
#[case(&["https://fake.host/hello#frag", "set", "user=::moo::"], "cases/03")]
#[case(&["https://curl.se", "set", "host=example.com"], "cases/04")]
#[case(&["https://curl.se/we/../are.html", "set", "port=8080"], "cases/05")]
#[case(&["https://curl.se/we/are.html", "get", "path"], "cases/06")]
#[case(&["https://curl.se/we/are.html", "get", "port"], "cases/07")]
#[case(&["https://example.com/hello.html", "get", "scheme", "port", "path"], "cases/08")]
fn test(#[case] args: &[&str], #[case] path: &str) {
    let path: PathBuf = ["tests"].into_iter().chain(path.split('/')).collect();
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
