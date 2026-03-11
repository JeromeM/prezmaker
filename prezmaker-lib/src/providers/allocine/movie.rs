use super::selectors;
use super::AllocineClient;
use crate::models::Rating;
use anyhow::Context;
use scraper::{Html, Selector};
use tracing::{debug, warn};

#[derive(Debug)]
pub struct AllocineRatings {
    pub press: Option<f64>,
    pub spectators: Option<f64>,
    pub page_url: Option<String>,
}

impl AllocineClient {
    pub async fn search_movie_ratings(
        &self,
        title: &str,
        year: Option<u16>,
    ) -> anyhow::Result<AllocineRatings> {
        let search_url = format!(
            "https://www.allocine.fr/recherche/film/?q={}",
            urlencoding::encode(title)
        );
        debug!("Allocine search: {}", search_url);

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

        // Find best matching result
        let mut best_url: Option<String> = None;

        for item in document.select(&item_sel) {
            if let Some(link) = item.select(&link_sel).next() {
                let href = link.value().attr("href").unwrap_or_default();
                let item_title = link.text().collect::<String>().trim().to_string();

                // Simple title matching
                let title_lower = title.to_lowercase();
                let item_title_lower = item_title.to_lowercase();

                if item_title_lower.contains(&title_lower)
                    || title_lower.contains(&item_title_lower)
                {
                    best_url = Some(format!("https://www.allocine.fr{}", href));
                    break;
                }

                // If year matches, take it
                if let Some(y) = year {
                    let year_str = y.to_string();
                    let item_text = item.text().collect::<String>();
                    if item_text.contains(&year_str) {
                        best_url = Some(format!("https://www.allocine.fr{}", href));
                        break;
                    }
                }

                // Default to first result
                if best_url.is_none() {
                    best_url = Some(format!("https://www.allocine.fr{}", href));
                }
            }
        }

        let detail_url =
            best_url.context("No Allocine results found")?;
        debug!("Allocine detail page: {}", detail_url);

        let mut ratings = self.scrape_ratings(&detail_url).await?;
        ratings.page_url = Some(detail_url);
        Ok(ratings)
    }

    pub async fn scrape_ratings(&self, url: &str) -> anyhow::Result<AllocineRatings> {
        let html = self.client.get(url).send().await?.text().await?;
        let document = Html::parse_document(&html);

        let press = Self::extract_rating(&document, selectors::RATING_PRESS);
        let spectators = Self::extract_rating(&document, selectors::RATING_SPECTATORS);

        if press.is_none() && spectators.is_none() {
            warn!("No ratings found on Allocine page: {}", url);
        }

        Ok(AllocineRatings { press, spectators, page_url: None })
    }

    fn extract_rating(document: &Html, selector_str: &str) -> Option<f64> {
        let selector = Selector::parse(selector_str).ok()?;
        let element = document.select(&selector).next()?;
        let text = element.text().collect::<String>();
        let cleaned = text.trim().replace(',', ".");
        cleaned.parse::<f64>().ok()
    }

    pub fn ratings_to_vec(ratings: &AllocineRatings) -> Vec<Rating> {
        let mut result = Vec::new();
        if let Some(press) = ratings.press {
            result.push(Rating {
                source: "Allocine Presse".to_string(),
                value: press,
                max: 5.0,
            });
        }
        if let Some(spectators) = ratings.spectators {
            result.push(Rating {
                source: "Allocine Spectateurs".to_string(),
                value: spectators,
                max: 5.0,
            });
        }
        result
    }
}
