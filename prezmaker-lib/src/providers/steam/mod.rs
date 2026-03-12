pub mod models;

use models::{SteamAppDetailsWrapper, SteamSearchResponse};
use reqwest::Client;
use tracing::debug;

use crate::models::{Game, Genre, Rating, SystemReqs};
use crate::providers::GameProvider;
use async_trait::async_trait;
use std::collections::HashMap;

pub struct SteamClient {
    client: Client,
    language: String,
}

impl SteamClient {
    pub fn new(language: String) -> Self {
        Self {
            client: Client::new(),
            language,
        }
    }

    /// Map Steam language code from our app format (fr-FR -> french, en-US -> english)
    fn steam_language(&self) -> &str {
        if self.language.starts_with("fr") {
            "french"
        } else {
            "english"
        }
    }

    fn country_code(&self) -> &str {
        if self.language.starts_with("fr") {
            "FR"
        } else {
            "US"
        }
    }
}

#[async_trait]
impl GameProvider for SteamClient {
    async fn search_games(&self, query: &str) -> anyhow::Result<Vec<Game>> {
        let url = "https://store.steampowered.com/api/storesearch/";
        debug!("Steam search: {}", query);

        let resp = self
            .client
            .get(url)
            .query(&[
                ("term", query),
                ("l", self.steam_language()),
                ("cc", self.country_code()),
            ])
            .send()
            .await?;

        let mut search: SteamSearchResponse = resp.json().await?;

        // Some games are not indexed in non-English locales; retry in English
        if search.items.is_empty() && self.steam_language() != "english" {
            debug!("Steam search: 0 results in {}, retrying in english", self.steam_language());
            let resp = self
                .client
                .get(url)
                .query(&[("term", query), ("l", "english"), ("cc", "US")])
                .send()
                .await?;
            search = resp.json().await?;
        }

        let games = search
            .items
            .into_iter()
            .map(|item| {
                let metacritic_rating = item
                    .metascore
                    .as_deref()
                    .and_then(|s| s.parse::<f64>().ok())
                    .filter(|&v| v > 0.0)
                    .map(|v| vec![Rating {
                        source: "Metacritic".to_string(),
                        value: v,
                        max: 100.0,
                    }])
                    .unwrap_or_default();

                Game {
                    title: item.name,
                    release_date: None,
                    year: None,
                    synopsis: None,
                    cover_url: item.tiny_image,
                    screenshots: vec![],
                    genres: vec![],
                    platforms: vec![],
                    developers: vec![],
                    publishers: vec![],
                    ratings: metacritic_rating,
                    igdb_id: Some(item.id), // Steam uses igdb_id for lookups
                    igdb_slug: None,
                    steam_appid: Some(item.id),
                    tech_info: None,
                    installation: None,
                    min_reqs: None,
                    rec_reqs: None,
                }
            })
            .collect();

        Ok(games)
    }

