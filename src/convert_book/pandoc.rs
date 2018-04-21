use std::{
    process::{Command, Stdio},
    io::Write
};

use failure::{Error, ResultExt, err_msg};

const PANDOC:   &str = "pandoc";
const MD_OPT:   &str = "--from=markdown+grid_tables+pipe_tables-simple_tables+raw_html+implicit_figures+footnotes+intraword_underscores+auto_identifiers-inline_code_attributes";
const HTML_OPT: &str = "--smart --normalize --standalone --self-contained --highlight-style=tango --table-of-contents --section-divs --template=lib/template.html --css=lib/pandoc.css --to=html5";
const EPUB_OPT: &str = "--smart --normalize --standalone --self-contained --highlight-style=tango --epub-stylesheet=lib/epub.css --table-of-contents";

pub enum OutputType { Html, Epub }

impl OutputType {
    fn file_extension(&self) -> &'static str {
        match self {
            OutputType::Html => "html",
            OutputType::Epub => "epub",
        }
    }

    fn pandoc_args(&self) -> &'static str {
        match self {
            OutputType::Html => HTML_OPT,
            OutputType::Epub => EPUB_OPT,
        }
    }
}

pub fn create(
    contents: &str,
    file_prefix: &str,
    output_type: OutputType,
    release_date: &str
) -> Result<(), Error> {
    let child = Command::new(PANDOC)
        .arg(MD_OPT)
        .arg(output_type.pandoc_args())
        .arg(
            &format!(
                "--output=dist/{}-{}.{}",
                file_prefix,
                release_date,
                output_type.file_extension()
            )
        )
        .stdin(Stdio::piped())
        .spawn()
        .context("Failed to execute pandoc")?;
    
    if let Some(mut stdin) = child.stdin {
        stdin.write(contents.as_bytes())?;
        Ok(())
    } else {
        Err(err_msg("Failed to get pandoc stdin"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
