use failure::Error;

use text::{
    AdjustExt,
    normalize::NormalizeExt,
    references::MdRefsExt,
    patterns::reg,
};

pub struct Chapter {
    pub filename: String,
    pub headline: String,
}

pub fn extract_chapters(toc: &str) -> Result<Vec<Chapter>, Error> {
    lazy_static_regex!(TOC,      reg::mdfile::TOC     );
    lazy_static_regex!(FILENAME, reg::mdfile::FILENAME);

    let chapters = toc
        .lines()
        .filter_map(|line| TOC.captures(line))
        .map(|toc_chapter_match| {
            let filename = toc_chapter_match.name("filename").unwrap().as_str().to_string();
            let headline = {
                let level  = (toc_chapter_match.name("indent"  ).unwrap().as_str().chars().count() / 4) + 1;
                let title  = toc_chapter_match.name("title"   ).unwrap().as_str();
                let target = FILENAME
                    .captures(&filename).unwrap()
                    .name("name").unwrap().as_str();

                format!(
                    "{empty:#^level$} {title} {{#sec--{target}}}\n",
                    empty = "",
                    level = level,
                    title = title,
                    target = target
                )
            };

            Chapter { filename, headline }
        }).collect();
    
    Ok(chapters)
}

pub fn convert(
    markdown: &str,
    title_level: usize,
    ref_prefix: &str
) -> String {
    markdown
        .increase_title_level(title_level)
        .remove_markdown_file_title()
        .prefix_refs_with(ref_prefix)
        .normalize_all()
}

#[cfg(test)]
mod tests {
    use super::*;
}
