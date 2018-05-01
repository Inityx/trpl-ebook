pub mod table_of_contents;
pub mod pandoc;

use std::{
    io::self,
    path::Path,
};

use failure::Error;
use self::table_of_contents::TableOfContents;
use super::file;

fn prepare_chapter_markdown(
    markdown: &str,
    title_level: usize,
    ref_prefix: &str
) -> String {
    use text::{
        AdjustExt,
        normalize::NormalizeExt,
        references::MdRefsExt,
    };

    markdown
        .increase_title_level(title_level)
        .remove_markdown_file_title()
        .prefix_refs_with(ref_prefix)
        .normalize_all()
}

pub fn aggregate<P: AsRef<Path>>(
    src_path: P,
    meta_path: P,
    release_date: &str
) -> Result<String, Error> {
    let src_path = |filename: &str| src_path.as_ref().join(filename);
    let mut book = String::new();
    
    let metadata = file::to_string(meta_path)?.replace("{release_date}", release_date) + "\n";
    book.push_str(&metadata);

    println!("  MD README.md");
    let readme_md = file::to_string(src_path("README.md"))?;
    let introduction = prepare_chapter_markdown(&readme_md, 1, "readme");
    book.push_str("\n\n# Introduction\n\n");
    book.push_str(&introduction);

    file::to_string(src_path("SUMMARY.md"))?
        .parse::<TableOfContents>()?
        .into_iter()
        .map(|chapter| {
            println!("  MD {}", chapter.filename);

            // Markdown chapter title
            book.push_str("\n\n");
            for _ in 0..=chapter.nest_level { book.push('#') }
            book.push(' ');
            book.push_str(&chapter.header);
            book.push('\n');

            let chapter_content = prepare_chapter_markdown(
                &file::to_string(src_path(&chapter.filename))?,
                3,
                &chapter.filename
            );

            book.push_str("\n");
            book.push_str(&chapter_content);

            Ok(())
        })
        .collect::<io::Result<()>>()?;

    Ok(book)
}

pub fn render_to(
    markdown: &str,
    prefix: &str,
    format: file::Format,
    release_date: &str,
) -> Result<(), Error> {
    match format {
        file::Format::Markdown => file::from_string(
            format!("dist/{}-{}.md", prefix, release_date),
            markdown
        ).map_err(Into::into),
        
        _ => pandoc::create(markdown, prefix, format, release_date),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
