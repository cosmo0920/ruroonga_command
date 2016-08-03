Ruroonga Command
===

[![Build Status](https://travis-ci.org/cosmo0920/ruroonga_command.svg?branch=master)](https://travis-ci.org/cosmo0920/ruroonga_command)
[![](http://meritbadge.herokuapp.com/ruroonga_command)](https://crates.io/crates/ruroonga_command)
[![Build status](https://ci.appveyor.com/api/projects/status/m77hivt5548cwqso/branch/master?svg=true)](https://ci.appveyor.com/project/cosmo0920/ruroonga-command/branch/master)

[Documentation](http://cosmo0920.github.io/ruroonga_command/ruroonga_command/index.html)

## An extensible Groonga Query Builder for Rust.

ruroonga_command provides extensible Groonga command query builder and generator. It reduces runtime errors about Groonga queries.

## Usage

Add following lines to your Cargo.toml:

```toml
[dependencies]
ruroonga_command = "~0.2.0"
```

and following lines to your crate root:

```rust,ignore
extern crate ruroonga_command;
```

### A complete example

#### For generating CLI Groonga command

```rust
extern crate ruroonga_command as ruroonga;

use ruroonga::dsl::*;
use ruroonga::commandable::Commandable;

fn select_cli_example() {
    let select = select("Entries".to_string())
                 .filter("content @ \"fast\"".to_string()).to_command();
    println!("command: {:?}", select);
}
fn main() {
    select_cli_example();
}
```

#### For generating HTTP Groonga command

```rust
extern crate ruroonga_command as ruroonga;

use ruroonga::dsl::*;
use ruroonga::queryable::Queryable;

fn select_query_example() {
    let select = select("Entries".to_string())
                 .filter("content @ \"fast\"".to_string()).to_query();
    println!("query: {:?}", select);
}
fn main() {
    select_query_example();
}
```

### Target Rust Version

1.9.0 or later.

### Minimum required Groonga Version

6.0.3 or later.

## LICENSE

[MIT](LICENSE).
