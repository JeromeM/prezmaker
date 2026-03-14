use serde::{Deserialize, Serialize};
use sha1::{Digest, Sha1};
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TorrentCreateOptions {
    pub source_path: String,
    pub piece_size: Option<u32>,
    pub private: bool,
    pub trackers: Vec<String>,
    pub comment: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TorrentCreateProgress {
    pub phase: String,
    pub percent: f64,
    pub message: String,
}

struct FileEntry {
    /// Path relative to the source root
    relative_path: Vec<String>,
    /// Absolute path on disk
    absolute_path: PathBuf,
    size: u64,
}

/// Collect files from `source_path`. If it's a single file, return one entry.
/// If it's a directory, recursively collect all files (sorted for determinism).
fn scan_files(source: &Path) -> Result<(String, Vec<FileEntry>), String> {
    if !source.exists() {
        return Err(format!("Le chemin n'existe pas : {}", source.display()));
    }

    let name = source
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("unknown")
        .to_string();

    if source.is_file() {
        let meta = source.metadata().map_err(|e| e.to_string())?;
        return Ok((
            name.clone(),
            vec![FileEntry {
                relative_path: vec![name],
                absolute_path: source.to_path_buf(),
                size: meta.len(),
            }],
        ));
    }

    // Directory: recursive walk
    let mut entries = Vec::new();
    walk_dir(source, source, &mut entries)?;
    entries.sort_by(|a, b| a.relative_path.cmp(&b.relative_path));

    if entries.is_empty() {
        return Err("Le dossier est vide".into());
    }

    Ok((name, entries))
}

fn walk_dir(root: &Path, current: &Path, entries: &mut Vec<FileEntry>) -> Result<(), String> {
    let read = fs::read_dir(current).map_err(|e| format!("Impossible de lire {}: {}", current.display(), e))?;
    for entry in read {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        if path.is_dir() {
            walk_dir(root, &path, entries)?;
        } else if path.is_file() {
            let meta = path.metadata().map_err(|e| e.to_string())?;
            if meta.len() == 0 {
                continue;
            }
            let rel = path
                .strip_prefix(root)
                .map_err(|e| e.to_string())?;
            let relative_path: Vec<String> = rel
                .components()
                .map(|c| c.as_os_str().to_string_lossy().into_owned())
                .collect();
            entries.push(FileEntry {
                relative_path,
                absolute_path: path,
                size: meta.len(),
            });
        }
    }
    Ok(())
}

/// Calculate piece size automatically to target ~1500 pieces.
/// Clamped between 16 KiB and 128 MiB, rounded to power of 2.
fn auto_piece_size(total_size: u64) -> u32 {
    const MIN: u32 = 16 * 1024;         // 16 KiB
    const MAX: u32 = 128 * 1024 * 1024;  // 128 MiB
    const TARGET_PIECES: u64 = 1500;

    let ideal = (total_size / TARGET_PIECES) as u32;
    // Round up to next power of 2
    let mut ps = MIN;
    while ps < ideal && ps < MAX {
        ps *= 2;
    }
    ps.clamp(MIN, MAX)
}

/// Bencode encoding helpers
fn bencode_string(s: &[u8]) -> Vec<u8> {
    let mut out = format!("{}:", s.len()).into_bytes();
    out.extend_from_slice(s);
    out
}

fn bencode_int(i: i64) -> Vec<u8> {
    format!("i{}e", i).into_bytes()
}

fn bencode_dict(entries: Vec<(&[u8], Vec<u8>)>) -> Vec<u8> {
    let mut sorted = entries;
    sorted.sort_by(|a, b| a.0.cmp(b.0));

    let mut out = vec![b'd'];
    for (key, val) in sorted {
        out.extend(bencode_string(key));
        out.extend(val);
    }
    out.push(b'e');
    out
}

fn bencode_list(items: Vec<Vec<u8>>) -> Vec<u8> {
    let mut out = vec![b'l'];
    for item in items {
        out.extend(item);
    }
    out.push(b'e');
    out
}

pub fn create_torrent<F>(
    opts: &TorrentCreateOptions,
    output_path: &Path,
    progress_cb: F,
) -> Result<(), String>
where
    F: Fn(TorrentCreateProgress),
{
    let source = Path::new(&opts.source_path);

    // Phase 1: Scanning
    progress_cb(TorrentCreateProgress {
        phase: "scanning".into(),
        percent: 0.0,
        message: "Analyse des fichiers...".into(),
    });

    let (name, files) = scan_files(source)?;
    let is_single = source.is_file();
    let total_size: u64 = files.iter().map(|f| f.size).sum();

    progress_cb(TorrentCreateProgress {
        phase: "scanning".into(),
        percent: 100.0,
        message: format!("{} fichier(s) trouvé(s)", files.len()),
    });

    // Phase 2: Determine piece size
    let piece_size = opts.piece_size.unwrap_or_else(|| auto_piece_size(total_size));
    let total_pieces = ((total_size as f64) / (piece_size as f64)).ceil() as u64;

    // Phase 3: Hashing
    let mut pieces: Vec<u8> = Vec::new();
    let mut buffer = vec![0u8; piece_size as usize];
    let mut buf_offset = 0usize;
    let mut pieces_done: u64 = 0;

    for file_entry in &files {
        let mut file = fs::File::open(&file_entry.absolute_path)
            .map_err(|e| format!("Impossible d'ouvrir {}: {}", file_entry.absolute_path.display(), e))?;

        loop {
            let remaining = piece_size as usize - buf_offset;
            let n = file.read(&mut buffer[buf_offset..buf_offset + remaining])
                .map_err(|e| e.to_string())?;
            if n == 0 {
                break;
            }
            buf_offset += n;

            if buf_offset == piece_size as usize {
                let hash = Sha1::digest(&buffer[..buf_offset]);
                pieces.extend_from_slice(&hash);
                buf_offset = 0;
                pieces_done += 1;

                if pieces_done % 50 == 0 || pieces_done == total_pieces {
                    let pct = (pieces_done as f64 / total_pieces as f64 * 100.0).min(100.0);
                    progress_cb(TorrentCreateProgress {
                        phase: "hashing".into(),
                        percent: pct,
                        message: format!("Pièce {}/{}", pieces_done, total_pieces),
                    });
                }
            }
        }
    }

    // Hash remaining partial piece
    if buf_offset > 0 {
        let hash = Sha1::digest(&buffer[..buf_offset]);
        pieces.extend_from_slice(&hash);
    }

    // Phase 4: Build torrent
    progress_cb(TorrentCreateProgress {
        phase: "writing".into(),
        percent: 50.0,
        message: "Construction du fichier .torrent...".into(),
    });

    // Build info dict
    let mut info_entries: Vec<(&[u8], Vec<u8>)> = vec![
        (b"name", bencode_string(name.as_bytes())),
        (b"piece length", bencode_int(piece_size as i64)),
        (b"pieces", bencode_string(&pieces)),
    ];

    if opts.private {
        info_entries.push((b"private", bencode_int(1)));
    }

    if is_single {
        info_entries.push((b"length", bencode_int(total_size as i64)));
    } else {
        let file_dicts: Vec<Vec<u8>> = files
            .iter()
            .map(|f| {
                let path_list: Vec<Vec<u8>> = f
                    .relative_path
                    .iter()
                    .map(|p| bencode_string(p.as_bytes()))
                    .collect();
                bencode_dict(vec![
                    (b"length" as &[u8], bencode_int(f.size as i64)),
                    (b"path", bencode_list(path_list)),
                ])
            })
            .collect();
        info_entries.push((b"files", bencode_list(file_dicts)));
    }

    let info_dict = bencode_dict(info_entries);

    // Build root dict
    let mut root_entries: Vec<(&[u8], Vec<u8>)> = Vec::new();

    // announce
    let trackers: Vec<&str> = opts.trackers.iter().map(|s| s.trim()).filter(|s| !s.is_empty()).collect();
    if let Some(first) = trackers.first() {
        root_entries.push((b"announce", bencode_string(first.as_bytes())));
    }

    // announce-list (each tracker in its own tier)
    if !trackers.is_empty() {
        let tiers: Vec<Vec<u8>> = trackers
            .iter()
            .map(|t| bencode_list(vec![bencode_string(t.as_bytes())]))
            .collect();
        root_entries.push((b"announce-list", bencode_list(tiers)));
    }

    // comment
    if let Some(ref comment) = opts.comment {
        if !comment.is_empty() {
            root_entries.push((b"comment", bencode_string(comment.as_bytes())));
        }
    }

    // created by
    root_entries.push((b"created by", bencode_string(b"PrezMaker")));

    // creation date
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0);
    root_entries.push((b"creation date", bencode_int(timestamp)));

    // info
    root_entries.push((b"info", info_dict));

    let torrent_data = bencode_dict(root_entries);

    // Phase 5: Write
    fs::write(output_path, &torrent_data)
        .map_err(|e| format!("Impossible d'écrire le torrent : {}", e))?;

    progress_cb(TorrentCreateProgress {
        phase: "writing".into(),
        percent: 100.0,
        message: "Torrent créé avec succès".into(),
    });

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_auto_piece_size() {
        // Small file: should use minimum 16 KiB
        assert_eq!(auto_piece_size(1_000_000), 16 * 1024);
        // ~700 MB → target ~470 KB → rounds to 512 KiB
        assert_eq!(auto_piece_size(700_000_000), 512 * 1024);
        // 4.7 GB → target ~3.1 MB → rounds to 4 MiB
        assert_eq!(auto_piece_size(4_700_000_000), 4 * 1024 * 1024);
        // Very large: should not exceed 128 MiB
        assert!(auto_piece_size(u64::MAX) <= 128 * 1024 * 1024);
    }

    #[test]
    fn test_bencode_string() {
        assert_eq!(bencode_string(b"spam"), b"4:spam");
    }

    #[test]
    fn test_bencode_int() {
        assert_eq!(bencode_int(42), b"i42e");
        assert_eq!(bencode_int(-3), b"i-3e");
    }

    #[test]
    fn test_bencode_list() {
        let list = bencode_list(vec![
            bencode_string(b"spam"),
            bencode_string(b"eggs"),
        ]);
        assert_eq!(list, b"l4:spam4:eggse");
    }

    #[test]
    fn test_bencode_dict() {
        let dict = bencode_dict(vec![
            (b"cow" as &[u8], bencode_string(b"moo")),
            (b"spam", bencode_string(b"eggs")),
        ]);
        // Keys should be sorted: cow < spam
        assert_eq!(dict, b"d3:cow3:moo4:spam4:eggse");
    }
}
