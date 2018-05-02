extern crate regex;
extern crate rayon;
extern crate docopt;
extern crate serde;
#[macro_use] extern crate failure;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate indoc;

#[macro_use] mod aux;
mod prepare;
mod render;

use std::process;

use rayon::iter::{
    ParallelIterator,
    IntoParallelRefIterator
};

const RELEASE_DATE: &str = "2016-10-01";

fn err_exit(error: &failure::Error) -> ! {
    eprintln!("Failed to create book:\n{}", error.backtrace());
    process::exit(1);
}

fn main() {
    use render::Format::*;
    let opt = aux::args::get();

    println!("Aggregating markdown");
    let book = prepare::create_book(
        &opt.flag_source,
        &opt.flag_meta,
        RELEASE_DATE
    ).unwrap_or_else(|e| err_exit(&e));

    println!("Done\n");

    [Markdown, Epub, Html]
        .par_iter() // Because Pandoc is slow and single threaded
        .map(|format| {
            println!("Rendering {}...", format);

            let result = render::to_file(
                &book,
                &opt.flag_prefix,
                *format,
                RELEASE_DATE,
            );

            if let Err(e) = result {
                eprintln!("Failed to render {}:\n{}", format, e.backtrace());
            } else {
                println!("Finished {}", format);
            }
        })
        .collect::<()>();
}
