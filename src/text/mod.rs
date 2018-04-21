#[macro_use] pub mod patterns;
pub mod normalize;
pub mod references;

use self::patterns::reg;

pub trait OrDefaultExt: AsRef<str> + Sized {
    #[inline(always)]
    fn to_option(self) -> Option<Self> {
        let has_arg = {
            let s: &str = self.as_ref();
            !s.is_empty() && !s.is_whitespace()
        };
        if has_arg { Some(self) } else { None }
    }

    fn or_default(self, other: Self) -> Self {
        self.to_option().unwrap_or(other)
    }

    fn or_else_default<F>(self, f: F) -> Self where F: FnOnce() -> Self {
        self.to_option().unwrap_or_else(f)
    }
}

impl<S> OrDefaultExt for S where S: AsRef<str> {}

pub trait MatchExt: AsRef<str> + Sized {
    fn toggles_code_block(&self) -> bool {
        self
            .as_ref()
            .starts_with(patterns::CODE_BLOCK_TOGGLE)
    }

    fn ending_newlines(&self) -> String {
        let is_newline = |c: &char| *c == '\n';
        self
            .as_ref()
            .chars()
            .rev()
            .take_while(is_newline)
            .collect()
    }
}

impl<S> MatchExt for S where S: AsRef<str> {}

pub trait AdjustExt: AsRef<str> + Sized {
    fn line_break_at(self, max_len: usize, separator: &str) -> String {
        let adjusted_len = max_len - separator.chars().count();

        let mut collector = String::with_capacity(self.as_ref().len());
        let (first, rest) = self.as_ref().split_at(max_len);
        
        collector.push_str(first);
        collector.push('\n');

        let rest_chars: Vec<char> = rest.chars().collect();
        let rest_lines = rest_chars.as_slice().chunks(adjusted_len);
        
        for line in rest_lines {
            collector.push_str(separator);
            for chr in line { collector.push(*chr) }
            collector.push('\n');
        }

        collector.pop(); // Remove extra newline
        collector
    }

    fn increase_title_level(self, increase: usize) -> String {
        lazy_static_regex!(HEADER_PATTERN, reg::mdfile::HEADER);

        let mut in_code_block = false;
        let mut collector = Vec::new();

        for line in self.as_ref().lines() {
            let toggling_code = line.toggles_code_block();
            
            if in_code_block && !toggling_code {
                collector.push(line.to_string());
                continue;
            }

            if toggling_code { in_code_block = !in_code_block }

            let to_push = if let Some(headline) = HEADER_PATTERN.captures(line) {
                // '#' is always 1 byte, so .len() is safe to use.
                let old_level = headline.name("level").unwrap().as_str().len();
                let new_level = old_level + increase - 1;

                format!(
                    "{empty:#^num_hashes$} {title}\n",
                    empty = "",
                    num_hashes = new_level,
                    title = headline.name("title").unwrap().as_str()
                )
            } else {
                line.to_string()
            };

            collector.push(to_push);
        }

        collector.join("\n")
    }

    fn remove_markdown_file_title(self) -> String {
        lazy_static_regex!(FILE_TITLE, reg::mdfile::TITLE);
        FILE_TITLE.replace(self.as_ref(), "").to_string()
    }
}

impl<S> AdjustExt for S where S: AsRef<str> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn code_block_toggle() {
        assert!("```wharglbargl".toggles_code_block());
        assert!(!"other thing".toggles_code_block());
    }

    #[test]
    fn ending_newlines() {
        assert_eq!(
            "\n\n\n",
            "here's\na thing\n\n\n".ending_newlines()
        )
    }

    const LONG_LINE: &str =
"markdown+grid_tables+pipe_tables+raw_html+implicit_figures\
+footnotes+intraword_underscores+auto_identifiers-inline_code_attributesmarkdow\
n+grid_tables+pipe_tables+raw_html+implicit_figures+footnotes+intraword_undersc\
ores+auto_identifiers-inline_code_attributes";

    const CORRECT_SPLIT: &str =
"markdown+grid_tables+pipe_tables+raw_html+implicit_figures+footnotes+intraword_u
↳ nderscores+auto_identifiers-inline_code_attributesmarkdown+grid_tables+pipe_ta
↳ bles+raw_html+implicit_figures+footnotes+intraword_underscores+auto_identifier
↳ s-inline_code_attributes";

    #[test]
    fn long_lines() {
        assert_eq!(
            CORRECT_SPLIT,
            LONG_LINE.line_break_at(80, "↳ ")
        );
    }

    #[test]
    fn markdown_file_title() {
        assert_eq!(
            "\nthing\nand\nthong",
            "% title\n\nthing\nand\nthong".remove_markdown_file_title()
        );
    }
}
