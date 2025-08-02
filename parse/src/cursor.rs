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
        if self.current_chars.is_none() {
            self.current_chars = Some(self.slice_iter.next()?.chars());
        }

        self.current_chars.as_ref()?.clone().next()
    }

    pub fn peek_str(&self) -> Option<&'a str> {
        self.current_chars
            .clone()
            .map(|s| s.as_str())
    }

    pub fn next_str(&mut self) -> Option<&'a str> {
        self.current_chars
            .clone()
            .map(|iter| {
                let s = iter.as_str();
                self.current_chars = self.slice_iter.next().map(str::chars);
                s
            })
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

                if self.slice_iter.peek().is_some() {
                    return Some(' ');
                }
            }

            match self.slice_iter.next() {
                Some(item) => {
                    self.current_chars = Some(item.chars());
                }
                None => return None,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use alloc::string::String;

    use super::*;

    #[test]
    fn test_with_str_slice() {
        let strings = ["hello", "world"].into_iter();
        let iter = FlatCursor::new(strings);
        let result: String = iter.collect();
        assert_eq!(result, "hello world");
    }

    #[test]
    fn another_test() {
        let strings = ["-ohedgeberry-s-mother.jpg", "zov", "--hello=world"].into_iter();
        let iter = FlatCursor::new(strings);
        let result: String = iter.collect();
        assert_eq!(result, "-ohedgeberry-s-mother.jpg zov --hello=world");
    }
}
