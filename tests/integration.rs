use predicates::prelude::*;

fn test_case(args: &[&str], stdout: &str) {
    assert_cmd::Command::cargo_bin("urlq")
        .unwrap()
        .args(args)
        .assert()
        .success()
        .stdout(predicate::eq(stdout));
}

#[test]
fn example1() {
    test_case(
        &["https://curl.se", "set", "host=example.com"],
        "https://example.com/\n",
    );
}

#[test]
fn example2() {
    test_case(
        &["https://curl.se/we/../are.html", "set", "port=8080"],
        "https://curl.se:8080/are.html\n",
    );
}

#[test]
fn example3() {
    test_case(
        &["https://curl.se/we/are.html", "get", "path"],
        "/we/are.html\n",
    );
}

#[test]
fn example4() {
    test_case(&["https://curl.se/we/are.html", "get", "port"], "443\n");
}

#[test]
fn example5() {
    test_case(
        &[
            "https://example.com/hello.html",
            "get",
            "scheme",
            "port",
            "path",
        ],
        "https 443 /hello.html\n",
    );
}

#[test]
fn example6() {
    assert_cmd::Command::cargo_bin("urlq")
        .unwrap()
        .args(["get", "host"])
        .write_stdin(std::fs::read("tests/url-list.txt").unwrap())
        .assert()
        .success()
        .stdout(predicate::eq(
            "curl.se
fake.host
example.net
",
        ));
}

#[test]
fn example7() {
    test_case(
        &[
            "https://fake.host/hello#frag",
            "--json",
            "set",
            "user=::moo::",
        ],
        r#"{"url":"https://%3A%3Amoo%3A%3A@fake.host/hello#frag","fragment":"frag","host":"fake.host","path":"/hello","port":"443","scheme":"https","user":"%3A%3Amoo%3A%3A"}
"#,
    )
}
