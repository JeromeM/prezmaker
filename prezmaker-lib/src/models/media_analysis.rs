use serde::{Deserialize, Serialize};

/// Structured media file analysis result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaAnalysis {
    pub format: String,
    pub file_name: String,
    pub file_size: String,
    pub duration: Option<String>,
    pub bitrate: Option<String>,
    pub video: Vec<VideoTrack>,
    pub audio: Vec<AudioTrack>,
    pub subtitles: Vec<SubtitleTrack>,
    /// Raw MediaInfo-style text output (for NFO)
    pub raw_text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoTrack {
    pub codec: String,
    pub width: u64,
    pub height: u64,
    pub fps: Option<String>,
    pub bitrate: Option<String>,
    pub language: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioTrack {
    pub codec: String,
    pub channels: String,
    pub sample_rate: Option<String>,
    pub language: Option<String>,
    pub is_default: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubtitleTrack {
    pub format: String,
    pub language: Option<String>,
    pub is_default: bool,
    pub is_forced: bool,
}

impl MediaAnalysis {
    /// Summary of audio languages (deduplicated)
    pub fn audio_languages(&self) -> String {
        let langs: Vec<&str> = self
            .audio
            .iter()
            .filter_map(|a| a.language.as_deref())
            .collect::<Vec<_>>();
        let mut unique = Vec::new();
        for l in &langs {
            if !unique.contains(l) {
                unique.push(*l);
            }
        }
        unique.join(", ")
    }

    /// Summary of subtitle languages (deduplicated)
    pub fn subtitle_languages(&self) -> String {
        let langs: Vec<&str> = self
            .subtitles
            .iter()
            .filter_map(|s| s.language.as_deref())
            .collect::<Vec<_>>();
        let mut unique = Vec::new();
        for l in &langs {
            if !unique.contains(l) {
                unique.push(*l);
            }
        }
        unique.join(", ")
    }

    /// First video track resolution as "WxH"
    pub fn resolution(&self) -> Option<String> {
        self.video.first().map(|v| format!("{}x{}", v.width, v.height))
    }

    /// First video track codec
    pub fn video_codec(&self) -> Option<&str> {
        self.video.first().map(|v| v.codec.as_str())
    }
}
