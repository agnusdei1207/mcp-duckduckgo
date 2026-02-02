//! DuckDuckGo web search implementation

use crate::models::{SearchParams, SearchResult, SearchResponse};
use anyhow::Result;
use scraper::{Html, Selector};
use std::time::Duration;
use tokio::time::timeout;

/// DuckDuckGo web scraper
pub struct DuckDuckGoScraper {
    client: reqwest::Client,
}

impl DuckDuckGoScraper {
    /// Create a new scraper instance
    pub fn new() -> Self {
        let client = reqwest::Client::builder()
            .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
            .timeout(Duration::from_secs(30))
            .build()
            .expect("Failed to create HTTP client");

        Self { client }
    }

    /// Perform web search with pagination support
    pub async fn search(&self, params: &SearchParams) -> Result<SearchResponse> {
        // For large result sets, use extended search
        if params.limit > 50 {
            self.search_extended(params).await
        } else {
            self.search_basic(params).await
        }
    }

    /// Basic single-page search
    async fn search_basic(&self, params: &SearchParams) -> Result<SearchResponse> {
        let url = format!(
            "https://html.duckduckgo.com/html/?q={}&kl=kr-kr",
            urlencoding::encode(&params.query)
        );

        let html = self.fetch_page(&url).await?;
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

    /// Extended multi-page search for large result sets
    async fn search_extended(&self, params: &SearchParams) -> Result<SearchResponse> {
        let mut all_results = Vec::new();
        let base_url = format!(
            "https://html.duckduckgo.com/html/?q={}&kl=kr-kr",
            urlencoding::encode(&params.query)
        );

        // Fetch first page
        let html = self.fetch_page(&base_url).await?;
        all_results.extend(self.parse_results(&html));

        // Calculate pages needed
        let max_results = params.limit + params.offset;
        let pages_needed = (max_results + 29) / 30;

        // Fetch additional pages in parallel
        if pages_needed > 1 {
            let mut tasks = Vec::new();

            for page in 2..=(pages_needed.min(100)) {
                let url = format!("{}&s={}", base_url, (page - 1) * 30);
                let client = self.client.clone();

                tasks.push(tokio::spawn(async move {
                    match Self::fetch_page_with_client(&client, &url).await {
                        Ok(h) => Some(h),
                        Err(_) => None,
                    }
                }));
            }

            // Collect results from all pages
            for task in tasks {
                match timeout(Duration::from_secs(10), task).await {
                    Ok(Ok(Some(html))) => {
                        all_results.extend(Self::parse_results_static(&html));
                        if all_results.len() >= max_results {
                            break;
                        }
                    }
                    _ => continue,
                }
            }
        }

        let total = all_results.len();
        let paginated: Vec<_> = all_results
            .into_iter()
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

    /// Fetch a single page
    async fn fetch_page(&self, url: &str) -> Result<String> {
        let resp = self.client.get(url).send().await?;
        Ok(resp.text().await?)
    }

    /// Fetch page with provided client (for parallel requests)
    async fn fetch_page_with_client(client: &reqwest::Client, url: &str) -> Result<String> {
        let resp = client.get(url).send().await?;
        Ok(resp.text().await?)
    }

    /// Parse HTML results into SearchResult objects
    fn parse_results(&self, html: &str) -> Vec<SearchResult> {
        Self::parse_results_static(html)
    }

    /// Static method to parse results (for use in async closures)
    fn parse_results_static(html: &str) -> Vec<SearchResult> {
        let document = Html::parse_document(html);
        let result_sel = Selector::parse(".web-result").unwrap();
        let title_sel = Selector::parse(".result__title").unwrap();
        let url_sel = Selector::parse(".result__a").unwrap();
        let snippet_sel = Selector::parse(".result__snippet").unwrap();

        document
            .select(&result_sel)
            .filter_map(|element| {
                let title = element
                    .select(&title_sel)
                    .next()?
                    .text()
                    .collect::<String>()
                    .trim()
                    .to_string();

                let url = element
                    .select(&url_sel)
                    .next()?
                    .value()
                    .attr("href")
                    .unwrap_or("")
                    .to_string();

                let snippet = element
                    .select(&snippet_sel)
                    .next()
                    .map(|el| el.text().collect::<String>())
                    .unwrap_or_default()
                    .trim()
                    .to_string();

                // Filter out redirect URLs and empty results
                if !title.is_empty()
                    && !url.is_empty()
                    && !url.contains("/l/?uddg=")
                    && !url.starts_with("/l/")
                {
                    Some(SearchResult { title, url, snippet })
                } else {
                    None
                }
            })
            .collect()
    }
}

impl Default for DuckDuckGoScraper {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests;
