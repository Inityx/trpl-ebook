use std::{
    process::{Command, Stdio},
    io::Write,
};

use failure::{Error, ResultExt};

use file::Format;

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

pub fn create(
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
