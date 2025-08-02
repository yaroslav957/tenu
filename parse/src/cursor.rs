use core::{iter::Peekable, slice::Iter, str::Chars};

/// Helper trait to flatly iterate over an iterator over strings.
pub struct FlatCursor<'a, I: Iterator> {
    slice_iter: Peekable<I>,
    current_chars: Option<Chars<'a>>,
}

impl<'a, I> FlatCursor<'a, I>
where 
    I: Iterator<Item = &'a str>
{
    pub fn new(iter: I) -> Self
    where 
    {
        FlatCursor {
            slice_iter: iter.peekable(),
            current_chars: None,
        }
    }

    pub fn peek(&mut self) -> Option<char> {
        if let Some(chars) = &mut self.current_chars {
            return chars.clone().next();
        }

        let peeked = self.slice_iter.peek()?;
        peeked.chars().next()
    }

    pub fn peek_str(&self) -> Option<&'a str> {
        self.current_chars
            .clone()
            .map(|s| s.as_str())
    }

    pub fn next_str(&mut self) -> Option<&'a str> {
        if let Some(ref mut chars) = self.current_chars {
            let s = chars.as_str();
            self.current_chars = None;
            return Some(s);
        }

        let next = self.slice_iter.next()?;
        Some(next)
    }
}

impl<'a, I> Iterator for FlatCursor<'a, I>
where 
    I: Iterator<Item = &'a str>
{
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(chars) = &mut self.current_chars {
                if let Some(ch) = chars.next() {
                    return Some(ch);
                }
                self.current_chars = None;
            }

            self.current_chars = Some(self.slice_iter.next().map(str::chars)?);

            // Loop will retry next time with new current_chars
        }
    }
}

#[cfg(test)]
mod tests {
    use alloc::string::String;

    use super::*;

    #[test]
    fn test_mixed() {
        let strings = ["hello", "world"].into_iter();
        let mut iter = FlatCursor::new(strings);
        assert_eq!(iter.next(), Some('h'));
        assert_eq!(iter.next(), Some('e'));
        assert_eq!(iter.next(), Some('l'));
        assert_eq!(iter.next_str(), Some("lo"));
        assert_eq!(iter.next(), Some('w'));
        assert_eq!(iter.next(), Some('o'));
        assert_eq!(iter.peek_str(), Some("rld"));
        assert_eq!(iter.next(), Some('r'));
        assert_eq!(iter.next(), Some('l'));
        assert_eq!(iter.next(), Some('d'));
    }

    #[test]
    fn test_with_str_slice() {
        let strings = ["hello", "world"].into_iter();
        let iter = FlatCursor::new(strings);
        let result: String = iter.collect();
        assert_eq!(result, "helloworld");
    }
}
