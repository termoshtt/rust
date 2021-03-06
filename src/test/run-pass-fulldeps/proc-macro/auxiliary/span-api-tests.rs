// Copyright 2017 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// force-host
// no-prefer-dynamic

#![crate_type = "proc-macro"]
#![feature(proc_macro_span)]

extern crate proc_macro;

use proc_macro::*;

// Re-emits the input tokens by parsing them from strings
#[proc_macro]
pub fn reemit(input: TokenStream) -> TokenStream {
    input.to_string().parse().unwrap()
}

#[proc_macro]
pub fn assert_fake_source_file(input: TokenStream) -> TokenStream {
    for tk in input {
        let source_file = tk.span().source_file();
        assert!(!source_file.is_real(), "Source file is real: {:?}", source_file);
    }

    "".parse().unwrap()
}

#[proc_macro]
pub fn assert_source_file(input: TokenStream) -> TokenStream {
    for tk in input {
        let source_file = tk.span().source_file();
        assert!(source_file.is_real(), "Source file is not real: {:?}", source_file);
    }

    "".parse().unwrap()
}
