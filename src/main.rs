#![feature(entry_or_default)]
#![recursion_limit = "1024"]

extern crate regex;
extern crate docopt;
extern crate serde;
#[macro_use] extern crate failure;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate indoc;

#[macro_use] mod text;
mod book;
mod args;
mod file;

use std::io::{stdout, Write};
use file::Format::*;

const RELEASE_DATE: &str = "2016-10-01";

fn main() {
    let options = args::get();

    println!("Aggregating markdown");
    let markdown = book::aggregate(
        options.flag_source,
        options.flag_meta,
        RELEASE_DATE
    ).unwrap();
    println!("Done\n");

    for format in &[Markdown, Epub, Html] {
        print!("Rendering {}... ", format);
        stdout().flush().unwrap();

        book::render_to(
            &markdown,
            &options.flag_prefix,
            *format,
            RELEASE_DATE,
        ).unwrap();

        println!("Done");
    }
}
