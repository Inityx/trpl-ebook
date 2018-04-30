use text::patterns::reg;
use std::str::FromStr;
use regex::Captures;
use failure::{self, Error, err_msg};

pub struct Chapter {
    pub filename: String,
    pub header: String,
    pub nest_level: usize,
}

pub struct TableOfContents(Vec<Chapter>);

impl FromStr for TableOfContents {
    type Err = failure::Error;

    fn from_str(source: &str) -> Result<Self, Self::Err> {
        lazy_static_regex!(TOC,      reg::mdfile::TOC     );
        lazy_static_regex!(FILENAME, reg::mdfile::FILENAME);

        let toc_captures = source
            .lines()
            .filter(|line| line.trim_left().starts_with('*'))
            .map(|line| TOC.captures(line))
            .map(|captures| captures.ok_or_else(|| err_msg("Failed extracting ToC chapter")))
            .collect::<Result<Vec<Captures>, Error>>()?;

        let chapters = toc_captures
            .into_iter()
            .map(|chapter_match| {
                let filename = chapter_match.name("filename").unwrap().as_str().to_string();
                
                let nest_level = {
                    let indent_level = chapter_match
                        .name("indent").unwrap().as_str()
                        .chars().count();
                    
                    (indent_level / 4) + 1
                };

                let header = {
                    let title = chapter_match.name("title").unwrap().as_str();
                    let anchor = FILENAME
                        .captures(&filename).unwrap()
                        .name("name").unwrap().as_str();

                    format!("{} {{#sec--{}}}\n", title, anchor)
                };

                Chapter {
                    filename,
                    header,
                    nest_level
                }
            })
            .collect();
        
        Ok(TableOfContents(chapters))
    }
}

impl IntoIterator for TableOfContents {
    type IntoIter = <Vec<Chapter> as IntoIterator>::IntoIter;
    type Item = Chapter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

pub fn convert(
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

#[cfg(test)]
mod tests {
    use super::*;
}
