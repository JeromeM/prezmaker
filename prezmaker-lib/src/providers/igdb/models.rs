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
