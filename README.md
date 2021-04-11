# rs-srcaper

## Installation

Install rust from [here](https://www.rust-lang.org/tools/install)

Clone this repo (`git clone https://github.com/krista-chan/rs-scraper.git`)

Build & run

```rust
cargo run -- <args go here>
```

## Help and commands

```sh
Arg HTML - is a file or URL (required)
-s | --selector => CSS queryselector (required)
-h | --help => prints the help info
-f | --is-fragment => tells the parser if the html passed in is a fragment (pass if the html is a fragment)
-j | --json => outputs the extracted html and info surrounding said html as a json
```

## Example

```sh
cargo run -- test.html -s div.a --json 
```

Will output `{"id": "NodeId(18)", "parentElem": "body", "innerHtml": "aaaaaaaaaaaaaa", "hasChildren": "true", "html": "<div class=\"a\">aaaaaaaaaaaaaa</div>"}`
