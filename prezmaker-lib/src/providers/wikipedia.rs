use reqwest::Client;
use serde::Deserialize;
use tracing::debug;

pub struct WikipediaClient {
    http: Client,
}

#[derive(Deserialize)]
struct SearchResponse {
    query: Option<SearchQuery>,
}

#[derive(Deserialize)]
struct SearchQuery {
    search: Vec<SearchItem>,
}

#[derive(Deserialize)]
struct SearchItem {
    title: String,
}

#[derive(Deserialize)]
struct ExtractResponse {
    query: Option<ExtractQuery>,
}

#[derive(Deserialize)]
struct ExtractQuery {
    pages: std::collections::HashMap<String, PageExtract>,
}

#[derive(Deserialize)]
struct PageExtract {
    extract: Option<String>,
}

impl WikipediaClient {
    pub fn new() -> Self {
        Self {
            http: Client::new(),
        }
    }

    pub async fn search_game_description(&self, title: &str) -> anyhow::Result<Option<String>> {
        debug!("Wikipedia FR recherche : {}", title);

        // 1. Search for the game page
        let search_url = format!(
            "https://fr.wikipedia.org/w/api.php?action=query&list=search&srsearch=\"{}\" jeu vidéo&format=json&srlimit=3",
            title
        );

        let resp: SearchResponse = self.http
            .get(&search_url)
            .send()
            .await?
            .json()
            .await?;

        let page_title = match resp.query.and_then(|q| q.search.into_iter().next()) {
            Some(item) => item.title,
            None => {
                debug!("Wikipedia : aucun resultat pour {}", title);
                return Ok(None);
            }
        };

        debug!("Wikipedia : page trouvee : {}", page_title);

        // 2. Get the extract (intro)
        let extract_url = format!(
            "https://fr.wikipedia.org/w/api.php?action=query&titles={}&prop=extracts&exintro=1&explaintext=1&format=json",
            urlencoding::encode(&page_title)
        );

        let resp: ExtractResponse = self.http
            .get(&extract_url)
            .send()
            .await?
            .json()
            .await?;

        let extract = resp.query
            .and_then(|q| {
                q.pages.into_values().next().and_then(|p| p.extract)
            })
            .unwrap_or_default();

        if extract.len() > 100 {
            debug!("Wikipedia : extrait de {} caracteres", extract.len());
            Ok(Some(extract))
        } else {
            debug!("Wikipedia : extrait trop court ({} chars)", extract.len());
            Ok(None)
        }
    }
}