    async fn get_game_details(&self, id: u64) -> anyhow::Result<Game> {
        let url = "https://store.steampowered.com/api/appdetails";
        debug!("Steam details: {}", id);

        let resp = self
            .client
            .get(url)
            .query(&[
                ("appids", &id.to_string()),
                ("l", &self.steam_language().to_string()),
                ("cc", &self.country_code().to_string()),
            ])
            .send()
            .await?;

        let body: HashMap<String, SteamAppDetailsWrapper> = resp.json().await?;
        let wrapper = body
            .into_values()
            .next()
            .ok_or_else(|| anyhow::anyhow!("Empty Steam response"))?;

        if !wrapper.success {
            return Err(anyhow::anyhow!("Steam app not found: {}", id));
        }

        let data = wrapper
            .data
            .ok_or_else(|| anyhow::anyhow!("No data for Steam app: {}", id))?;

        let genres = data
            .genres
            .into_iter()
            .map(|g| Genre {
                name: g.description,
            })
            .collect();

        let screenshots = data
            .screenshots
            .into_iter()
            .map(|s| s.path_full)
            .collect();

        let platforms = {
            let mut p = Vec::new();
            if let Some(ref plat) = data.platforms {
                if plat.windows {
                    p.push("Windows".to_string());
                }
                if plat.mac {
                    p.push("macOS".to_string());
                }
                if plat.linux {
                    p.push("Linux".to_string());
                }
            }
            p
        };

        let year = data
            .release_date
            .as_ref()
            .and_then(|rd| extract_year_from_steam_date(&rd.date));

        let release_date = data.release_date.as_ref().map(|rd| rd.date.clone());

        let mut ratings = Vec::new();
        if let Some(mc) = data.metacritic {
            if mc.score > 0 {
                ratings.push(Rating {
                    source: "Metacritic".to_string(),
                    value: mc.score as f64,
                    max: 100.0,
                });
            }
        }

        // Clean HTML from short_description
        let synopsis = data.short_description.map(|s| strip_html_tags(&s));

        // Parse system requirements from Steam HTML
        let min_reqs = data
            .pc_requirements
            .as_ref()
            .and_then(|r| r.minimum.as_deref())
            .map(parse_steam_requirements)
            .filter(|r| !r.is_empty());
        let rec_reqs = data
            .pc_requirements
            .as_ref()
            .and_then(|r| r.recommended.as_deref())
            .map(parse_steam_requirements)
            .filter(|r| !r.is_empty());

        Ok(Game {
            title: data.name,
            release_date,
            year,
            synopsis,
            cover_url: data.header_image,
            screenshots,
            genres,
            platforms,
            developers: data.developers,
            publishers: data.publishers,
            ratings,
            igdb_id: Some(data.steam_appid),
            igdb_slug: None,
            steam_appid: Some(data.steam_appid),
            tech_info: None,
            installation: None,
            min_reqs,
            rec_reqs,
        })
    }
}

/// Extract year from Steam date formats like "9 déc. 2020" or "Dec 9, 2020"
fn extract_year_from_steam_date(date: &str) -> Option<u16> {
    // Try to find a 4-digit year
    date.split(|c: char| !c.is_ascii_digit())
        .filter(|s| s.len() == 4)
        .next()
        .and_then(|y| y.parse::<u16>().ok())
}

/// Parse Steam HTML requirements into a SystemReqs struct.
/// Steam format: `<strong>Label :</strong> value<br>` inside `<li>` elements.
fn parse_steam_requirements(html: &str) -> SystemReqs {
    let mut reqs = SystemReqs::default();

    // Extract each <li> content, strip tags, then match known labels
    for segment in html.split("<li>") {
        let clean = strip_html_tags(segment)
            .replace('\u{00a0}', " ") // non-breaking space → normal space
            .replace('\n', " ")
            .trim()
            .to_string();

        if let Some(v) = extract_after_label(&clean, &[
            "Système d'exploitation :", "Système d'exploitation:", "OS :", "OS:",
        ]) {
            reqs.os = v;
        } else if let Some(v) = extract_after_label(&clean, &[
            "Processeur :", "Processeur:", "Processor :", "Processor:",
        ]) {
            reqs.cpu = v;
        } else if let Some(v) = extract_after_label(&clean, &[
            "Mémoire vive :", "Mémoire vive:", "Memory :", "Memory:",
        ]) {
            reqs.ram = v;
        } else if let Some(v) = extract_after_label(&clean, &[
            "Graphiques :", "Graphiques:", "Graphics :", "Graphics:",
        ]) {
            reqs.gpu = v;
        } else if let Some(v) = extract_after_label(&clean, &[
            "Espace disque :", "Espace disque:", "Storage :", "Storage:",
        ]) {
            reqs.storage = v;
        }
    }

    reqs
}

