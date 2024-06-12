use reqwest::Client;
use scraper::{Html, Selector};
use std::fmt::{Formatter, Result};

#[tokio::main]
async fn main() {
    struct Article {
        url: String,
        date: String,
        summary: String,
    }

    impl std::fmt::Debug for Article {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(
                f,
                "Article {{url: {}, date: {}, summary: {}}}",
                self.url, self.date, self.summary
            )
        }
    }

    let mut articles: Vec<Article> = Vec::new();

    let client: Client = Client::new();

    let response = client
        .get("https://oilprice.com/Latest-Energy-News/World-News/Page-1.html")
        .send()
        .await
        .unwrap();

    let html_content = response.text().await.unwrap();

    let document = Html::parse_document(&html_content);

    let html_article_selector = Selector::parse("div.categoryArticle__content").unwrap();

    let html_articles = document.select(&html_article_selector);

    for article in html_articles {
        let article_url = article
            .select(&Selector::parse("a").unwrap())
            .next()
            .and_then(|a| a.value().attr("href"))
            .map(str::to_owned);

        let article_summary = article
            .select(&Selector::parse("p.categoryArticle__excerpt").unwrap())
            .next()
            .map(|p| p.text().collect::<String>());

        let article_date = article
            .select(&Selector::parse("p.categoryArticle__meta").unwrap())
            .next()
            .map(|p| p.text().collect::<String>());

        let single_article = Article {
            url: String::from(article_url.unwrap()),
            date: String::from(article_date.unwrap()),
            summary: String::from(article_summary.unwrap()),
        };

        articles.push(single_article);
    }
    println!("{:#?}", articles);
}
