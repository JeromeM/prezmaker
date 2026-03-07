use super::movie::AllocineRatings;
use super::selectors;
use super::AllocineClient;
use scraper::{Html, Selector};
use tracing::debug;

impl AllocineClient {
    pub async fn search_series_ratings(
        &self,
        title: &str,
        year: Option<u16>,
    ) -> anyhow::Result<AllocineRatings> {
        let search_url = format!(
            "https://www.allocine.fr/recherche/series/?q={}",
            urlencoding::encode(title)
        );
        debug!("Allocine series search: {}", search_url);

        let html = self
            .client
            .get(&search_url)
            .send()
            .await?
            .text()
            .await?;

        let document = Html::parse_document(&html);
        let item_sel =
            Selector::parse(selectors::SEARCH_RESULT_ITEM).expect("Invalid selector");
        let link_sel =
            Selector::parse(selectors::SEARCH_RESULT_LINK).expect("Invalid selector");

        let mut best_url: Option<String> = None;

        for item in document.select(&item_sel) {
            if let Some(link) = item.select(&link_sel).next() {
                let href = link.value().attr("href").unwrap_or_default();
                let item_title = link.text().collect::<String>().trim().to_string();

                let title_lower = title.to_lowercase();
                let item_title_lower = item_title.to_lowercase();

                if item_title_lower.contains(&title_lower)
                    || title_lower.contains(&item_title_lower)
                {
                    best_url = Some(format!("https://www.allocine.fr{}", href));
                    break;
                }

                if let Some(y) = year {
                    let year_str = y.to_string();
                    let item_text = item.text().collect::<String>();
                    if item_text.contains(&year_str) {
                        best_url = Some(format!("https://www.allocine.fr{}", href));
                        break;
                    }
                }

                if best_url.is_none() {
                    best_url = Some(format!("https://www.allocine.fr{}", href));
                }
            }
        }

        let detail_url = best_url
            .ok_or_else(|| anyhow::anyhow!("No Allocine series results found"))?;
        debug!("Allocine series detail page: {}", detail_url);

        self.scrape_ratings(&detail_url).await
    }
}
