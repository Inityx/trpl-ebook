use regex::Captures;

use std::{
    borrow::Cow,
    ops::Add,
};

use super::{
    MatchExt,
    patterns::reg,
};

lazy_static_regex!(REF_LINK, reg::reference::LINK);
lazy_static_regex!(FOOTNOTE, reg::reference::FOOTNOTE);

fn replace_ref_link<'t>(line: &'t str, prefix: &str) -> Cow<'t, str> {
    let reference_replacer = |capture: &Captures| format!(
        "][{prefix}--{id}]",
        prefix = prefix,
        id = capture
            .name("id")
            .expect("no id in ref link")
            .as_str()
    );
    REF_LINK.replace_all(line, reference_replacer)
}

fn replace_footnote<'t>(line: &'t str, prefix: &str) -> Cow<'t, str> {
    let footnote_replacer = |capture: &Captures|  format!(
        "[^{prefix}--{id}]",
        prefix = prefix,
        id = capture
            .name("id")
            .expect("no id in footnote")
            .as_str()
    );

    FOOTNOTE.replace_all(line, footnote_replacer)
}

fn replace_ref_defn<'t>(line: &'t str, prefix: &str) -> Cow<'t, str> {
    lazy_static_regex!(REF_DEFN, reg::reference::DEFINITION);
    let defn_replacer = |capture: &Captures| format!(
        "[{footnote}{prefix}--{id}]: {link}",
        footnote = capture.name("footnote").map(|s| s.as_str()).unwrap_or(""),
        prefix = prefix,
        id   = capture.name("id"  ).unwrap().as_str(),
        link = capture.name("link").unwrap().as_str()
    );

    REF_DEFN.replace_all(line, defn_replacer)
}

fn replace_all<'t>(
    line: &'t str,
    in_code_block: bool,
    prefix: &str
) -> Cow<'t, str> 
{
    if in_code_block && !line.toggles_code_block() {
        Cow::from(line)
    } else {
        // TODO: Streamline?
        match line {
            l if REF_LINK.is_match(l) => replace_ref_link(line, prefix),
            l if FOOTNOTE.is_match(l) => replace_footnote(line, prefix),
            l => replace_ref_defn(l, prefix),
        }
    }
}

pub trait MdRefsExt: AsRef<str> + Sized {
    fn prefix_refs_with(self, prefix: &str) -> String {
        self
            .as_ref()
            .lines()
            .scan(
                false,
                |in_code_block, line| {
                    let new_line = replace_all(line, *in_code_block, prefix);
                    if line.toggles_code_block() { *in_code_block = !*in_code_block; }
                    Some(new_line)
                }
            )
            .collect::<String>()
            .add("\n")
    }
}

impl<S> MdRefsExt for S where S: AsRef<str> {}

#[cfg(test)]
mod tests {
    use super::*;

    const REFERENCE_PREFIX: &str = "PREFIX";
    const WITH_REFERENCES: &str = indoc!("
        Lorem ipsum [dolor sit][amet], [consectetur adipisicing][elit]. \
        Odio provident repellendus temporibus possimus magnam odit \
        [neque obcaecati][illo], ab tenetur deserunt quae quia? \
        Asperiores a hic, maiores quaerat, autem ea!
        ");
    const WITH_PREFIXED_REFERENCES: &str = indoc!("
        Lorem ipsum [dolor sit][PREFIX--amet], \
        [consectetur adipisicing][PREFIX--elit]. Odio provident \
        repellendus temporibus possimus magnam odit \
        [neque obcaecati][PREFIX--illo], ab tenetur deserunt quae quia? \
        Asperiores a hic, maiores quaerat, autem ea!
        ");

    #[test]
    fn reference_renaming() {
        assert_eq!(
            WITH_PREFIXED_REFERENCES.to_string(),
            WITH_REFERENCES.prefix_refs_with(REFERENCE_PREFIX)
        );
    }
}
