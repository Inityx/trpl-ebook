mod text_traits;
mod toc;

use std::{
    io,
    path::Path,
};

use failure::Error;
use ::aux::file;

fn prepare_chapter_md(markdown: String, title_bump: usize, ref_prefix: &str) -> String {
    use self::text_traits::{
        adjust::AdjustExt,
        normalize::NormalizeExt,
        references::MdRefsExt,
    };

    markdown
        .increase_title_level(title_bump)
        .remove_markdown_file_title()
        .prefix_refs_with(ref_prefix)
        .normalize()
}

fn create_introduction(path_prefix: &Path) -> io::Result<String> {
    println!("  README.md");
    let mut markdown = String::with_capacity(256);

    markdown.push_str("\n\n# Introduction\n\n");
    let readme_raw = file::to_string(path_prefix.join("README.md"))?;
    let introduction = prepare_chapter_md(readme_raw, 1, "readme");
    markdown.push_str(&introduction);

    Ok(markdown)
}

fn create_chapter(chapter: &toc::Chapter, path_prefix: &Path) -> io::Result<String> {
    println!("  {}", chapter.filename);
    let mut markdown = String::with_capacity(512);

    // Markdown chapter title
    markdown.push_str("\n\n");
    for _ in 0..=chapter.nest_level { markdown.push('#') }
    markdown.push(' ');
    markdown.push_str(&chapter.header);
    markdown.push('\n');

    let chapter_raw = file::to_string(path_prefix.join(&chapter.filename))?;

    let chapter_contents = prepare_chapter_md(
        chapter_raw,
        chapter.nest_level + 1,
        &chapter.filename
    );

    markdown.push_str(&chapter_contents);

    Ok(markdown)
}

pub fn create_book<P: AsRef<Path>>(
    path_prefix: P,
    meta_path: P,
    release_date: &str
) -> Result<String, Error> {
    let path_prefix = path_prefix.as_ref();
    let mut book = String::new();
    
    book.push_str(&(
        file::to_string(meta_path)?
            .replace("{release_date}", release_date) + "\n"
    ));

    book.push_str(&create_introduction(path_prefix)?);

    file::to_string(path_prefix.join("SUMMARY.md"))?
        .parse::<toc::TableOfContents>()?
        .into_iter()
        .map(|chapter| create_chapter(&chapter, path_prefix))
        .map(|contents| contents.map(|c| book.push_str(&c)))
        .collect::<io::Result<()>>()?;

    Ok(book)
}

#[cfg(test)]
mod tests {
    use super::*;
}
