use super::patterns::reg;

pub trait NormalizeExt {
    fn normalize_links(self) -> String;
    fn normalize_math(self) -> String;
    fn normalize_code_start(self) -> String;
    fn break_code_blocks(self, max_len: usize, separator: &str) -> String;

    fn normalize_all(self, code_max_len: usize, code_separator: &str) -> String
    where
        Self: Sized
    {
        self
            .break_code_blocks(code_max_len, code_separator)
            .normalize_code_start()
            .normalize_links()
            .normalize_math()
    }
}

impl<S> NormalizeExt for S where S: AsRef<str> {
    fn normalize_links(self) -> String {
        lazy_static_regex!(SEC_LINK,    reg::links::src::SEC       );
        lazy_static_regex!(SEC_REF,     reg::links::src::SEC_REF   );
        lazy_static_regex!(SUBSEC_LINK, reg::links::src::SUBSEC    );
        lazy_static_regex!(SUBSEC_REF,  reg::links::src::SUBSEC_REF);

        let output = self
            .as_ref()
            .replace(r"../std",       r"http://doc.rust-lang.org/std"      )
            .replace(r"../reference", r"http://doc.rust-lang.org/reference")
            .replace(r"../rustc",     r"http://doc.rust-lang.org/rustc"    )
            .replace(r"../syntax",    r"http://doc.rust-lang.org/syntax"   )
            .replace(r"../book",      r"http://doc.rust-lang.org/book"     )
            .replace(r"../adv-book",  r"http://doc.rust-lang.org/adv-book" )
            .replace(r"../core",      r"http://doc.rust-lang.org/core"     );
        
        let output = SEC_LINK   .replace_all(&output, reg::links::replace::SEC       );
        let output = SEC_REF    .replace_all(&output, reg::links::replace::SEC_REF   );
        let output = SUBSEC_LINK.replace_all(&output, reg::links::replace::SUBSEC    );
        let output = SUBSEC_REF .replace_all(&output, reg::links::replace::SUBSEC_REF);

        output.to_string()
    }

    fn normalize_math(self) -> String {
        lazy_static_regex!(SUPERSCRIPT, reg::math::SUPERSCRIPT_SRC);
        SUPERSCRIPT
            .replace_all(
                self.as_ref(),
                reg::math::SUPERSCRIPT_REPLACE
            )
            .to_string()
    }

    fn break_code_blocks(self, max_len: usize, separator: &str) -> String {
        use text::{MatchExt, AdjustExt};

        let mut in_code_block = false;
        let mut collector = Vec::new();

        for line in self.as_ref().lines() {
            let toggling_code = line.toggles_code_block();

            let to_push = if in_code_block && !toggling_code {
                line.line_break_at(max_len, separator)
            } else {
                if toggling_code { in_code_block = !in_code_block; }
                line.to_string()
            };

            collector.push(to_push);
        }

        collector.join("\n")
    }

    fn normalize_code_start(self) -> String {
        use text::MatchExt;

        lazy_static_regex!(RUST_START,  reg::code::BLOCK_RUST);
        lazy_static_regex!(HIDDEN_CODE, reg::code::BLOCK_HIDDEN);

        let mut in_code_block = false;
        let mut collector = Vec::new();

        for line in self.as_ref().lines() {
            if in_code_block && HIDDEN_CODE.is_match(line) { continue; }
            
            if RUST_START.is_match(line) {
                in_code_block = true;
                collector.push("```rust");
                continue;
            }

            if line.toggles_code_block() { in_code_block = false; }
            
            collector.push(line);
        }

        collector.join("\n")
    }
}


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

    const LONG_CODE_BLOCK: &str = "If we truly want a reference, we need the \
other option: ensure that our reference goes out of scope before we try to do \
the mutation. That looks like this:

```text
Whew! The Rust compiler gives quite detailed errors at times, and this is one \
of those times. As the error explains, while we made our binding mutable, we \
still cannot call `push`. This is because we already have a reference to an \
element of the vector, `y`. Mutating something while another reference exists \
is dangerous, because we may invalidate the reference. In this specific case, \
when we create the vector, we may have only allocated space for three \
elements. Adding a fourth would mean allocating a new chunk of memory for all \
those elements, copying the old values over, and updating the internal \
pointer to that memory. That all works just fine.
```

We created an inner scope with an additional set of curly braces. `y` will go \
out of scope before we call `push()`, and so we’re all good.";

    const CODE_BLOCK_BROKEN_DOWN: &str = "If we truly want a reference, we \
need the other option: ensure that our reference goes out of scope before we \
try to do the mutation. That looks like this:

```text
Whew! The Rust compiler gives quite detailed errors at times, and this is one of
↳  those times. As the error explains, while we made our binding mutable, we sti
↳ ll cannot call `push`. This is because we already have a reference to an eleme
↳ nt of the vector, `y`. Mutating something while another reference exists is da
↳ ngerous, because we may invalidate the reference. In this specific case, when 
↳ we create the vector, we may have only allocated space for three elements. Add
↳ ing a fourth would mean allocating a new chunk of memory for all those element
↳ s, copying the old values over, and updating the internal pointer to that memo
↳ ry. That all works just fine.
```

We created an inner scope with an additional set of curly braces. `y` will go \
out of scope before we call `push()`, and so we’re all good.";

    #[test]
    fn code_block_breaking() {
        assert_eq!(
            CODE_BLOCK_BROKEN_DOWN,
            LONG_CODE_BLOCK.break_code_blocks(80, "↳ ")
        );
    }

    const CODE_BLOCKS: &str = "Code:

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
```";

    const CODE_BLOCKS_CLEAN: &str = "Code:

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
```";

    #[test]
    fn code_block_starts() {
        assert_eq!(
            CODE_BLOCKS_CLEAN,
            CODE_BLOCKS.normalize_code_start()
        );
    }
}