/// Try to extract the value after one of the given label prefixes.
fn extract_after_label(text: &str, labels: &[&str]) -> Option<String> {
    for label in labels {
        if let Some(pos) = text.find(label) {
            let value = text[pos + label.len()..].trim().to_string();
            if !value.is_empty() {
                return Some(value);
            }
        }
    }
    None
}

/// Strip basic HTML tags from a string
fn strip_html_tags(input: &str) -> String {
    let mut result = String::with_capacity(input.len());
    let mut in_tag = false;
    for ch in input.chars() {
        match ch {
            '<' => in_tag = true,
            '>' => in_tag = false,
            _ if !in_tag => result.push(ch),
            _ => {}
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_year_from_steam_date() {
        assert_eq!(extract_year_from_steam_date("9 déc. 2020"), Some(2020));
        assert_eq!(extract_year_from_steam_date("Dec 9, 2020"), Some(2020));
        assert_eq!(extract_year_from_steam_date("2023"), Some(2023));
        assert_eq!(extract_year_from_steam_date("Coming soon"), None);
    }

    #[test]
    fn test_parse_steam_requirements() {
        let html = r#"<strong>Minimale :</strong><br><ul class="bb_ul"><li><strong>Système d'exploitation :</strong> 64-bit Windows 10<br></li><li><strong>Processeur :</strong> Core i7-6700<br></li><li><strong>Mémoire vive :</strong> 12 GB de mémoire<br></li><li><strong>Graphiques :</strong> GeForce GTX 1060 6GB<br></li><li><strong>Espace disque :</strong> 70 GB d'espace disque disponible<br></li></ul>"#;
        let reqs = parse_steam_requirements(html);
        assert_eq!(reqs.os, "64-bit Windows 10");
        assert_eq!(reqs.cpu, "Core i7-6700");
        assert_eq!(reqs.ram, "12 GB de mémoire");
        assert_eq!(reqs.gpu, "GeForce GTX 1060 6GB");
        assert_eq!(reqs.storage, "70 GB d'espace disque disponible");
    }

    #[test]
    fn test_parse_steam_requirements_nbsp() {
        // Real Steam HTML uses \u{00a0} (non-breaking space) before colons
        let html = "<strong>Minimale\u{00a0}:</strong><br><ul class=\"bb_ul\"><li><strong>Syst\u{00e8}me d'exploitation\u{00a0}:</strong> Windows 10 64 bit\u{ff08}1903\u{ff09}<br></li><li><strong>Processeur\u{00a0}:</strong> INTEL E3-1230v2<br></li><li><strong>M\u{00e9}moire vive\u{00a0}:</strong> 8\u{00a0}GB de m\u{00e9}moire<br></li><li><strong>Graphiques\u{00a0}:</strong> NVIDIA GTX960(4G)<br></li><li><strong>Espace disque\u{00a0}:</strong> 13\u{00a0}GB d'espace disque disponible<br></li></ul>";
        let reqs = parse_steam_requirements(html);
        assert!(!reqs.is_empty(), "Requirements should not be empty");
        assert!(reqs.os.contains("Windows"), "OS should contain Windows, got: {}", reqs.os);
        assert!(reqs.cpu.contains("E3-1230"), "CPU should contain E3-1230, got: {}", reqs.cpu);
        assert!(reqs.ram.contains("8"), "RAM should contain 8, got: {}", reqs.ram);
        assert!(reqs.gpu.contains("GTX960"), "GPU should contain GTX960, got: {}", reqs.gpu);
        assert!(reqs.storage.contains("13"), "Storage should contain 13, got: {}", reqs.storage);
    }

    #[test]
    fn test_strip_html_tags() {
        assert_eq!(strip_html_tags("<b>Bold</b>"), "Bold");
        assert_eq!(strip_html_tags("No tags"), "No tags");
        assert_eq!(
            strip_html_tags("<p>First</p><br>Second"),
            "FirstSecond"
        );
    }
}
