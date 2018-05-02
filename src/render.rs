use aux::file;
use failure::Error;
use std::fmt::{self, Display, Formatter};

#[derive(Clone, Copy)]
pub enum Format { Html, Epub, Markdown }

impl Format {
    pub fn file_extension(&self) -> &'static str {
        match self {
            Format::Html => "html",
            Format::Epub => "epub",
            Format::Markdown => "md",
        }
    }
}

impl Display for Format {
    fn fmt(&self, mut fmt: &mut Formatter) -> fmt::Result {
        write!(&mut fmt, "{}", match self {
            Format::Html => "HTML",
            Format::Epub => "ePub",
            Format::Markdown => "Markdown",
        })
    }
}

pub fn to_file(
    markdown: &str,
    prefix: &str,
    format: Format,
    release_date: &str,
) -> Result<(), Error> {
    match format {
        Format::Markdown => file::from_string(
            format!("dist/{}-{}.md", prefix, release_date),
            markdown
        ).map_err(Into::into),
        
        _ => pandoc::render_to_file(markdown, prefix, format, release_date),
    }
}

mod pandoc {
    use super::Format;
    use failure::{Error, ResultExt};
    use std::{
        process::{Command, Stdio},
        io::Write,
    };

    const PANDOC: &str = "pandoc";

    mod options {
        pub const TO_ALL : &str = "--standalone --self-contained --highlight-style=tango --table-of-contents";
        pub const TO_HTML: &str = "--css=lib/pandoc.css --to=html5 --section-divs --template=lib/template.html";
        pub const TO_EPUB: &str = "--css=lib/epub.css";
        pub const FROM_MD: &str = indoc!("
            --from=markdown+grid_tables+pipe_tables-simple_tables+raw_html+implicit\
            _figures+footnotes+intraword_underscores+auto_identifiers-inline_code\
            _attributes
            ");
    }

    fn options_for(format: Format) -> &'static str {
        use self::Format::*;
        match format {
            Html => options::TO_HTML,
            Epub => options::TO_EPUB,
            Markdown => unreachable!(),
        }
    }

    pub fn render_to_file(
        contents: &str,
        file_prefix: &str,
        format: Format,
        release_date: &str
    ) -> Result<(), Error> {
        let mut child = Command::new(PANDOC)
            .arg(options::FROM_MD)
            .args(options::TO_ALL.split(' '))
            .args(options_for(format).split(' '))
            .arg(&format!(
                "--output=dist/{}-{}.{}",
                file_prefix,
                release_date,
                format.file_extension()
            ))
            .stdin(Stdio::piped())
            .spawn()
            .context("Failed to execute pandoc")?;
        
        if let Some(stdin) = child.stdin.as_mut() {
            stdin.write_all(contents.as_bytes())?;
        } else {
            bail!("Failed to get pandoc stdin");
        }

        if !child.wait()?.success() {
            bail!("Pandoc exited unsuccessfully.");
        }

        Ok(())
    }

    #[cfg(test)]
    mod tests {
        use super::*;
    }
}
