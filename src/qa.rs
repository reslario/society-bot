use once_cell::sync::Lazy;
use regex::Regex;

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

pub fn lowercase_first_char(mut s: String) -> String {
    if !s.starts_with("I ") {
        s.replace_range(
            0..1,
            &s.chars()
                .next()
                .unwrap()
                .to_ascii_lowercase()
                .to_string()
        );
    }
    s
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