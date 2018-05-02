use std::{
    fs::File,
    io::{Read, Write, self},
    path::Path,
};

pub fn to_string<P>(path: P) -> io::Result<String>
where
    P: AsRef<Path>
{
    let mut buffer = String::new();
    File::open(path)?.read_to_string(&mut buffer)?;
    Ok(buffer)
}

pub fn from_string<P, S>(path: P, contents: S) -> io::Result<()>
where
    P: AsRef<Path>,
    S: AsRef<str>,
{
    File::create(path)?.write_all(contents.as_ref().as_bytes())
}
