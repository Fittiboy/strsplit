#[derive(Debug)]
struct StrSplit<'a> {
    remainder: &'a str,
    delimiter: &'a str,
}

impl<'a> StrSplit<'a> {
    pub fn new(haystack: &'a str, delimiter: &'a str) -> Self {
        StrSplit {
            remainder: haystack,
            delimiter,
        }
    }
}

impl<'a> Iterator for StrSplit<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.delimiter.is_empty() {
            if let Some(idx) = self.remainder.find(self.delimiter) {
                let item = &self.remainder[..idx];
                self.remainder = &self.remainder[(idx + self.delimiter.len())..];
                if self.remainder.is_empty() && idx > 0 {
                    self.remainder = " ";
                }
                return Some(item);
            }
        }
        if self.remainder.is_empty() {
            return None;
        }
        let item = self.remainder;
        self.remainder = "";
        Some(item)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_letters_by_comma() {
        let haystack = "a, b, c, d, e, f, g";
        let delim = ", ";
        let letters: Vec<_> = StrSplit::new(haystack, delim).collect();
        assert_eq!(letters, ["a", "b", "c", "d", "e", "f", "g"]);
    }

    #[test]
    fn split_by_empty_string() {
        let haystack = "a, b, c, d, e, f, g";
        let delim = "";
        let letters: Vec<_> = StrSplit::new(haystack, delim).collect();
        assert_eq!(letters, ["a, b, c, d, e, f, g"]);
    }

    #[test]
    fn yield_trailing_empty() {
        let haystack = "a b c d ";
        let delim = " ";
        let letters: Vec<_> = StrSplit::new(haystack, delim).collect();
        assert_eq!(letters, ["a", "b", "c", "d", ""]);
    }

    #[test]
    fn yield_none_for_empty() {
        let haystack = "";
        let delim = " ";
        let letters: Vec<_> = StrSplit::new(haystack, delim).collect();
        assert_eq!(letters, [""; 0]);
    }
}
