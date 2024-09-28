use reqwest::Client;
use scraper::{Html, Selector};
use anyhow::Result;

pub struct WebAPI {
    client: Client,
}

impl WebAPI {
    pub fn new() -> Self {
        WebAPI {
            client: Client::new(),
        }
    }

    pub async fn fetch_webpage_content(&self, url: &str) -> Result<String> {
        Ok(self.client.get(url).send().await?.text().await?)
    }

    pub fn extract_text_from_html(&self, html: &str) -> String {
        let document = Html::parse_document(html);
        let selector = Selector::parse("body").unwrap();
        document
            .select(&selector)
            .flat_map(|element| element.text())
            .collect::<Vec<_>>()
            .join(" ")
    }

    pub async fn fetch_and_extract_text(&self, url: &str) -> Result<String> {
        let html = self.fetch_webpage_content(url).await?;
        Ok(self.extract_text_from_html(&html))
    }
}
