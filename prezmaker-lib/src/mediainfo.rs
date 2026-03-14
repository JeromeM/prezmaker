use std::fmt::Write;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::time::Duration;

use crate::models::{AudioTrack, MediaAnalysis, SubtitleTrack, VideoTrack};

/// Analyse a media file and return a MediaInfo-style text output.
pub fn analyze(path: &str) -> Result<String, String> {
    let analysis = analyze_structured(path)?;
    Ok(analysis.raw_text)
}

/// Analyse a media file and return structured data + raw text.
pub fn analyze_structured(path: &str) -> Result<MediaAnalysis, String> {
    let ext = Path::new(path)
        .extension()
        .and_then(|e| e.to_str())
        .map(|e| e.to_lowercase())
        .unwrap_or_default();

    let file_size = std::fs::metadata(path)
        .map(|m| m.len())
        .map_err(|e| format!("Impossible de lire le fichier: {}", e))?;

    let file_name = Path::new(path)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("unknown")
        .to_string();

    match ext.as_str() {
        "mkv" | "mka" | "mks" | "webm" => analyze_matroska_structured(path, &file_name, file_size),
        "mp4" | "m4v" | "m4a" | "mov" => analyze_mp4_structured(path, &file_name, file_size),
        _ => Err(format!(
            "Format non supporté: .{}. Formats acceptés: MKV, MP4, M4V, MOV, WebM",
            ext
        )),
    }
}

fn format_size(bytes: u64) -> String {
    const GIB: f64 = 1024.0 * 1024.0 * 1024.0;
    const MIB: f64 = 1024.0 * 1024.0;
    let b = bytes as f64;
    if b >= GIB {
        format!("{:.2} GiB", b / GIB)
    } else {
        format!("{:.1} MiB", b / MIB)
    }
}

fn format_duration(d: Duration) -> String {
    let total_secs = d.as_secs();
    let hours = total_secs / 3600;
    let mins = (total_secs % 3600) / 60;
    if hours > 0 {
        format!("{} h {} min", hours, mins)
    } else {
        format!("{} min", mins)
    }
}

fn format_bitrate(bytes: u64, duration: &Duration) -> String {
    let secs = duration.as_secs_f64();
    if secs <= 0.0 {
        return String::new();
    }
    let bits_per_sec = (bytes as f64 * 8.0) / secs;
    if bits_per_sec >= 1_000_000.0 {
        format!("{:.1} Mb/s", bits_per_sec / 1_000_000.0)
    } else {
        format!("{:.0} kb/s", bits_per_sec / 1_000.0)
    }
}

fn field(out: &mut String, name: &str, value: &str) {
    if !value.is_empty() {
        let _ = writeln!(out, "{:<40} : {}", name, value);
    }
}

fn mkv_codec_name(codec_id: &str) -> &str {
    match codec_id {
        "V_MPEG4/ISO/AVC" => "AVC (H.264)",
        "V_MPEGH/ISO/HEVC" => "HEVC (H.265)",
        "V_MPEG4/ISO/SP" | "V_MPEG4/ISO/ASP" | "V_MPEG4/ISO/AP" => "MPEG-4 Visual",
        "V_VP8" => "VP8",
        "V_VP9" => "VP9",
        "V_AV1" => "AV1",
        "A_AAC" | "A_AAC/MPEG4/LC" | "A_AAC/MPEG2/LC" => "AAC",
        "A_AC3" => "AC-3 (Dolby Digital)",
        "A_EAC3" => "E-AC-3 (Dolby Digital Plus)",
        "A_DTS" => "DTS",
        "A_DTS/EXPRESS" => "DTS Express",
        "A_DTS/LOSSLESS" => "DTS-HD Master Audio",
        "A_TRUEHD" => "TrueHD",
        "A_VORBIS" => "Vorbis",
        "A_OPUS" => "Opus",
        "A_FLAC" => "FLAC",
        "A_PCM/INT/LIT" | "A_PCM/INT/BIG" => "PCM",
        "A_MP3" | "A_MPEG/L3" => "MP3",
        "S_TEXT/UTF8" => "UTF-8 (SRT)",
        "S_TEXT/ASS" | "S_TEXT/SSA" => "ASS/SSA",
        "S_HDMV/PGS" => "PGS (Blu-ray)",
        "S_VOBSUB" => "VobSub",
        "S_TEXT/WEBVTT" => "WebVTT",
        other => other,
    }
}

