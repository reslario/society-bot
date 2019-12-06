use select::document::Document;
use once_cell::sync::Lazy;
use reqwest::Client;

const RANDOM_ARTICLE_URL: &str = "https://www.wikipedia.org/w/api.php?action=query&list=random&format=json";
const GET_ARTICLE_URL: &str = "https://www.wikipedia.org/w/api.php?action=query&prop=info&format=json&inprop=url&pageids=";
static CLIENT: Lazy<Client> = Lazy::new(|| Client::new());

pub struct RandomArticles;

impl Iterator for RandomArticles {
    type Item = RandomArticle;

    fn next(&mut self) -> Option<Self::Item> {
        CLIENT
            .get(RANDOM_ARTICLE_URL)
            .send()
            .and_then(|mut resp| resp.text())
            .map(|text| json::parse(&text).unwrap())
            .map(|json| json["query"]["random"][0]
                .to_string()
            ).map(|json| serde_json::from_str(&json)
                .unwrap()
            ).ok()
    }
}

#[derive(serde::Deserialize)]
pub struct RandomArticle {
    id: i32,
    #[allow(dead_code)]
    ns: i32,
    title: String
}

impl RandomArticle {
    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn title(&self) -> &str {
        &self.title
    }
}

pub struct Downloaded<I> {
    inner: I,
}

impl <I> Iterator for Downloaded<I>
where I: Iterator<Item=RandomArticle> {
    type Item = Document;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner
            .next()
            .as_ref()
            .map(RandomArticle::id)
            .and_then(|id| CLIENT
                .get(&format!("{}{}", GET_ARTICLE_URL, id))
                .send()
                .and_then(|mut resp| resp.text())
                .map(|text| json::parse(&text).unwrap())
                .map(|json| json["query"]["pages"][&id.to_string()]["fullurl"]
                    .to_string()
                ).and_then(|url| CLIENT
                    .get(&url)
                    .send()
                    .and_then(|mut resp| resp.text())
                    .map(|text| Document::from(text.as_str()))
                ).ok()
            )
    }
}

pub trait Download: Sized {
    fn downloaded(self) -> Downloaded<Self>;
}

impl <I> Download for I
where I: Iterator<Item=RandomArticle> {
    fn downloaded(self) -> Downloaded<Self> {
        Downloaded {
            inner: self,
        }
    }
}

pub fn random_articles() -> RandomArticles {
    RandomArticles
}