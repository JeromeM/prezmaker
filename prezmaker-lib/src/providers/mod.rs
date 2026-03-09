pub mod allocine;
pub mod igdb;
pub mod llm;
pub mod steam;
pub mod tmdb;
pub mod translator;
pub mod wikipedia;

use crate::models::{Game, Movie, Series};
use async_trait::async_trait;

#[async_trait]
pub trait MovieProvider: Send + Sync {
    async fn search_movies(&self, query: &str) -> anyhow::Result<Vec<Movie>>;
    async fn get_movie_details(&self, id: u64) -> anyhow::Result<Movie>;
}

#[async_trait]
pub trait SeriesProvider: Send + Sync {
    async fn search_series(&self, query: &str) -> anyhow::Result<Vec<Series>>;
    async fn get_series_details(&self, id: u64) -> anyhow::Result<Series>;
}

#[async_trait]
pub trait GameProvider: Send + Sync {
    async fn search_games(&self, query: &str) -> anyhow::Result<Vec<Game>>;
    async fn get_game_details(&self, id: u64) -> anyhow::Result<Game>;
}
