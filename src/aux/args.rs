use ::docopt::Docopt;

const USAGE: &str = indoc!(r#"
    Compile Rustbook to EBook formats.

    Usage:
    compile-trpl [--prefix PREFIX] [--source DIRECTORY] [--meta FILE]

    Options:
    -h, --help          Show this message
    --prefix PREFIX     Book prefix/short name [default: trpl]
    --source DIRECTORY  Book markdown directory [default: trpl]
    --meta FILE         Metadata, must contain `date: {release_date}` [default: trpl_meta.yml]
    "#);

#[derive(Deserialize, Debug)]
pub struct Args {
    pub flag_prefix: String,
    pub flag_source: String,
    pub flag_meta:   String,
}

pub fn get() -> Args {
    Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn docopt_parsing() {
        let input = ["trpl-ebook", "--prefix=foo", "--source=bar", "--meta=baz"].into_iter();
        let args: Args = Docopt::new(USAGE)
            .and_then(|d| d.argv(input).deserialize())
            .unwrap();
        
        println!("args:");
        println!("{:#?}", args);

        assert_eq!("foo", args.flag_prefix);
        assert_eq!("bar", args.flag_source);
        assert_eq!("baz", args.flag_meta  );
    }

    #[test]
    fn docopt_defaults() {
        let input = ["trpl-ebook"].into_iter();
        let args: Args = Docopt::new(USAGE)
            .and_then(|d| d.argv(input).deserialize())
            .unwrap();

        println!("args:");
        println!("{:#?}", args);

        assert_eq!("trpl",          args.flag_prefix);
        assert_eq!("trpl",          args.flag_source);
        assert_eq!("trpl_meta.yml", args.flag_meta  );
    }
}
