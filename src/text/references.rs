use regex::Captures;

use super::{
    MatchExt,
    patterns::reg,
};

lazy_static_regex!(REF_LINK, reg::reference::LINK);
lazy_static_regex!(FOOTNOTE, reg::reference::FOOTNOTE);

fn replace_ref_link(line: &str, prefix: &str) -> String {
    let reference_replacer = |matches: &::regex::Captures| {
        format!(
            "][{prefix}--{id}]",
            prefix = prefix,
            id = matches
                .name("id")
                .expect("no id in ref link")
                .as_str()
        )
    };
    REF_LINK.replace_all(line, reference_replacer).to_string()
}

fn replace_footnote(line: &str, prefix: &str) -> String {
    let reference_replacer = |matches: &Captures| {
        format!(
            "[^{prefix}--{id}]",
            prefix = prefix,
            id = matches
                .name("id")
                .expect("no id in footnote")
                .as_str()
        )
    };

    FOOTNOTE
        .replace_all(line, reference_replacer)
        .to_string()
}

fn replace_ref_defn(line: &str, prefix: &str) -> Option<String> {
    lazy_static_regex!(REF_DEFN, reg::reference::DEFINITION);
    REF_DEFN.captures(line).map(|matches|
        format!(
            "[{footnote}{prefix}--{id}]: {link}",
            footnote = matches.name("footnote").map(|s| s.as_str()).unwrap_or(""),
            prefix = prefix,
            id   = matches.name("id"  ).unwrap().as_str(),
            link = matches.name("link").unwrap().as_str()
        )
    )
}

pub trait MdRefsExt: AsRef<str> + Sized {
    fn prefix_refs_with(self, prefix: &str) -> String {

        let mut in_code_block = false;
        let mut collector = Vec::new();

        for line in self.as_ref().lines() {
            let toggling_code = line.toggles_code_block();

            if in_code_block && !toggling_code {
                collector.push(line.to_string());
                continue;
            }

            if toggling_code {
                in_code_block = !in_code_block;
            }
            
            let to_push = match line {
                l if REF_LINK.is_match(l) => replace_ref_link(line, prefix),
                l if FOOTNOTE.is_match(l) => replace_footnote(line, prefix),
                l => replace_ref_defn(l, prefix).unwrap_or_else(|| l.to_string()),
            };

            collector.push(to_push);
        }

        collector.join("\n")
    }
}

impl<S> MdRefsExt for S where S: AsRef<str> {}

#[cfg(test)]
mod tests {
    use super::*;

    const REFERENCE_PREFIX: &str = "PREFIX";
    const WITH_REFERENCES: &str =
        "Lorem ipsum [dolor sit][amet], [consectetur adipisicing][elit]. \
        Odio provident repellendus temporibus possimus magnam odit \
        [neque obcaecati][illo], ab tenetur deserunt quae quia? \
        Asperiores a hic, maiores quaerat, autem ea!";
    const WITH_PREFIXED_REFERENCES: &str =
        "Lorem ipsum [dolor sit][PREFIX--amet], \
        [consectetur adipisicing][PREFIX--elit]. Odio provident \
        repellendus temporibus possimus magnam odit \
        [neque obcaecati][PREFIX--illo], ab tenetur deserunt quae quia? \
        Asperiores a hic, maiores quaerat, autem ea!";

    #[test]
    fn reference_renaming() {
        assert_eq!(
            WITH_PREFIXED_REFERENCES.to_string(),
            WITH_REFERENCES.prefix_refs_with(REFERENCE_PREFIX)
        );
    }
}
