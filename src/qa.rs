use once_cell::sync::Lazy;
use regex::Regex;
use std::iter::FromIterator;

pub fn is_suitable(s: &&str) -> bool {
    let s = s.to_lowercase();

    not_ambiguous(&s)
        && has_to_be(&s)
}

pub fn has_to_be(s: &str) -> bool {
    let word_count = s
        .split_whitespace()
        .count();
    s.split_whitespace()
        .skip(1)
        .take(word_count / 2)
        .any(is_to_be)
}

pub fn not_ambiguous(s: &str) -> bool {
    let starts_with = |start| s.starts_with(start);

    !starts_with("he")
        && !starts_with("she")
        && !starts_with("it")
        && !starts_with("this")
}

pub fn is_to_be(s: &str) -> bool {
    let is = |needle| s == needle;
    is("is")
        || is("are")
        || is("was")
        || is("were")
}

pub fn lowercase_first_char(s: String) -> String {
    if !s.starts_with("I ") {
        let mut chars = s.chars();
        chars
            .next()
            .map(char::to_lowercase)
            .map(String::from_iter)
            .unwrap_or_default()
            + chars.as_str()
    } else { s }
}

pub fn remove_citations(s: String) -> String {
    static CITATION_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"\[(?:\d+|[[:alpha:]])]").unwrap());

    CITATION_REGEX
        .replace_all(&s, "")
        .into()
}

pub fn not_common_junk(s: &&str) -> bool {
    use std::ops::Not;

    [
        "pages are in this category, out of",
        "article",
        "Wikipedia",
        "page",
        "the following",
        "MIME type: image",
        "\n",
        "logo is of a size and resolution sufficient"
    ].iter()
        .any(|junk| s.contains(junk))
        .not()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_lowercase_first_char() {
        assert_eq!(
            lowercase_first_char("Test string".into()),
            "test string"
        );
        assert_eq!(
            lowercase_first_char("test string".into()),
            "test string"
        );
        assert_eq!(
            lowercase_first_char("I am a test string".into()),
            "I am a test string"
        )
    }

    #[test]
    fn test_remove_citations() {
        assert_eq!(
            remove_citations("citation.[24][A]".into()),
            "citation."
        )
    }
}