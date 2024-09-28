use std::collections::{HashMap, HashSet};
use std::fs;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc, Duration};
use reqwest::blocking::Client;
use scraper::{Html, Selector};
use url::Url;

#[derive(Serialize, Deserialize)]
pub struct LearningModule {
    knowledge_base: HashMap<String, String>,
    #[serde(with = "chrono::serde::ts_seconds")]
    last_backup: DateTime<Utc>,
    #[serde(with = "chrono::serde::ts_seconds")]
    last_update: DateTime<Utc>,
    visited_urls: HashSet<String>,
}

impl LearningModule {
    pub fn new() -> Self {
        let now = Utc::now();
        Self {
            knowledge_base: HashMap::new(),
            last_backup: now,
            last_update: now,
            visited_urls: HashSet::new(),
        }
    }

    pub fn learn_from_web(&mut self, start_url: &str) -> Result<(), Box<dyn std::error::Error>> {
        let client = Client::new();
        let mut to_visit = vec![start_url.to_string()];

        while let Some(url) = to_visit.pop() {
            if self.visited_urls.contains(&url) {
                continue;
            }

            let body = client.get(&url).send()?.text()?;
            let document = Html::parse_document(&body);

            self.process_and_learn(&document);

            self.find_new_links(&document, &url, &mut to_visit);

            self.visited_urls.insert(url);
        }

        self.last_update = Utc::now();
        Ok(())
    }

    fn process_and_learn(&mut self, document: &Html) {
        let text = document.root_element().text().collect::<Vec<_>>().join(" ");
        let words: Vec<&str> = text.split_whitespace().collect();
        for window in words.windows(3) {
            let key = window[0..2].join(" ");
            let value = window[2].to_string();
            if !self.is_personal_data(&key, &value) {
                self.knowledge_base.entry(key).or_insert(value);
            }
        }
    }

    fn find_new_links(&self, document: &Html, base_url: &str, to_visit: &mut Vec<String>) {
        let base_url = Url::parse(base_url).unwrap();
        let selector = Selector::parse("a[href]").unwrap();
        for element in document.select(&selector) {
            if let Some(href) = element.value().attr("href") {
                if let Ok(absolute_url) = base_url.join(href) {
                    let url_string = absolute_url.as_str().to_string();
                    if !self.is_login_page(&url_string) {
                        to_visit.push(url_string);
                    }
                }
            }
        }
    }

    fn backup_knowledge_base(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let content = serde_json::to_string(&self.knowledge_base)?;
        let backup_path = format!("knowledge_base_backup_{}.json", Utc::now().format("%Y%m%d%H%M%S"));
        fs::write(backup_path, content)?;
        self.last_backup = Utc::now();
        Ok(())
    }

    fn update_knowledge_base(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Implement knowledge base update logic here
        // This could involve fetching new data from various sources
        self.last_update = Utc::now();
        Ok(())
    }

    pub fn learn_from_browser(&mut self, url: &str) -> Result<(), Box<dyn std::error::Error>> {
        // Implement web scraping and learning logic here
        // This is a placeholder implementation
        println!("Learning from URL: {}", url);
        // In a real implementation, you would:
        // 1. Fetch the webpage content
        // 2. Parse the HTML
        // 3. Extract relevant information
        // 4. Add the extracted information to the knowledge base
        self.last_update = Utc::now();
        Ok(())
    }

    pub fn learn_from_interaction(&mut self, input: &str, response: &str) {
        if !self.is_sensitive_information(input) {
            self.knowledge_base.insert(input.to_string(), response.to_string());
            self.last_update = Utc::now();
            self.save_knowledge_base().unwrap_or_else(|e| eprintln!("Failed to save knowledge base: {}", e));
        }
    }

    pub fn get_learned_response(&self, input: &str) -> Option<&String> {
        self.knowledge_base.get(input)
    }

    fn load_knowledge_base() -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string("knowledge_base.json")?;
        let module: LearningModule = serde_json::from_str(&content)?;
        Ok(module)
    }

    fn save_knowledge_base(&self) -> Result<(), Box<dyn std::error::Error>> {
        let content = serde_json::to_string(&self)?;
        fs::write("knowledge_base.json", content)?;
        Ok(())
    }

    fn is_sensitive_information(&self, input: &str) -> bool {
        // Implement logic to detect sensitive information
        // For now, we'll just check for some keywords
        let sensitive_keywords = ["password", "credit card", "social security"];
        sensitive_keywords.iter().any(|&keyword| input.to_lowercase().contains(keyword))
    }

    fn is_login_page(&self, url: &str) -> bool {
        url.contains("login") || url.contains("signin") || url.contains("auth")
    }

    fn is_personal_data(&self, key: &str, value: &str) -> bool {
        let personal_data_patterns = [
            "name", "address", "phone", "email", "ssn", "credit card",
            "password", "username", "birth", "social security"
        ];
        personal_data_patterns.iter().any(|&pattern| key.contains(pattern) || value.contains(pattern))
    }

    fn open_url_in_edge(&self, url: &str) -> Result<(), Box<dyn std::error::Error>> {
        use std::process::Command;
        Command::new("cmd")
            .args(&["/C", "start", "msedge", url])
            .spawn()?;
        std::thread::sleep(std::time::Duration::from_secs(5)); // Wait for page to load
        Ok(())
    }
}
