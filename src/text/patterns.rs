#[macro_use]
pub mod reg {
    macro_rules! lazy_static_regex {
        ($name:ident, $source:expr) => {
            lazy_static! {
                static ref $name: ::regex::Regex = ::regex::Regex::new($source)
                    .expect(format!("Invalid regex: {}", $source).as_str());
            }
        };
    }

    pub mod code {
        pub const BLOCK_RUST:   &str = r"^```(.*)rust(.*)";
        pub const BLOCK_HIDDEN: &str = r"^(#\s.*|#$)";
    }

    pub mod links {
        pub mod src {
            pub const SEC       : &str = r"]\((?P<file>[\w-_]+)\.html\)";
            pub const SEC_REF   : &str = r"(?m)^\[(?P<id>.+)\]:\s(?P<file>[^:^/]+)\.html$";
            pub const SUBSEC    : &str = r"]\((?P<file>[\w-_]+)\.html#(?P<subsection>[\w-_]+)\)";
            pub const SUBSEC_REF: &str = r"(?m)^\[(?P<id>.+)\]:\s(?P<file>[^:^/]+)\.html#(?P<subsection>[\w-_]+)$";
        }
        pub mod replace {
            pub const SEC       : &str = r"](#sec--$file)";
            pub const SEC_REF   : &str = r"[$id]: #sec--$file";
            pub const SUBSEC    : &str = r"](#$subsection)";
            pub const SUBSEC_REF: &str = r"[$id]: #$subsection";
        }
    }

    pub mod reference {
        pub const LINK      : &str = r"(?x)\]\[(?P<id>.+?)\]";
        pub const FOOTNOTE  : &str = r"(?x)\[\^(?P<id>.+?)\]";
        pub const DEFINITION: &str = r"(?x)^\[(?P<footnote>\^)?(?P<id>.+)\]:\s(?P<link>.+)$";
    }

    pub mod math {
        pub const SUPERSCRIPT_SRC:     &str = r"(\d+)<sup>(\d+)</sup>";
        pub const SUPERSCRIPT_REPLACE: &str = r"$1^$2^";
    }

    pub mod mdfile {
        pub const TITLE:  &str = r"^%\s(.+)\n";
        pub const HEADER: &str = r"(?x)^(?P<level>[\#]+)\s(?P<title>.+)$";
        pub const NAME:   &str = r"^(?P<prefix>\w+)-(?P<date>\d{4}-\d{2}-\d{2})\.(?P<ext>.+)$";
    }
}

pub const CODE_BLOCK_TOGGLE: &str = "```";
