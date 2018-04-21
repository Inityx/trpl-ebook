//! Compile EBooks for 'The Rust Programming Language'
//!
//! ['The Rust Programming Language'][trpl] is originally published as Markdown
//! and rendered by _rustbook_. This set of scripts does some transformations
//! and uses _Pandoc_ to render it as HTML, EPUB and PDF (using LaTeX).
//!
//! [trpl]: http://doc.rust-lang.org/book/

#![feature(plugin)]
#![feature(entry_or_default)]
#![cfg_attr(feature = "dev", feature(plugin))]
#![cfg_attr(feature = "dev", plugin(clippy))]
#![recursion_limit = "1024"]

extern crate regex;
extern crate docopt;
#[macro_use] extern crate lazy_static;
extern crate failure;

#[macro_use] mod text;
mod convert_book;

use docopt::Docopt;
use std::{
    fs::File,
    io::Write,
};
use text::OrDefaultExt;

const USAGE: &str = r#"
Compile Rustbook to EBook formats.

Usage:
  compile-trpl [--prefix=<prefix>] [--source=<directory>] [--meta=<meta_file>]

Options:
  --prefix=<prefix>     Prefix/short name of your book, e.g. "trpl" or "nomicon".
  --source=<directory>  Directory containing the git book files, especially SUMMARY.md and README.md.
  --meta=<meta_file>    Meta data of your book, needs to contain `date: {release_date}`.
"#;

fn main() {
    let args = Docopt::new(USAGE)
        .and_then(|d| d.parse())
        .unwrap_or_else(|e| e.exit());

    convert_book::render_book(
        &args.get_str("<prefix>"   ).or_default("trpl"),
        &args.get_str("<directory>").or_default("trpl"),
        &args.get_str("<flag_meta>").or_default("trpl_meta.yml"),
    ).unwrap();

    let index = convert_book::index::render_index("dist/").unwrap();
    File::create("dist/index.html")
        .and_then(|mut f| f.write_all(index.as_bytes()))
        .unwrap();
    
    println!("[✓] Index");
}
