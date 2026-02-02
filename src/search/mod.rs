//! DuckDuckGo web search implementation

use crate::models::{SearchParams, SearchResult, SearchResponse};
use anyhow::Result;
use scraper::{Html, Selector};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::Mutex;

/// Rate limiter to avoid getting blocked
pub struct RateLimiter {
    requests: Arc<Mutex<Vec<Instant>>>,
    max_requests_per_minute: usize,
}

impl RateLimiter {
    pub fn new(max_requests_per_minute: usize) -> Self {
        Self {
            requests: Arc::new(Mutex::new(Vec::new())),
            max_requests_per_minute,
        }
    }

    pub async fn acquire(&self) {
        let wait_time = {
            let mut requests = self.requests.lock().await;
            let now = Instant::now();

            // Remove requests older than 1 minute
            requests.retain(|req| now.duration_since(*req) < Duration::from_secs(60));

            if requests.len() >= self.max_requests_per_minute {
                // Calculate wait time
                if let Some(oldest) = requests.first() {
                    let elapsed = now.duration_since(*oldest);
                    Some(Duration::from_secs(60).saturating_sub(elapsed))
                } else {
                    None
                }
            } else {
                None
            }
        };

        // Sleep outside the lock if needed
        if let Some(wait) = wait_time {
            if wait > Duration::ZERO {
                tokio::time::sleep(wait).await;
            }
        }

        // Add the new request
        let mut requests = self.requests.lock().await;
        requests.push(Instant::now());
    }
}

/// DuckDuckGo web scraper
pub struct DuckDuckGoScraper {
    client: reqwest::Client,
    rate_limiter: RateLimiter,
}

impl DuckDuckGoScraper {
    /// Create a new scraper instance
    pub fn new() -> Self {
        let client = reqwest::Client::builder()
            .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
            .timeout(Duration::from_secs(30))
            .build()
            .expect("Failed to create HTTP client");

        Self {
            client,
            rate_limiter: RateLimiter::new(30), // 30 requests per minute
        }
    }

    /// Perform web search
    pub async fn search(&self, params: &SearchParams) -> Result<SearchResponse> {
        // Apply rate limiting
        self.rate_limiter.acquire().await;
        self.search_basic(params).await
    }

    /// Basic single-page search using POST (more reliable than GET)
    async fn search_basic(&self, params: &SearchParams) -> Result<SearchResponse> {
        // Use POST request like the Python version
        let form_data = &[("q", params.query.as_str()), ("b", ""), ("kl", "")];

        let html = self.fetch_page_post(form_data).await?;

        // Check for CAPTCHA/challenge
        if html.contains("anomaly-modal") || html.contains("challenge-submit") {
            return Ok(SearchResponse {
                query: params.query.clone(),
                results: vec![],
                total_results: 0,
                returned: 0,
                offset: params.offset,
            });
        }

        let mut results = self.parse_results(&html);

        let total = results.len();
        let paginated: Vec<_> = results
            .drain(..)
            .skip(params.offset)
            .take(params.limit)
            .collect();

        let returned = paginated.len();

        Ok(SearchResponse {
            query: params.query.clone(),
            results: paginated,
            total_results: total,
            returned,
            offset: params.offset,
        })
    }

    /// Fetch a single page using POST (more reliable)
    async fn fetch_page_post(&self, form_data: &[(&str, &str)]) -> Result<String> {
        let resp = self.client
            .post("https://html.duckduckgo.com/html/")
            .form(form_data)
            .header("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8")
            .header("Accept-Language", "en-US,en;q=0.9")
            .header("DNT", "1")
            .send()
            .await?;
        Ok(resp.text().await?)
    }

    /// Extract real URL from DuckDuckGo redirect URL
    fn extract_real_url(ddg_url: &str) -> Option<String> {
        // DDG uses redirect URLs like: //duckduckgo.com/l/?uddg=<encoded_url>
        if ddg_url.contains("uddg=") {
            let url_part = ddg_url.split("uddg=").nth(1)?;
            let encoded = url_part.split('&').next().unwrap_or(url_part);
            match urlencoding::decode(encoded) {
                Ok(decoded) => Some(decoded.to_string()),
                Err(_) => None,
            }
        } else if ddg_url.starts_with("//") {
            Some(format!("https:{}", ddg_url))
        } else if ddg_url.starts_with("http://") || ddg_url.starts_with("https://") {
            Some(ddg_url.to_string())
        } else {
            None
        }
    }

    /// Parse HTML results into SearchResult objects
    fn parse_results(&self, html: &str) -> Vec<SearchResult> {
        Self::parse_results_static(html)
    }

    /// Static method to parse results (for use in async closures)
    fn parse_results_static(html: &str) -> Vec<SearchResult> {
        let document = Html::parse_document(html);
        let result_sel = Selector::parse(".web-result, .result").unwrap();
        let title_sel = Selector::parse(".result__a").unwrap();
        let snippet_sel = Selector::parse(".result__snippet").unwrap();

        let mut results = Vec::new();

        for element in document.select(&result_sel) {
            // Get title and URL from .result__a
            let title_el = match element.select(&title_sel).next() {
                Some(el) => el,
                None => continue,
            };

            let title = title_el
                .text()
                .collect::<String>()
                .trim()
                .to_string();

            let ddg_url = title_el
                .value()
                .attr("href")
                .unwrap_or("");

            // Skip ad results (like the Python version does)
            if ddg_url.contains("y.js") {
                continue;
            }

            let url = Self::extract_real_url(ddg_url).unwrap_or_default();

            // Get snippet from .result__snippet
            let snippet = element
                .select(&snippet_sel)
                .next()
                .map(|el| el.text().collect::<String>())
                .unwrap_or_default()
                .trim()
                .to_string();

            if !title.is_empty() && !url.is_empty() {
                results.push(SearchResult { title, url, snippet });
            }
        }

        results
    }

    /// Format results in LLM-friendly natural language style
    pub fn format_results_for_llm(&self, response: &SearchResponse) -> String {
        if response.results.is_empty() {
            return format!(
                "No results found for: {}\n\nThis could be due to rate limiting or no matches. Try rephrasing your search.",
                response.query
            );
        }

        let mut output = format!("Found {} search results for \"{}\":\n\n", response.returned, response.query);

        for (i, result) in response.results.iter().enumerate() {
            output.push_str(&format!(
                "{}. {}\n   URL: {}\n   Summary: {}\n\n",
                i + 1,
                result.title,
                result.url,
                result.snippet
            ));
        }

        output
    }
}

impl Default for DuckDuckGoScraper {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests;
