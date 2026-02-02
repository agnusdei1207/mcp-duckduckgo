//! Unit tests for search module

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_results_static() {
        let html = r#"
            <div class="web-result">
                <a class="result__a" href="https://example.com/test">
                    <div class="result__title">Test Title</div>
                </a>
                <div class="result__snippet">Test snippet content</div>
            </div>
            <div class="web-result">
                <a class="result__a" href="/l/?uddg=https://bad-example.com">
                    <div class="result__title">Redirect Link</div>
                </a>
                <div class="result__snippet">This should be filtered</div>
            </div>
        "#;

        let results = DuckDuckGoScraper::parse_results_static(html);

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].title, "Test Title");
        assert_eq!(results[0].url, "https://example.com/test");
        assert_eq!(results[0].snippet, "Test snippet content");
    }

    #[test]
    fn test_parse_results_filters_redirects() {
        let html = r#"
            <div class="web-result">
                <a class="result__a" href="/l/?uddg=https://redirect.com">
                    <div class="result__title">Redirect Title</div>
                </a>
                <div class="result__snippet">Redirect snippet</div>
            </div>
            <div class="web-result">
                <a class="result__a" href="/l/another">
                    <div class="result__title">Another Redirect</div>
                </a>
                <div class="result__snippet">Another redirect snippet</div>
            </div>
        "#;

        let results = DuckDuckGoScraper::parse_results_static(html);

        // Both should be filtered out
        assert_eq!(results.len(), 0);
    }

    #[test]
    fn test_parse_results_empty_html() {
        let html = r#"<div>No results here</div>"#;
        let results = DuckDuckGoScraper::parse_results_static(html);
        assert_eq!(results.len(), 0);
    }

    #[test]
    fn test_parse_results_malformed_html() {
        let html = r#"
            <div class="web-result">
                <a class="result__a" href="">
                    <div class="result__title"></div>
                </a>
            </div>
            <div class="web-result">
                <a class="result__a" href="https://valid.com">
                    <div class="result__title">Valid Result</div>
                </a>
            </div>
        "#;

        let results = DuckDuckGoScraper::parse_results_static(html);

        // Only the valid result should be included
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].title, "Valid Result");
        assert_eq!(results[0].url, "https://valid.com");
    }

    #[test]
    fn test_default_scraper() {
        let scraper = DuckDuckGoScraper::default();
        // Just verify it can be created
        assert!(scraper.client.timeout().is_some());
    }
}