fn language_name(lang: &str) -> &str {
    match lang {
        "fre" | "fra" | "fr" => "Français",
        "eng" | "en" => "Anglais",
        "spa" | "es" => "Espagnol",
        "ger" | "deu" | "de" => "Allemand",
        "ita" | "it" => "Italien",
        "por" | "pt" => "Portugais",
        "jpn" | "ja" => "Japonais",
        "kor" | "ko" => "Coréen",
        "chi" | "zho" | "zh" => "Chinois",
        "rus" | "ru" => "Russe",
        "ara" | "ar" => "Arabe",
        "dut" | "nld" | "nl" => "Néerlandais",
        "pol" | "pl" => "Polonais",
        "swe" | "sv" => "Suédois",
        "nor" | "no" | "nob" | "nno" => "Norvégien",
        "dan" | "da" => "Danois",
        "fin" | "fi" => "Finnois",
        "tur" | "tr" => "Turc",
        "hin" | "hi" => "Hindi",
        "tha" | "th" => "Thaï",
        "vie" | "vi" => "Vietnamien",
        "heb" | "he" => "Hébreu",
        "gre" | "ell" | "el" => "Grec",
        "ces" | "cze" | "cs" => "Tchèque",
        "hun" | "hu" => "Hongrois",
        "ron" | "rum" | "ro" => "Roumain",
        "hrv" | "hr" => "Croate",
        "srp" | "sr" => "Serbe",
        "bul" | "bg" => "Bulgare",
        "ukr" | "uk" => "Ukrainien",
        "cat" | "ca" => "Catalan",
        "ind" | "id" => "Indonésien",
        "may" | "msa" | "ms" => "Malais",
        "und" => "Indéfini",
        other => other,
    }
}

fn mkv_language_str(lang: &matroska::Language) -> &str {
    match lang {
        matroska::Language::ISO639(s) => s.as_str(),
        matroska::Language::IETF(s) => s.as_str(),
    }
}

fn channels_label(ch: u64) -> String {
    match ch {
        1 => "1 channel (Mono)".to_string(),
        2 => "2 channels (Stereo)".to_string(),
        6 => "6 channels (5.1)".to_string(),
        8 => "8 channels (7.1)".to_string(),
        _ => format!("{} channels", ch),
    }
}

