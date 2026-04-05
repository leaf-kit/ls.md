use std::collections::hash_map::DefaultHasher;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::{BufRead, BufReader};
use std::path::Path;

use colored::Colorize;

const MAX_LINES: usize = 50;

#[derive(Debug, Default)]
pub struct MdMeta {
    pub title: Option<String>,
    pub date: Option<String>,
    pub tags: Vec<String>,
    pub h1: Option<String>,
}

/// Parse .md file: extract YAML frontmatter or fallback to first H1 heading.
pub fn parse_md(path: &Path) -> MdMeta {
    let file = match File::open(path) {
        Ok(f) => f,
        Err(_) => return MdMeta::default(),
    };

    let reader = BufReader::new(file);
    let mut lines: Vec<String> = Vec::new();

    for (i, line) in reader.lines().enumerate() {
        if i >= MAX_LINES {
            break;
        }
        match line {
            Ok(l) => lines.push(l),
            Err(_) => break,
        }
    }

    let mut meta = MdMeta::default();

    // Try parsing YAML frontmatter
    if !lines.is_empty() && lines[0].trim() == "---" {
        if let Some(end) = lines[1..].iter().position(|l| l.trim() == "---") {
            let yaml_str = lines[1..=end].join("\n");
            if let Ok(yaml) = serde_yaml::from_str::<serde_yaml::Value>(&yaml_str) {
                if let Some(map) = yaml.as_mapping() {
                    if let Some(t) = map.get(serde_yaml::Value::String("title".into())) {
                        meta.title = t.as_str().map(|s| s.to_string());
                    }
                    if let Some(d) = map.get(serde_yaml::Value::String("date".into())) {
                        meta.date = d.as_str().map(|s| s.to_string());
                    }
                    if let Some(tags) = map.get(serde_yaml::Value::String("tags".into())) {
                        if let Some(seq) = tags.as_sequence() {
                            meta.tags = seq
                                .iter()
                                .filter_map(|v| v.as_str().map(|s| s.to_string()))
                                .collect();
                        }
                    }
                }
            }
            return meta;
        }
    }

    // Fallback: find first H1 heading
    for line in &lines {
        let trimmed = line.trim();
        if let Some(heading) = trimmed.strip_prefix("# ") {
            meta.h1 = Some(heading.to_string());
            break;
        }
    }

    meta
}

/// Format metadata as inline summary string.
pub fn format_md_summary(meta: &MdMeta) -> String {
    let mut parts: Vec<String> = Vec::new();

    if let Some(ref title) = meta.title {
        parts.push(title.clone());
    }

    if let Some(ref date) = meta.date {
        parts.push(date.dimmed().to_string());
    }

    if !meta.tags.is_empty() {
        let tag_strs: Vec<String> = meta
            .tags
            .iter()
            .map(|tag| format_tag_badge(tag))
            .collect();
        parts.push(tag_strs.join(" "));
    }

    if parts.is_empty() {
        if let Some(ref h1) = meta.h1 {
            return h1.dimmed().to_string();
        }
    }

    parts.join(" · ")
}

/// Render a tag as a colored badge using hash-based color.
fn format_tag_badge(tag: &str) -> String {
    let colors = [
        "red",
        "green",
        "yellow",
        "blue",
        "magenta",
        "cyan",
        "bright red",
        "bright green",
        "bright yellow",
        "bright blue",
        "bright magenta",
        "bright cyan",
    ];

    let mut hasher = DefaultHasher::new();
    tag.hash(&mut hasher);
    let idx = (hasher.finish() % colors.len() as u64) as usize;

    let badge = format!(" {} ", tag);
    match colors[idx] {
        "red" => badge.on_red().white().bold().to_string(),
        "green" => badge.on_green().white().bold().to_string(),
        "yellow" => badge.on_yellow().black().bold().to_string(),
        "blue" => badge.on_blue().white().bold().to_string(),
        "magenta" => badge.on_magenta().white().bold().to_string(),
        "cyan" => badge.on_cyan().black().bold().to_string(),
        "bright red" => badge.on_bright_red().white().bold().to_string(),
        "bright green" => badge.on_bright_green().black().bold().to_string(),
        "bright yellow" => badge.on_bright_yellow().black().bold().to_string(),
        "bright blue" => badge.on_bright_blue().white().bold().to_string(),
        "bright magenta" => badge.on_bright_magenta().white().bold().to_string(),
        "bright cyan" => badge.on_bright_cyan().black().bold().to_string(),
        _ => badge.dimmed().to_string(),
    }
}

