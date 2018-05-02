macro_rules! lazy_static_regex {
    ($name:ident, $source:expr) => {
        lazy_static! {
            static ref $name: ::regex::Regex = ::regex::Regex::new($source)
                .expect(&format!("Invalid regex: {}", $source));
        }
    };
}
