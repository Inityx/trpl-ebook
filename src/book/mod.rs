pub mod markdown;
pub mod pandoc;

use std::{
    fs::File,
    io::{Write, Read},
    fmt::{self, Display, Formatter},
    path::Path,
};

use failure::{Error, ResultExt};
use self::markdown::TableOfContents;

#[derive(Clone, Copy)]
pub enum Format { Html, Epub, Markdown }

impl Format {
    fn file_extension(&self) -> &'static str {
        use self::Format::*;
        match self {
            Html => "html",
            Epub => "epub",
            Markdown => "md",
        }
    }
}

impl Display for Format {
    fn fmt(&self, mut fmt: &mut Formatter) -> Result<(), fmt::Error> {
        use self::Format::*;
        write!(&mut fmt, "{}", match self {
            Html => "HTML",
            Epub => "ePub",
            Markdown => "Markdown",
        })
    }
}

fn file_to_string<P: AsRef<Path>>(path: P) -> Result<String, Error> {
    let mut buffer = String::new();
    let mut file = File::open(&path).context(
        format!(
            "Failed opening {}",
            path.as_ref().to_string_lossy()
        )
    )?;
    file.read_to_string(&mut buffer).context(
        format!(
            "Failed reading {}",
            path.as_ref().to_string_lossy()
        )
    )?;
    Ok(buffer)
}

pub fn aggregate<P>(
    src_path: P,
    meta_path: P,
    release_date: &str
) -> Result<String, Error>
where
    P: AsRef<Path>,
{
    let src_path = |filename: &str| src_path.as_ref().join(filename);
    let mut book = String::new();

    {
        println!("Reading metadata...");
        let metadata = file_to_string(meta_path)?
            .replace("{release_date}", release_date);
        
        book.push_str(&metadata);
        book.push('\n');
    }

    println!("Aggregating markdown...");
    {
        println!("  MD README.md");
        let readme_md = &file_to_string(&src_path("README.md"))?;
        let introduction = markdown::convert(readme_md, 1, "readme");
        book.push_str("\n\n# Introduction\n\n");
        book.push_str(&introduction);
    }

    let table_of_contents = file_to_string(&src_path("SUMMARY.md"))?
        .parse::<TableOfContents>()?;

    for chapter in table_of_contents {
        println!("  MD {}", chapter.filename);

        // Markdown chapter title
        book.push_str("\n\n");
        for _ in 0..chapter.nest_level { book.push('#') }
        book.push(' ');
        book.push_str(&chapter.header);

        let chapter_content = markdown::convert(
            &file_to_string(&src_path(&chapter.filename))?,
            3,
            &chapter.filename
        );

        book.push_str("\n");
        book.push_str(&chapter_content);
    }

    println!();
    Ok(book)
}

fn write_markdown(
    markdown: &str,
    prefix: &str,
    release_date: &str
) -> Result<(), Error> {
    let filename = format!("dist/{}-{}.md", prefix, release_date);
    let mut file = File::create(&filename)?;
    file.write_all(markdown.as_bytes()).map_err(Into::into)
}

pub fn render_to(
    markdown: &str,
    prefix: &str,
    format: Format,
    release_date: &str,
) -> Result<(), Error> {
    use self::Format::*;
    match format {
        Markdown => write_markdown(markdown, prefix, release_date),
        _ => pandoc::create(
            &markdown.as_ref(),
            prefix.as_ref(),
            format,
            release_date.as_ref()
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
