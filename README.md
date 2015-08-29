This tool may be used to parse bootstrap nodes to format used by qTox.

To get parsed list of nodes from Tox wiki, you may want to use [tox-wiki-nodes-parser](https://github.com/zetok/tox-wiki-nodes-parser).

To make sure that those nodes are working, you may want to use [tox-check-online-bootstraps](https://github.com/zetok/tox-check-online-bootstraps).


# Installation
Running it is fairly simple.

Download [binary for Linux x86_64](https://github.com/zetok/nodes-parser-for-qtox/releases/download/v0.0.0/nodes-parser-for-qtox) and run it.


To compile yourself:

1. Install [Rust](http://www.rust-lang.org/)
2. Make with `cargo build`
3. Run with `./target/debug/./tox-check-online-bootstraps`

# Usage

Parser takes content of file `nodes_working` from working directory and prints to stdout parsed list of nodes.

Provide file `nodes_working` in working directory, with content from from [tox-wiki-nodes-parser](https://github.com/zetok/tox-wiki-nodes-parser) or [tox-check-online-bootstraps](https://github.com/zetok/tox-check-online-bootstraps), or in format
```
<IP> <PORT> <PUBLIC_KEY> <NAME>
```
and run parser.



# License

Licensed under GPLv3+, for details see [COPYING](/COPYING).