fn analyze_matroska_structured(path: &str, file_name: &str, file_size: u64) -> Result<MediaAnalysis, String> {
    let file = File::open(path).map_err(|e| format!("Impossible d'ouvrir le fichier: {}", e))?;
    let mkv = matroska::Matroska::open(file)
        .map_err(|e| format!("Erreur de lecture MKV: {}", e))?;

    let mut out = String::new();
    let duration_str = mkv.info.duration.as_ref().map(|d| format_duration(*d));
    let bitrate_str = mkv.info.duration.as_ref().map(|d| format_bitrate(file_size, d));

    // Raw text: General
    out.push_str("General\n");
    field(&mut out, "Complete name", file_name);
    field(&mut out, "Format", "Matroska");
    if let Some(ref title) = mkv.info.title {
        field(&mut out, "Title", title);
    }
    field(&mut out, "File size", &format_size(file_size));
    if let Some(ref dur) = duration_str {
        field(&mut out, "Duration", dur);
    }
    if let Some(ref br) = bitrate_str {
        field(&mut out, "Overall bit rate", br);
    }
    if !mkv.info.writing_app.is_empty() {
        field(&mut out, "Writing application", &mkv.info.writing_app);
    }
    if !mkv.info.muxing_app.is_empty() {
        field(&mut out, "Writing library", &mkv.info.muxing_app);
    }

    // Video tracks
    let mut video_tracks = Vec::new();
    for track in mkv.video_tracks() {
        out.push('\n');
        let label = match &track.name {
            Some(n) => format!("Video ({})", n),
            None => "Video".to_string(),
        };
        out.push_str(&label);
        out.push('\n');
        field(&mut out, "Format", mkv_codec_name(&track.codec_id));

        let mut width = 0u64;
        let mut height = 0u64;
        if let matroska::Settings::Video(ref v) = track.settings {
            width = v.pixel_width;
            height = v.pixel_height;
            field(&mut out, "Width", &format!("{} pixels", width));
            field(&mut out, "Height", &format!("{} pixels", height));
            if let (Some(dw), Some(dh)) = (v.display_width, v.display_height) {
                if dh > 0 {
                    let ratio = dw as f64 / dh as f64;
                    field(&mut out, "Display aspect ratio", &format!("{:.2}:1", ratio));
                }
            }
        }
        let fps = track.default_duration.as_ref().map(|dur| {
            let f = 1.0 / dur.as_secs_f64();
            format!("{:.3} FPS", f)
        });
        if let Some(ref f) = fps {
            field(&mut out, "Frame rate", f);
        }
        let lang = track.language.as_ref().map(|l| language_name(mkv_language_str(l)).to_string());
        if let Some(ref l) = lang {
            field(&mut out, "Language", l);
        }
        field(&mut out, "Default", if track.default { "Yes" } else { "No" });

        video_tracks.push(VideoTrack {
            codec: mkv_codec_name(&track.codec_id).to_string(),
            width,
            height,
            fps,
            bitrate: None,
            language: lang,
        });
    }

    // Audio tracks
    let mut audio_tracks = Vec::new();
    for (i, track) in mkv.audio_tracks().enumerate() {
        out.push('\n');
        let label = match &track.name {
            Some(n) => format!("Audio #{} ({})", i + 1, n),
            None => format!("Audio #{}", i + 1),
        };
        out.push_str(&label);
        out.push('\n');
        field(&mut out, "Format", mkv_codec_name(&track.codec_id));

        let mut channels = String::new();
        let mut sample_rate = None;
        if let matroska::Settings::Audio(ref a) = track.settings {
            channels = channels_label(a.channels);
            field(&mut out, "Channel(s)", &channels);
            let sr = format!("{:.1} kHz", a.sample_rate / 1000.0);
            field(&mut out, "Sampling rate", &sr);
            sample_rate = Some(sr);
            if let Some(bd) = a.bit_depth {
                field(&mut out, "Bit depth", &format!("{} bits", bd));
            }
        }
        let lang = track.language.as_ref().map(|l| language_name(mkv_language_str(l)).to_string());
        if let Some(ref l) = lang {
            field(&mut out, "Language", l);
        }
        field(&mut out, "Default", if track.default { "Yes" } else { "No" });

        audio_tracks.push(AudioTrack {
            codec: mkv_codec_name(&track.codec_id).to_string(),
            channels,
            sample_rate,
            bitrate: None,
            language: lang,
            is_default: track.default,
        });
    }

    // Subtitle tracks
    let mut subtitle_tracks = Vec::new();
    for (i, track) in mkv.subtitle_tracks().enumerate() {
        out.push('\n');
        let label = match &track.name {
            Some(n) => format!("Text #{} ({})", i + 1, n),
            None => format!("Text #{}", i + 1),
        };
        out.push_str(&label);
        out.push('\n');
        field(&mut out, "Format", mkv_codec_name(&track.codec_id));
        let lang = track.language.as_ref().map(|l| language_name(mkv_language_str(l)).to_string());
        if let Some(ref l) = lang {
            field(&mut out, "Language", l);
        }
        field(&mut out, "Default", if track.default { "Yes" } else { "No" });
        field(&mut out, "Forced", if track.forced { "Yes" } else { "No" });

        let sub_title = track.name.clone();
        subtitle_tracks.push(SubtitleTrack {
            format: mkv_codec_name(&track.codec_id).to_string(),
            language: lang,
            title: sub_title,
            is_default: track.default,
            is_forced: track.forced,
        });
    }

    Ok(MediaAnalysis {
        format: "Matroska".to_string(),
        file_name: file_name.to_string(),
        file_size: format_size(file_size),
        duration: duration_str,
        bitrate: bitrate_str,
        video: video_tracks,
        audio: audio_tracks,
        subtitles: subtitle_tracks,
        raw_text: out,
    })
}

fn mp4_media_type_name(mt: &mp4::MediaType) -> &'static str {
    match mt {
        mp4::MediaType::H264 => "AVC (H.264)",
        mp4::MediaType::H265 => "HEVC (H.265)",
        mp4::MediaType::VP9 => "VP9",
        mp4::MediaType::AAC => "AAC",
        mp4::MediaType::TTXT => "Timed Text",
    }
}

