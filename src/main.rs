extern crate regex;
extern crate rayon;
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

use rayon::iter::{
    ParallelIterator,
    IntoParallelRefIterator
};

const RELEASE_DATE: &str = "2016-10-01";

fn main() {
    use file::Format::*;
    let opt = args::get();

    println!("Aggregating markdown");
    let markdown = book::aggregate(
        &opt.flag_source,
        &opt.flag_meta,
        RELEASE_DATE
    ).unwrap();
    println!("Done\n");

    
    [Markdown, Epub, Html]
        .par_iter() // Because Pandoc is slow and single threaded
        .map(|format| {
            println!("Rendering {}...", format);

            let result = book::render_to(
                &markdown,
                &opt.flag_prefix,
                *format,
                RELEASE_DATE,
            );

            if let Err(e) = result {
                // TODO backtrace?
            } else {
                println!("Finished {}", format);
            }
        })
        .collect::<()>();
}
