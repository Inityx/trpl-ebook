use std::{
    fmt::{Display, Formatter, self},
    fs::File,
    io::{Read, Write, self},
    path::Path,
};

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

pub fn to_string<P: AsRef<Path>>(path: P) -> io::Result<String> {
    let mut buffer = String::new();
    let mut file = File::open(path)?;
    file.read_to_string(&mut buffer)?;
    Ok(buffer)
}

pub fn from_string<P, S>(path: P, contents: S) -> io::Result<()>
where
    P: AsRef<Path>,
    S: AsRef<str>,
{
    let mut file = File::create(path)?;
    file.write_all(contents.as_ref().as_bytes())
}
