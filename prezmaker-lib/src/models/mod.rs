pub mod application;
pub mod common;
pub mod game;
pub mod movie;
pub mod series;

pub use application::Application;
pub use common::{Country, Genre, MediaTechInfo, Person, Rating};
pub use game::{Game, TechInfo};
pub use movie::Movie;
pub use series::Series;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Tracker {
    C411,
    TorrXyz,
}

impl std::fmt::Display for Tracker {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tracker::C411 => write!(f, "C411"),
            Tracker::TorrXyz => write!(f, "torr.xyz"),
        }
    }
}
