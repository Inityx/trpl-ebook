pub mod index;
pub mod markdown;
pub mod pandoc;

use std::{
    fs::File,
    io::Write,
    path::Path,
};

use failure::{Error, ResultExt};

const RELEASE_DATE: &str = "2016-10-01";

pub fn render_book<P, S>(
    prefix_param: S,
    src_path: P,
    meta_path: P
) -> Result<(), Error>
where
    P: AsRef<Path>,
    S: AsRef<str>,
{
    let prefix: &str = prefix_param.as_ref();

    let book_markdown = markdown::to_single_file(src_path, meta_path, RELEASE_DATE)?;

    let filename = format!("dist/{}-{}.md", prefix, RELEASE_DATE);
    File::create(&filename)
        .and_then(|mut f| f.write_all(book_markdown.as_bytes()))
        .context("Failed to write aggregated markdown")?;

    println!("[✓] MD");

    pandoc::create(
        &book_markdown,
        prefix,
        pandoc::OutputType::Html,
        RELEASE_DATE
    ).context("Failed writing Pandoc HTML")?;
    println!("[✓] HTML");

    pandoc::create(
        &book_markdown,
        prefix,
        pandoc::OutputType::Epub,
        &RELEASE_DATE
    ).context("Failed writing Pandoc EPUB")?;
    println!("[✓] EPUB");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
}
