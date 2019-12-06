use crate::wikipedia;
use wikipedia::Download;
use select::predicate::Name;
use crate::qa::*;

pub fn get_statement() -> Option<String> {
    wikipedia::random_articles()
        .filter(|article| !article
            .title()
            .contains("talk:")
        ).downloaded()
        .map(|article| article
            .find(Name("p"))
            .map(|node| node.text())
            .collect::<String>()
        ).filter_map(|text| text
        .split('.')
        .map(str::trim)
        .filter(|s| s.len() > 1)
        .filter(not_common_junk)
        .filter(is_suitable)
        .next()
        .map(String::from)
    ).next()
        .map(remove_citations)
        .map(lowercase_first_char)
}