fn analyze_mp4_structured(path: &str, file_name: &str, file_size: u64) -> Result<MediaAnalysis, String> {
    let file = File::open(path).map_err(|e| format!("Impossible d'ouvrir le fichier: {}", e))?;
    let reader = BufReader::new(file);
    let mp4 = mp4::Mp4Reader::read_header(reader, file_size)
        .map_err(|e| format!("Erreur de lecture MP4: {}", e))?;

    let mut out = String::new();
    let duration = mp4.duration();
    let duration_str = format_duration(duration);
    let bitrate_str = format_bitrate(file_size, &duration);

    // General
    out.push_str("General\n");
    field(&mut out, "Complete name", file_name);
    field(&mut out, "Format", "MPEG-4");
    field(&mut out, "File size", &format_size(file_size));
    field(&mut out, "Duration", &duration_str);
    field(&mut out, "Overall bit rate", &bitrate_str);

    // Tracks
    let mut video_tracks = Vec::new();
    let mut audio_tracks = Vec::new();
    let mut subtitle_tracks = Vec::new();
    let mut audio_idx = 0u32;
    let mut text_idx = 0u32;

    for track in mp4.tracks().values() {
        let track_type = match track.track_type() {
            Ok(t) => t,
            Err(_) => continue,
        };
        let media_type_str = track.media_type()
            .map(|mt| mp4_media_type_name(&mt))
            .unwrap_or("Unknown");

        match track_type {
            mp4::TrackType::Video => {
                out.push('\n');
                out.push_str("Video\n");
                field(&mut out, "Format", media_type_str);
                let w = track.width() as u64;
                let h = track.height() as u64;
                field(&mut out, "Width", &format!("{} pixels", w));
                field(&mut out, "Height", &format!("{} pixels", h));
                let dur = track.duration();
                let fps = if dur.as_secs() > 0 {
                    let f = track.sample_count() as f64 / dur.as_secs_f64();
                    let s = format!("{:.3} FPS", f);
                    field(&mut out, "Frame rate", &s);
                    Some(s)
                } else {
                    None
                };
                let br = track.bitrate();
                let bitrate = if br > 0 {
                    let s = format!("{} kb/s", br / 1000);
                    field(&mut out, "Bit rate", &s);
                    Some(s)
                } else {
                    None
                };
                let lang_raw = track.language();
                let lang = language_name(lang_raw);
                field(&mut out, "Language", lang);
                let lang_opt = if lang_raw != "und" { Some(lang.to_string()) } else { None };

                video_tracks.push(VideoTrack {
                    codec: media_type_str.to_string(),
                    width: w,
                    height: h,
                    fps,
                    bitrate,
                    language: lang_opt,
                });
            }
            mp4::TrackType::Audio => {
                audio_idx += 1;
                out.push('\n');
                let _ = writeln!(out, "Audio #{}", audio_idx);
                field(&mut out, "Format", media_type_str);
                let channels = if let Ok(ch) = track.channel_config() {
                    let s = channels_label(ch as u64);
                    field(&mut out, "Channel(s)", &s);
                    s
                } else {
                    String::new()
                };
                let sample_rate = if let Ok(freq) = track.sample_freq_index() {
                    let s = format!("{:.1} kHz", freq.freq() as f64 / 1000.0);
                    field(&mut out, "Sampling rate", &s);
                    Some(s)
                } else {
                    None
                };
                let br = track.bitrate();
                if br > 0 {
                    field(&mut out, "Bit rate", &format!("{} kb/s", br / 1000));
                }
                let lang_raw = track.language();
                let lang = language_name(lang_raw);
                field(&mut out, "Language", lang);
                let lang_opt = if lang_raw != "und" { Some(lang.to_string()) } else { None };

                let audio_bitrate = if br > 0 { Some(format!("{} kb/s", br / 1000)) } else { None };
                audio_tracks.push(AudioTrack {
                    codec: media_type_str.to_string(),
                    channels,
                    sample_rate,
                    bitrate: audio_bitrate,
                    language: lang_opt,
                    is_default: audio_idx == 1,
                });
            }
            mp4::TrackType::Subtitle => {
                text_idx += 1;
                out.push('\n');
                let _ = writeln!(out, "Text #{}", text_idx);
                field(&mut out, "Format", media_type_str);
                let lang_raw = track.language();
                let lang = language_name(lang_raw);
                field(&mut out, "Language", lang);
                let lang_opt = if lang_raw != "und" { Some(lang.to_string()) } else { None };

                subtitle_tracks.push(SubtitleTrack {
                    format: media_type_str.to_string(),
                    language: lang_opt,
                    title: None,
                    is_default: text_idx == 1,
                    is_forced: false,
                });
            }
        }
    }

    Ok(MediaAnalysis {
        format: "MPEG-4".to_string(),
        file_name: file_name.to_string(),
        file_size: format_size(file_size),
        duration: Some(duration_str),
        bitrate: if bitrate_str.is_empty() { None } else { Some(bitrate_str) },
        video: video_tracks,
        audio: audio_tracks,
        subtitles: subtitle_tracks,
        raw_text: out,
    })
}
