use tap::Pipe;

use std::path::PathBuf;

#[rstest::rstest]
#[case(&["get", "host"], "cases/01", 0)]
#[case(&["https://fake.host/hello#frag", "--json", "set", "user=::moo::"], "cases/02", 0)]
#[case(&["https://fake.host/hello#frag", "set", "user=::moo::"], "cases/03", 0)]
#[case(&["https://curl.se", "set", "host=example.com"], "cases/04", 0)]
#[case(&["https://curl.se/we/../are.html", "set", "port=8080"], "cases/05", 0)]
#[case(&["https://curl.se/we/are.html", "get", "path"], "cases/06", 0)]
#[case(&["https://curl.se/we/are.html", "get", "port"], "cases/07", 0)]
#[case(&["https://example.com/hello.html", "get", "scheme", "port", "path"], "cases/08", 0)]
#[case(&["get", "port"], "cases/09", 1)]
#[case(&["https://curl.se?name=hello", "--json", "set", "host=example.net"], "cases/10", 0)]
#[case(&["https://curl.se", "set", "port=TTTT"], "cases/11", 1)]
fn test(#[case] args: &[&str], #[case] path: &str, #[case] exit_code: i32) {
    let path: PathBuf = ["tests"].into_iter().chain(path.split('/')).collect();
    let stdin = std::fs::read_to_string(path.join("stdin.txt"));
    let stderr = std::fs::read_to_string(path.join("stderr.txt")).unwrap_or_default();
    let stdout = std::fs::read_to_string(path.join("stdout.txt")).unwrap_or_default();
    assert_cmd::Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .env("NO_COLOR", "1")
        .args(args)
        .pipe(|cmd| {
            if let Ok(stdin) = stdin {
                cmd.write_stdin(stdin)
            } else {
                cmd
            }
        })
        .assert()
        .code(exit_code)
        .stdout(stdout)
        .stderr(stderr);
}

#[cfg(target_os = "linux")]
#[rstest::rstest]
#[case(&["http://example.com", "get", "scheme", "host"])]
fn valgrind(#[case] args: &[&str]) {
    use assert_cmd::cargo::CommandCargoExt;
    let urlq = std::process::Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    assert_cmd::Command::new("valgrind")
        .arg("--error-exitcode=1")
        .arg(urlq.get_program())
        .args(args)
        .assert()
        .success();
}
