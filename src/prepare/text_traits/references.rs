use std::borrow::Cow;
use regex::Captures;
use super::patterns::{CODE_BLOCK_TOGGLE, reg};

lazy_static_regex!(REF_LINK, reg::reference::LINK);
lazy_static_regex!(FOOTNOTE, reg::reference::FOOTNOTE);

fn replace_ref_link<'t>(line: &'t str, filename: &str) -> Cow<'t, str> {
    let reference_replacer = |capture: &Captures| {
        let link_id = capture.name("id").expect("No id in ref link").as_str();
        format!("][{}--{}]", filename, link_id)
    };

    REF_LINK.replace_all(line, reference_replacer)
}

fn replace_footnote<'t>(line: &'t str, filename: &str) -> Cow<'t, str> {
    let footnote_replacer = |capture: &Captures| {
        let link_id = capture.name("id").expect("No id in footnote").as_str();
        format!("[^{}--{}]", filename, link_id)
    };

    FOOTNOTE.replace_all(line, footnote_replacer)
}

fn replace_ref_url<'t>(line: &'t str, filename: &str) -> Cow<'t, str> {
    lazy_static_regex!(URL, reg::reference::URL);

    let url_replacer = |capture: &Captures| {
        let footnote = capture.name("footnote").map(|s| s.as_str()).unwrap_or("");
        let id       = capture.name("id"  ).expect("No id in ref def"  ).as_str();
        let link     = capture.name("link").expect("No link in ref def").as_str();

        format!("[{}{}--{}]: {}", footnote, filename, id, link)
    };

    URL.replace_all(line, url_replacer)
}

fn replace_all<'t>(
    line: &'t str,
    in_code_block: bool,
    prefix: &str
) -> Cow<'t, str> {
    if in_code_block && !line.starts_with(CODE_BLOCK_TOGGLE) {
        return Cow::from(line);
    }
    
    // TODO: Streamline?
    match line {
        l if REF_LINK.is_match(l) => replace_ref_link(line, prefix),
        l if FOOTNOTE.is_match(l) => replace_footnote(line, prefix),
        l => replace_ref_url(l, prefix),
    }
}

pub trait MdRefsExt: AsRef<str> + Sized{
    fn prefix_refs_with(self, prefix: &str) -> String {
        use super::str_iter::UnlineExt;
        self
            .as_ref()
            .lines()
            .scan(
                false,
                |in_code_block, line| {
                    let new_line = replace_all(line, *in_code_block, prefix);
                    if line.starts_with(CODE_BLOCK_TOGGLE) {
                        *in_code_block = !*in_code_block;
                    }
                    Some(new_line)
                }
            )
            .unlines_hinted(self.as_ref().len())
    }
}

impl<S> MdRefsExt for S where S: AsRef<str> {}

#[cfg(test)]
mod tests {
    use super::*;

    const REFERENCE_PREFIX: &str = "PREFIX";
    const WITH_REFERENCES: &str = indoc!("
        Lorem ipsum [dolor sit][amet], [consectetur adipisicing][elit].

        Odio provident repellendus temporibus possimus magnam odit \
        [neque obcaecati][illo], ab tenetur deserunt quae quia? \
        Asperiores a hic, maiores quaerat, autem ea!
        ");
    const WITH_PREFIXED_REFERENCES: &str = indoc!("
        Lorem ipsum [dolor sit][PREFIX--amet], \
        [consectetur adipisicing][PREFIX--elit].
        
        Odio provident repellendus temporibus possimus magnam odit \
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
