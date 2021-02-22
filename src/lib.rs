#[cfg(test)]
mod tests {
    use super::*;
    fn tst(input: &str, expected: &str) {
        assert_eq!(next(input).unwrap().1, expected);
    }
    #[test]
    fn next_word1() {
        tst("hello world", "hello");
    }

    #[test]
    fn next_word2() {
        tst("  \nhello world", "hello");
    }

    #[test]
    fn next_word3() {
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

fn is_letter(c: char) -> bool {
    c >= 'a' && c <= 'z' || c >= 'A' && c <= 'Z' || c >= '0' && c <= '9'
}

fn next(i: &str) -> nom::IResult<&str, &str> {
    use nom::bytes::complete::*;
    let (rest, _whitespace) = take_till(is_letter)(i)?;

    take_while1(|c| is_letter(c as char))(rest)
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
        let (rest, word) = next(self.rest).ok()?;
        self.rest = rest;
        Some(word)
    }
}
