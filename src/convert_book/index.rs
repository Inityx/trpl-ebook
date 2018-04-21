use std::{
    collections::BTreeMap,
    fs,
    io,
    path::Path,
};

use regex::Match;
use failure::{Error, ResultExt};
use ::text::patterns::reg;

lazy_static_regex!(MD_FILENAME, reg::mdfile::NAME);

struct VerFname {
    version: String,
    filename: String,
}

fn list_file_groups<P: AsRef<Path>>(path: P) -> Result<Vec<VerFname>, io::Error> {
    let files = fs::read_dir(&path)?
        .filter_map(Result::ok)
        .filter_map(|x| x.file_name().to_str().map(str::to_string))
        .flat_map(|filename|
            MD_FILENAME
                .captures(&filename)
                // Extract the date from names like
                // 'trpl-2015-05-13.a4.pdf'. This also excludes
                // the `index.html` file as it contains no date.
                .and_then(|caps| caps.name("date"))
                .as_ref()
                .map(Match::as_str)
                .map(str::to_string)
                .map(|version| VerFname { version, filename })
        )
        .collect();

    Ok(files)
}

pub fn render_index<P: AsRef<Path>>(path: P) -> Result<String, Error> {
    let ver_fnames = list_file_groups(path)
        .context("Failed to get file group listing")?
        .into_iter()
        .fold(
            BTreeMap::<String, Vec<String>>::new(),
            |mut acc, vf| {
                acc.entry(vf.version).or_default().push(vf.filename);
                acc
            }
        );

    let mut file_listing = String::new();

    for (version, filenames) in ver_fnames.into_iter().rev() {
        file_listing.push_str("<li>\n<h2>");
        file_listing.push_str(&version);
        file_listing.push_str("</h2>\n<ul>");

        for filename in filenames {
            file_listing.push_str(&format!(
                "<li><a href='{name}'>{title}</a></li>\n",
                name = filename,
                title = MD_FILENAME
                    .replace_all(&filename, "$prefix $ext")
                    .to_ascii_uppercase()
            ));
        }

        file_listing.push_str("</ul>\n</li>");
    }

    let output = format!(
        include_str!("../../lib/index_template.html"),
        css = include_str!("../../lib/index.css"),
        file_listing = file_listing
    );

    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;
}
