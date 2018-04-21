use std::{
    path::Path,
    fs::File,
    io::Read,
};

use failure::{
    Error,
    ResultExt
};

use text::{
    AdjustExt,
    normalize::NormalizeExt,
    references::MdRefsExt
};

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

const TOC_PATTERN: &str = 
    r"(?x)
    (?P<indent>\s*?)
    \*\s
    \[
    (?P<title>.+?)
    \]
    \(
    (?P<filename>.+?)
    \)
";
const FILENAME_PATTERN: &str =
    r"(?x)
    ^
    (?P<path>(.*)/)?
    (?P<name>(.*?))
    (?P<ext>\.(\w*))?
    $
";

struct Chapter {
    file: String,
    headline: String,
}

fn extract_chapters(toc: &str) -> Result<Vec<Chapter>, Error> {
    lazy_static_regex!(TOC_MATCHER,      TOC_PATTERN     );
    lazy_static_regex!(FILENAME_MATCHER, FILENAME_PATTERN);

    let chapters = toc
        .lines()
        .filter_map(|line| TOC_MATCHER.captures(line))
        .map(|chapter_match| {
            let level         = chapter_match.name("indent"  ).unwrap().as_str().chars().count();
            let link_filename = chapter_match.name("filename").unwrap().as_str();
            let link_title    = chapter_match.name("title"   ).unwrap().as_str();

            let link_value = FILENAME_MATCHER
                .captures(link_filename).unwrap()
                .name("name").unwrap()
                .as_str();

            let headline = format!(
                "{empty:#^level$} {name} {{#sec--{link}}}\n",
                empty = "",
                level = level,
                name = link_title,
                link = link_value
            );

            Chapter {
                file: link_filename.to_string(),
                headline: headline,
            }
        }).collect();
    
    Ok(chapters)
}

pub fn import_markdown<P>(
    filepath: P,
    title_level: usize,
    ref_prefix: &str
) -> Result<String, Error>
where
    P: AsRef<Path>
{
    let markdown = file_to_string(filepath)?
        .increase_title_level(title_level)
        .remove_markdown_file_title()
        .prefix_refs_with(ref_prefix)
        .normalize_all(87, "↳ "); // TODO make constant
    
    Ok(markdown)
}

pub fn to_single_file<P>(
    src_path: P,
    meta_path: P,
    release_date: &str
) -> Result<String, Error>
where
    P: AsRef<Path>,
{
    let src_path = |filename: &str| src_path.as_ref().join(filename);
    let mut book = String::new();

    println!("Reading metadata...");
    let metadata = file_to_string(meta_path)?.replace("{release_date}", release_date);
    book.push_str(metadata.as_str());
    book.push('\n');

    println!("Aggregating markdown...");
    
    println!("  MD README.md");
    let cover = import_markdown(src_path("README.md"), 1, "readme")?;
    book.push_str("\n\n# Introduction\n\n");
    book.push_str(&cover);

    let toc = file_to_string(src_path("SUMMARY.md"))?;
    let chapters = extract_chapters(&toc)?;

    for chapter_match in chapters {
        println!("  MD {}", chapter_match.file);

        let chapter_content = import_markdown(
            src_path(&chapter_match.file), 3, &chapter_match.file
        )?;

        book.push_str("\n\n");
        book.push_str(&chapter_match.headline);
        book.push_str("\n");
        book.push_str(&chapter_content);
    }

    Ok(book)
}

#[cfg(test)]
mod tests {
    use super::*;
}
