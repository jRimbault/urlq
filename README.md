# urlq

CLI tool to parse and manipulate URLs.

Reimplementation of [`trurl`](https://github.com/curl/trurl).

## Examples

```
$ urlq https://curl.se set host=example.com
https://example.com/

$ urlq https://curl.se/we/../are.html set port=8080
https://curl.se:8080/are.html

$ urlq https://curl.se/we/are.html get path
/we/are.html

$ urlq https://curl.se/we/are.html get port
443

$ urlq https://example.com/hello.html get scheme port path
https 443 /hello.html

$ cat url-list.txt | urlq get host
[one host name per URL in the input file]
```

<details>
<summary>Help</summary>

```
Tool to explore and modify URLs programmatically

Usage: urlq [OPTIONS] [URL] <COMMAND>

Commands:
  get   Parts of the url to obtain
  set   Parts of the url to update
  help  Print this message or the help of the given subcommand(s)

Arguments:
  [URL]  URL to explore, if absent urlq reads from stdin line by line

Options:
  -j, --json     Output as newline delimited JSON
  -h, --help     Print help
  -V, --version  Print version
```

</details>

## Install

If you have a rust toolchain installed it should be as simple as:

```shell
cargo install --git https://github.com/jRimbault/urlq.git
```
