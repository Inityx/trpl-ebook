pub trait UnlineExt<S>: Iterator<Item = S> + Sized
where
    S: AsRef<str>
{
    fn unlines_hinted(self, size_hint: usize) -> String {
        self.fold(
            String::with_capacity(size_hint),
            |acc, s| acc + s.as_ref() + "\n",
        )
    }
    
    fn unlines(self) -> String {
        // If no string size hint available, default to iterator size hint
        let size_hint = self.size_hint().0;
        self.unlines_hinted(size_hint)
    }
}

impl<S, I> UnlineExt<S> for I
where
    S: AsRef<str>,
    I: Iterator<Item = S>,
{}

#[cfg(tests)]
mod test {
    use super::*;

    #[test]
    fn unlines_single() {
        assert_eq!(
            "foo\n",
            std::iter::once("foo").unlines(),
        )
    }

    #[test]
    fn unlines_multiple() {
        const FBB: &str = "foo\nbar\nbaz\n";
        assert_eq!(
            FBB,
            FBB.lines().unlines();
        )
    }
}
