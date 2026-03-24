use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct IgdbGame {
    pub id: u64,
    pub name: String,
    pub slug: Option<String>,
    pub summary: Option<String>,
    pub first_release_date: Option<i64>,
    pub genres: Option<Vec<IgdbGenre>>,
    pub platforms: Option<Vec<IgdbPlatform>>,
    pub involved_companies: Option<Vec<IgdbInvolvedCompany>>,
    pub cover: Option<IgdbCover>,
    pub screenshots: Option<Vec<IgdbScreenshot>>,
    pub total_rating: Option<f64>,
    pub aggregated_rating: Option<f64>,
    #[serde(default, deserialize_with = "deserialize_external_games")]
    pub external_games: Option<Vec<IgdbExternalGame>>,
    pub videos: Option<Vec<IgdbVideo>>,
}

#[derive(Debug, Deserialize)]
pub struct IgdbExternalGame {
    pub category: u32,
    pub uid: Option<String>,
}

/// IGDB peut retourner external_games comme des objets expandés OU des IDs bruts (u64).
/// Ce désérialiseur gère les deux cas sans crasher.
fn deserialize_external_games<'de, D>(
    deserializer: D,
) -> Result<Option<Vec<IgdbExternalGame>>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let val = Option::<Vec<serde_json::Value>>::deserialize(deserializer)?;
    match val {
        Some(arr) => {
            let games: Vec<IgdbExternalGame> = arr
                .into_iter()
                .filter_map(|item| serde_json::from_value::<IgdbExternalGame>(item).ok())
                .collect();
            if games.is_empty() {
                Ok(None)
            } else {
                Ok(Some(games))
            }
        }
        None => Ok(None),
    }
}

#[derive(Debug, Deserialize)]
pub struct IgdbVideo {
    pub video_id: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct IgdbGenre {
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct IgdbPlatform {
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct IgdbInvolvedCompany {
    pub company: IgdbCompany,
    pub developer: bool,
    pub publisher: bool,
}

#[derive(Debug, Deserialize)]
pub struct IgdbCompany {
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct IgdbCover {
    pub image_id: String,
}

#[derive(Debug, Deserialize)]
pub struct IgdbScreenshot {
    pub image_id: String,
}

impl IgdbCover {
    pub fn url_big(&self) -> String {
        format!(
            "https://images.igdb.com/igdb/image/upload/t_cover_big/{}.jpg",
            self.image_id
        )
    }
}

impl IgdbScreenshot {
    pub fn url_hd(&self) -> String {
        format!(
            "https://images.igdb.com/igdb/image/upload/t_screenshot_huge/{}.jpg",
            self.image_id
        )
    }
}
