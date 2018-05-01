use text::patterns::reg;
use std::{
    ops::Div,
    path::Path,
    str::FromStr,
};
use failure::{self, Error, err_msg};

#[derive(Debug, PartialEq)]
pub struct Chapter {
    pub filename: String,
    pub header: String,
    pub nest_level: usize,
}

fn build_header(title: &str, filename: &str) -> Result<String, failure::Error> {
    let section_slug = Path::new(&filename)
        .file_stem()
        .ok_or_else(|| format_err!(
            "No ToC filename for chapter '{}'",
            title
        ))?
        .to_str()
        .ok_or_else(|| format_err!(
            "Illegal characters in filename for chapter '{}'",
            title
        ))?;
      
    Ok(format!("{} {{#sec--{}}}", title, section_slug))
}

impl FromStr for Chapter {
    type Err = failure::Error;

    fn from_str(source: &str) -> Result<Self, Self::Err> {
        lazy_static_regex!(TOC, reg::mdfile::TOC);

        let capture = TOC
            .captures(source)
            .ok_or_else(|| err_msg("Failed extracting ToC chapter"))?;
        
        let filename = capture
            .name("filename").unwrap().as_str()
            .to_string();
        
        let nest_level = capture
            .name("indent").unwrap().as_str()
            .chars().count().div(4);

        let header = build_header(
            capture.name("title").unwrap().as_str(),
            &filename,
        )?;

        Ok(Chapter { filename, header, nest_level })
    }
}

#[derive(Debug, PartialEq)]
pub struct TableOfContents(Vec<Chapter>);

impl FromStr for TableOfContents {
    type Err = failure::Error;

    fn from_str(source: &str) -> Result<Self, Self::Err> {
        source
            .lines()
            .filter(|line| line.trim_left().starts_with('*'))
            .map(Chapter::from_str)
            .collect::<Result<Vec<Chapter>, Error>>()
            .map(TableOfContents)
    }
}

impl IntoIterator for TableOfContents {
    type IntoIter = <Vec<Chapter> as IntoIterator>::IntoIter;
    type Item = Chapter;

    fn into_iter(self) -> Self::IntoIter { self.0.into_iter() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn chapter_from_str() {
        assert_eq!(
            Chapter {
                filename: "guessing-game.md".into(),
                header: "Tutorial: Guessing Game {#sec--guessing-game}".into(),
                nest_level: 0
            },
            "* [Tutorial: Guessing Game](guessing-game.md)".parse().unwrap()
        );

        assert_eq!(
            Chapter {
                filename: "variable-bindings.md".into(),
                header: "Variable Bindings {#sec--variable-bindings}".into(),
                nest_level: 1
            },
            "    * [Variable Bindings](variable-bindings.md)".parse().unwrap()
        );
    }

    const TOC_TEXT: &str = indoc!("
        # Sample TOC

        * [Tutorial: Guessing Game](guessing-game.md)
        * [Syntax and Semantics](syntax-and-semantics.md)
            * [Functions](functions.md)
        * [Effective Rust](effective-rust.md)
            * [The Stack and the Heap](the-stack-and-the-heap.md)
            * [Testing](testing.md)
    ");

    #[test]
    fn toc_from_str() {
        let correct_toc = vec![
            Chapter {
                filename: "guessing-game.md".into(),
                header: "Tutorial: Guessing Game {#sec--guessing-game}".into(),
                nest_level: 0,
            },
            Chapter {
                filename: "syntax-and-semantics.md".into(),
                header: "Syntax and Semantics {#sec--syntax-and-semantics}".into(),
                nest_level: 0,
            },
            Chapter {
                filename: "functions.md".into(),
                header: "Functions {#sec--functions}".into(),
                nest_level: 1,
            },
            Chapter {
                filename: "effective-rust.md".into(),
                header: "Effective Rust {#sec--effective-rust}".into(),
                nest_level: 0,
            },
            Chapter {
                filename: "the-stack-and-the-heap.md".into(),
                header: "The Stack and the Heap {#sec--the-stack-and-the-heap}".into(),
                nest_level: 1,
            },
            Chapter {
                filename: "testing.md".into(),
                header: "Testing {#sec--testing}".into(),
                nest_level: 1,
            },
        ];

        assert_eq!(
            correct_toc,
            TOC_TEXT.parse::<TableOfContents>().unwrap().0
        );
    }
}
