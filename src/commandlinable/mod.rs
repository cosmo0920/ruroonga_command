use std::fmt;

// Following code is modified from
// https://github.com/rust-lang/rust/blob/master/src/librustdoc/html/escape.rs
//
// Escape type and impl fmt::Display is:
//
// Copyright 2013 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

pub struct Escape<'a>(pub &'a str);

impl<'a> fmt::Display for Escape<'a> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let Escape(s) = *self;
        let pile_o_bits = s;
        let mut last = 0;
        for (i, ch) in s.bytes().enumerate() {
            match ch as char {
                // TODO: Verify escaping (\") is neccesary?
                /// It should escape aginst special characters (\n, \\)
                /// for creating command line style command.
                '\n' | '\\' => {
                    try!(fmt.write_str(&pile_o_bits[last..i]));
                    let s = match ch as char {
                        '\n' => "\\n",
                        '\\' => "\\\\",
                        _ => unreachable!(),
                    };
                    try!(fmt.write_str(s));
                    last = i + 1;
                }
                _ => {}
            }
        }

        if last < s.len() {
            try!(fmt.write_str(&pile_o_bits[last..]));
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_fmt() {
        assert_eq!("\\n", format!("{}", Escape("\n")));
        assert_eq!("\\\\", format!("{}", Escape("\\")));
    }
}
