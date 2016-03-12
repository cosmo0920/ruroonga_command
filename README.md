Ruroonga Command
===

[![Build Status](https://travis-ci.org/cosmo0920/ruroonga_command.svg?branch=master)](https://travis-ci.org/cosmo0920/ruroonga_command)

## An extensible Groonga Query Builder for Rust.

ruroonga_command provides extensible Groonga command query builder and generator. It reduces runtime errors about Groonga queries.

## Usage

Add following lines to your Cargo.toml:

```toml
[dependencies]
ruroonga_command = "*"
```

and following lines to your crate root:

```rust
extern crate ruroonga_command;

use ruroonga_command as command;
```

## LICENSE

[MIT](LICENSE).
