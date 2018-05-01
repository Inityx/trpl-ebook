use super::patterns::reg;

pub trait NormalizeExt: AsRef<str> + Sized {
    fn normalize_links(self) -> String {
        use self::reg::links::{src, replace};

        lazy_static_regex!(SEC_LINK,    src::SEC       );
        lazy_static_regex!(SEC_REF,     src::SEC_REF   );
        lazy_static_regex!(SUBSEC_LINK, src::SUBSEC    );
        lazy_static_regex!(SUBSEC_REF,  src::SUBSEC_REF);

        let output = self
            .as_ref()
            .replace(r"../std",       r"http://doc.rust-lang.org/std"      )
            .replace(r"../reference", r"http://doc.rust-lang.org/reference")
            .replace(r"../rustc",     r"http://doc.rust-lang.org/rustc"    )
            .replace(r"../syntax",    r"http://doc.rust-lang.org/syntax"   )
            .replace(r"../book",      r"http://doc.rust-lang.org/book"     )
            .replace(r"../adv-book",  r"http://doc.rust-lang.org/adv-book" )
            .replace(r"../core",      r"http://doc.rust-lang.org/core"     );
        
        let output = SEC_LINK   .replace_all(&output, replace::SEC       );
        let output = SEC_REF    .replace_all(&output, replace::SEC_REF   );
        let output = SUBSEC_LINK.replace_all(&output, replace::SUBSEC    );
        let output = SUBSEC_REF .replace_all(&output, replace::SUBSEC_REF);

        output.to_string()
    }

    fn normalize_math(self) -> String {
        lazy_static_regex!(SUPERSCRIPT, reg::math::SUPERSCRIPT_SRC);
        
        SUPERSCRIPT
            .replace_all(self.as_ref(), reg::math::SUPERSCRIPT_REPLACE)
            .to_string()
    }

    fn normalize_code_start(self) -> String {
        use text::MatchExt;

        lazy_static_regex!(RUST_START,  reg::code::BLOCK_RUST);
        lazy_static_regex!(HIDDEN_CODE, reg::code::BLOCK_HIDDEN);

        let mut in_code_block = false;
        let mut collector = String::with_capacity(self.as_ref().len());

        for line in self.as_ref().lines() {
            if in_code_block && HIDDEN_CODE.is_match(line) { continue; }
            
            if RUST_START.is_match(line) {
                in_code_block = true;
                collector.push_str("```rust");
            } else {
                collector.push_str(line);
                if line.toggles_code_block() { in_code_block = false; }
            }

            collector.push('\n');
        }

        collector
    }

    fn normalize_all(self) -> String {
        self
            .normalize_code_start()
            .normalize_links()
            .normalize_math()
    }
}

impl<S> NormalizeExt for S where S: AsRef<str> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn math() {
        assert_eq!(
            "123^1234^thing",
            "123<sup>1234</sup>thing".normalize_math(),
        )
    }

    const CODE_BLOCKS: &str = indoc!("
        Code:

        ```sh
        $ lol
        ```

        ```{rust,ignore}
        let x = true;
        ```

        ``` rust,no_extras
        let x = true;
        ```

        ```rust
        # use magic::from_the_future::*;
        #
        # #[inline]
        # fn extension(file_name: &str) -> Option<&str> {
        #     find(file_name, '.').map(|i| &file_name[i+1..])
        # }
        let x = true;
        ```
        ");

    const CODE_BLOCKS_CLEAN: &str = indoc!("
        Code:

        ```sh
        $ lol
        ```

        ```rust
        let x = true;
        ```

        ```rust
        let x = true;
        ```

        ```rust
        let x = true;
        ```
        ");

    #[test]
    fn code_block_starts() {
        assert_eq!(
            CODE_BLOCKS_CLEAN,
            CODE_BLOCKS.normalize_code_start()
        );
    }
}
