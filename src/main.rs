#![feature(entry_or_default)]
#![recursion_limit = "1024"]

extern crate regex;
extern crate docopt;
extern crate failure;
extern crate serde;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate indoc;

#[macro_use] mod text;
mod book;
mod args;

use std::io::{stdout, Write};
use book::Format::*;

const RELEASE_DATE: &str = "2016-10-01";

fn main() {
    let options = args::get();
    let markdown = book::aggregate(
        options.flag_source,
        options.flag_meta,
        RELEASE_DATE
    ).unwrap();

    for format in &[Markdown, Epub, Html] {
        print!("Rendering Pandoc {}... ", format);
        stdout().flush().unwrap();

        book::render_to(
            &markdown,
            &options.flag_prefix,
            *format,
            RELEASE_DATE,
        ).unwrap();

        println!("Done.");
    }
}
