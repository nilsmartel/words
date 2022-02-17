#[cfg(test)]
mod tests {
    use super::*;
    fn tst(input: &str, expected: &str) {
        assert_eq!(input.words().next().unwrap(), expected);
    }
    #[test]
    fn next_word() {
        tst("hello world", "hello");
    }

    #[test]
    fn cut_whitespace() {
        tst("  \nhello world", "hello");
    }

    #[test]
    fn remove_redundant_characters() {
        tst(".hello world", "hello");
    }

    #[test]
    fn sentence() {
        let input = "Hello world, my name is Nils Martel. I love coding in rust";
        let expected = vec![
            "Hello", "world", "my", "name", "is", "Nils", "Martel", "I", "love", "coding", "in",
            "rust",
        ];

        assert_eq!(input.words().collect::<Vec<_>>(), expected);
    }
}

fn next(i: &str) -> Option<(&str, &str)> {
    // start by cutting of all characters, that are not alphabetic
    let mut start = 0;
    for c in i.chars() {
        if c.is_alphabetic() {
            break;
        }
        start += c.len_utf8();
    }

    // now i is guaranteed to start with some alphabetic character
    let i = &i[start..];

    let mut end = 0;
    for c in i.chars() {
        if !c.is_alphabetic() {
            break;
        }

        end += c.len_utf8();
    }
    if end == 0 {
        return None;
    }

    Some((&i[..end], &i[end..]))
}

pub trait Words<'a> {
    fn words(self) -> WordIter<'a>;
}

impl<'a> Words<'a> for &'a str {
    fn words(self) -> WordIter<'a> {
        WordIter { rest: self }
    }
}

pub struct WordIter<'a> {
    rest: &'a str,
}

impl<'a> Iterator for WordIter<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        let (word, rest) = next(self.rest)?;
        self.rest = rest;
        Some(word)
    }
}
