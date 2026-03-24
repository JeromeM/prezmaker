pub mod cache;
pub mod collections;
pub mod config;
pub mod db;
pub mod error;
pub mod formatters;
pub mod models;
pub mod orchestrator_api;
pub mod providers;
pub mod torrent;
pub mod torrent_creator;
pub mod template_engine;
pub mod default_templates;
pub mod default_templates_html;
pub mod mediainfo;
pub mod nfo;
pub mod upload;

/// Shared HTTP client for simple requests
pub fn http_client() -> reqwest::Client {
    reqwest::Client::new()
}