/// Read first non-empty line of a .txt file, truncated to 60 chars.
pub fn read_txt_preview(path: &Path) -> Option<String> {
    let file = File::open(path).ok()?;
    let reader = BufReader::new(file);

    for line in reader.lines().take(MAX_LINES) {
        if let Ok(l) = line {
            let trimmed = l.trim().to_string();
            if !trimmed.is_empty() {
                let preview = if trimmed.len() > 60 {
                    format!("{}…", &trimmed[..59])
                } else {
                    trimmed
                };
                return Some(preview.dimmed().to_string());
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn test_parse_md_with_frontmatter() {
        let dir = tempfile::tempdir().unwrap();
        let file_path = dir.path().join("test.md");
        let mut f = File::create(&file_path).unwrap();
        write!(
            f,
            "---\ntitle: Hello World\ndate: 2026-01-01\ntags:\n  - rust\n  - cli\n---\n# Content"
        )
        .unwrap();

        let meta = parse_md(&file_path);
        assert_eq!(meta.title, Some("Hello World".to_string()));
        assert_eq!(meta.date, Some("2026-01-01".to_string()));
        assert_eq!(meta.tags, vec!["rust", "cli"]);
        assert!(meta.h1.is_none());
    }

    #[test]
    fn test_parse_md_with_h1_only() {
        let dir = tempfile::tempdir().unwrap();
        let file_path = dir.path().join("test.md");
        let mut f = File::create(&file_path).unwrap();
        write!(f, "# My Heading\nSome content").unwrap();

        let meta = parse_md(&file_path);
        assert!(meta.title.is_none());
        assert_eq!(meta.h1, Some("My Heading".to_string()));
    }

    #[test]
    fn test_parse_md_empty() {
        let dir = tempfile::tempdir().unwrap();
        let file_path = dir.path().join("empty.md");
        File::create(&file_path).unwrap();

        let meta = parse_md(&file_path);
        assert!(meta.title.is_none());
        assert!(meta.h1.is_none());
    }

    #[test]
    fn test_read_txt_preview() {
        let dir = tempfile::tempdir().unwrap();
        let file_path = dir.path().join("test.txt");
        let mut f = File::create(&file_path).unwrap();
        write!(f, "\n\nHello from text file\nSecond line").unwrap();

        let preview = read_txt_preview(&file_path);
        assert!(preview.is_some());
    }

    #[test]
    fn test_read_txt_preview_long_line() {
        let dir = tempfile::tempdir().unwrap();
        let file_path = dir.path().join("long.txt");
        let mut f = File::create(&file_path).unwrap();
        let long_line = "A".repeat(100);
        write!(f, "{}", long_line).unwrap();

        let preview = read_txt_preview(&file_path);
        assert!(preview.is_some());
    }

    #[test]
    fn test_read_txt_preview_empty() {
        let dir = tempfile::tempdir().unwrap();
        let file_path = dir.path().join("empty.txt");
        File::create(&file_path).unwrap();

        let preview = read_txt_preview(&file_path);
        assert!(preview.is_none());
    }

    #[test]
    fn test_parse_md_broken_yaml() {
        let dir = tempfile::tempdir().unwrap();
        let file_path = dir.path().join("broken.md");
        let mut f = File::create(&file_path).unwrap();
        write!(f, "---\ntitle: [invalid yaml\n---\n# Fallback").unwrap();

        let meta = parse_md(&file_path);
        // Broken YAML: frontmatter block found but parse fails, no fallback to H1
        assert!(meta.title.is_none());
    }
}
