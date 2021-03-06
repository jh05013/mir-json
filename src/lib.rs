#![feature(rustc_private)]
#![feature(never_type)]

extern crate serde;
#[macro_use] extern crate serde_json;
extern crate serde_cbor;
#[macro_use] extern crate serde_derive;
extern crate tar;

extern crate rustc;
extern crate rustc_ast;
//extern crate rustc_codegen_utils;
extern crate rustc_driver;
extern crate rustc_data_structures;
extern crate rustc_hir;
extern crate rustc_index;
extern crate rustc_interface;
extern crate rustc_mir;
extern crate rustc_session;
extern crate rustc_span;
extern crate rustc_target;
//extern crate syntax;

pub mod analyz;
pub mod lib_util;
pub mod link;

mod tar_stream;
