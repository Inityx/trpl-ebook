use std::{
    process::{Command, Stdio},
    io::Write,
};

use failure::{Error, ResultExt, err_msg};

use super::FileFormat;

const PANDOC: &str = "pandoc";

mod options {
    pub const FROM_MD: &str = "--from=markdown+grid_tables+pipe_tables-simple_tables+raw_html+implicit_figures+footnotes+intraword_underscores+auto_identifiers-inline_code_attributes";
    pub const TO_ALL : &str = "--standalone --self-contained --highlight-style=tango --table-of-contents";
    pub const TO_HTML: &str = "--css=lib/pandoc.css --to=html5 --section-divs --template=lib/template.html";
    pub const TO_EPUB: &str = "--css=lib/epub.css";
}

fn options_for(format: FileFormat) -> &'static str {
    use self::FileFormat::*;
    match format {
        Html => options::TO_HTML,
        Epub => options::TO_EPUB,
        Markdown => unreachable!(),
    }
}

pub fn create(
    contents: &str,
    file_prefix: &str,
    output_type: FileFormat,
    release_date: &str
) -> Result<(), Error> {
    let mut child = Command::new(PANDOC)
        .arg(options::FROM_MD)
        .args(options::TO_ALL.split(' '))
        .args(options_for(output_type).split(' '))
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
    
    if let Some(stdin) = child.stdin.as_mut() {
        stdin.write_all(contents.as_bytes())?;
    } else {
        return Err(err_msg("Failed to get pandoc stdin"));
    }

    if child.wait()?.success() {
        Ok(())
    } else {
        Err(err_msg("Pandoc exited unsuccessfully."))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
