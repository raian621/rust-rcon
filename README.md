# Yet Another RCON Client (written in Rust btw)

[![codecov](https://codecov.io/gh/raian621/yarcon/graph/badge.svg?token=GQH1KQBTAI)](https://codecov.io/gh/raian621/yarcon) [![ci](https://github.com/raian621/yarcon/actions/workflows/checks.yml/badge.svg?branch=main)](https://github.com/raian621/yarcon/actions/workflows/checks.yml) [![Rust Report Card](https://rust-reportcard.xuri.me/badge/github.com/raian621/yarcon)](https://rust-reportcard.xuri.me/report/github.com/raian621/yarcon)

Currently, this RCON client is pretty basic, but should be able to be used to interface with most servers that use the RCON protocol. Other RCON clients are kind of disappointing in my opinion, as they don't allow the user to move their cursor back to modify their previous input and don't support any form of autocomplete. I want this RCON client to be as easy to use as possible, with modifiable input lines and autocomplete.

## Usage

```sh
yarcon -h HOSTNAME_HERE -p PORT_HERE -P PASSWORD_HERE
```
