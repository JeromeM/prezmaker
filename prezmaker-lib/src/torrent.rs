use regex::Regex;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TorrentFile {
    pub path: String,
    pub size: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TorrentMeta {
    pub name: String,
    pub files: Vec<TorrentFile>,
    pub total_size: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum DetectedContentType {
    Film,
    Serie,
    Jeu,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReleaseParsed {
    pub content_type: DetectedContentType,
    pub title: String,
    pub year: Option<u32>,
    pub quality: Option<String>,
    pub video_codec: Option<String>,
    pub audio: Option<String>,
    pub language: Option<String>,
    pub group: Option<String>,
    pub season: Option<u32>,
    pub episode: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TorrentInfo {
    pub meta: TorrentMeta,
    pub parsed: ReleaseParsed,
    pub size_formatted: String,
}

// Bencode value for parsing
#[derive(Debug)]
enum BValue {
    Str(Vec<u8>),
    Int(i64),
    List(Vec<BValue>),
    Dict(Vec<(Vec<u8>, BValue)>),
}

fn decode_bencode(data: &[u8], pos: &mut usize) -> Result<BValue, String> {
    if *pos >= data.len() {
        return Err("Unexpected end of data".into());
    }
    match data[*pos] {
        b'i' => {
            *pos += 1;
            let end = data[*pos..]
                .iter()
                .position(|&b| b == b'e')
                .ok_or("Missing 'e' for integer")?
                + *pos;
            let s = std::str::from_utf8(&data[*pos..end]).map_err(|e| e.to_string())?;
            let val = s.parse::<i64>().map_err(|e| e.to_string())?;
            *pos = end + 1;
            Ok(BValue::Int(val))
        }
        b'l' => {
            *pos += 1;
            let mut list = Vec::new();
            while *pos < data.len() && data[*pos] != b'e' {
                list.push(decode_bencode(data, pos)?);
            }
            if *pos < data.len() {
                *pos += 1;
            }
            Ok(BValue::List(list))
        }
        b'd' => {
            *pos += 1;
            let mut dict = Vec::new();
            while *pos < data.len() && data[*pos] != b'e' {
                let key = match decode_bencode(data, pos)? {
                    BValue::Str(s) => s,
                    _ => return Err("Dict key must be string".into()),
                };
                let val = decode_bencode(data, pos)?;
                dict.push((key, val));
            }
            if *pos < data.len() {
                *pos += 1;
            }
            Ok(BValue::Dict(dict))
        }
        b'0'..=b'9' => {
            let colon = data[*pos..]
                .iter()
                .position(|&b| b == b':')
                .ok_or("Missing ':' for string")?
                + *pos;
            let len_s = std::str::from_utf8(&data[*pos..colon]).map_err(|e| e.to_string())?;
            let len = len_s.parse::<usize>().map_err(|e| e.to_string())?;
            *pos = colon + 1;
            if *pos + len > data.len() {
                return Err("String length exceeds data".into());
            }
            let s = data[*pos..*pos + len].to_vec();
            *pos += len;
            Ok(BValue::Str(s))
        }
        b => Err(format!("Unexpected byte: {}", b)),
    }
}

impl BValue {
    fn as_dict(&self) -> Option<&Vec<(Vec<u8>, BValue)>> {
        match self {
            BValue::Dict(d) => Some(d),
            _ => None,
        }
    }

    fn get(&self, key: &[u8]) -> Option<&BValue> {
        self.as_dict()?.iter().find(|(k, _)| k == key).map(|(_, v)| v)
    }

    fn as_str(&self) -> Option<String> {
        match self {
            BValue::Str(s) => String::from_utf8(s.clone()).ok(),
            _ => None,
        }
    }

    fn as_int(&self) -> Option<i64> {
        match self {
            BValue::Int(i) => Some(*i),
            _ => None,
        }
    }

    fn as_list(&self) -> Option<&Vec<BValue>> {
        match self {
            BValue::List(l) => Some(l),
            _ => None,
        }
    }
}

pub fn parse_torrent_file(path: &Path) -> Result<TorrentMeta, String> {
    let data = std::fs::read(path).map_err(|e| format!("Cannot read torrent: {}", e))?;
    let mut pos = 0;
    let root = decode_bencode(&data, &mut pos)?;

    let info = root.get(b"info").ok_or("Missing 'info' dict")?;
    let name = info
        .get(b"name")
        .and_then(|v| v.as_str())
        .unwrap_or_default();

    let mut files = Vec::new();
    let mut total_size: u64 = 0;

    if let Some(file_list) = info.get(b"files").and_then(|v| v.as_list()) {
        // Multi-file torrent
        for f in file_list {
            let size = f
                .get(b"length")
                .and_then(|v| v.as_int())
                .unwrap_or(0) as u64;
            let path_parts: Vec<String> = f
                .get(b"path")
                .and_then(|v| v.as_list())
                .map(|parts| parts.iter().filter_map(|p| p.as_str()).collect())
                .unwrap_or_default();
            let file_path = path_parts.join("/");
            total_size += size;
            files.push(TorrentFile {
                path: file_path,
                size,
            });
        }
    } else if let Some(length) = info.get(b"length").and_then(|v| v.as_int()) {
        // Single-file torrent
        total_size = length as u64;
        files.push(TorrentFile {
            path: name.clone(),
            size: total_size,
        });
    }

    Ok(TorrentMeta {
        name,
        files,
        total_size,
    })
}

pub fn format_size(bytes: u64) -> String {
    const GO: u64 = 1_073_741_824;
    const MO: u64 = 1_048_576;

    if bytes >= GO {
        format!("{:.2} Go", bytes as f64 / GO as f64)
    } else {
        format!("{:.0} Mo", bytes as f64 / MO as f64)
    }
}

pub fn parse_release_name(name: &str) -> ReleaseParsed {
    let clean = name.replace('.', " ").replace('_', " ");

    // Season/Episode
    let season_re = Regex::new(r"(?i)S(\d{2})(?:E(\d{2}))?").unwrap();
    let (season, episode) = season_re
        .captures(&clean)
        .map(|c| {
            let s = c.get(1).and_then(|m| m.as_str().parse().ok());
            let e = c.get(2).and_then(|m| m.as_str().parse().ok());
            (s, e)
        })
        .unwrap_or((None, None));

    // Year (only realistic release years: 1900-2030)
    let year_re = Regex::new(r"\b((?:19|20)\d{2})\b").unwrap();
    let year: Option<u32> = year_re
        .captures_iter(&clean)
        .filter_map(|c| c.get(1)?.as_str().parse::<u32>().ok())
        .find(|&y| y >= 1900 && y <= 2030);

    // Quality
    let quality_re =
        Regex::new(r"(?i)\b(2160p|1080p|720p|480p|4K|UHD|REMUX|BDRip|BRRip|HDRip|WEB-?DL|WEBRip|HDTV|DVDRip|BluRay|Blu-Ray)\b")
            .unwrap();
    let quality = quality_re
        .captures(&clean)
        .map(|c| c.get(0).unwrap().as_str().to_string());

    // Video codec
    let codec_re =
        Regex::new(r"(?i)\b(x264|x265|H\.?264|H\.?265|HEVC|AV1|AVC|MPEG-?2|XviD|DivX)\b")
            .unwrap();
    let video_codec = codec_re
        .captures(&clean)
        .map(|c| c.get(0).unwrap().as_str().to_string());

    // Audio
    let audio_re = Regex::new(
        r"(?i)\b(DTS-HD(?:\s?MA)?|DTS|TrueHD|Atmos|DD\+?5\.1|DD\+?7\.1|AAC|AC3|EAC3|E-AC-?3|FLAC|MP3|LPCM|DDP?5\.1|DDP?7\.1)\b",
    )
    .unwrap();
    let audio = audio_re
        .captures(&clean)
        .map(|c| c.get(0).unwrap().as_str().to_string());

    // Language
    let lang_re = Regex::new(
        r"(?i)\b(MULTI|MULTi|FRENCH|VOSTFR|VOST|VFF|VFQ|VF2|VFI|TRUEFRENCH|ENGLISH|ENG|GERMAN|SPANISH|ITALIAN|SUBFRENCH)\b",
    )
    .unwrap();
    let language = lang_re
        .captures(&clean)
        .map(|c| c.get(0).unwrap().as_str().to_uppercase());

    // Group (after last -)
    let group_re = Regex::new(r"-([A-Za-z0-9]+)(?:\s*\[.*\])?$").unwrap();
    let group = group_re
        .captures(name)
        .map(|c| c.get(1).unwrap().as_str().to_string());

    // Game scene groups
    let game_groups = [
        "CODEX", "PLAZA", "GOG", "SKIDROW", "RELOADED", "CPY", "HOODLUM", "RAZOR1911",
        "FLT", "TENOKE", "RUNE", "DARKSiDERS", "EMPRESS", "TiNYiSO", "DOGE",
        "KaOs", "ElAmigos", "PROPHET",
    ];

    let is_game_group = group
        .as_ref()
        .map(|g| {
            let gu = g.to_uppercase();
            game_groups.iter().any(|gg| gg.to_uppercase() == gu)
        })
        .unwrap_or(false);

    // Also check FitGirl in the full name
    let is_fitgirl = clean.to_lowercase().contains("fitgirl");

    // Detect content type
    let content_type = if season.is_some() {
        DetectedContentType::Serie
    } else if is_game_group || is_fitgirl {
        DetectedContentType::Jeu
    } else {
        DetectedContentType::Unknown
    };

    // Extract title: everything before year, quality, season, or group markers
    let title = extract_title(&clean, year, season.is_some());

    ReleaseParsed {
        content_type,
        title,
        year,
        quality,
        video_codec,
        audio,
        language,
        group,
        season,
        episode,
    }
}

fn extract_title(clean: &str, year: Option<u32>, has_season: bool) -> String {
    // Strip bracketed content like [FitGirl Repack], [DODI], [elamigos], etc.
    let bracket_re = Regex::new(r"\s*\[.*?\]").unwrap();
    let mut title = bracket_re.replace_all(clean, "").to_string();

    // Cut at season pattern first (highest priority)
    if has_season {
        let re = Regex::new(r"(?i)\bS\d{2}").unwrap();
        if let Some(m) = re.find(&title) {
            title = title[..m.start()].to_string();
        }
    }

    // Cut at quality/codec/language/version markers
    let markers = Regex::new(
        r"(?i)\b(2160p|1080p|720p|480p|4K|UHD|REMUX|BDRip|BRRip|HDRip|WEB-?DL|WEBRip|HDTV|DVDRip|BluRay|Blu-Ray|x264|x265|H\.?264|H\.?265|HEVC|AV1|MULTI|MULTi|FRENCH|VOSTFR|COMPLETE|v\d+[\. ]?\d+)",
    )
    .unwrap();
    if let Some(m) = markers.find(&title) {
        title = title[..m.start()].to_string();
    }

    // Cut at year if it appears as a standalone release year (not part of game name like "2077")
    // Strategy: only cut at year if there's additional content after it in the original clean string
    // that got removed by markers (meaning year was between title and markers)
    if let Some(y) = year {
        let year_str = y.to_string();
        let year_re = Regex::new(&format!(r"\b{}\b", year_str)).unwrap();
        if let Some(m) = year_re.find(&title) {
            let before = title[..m.start()].trim();
            let after = title[m.end()..].trim();
            // Only cut if: there's a title before the year AND the year is at/near the end
            // This avoids cutting "2077" from "Cyberpunk 2077" when 2077 IS the title
            // But cuts "2024" from "Dune Part Two 2024" (standalone year)
            if !before.is_empty() && after.is_empty() {
                // Check if the year was followed by quality/codec markers in the original string
                // by checking if the original clean string has content after the year
                let orig_year_re = Regex::new(&format!(r"\b{}\b", year_str)).unwrap();
                if let Some(om) = orig_year_re.find(clean) {
                    let orig_after = clean[om.end()..].trim();
                    if !orig_after.is_empty() {
                        // Year had content after it in original → it's a release year
                        title = title[..m.start()].to_string();
                    }
                }
            } else if !before.is_empty() && !after.is_empty() {
                // Year is in the middle of remaining title → release year
                title = title[..m.start()].to_string();
            }
        }
    }

    title.trim().trim_end_matches('-').trim().to_string()
}

pub fn analyze_torrent(path: &Path) -> Result<TorrentInfo, String> {
    let meta = parse_torrent_file(path)?;
    let mut parsed = parse_release_name(&meta.name);

    // Refine type via file extensions if Unknown
    if parsed.content_type == DetectedContentType::Unknown {
        let has_video = meta.files.iter().any(|f| {
            let lower = f.path.to_lowercase();
            lower.ends_with(".mkv")
                || lower.ends_with(".mp4")
                || lower.ends_with(".avi")
                || lower.ends_with(".m2ts")
        });
        let has_exe = meta.files.iter().any(|f| {
            let lower = f.path.to_lowercase();
            lower.ends_with(".exe") || lower.ends_with(".msi")
        });
        let has_iso = meta.files.iter().any(|f| f.path.to_lowercase().ends_with(".iso"));
        let has_setup = meta.files.iter().any(|f| {
            let lower = f.path.to_lowercase();
            lower.contains("setup") || lower.contains("install")
        });

        if (has_exe || has_iso) && (has_setup || !has_video) {
            parsed.content_type = DetectedContentType::Jeu;
        } else if has_video {
            parsed.content_type = DetectedContentType::Film;
        }
    }

    let size_formatted = format_size(meta.total_size);

    Ok(TorrentInfo {
        meta,
        parsed,
        size_formatted,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_size() {
        assert_eq!(format_size(4_500_000_000), "4.19 Go");
        assert_eq!(format_size(750_000_000), "715 Mo");
        assert_eq!(format_size(1_073_741_824), "1.00 Go");
    }

    #[test]
    fn test_parse_movie_release() {
        let p = parse_release_name("The.Matrix.1999.1080p.BluRay.x264-GROUP");
        assert_eq!(p.title, "The Matrix");
        assert_eq!(p.year, Some(1999));
        assert_eq!(p.quality.as_deref(), Some("1080p"));
        assert_eq!(p.video_codec.as_deref(), Some("x264"));
        assert_eq!(p.group.as_deref(), Some("GROUP"));
    }

    #[test]
    fn test_parse_series_release() {
        let p = parse_release_name("Breaking.Bad.S05E16.1080p.BluRay.x265-RARBG");
        assert_eq!(p.content_type, DetectedContentType::Serie);
        assert_eq!(p.title, "Breaking Bad");
        assert_eq!(p.season, Some(5));
        assert_eq!(p.episode, Some(16));
        assert_eq!(p.quality.as_deref(), Some("1080p"));
    }

    #[test]
    fn test_parse_game_release() {
        let p = parse_release_name("Cyberpunk.2077.v1.6-GOG");
        assert_eq!(p.content_type, DetectedContentType::Jeu);
        assert_eq!(p.title, "Cyberpunk 2077");
        assert_eq!(p.group.as_deref(), Some("GOG"));
    }

    #[test]
    fn test_parse_game_codex() {
        let p = parse_release_name("Elden.Ring.v1.09-CODEX");
        assert_eq!(p.content_type, DetectedContentType::Jeu);
        assert_eq!(p.group.as_deref(), Some("CODEX"));
    }

    #[test]
    fn test_parse_french_movie() {
        let p = parse_release_name("Intouchables.2011.FRENCH.1080p.BluRay.x264.DTS-FGT");
        assert_eq!(p.title, "Intouchables");
        assert_eq!(p.year, Some(2011));
        assert_eq!(p.language.as_deref(), Some("FRENCH"));
        assert_eq!(p.quality.as_deref(), Some("1080p"));
        assert_eq!(p.audio.as_deref(), Some("DTS"));
    }

    #[test]
    fn test_parse_multi_release() {
        let p = parse_release_name("Dune.Part.Two.2024.MULTi.2160p.WEB-DL.DDP5.1.H.265-GROUP");
        assert_eq!(p.title, "Dune Part Two");
        assert_eq!(p.year, Some(2024));
        assert_eq!(p.language.as_deref(), Some("MULTI"));
        assert_eq!(p.quality.as_deref(), Some("2160p"));
    }

    #[test]
    fn test_parse_fitgirl_release() {
        let p = parse_release_name("Baldurs.Gate.3.v4.1.1-FitGirl.Repack");
        assert_eq!(p.content_type, DetectedContentType::Jeu);
    }

    #[test]
    fn test_parse_fitgirl_bracket() {
        let p = parse_release_name("Chernobylite [FitGirl Repack]");
        assert_eq!(p.content_type, DetectedContentType::Jeu);
        assert_eq!(p.title, "Chernobylite");
    }

    #[test]
    fn test_parse_bracket_repack() {
        let p = parse_release_name("Hogwarts.Legacy.v1121023 [DODI Repack]");
        assert_eq!(p.title, "Hogwarts Legacy");
    }

    #[test]
    fn test_parse_season_only() {
        let p = parse_release_name("The.Last.of.Us.S01.COMPLETE.1080p.AMZN.WEB-DL");
        assert_eq!(p.content_type, DetectedContentType::Serie);
        assert_eq!(p.title, "The Last of Us");
        assert_eq!(p.season, Some(1));
        assert_eq!(p.episode, None);
    }
}
