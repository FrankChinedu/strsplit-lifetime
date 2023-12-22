// #![warn(missing_debug_implementations, missing_docs)]

struct StrSplit<'haystack, 'delimeter> {
    remainder: Option<&'haystack str>,
    delimeter: &'delimeter str,
}

impl<'haystack, 'delimeter> StrSplit<'haystack, 'delimeter> {
    pub fn new(haystack: &'haystack str, delimeter: &'delimeter str) -> Self {
        Self {
            remainder: Some(haystack),
            delimeter,
        }
    }
}

impl<'haystack, 'delimeter> Iterator for StrSplit<'haystack, 'delimeter> {
    type Item = &'haystack str;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(ref mut remainder) = self.remainder {
            if let Some(next_delim) = remainder.find(self.delimeter) {
                let until_delimeter = &remainder[..next_delim];
                *remainder = &remainder[(next_delim + self.delimeter.len())..];
                Some(until_delimeter)
            } else {
                self.remainder.take()
            }
        } else {
            None
        }
    }
}

fn until_char(s: &str, c: char) -> &str {
    StrSplit::new(s, &format!("{}", c))
        .next()
        .expect("StrSplit always gives at least one result")
}

#[test]
fn until_char_test() {
    assert_eq!(until_char("hello world", 'o'), "hell");
}

#[test]
fn it_works() {
    let haystack = "a b c d e";
    let letters = StrSplit::new(haystack, " ").collect::<Vec<_>>();
    assert_eq!(letters, vec!["a", "b", "c", "d", "e"])
}

#[test]
fn tail() {
    let haystack = "a b c d ";
    let letters = StrSplit::new(haystack, " ").collect::<Vec<_>>();
    assert_eq!(letters, vec!["a", "b", "c", "d", ""])
}
