use std::fs;
use std::path::PathBuf;
use std::time::SystemTime;

/// Represents a single directory entry with metadata.
#[derive(Debug)]
pub struct DirEntry {
    pub name: String,
    pub path: PathBuf,
    pub is_dir: bool,
    pub is_hidden: bool,
    pub extension: Option<String>,
    pub size: u64,
    pub modified: Option<SystemTime>,
}

impl DirEntry {
    pub fn from_path(path: PathBuf) -> Option<Self> {
        let name = path.file_name()?.to_str()?.to_string();
        let metadata = fs::metadata(&path).ok()?;
        let is_dir = metadata.is_dir();
        let is_hidden = name.starts_with('.');
        let extension = if is_dir {
            None
        } else {
            path.extension().and_then(|e| e.to_str()).map(|s| s.to_string())
        };
        let size = metadata.len();
        let modified = metadata.modified().ok();

        Some(DirEntry {
            name,
            path,
            is_dir,
            is_hidden,
            extension,
            size,
            modified,
        })
    }

    pub fn is_md(&self) -> bool {
        matches!(self.extension.as_deref(), Some("md" | "markdown"))
    }

    pub fn is_txt(&self) -> bool {
        matches!(self.extension.as_deref(), Some("txt"))
    }
}

/// Format file size in human-readable form.
pub fn format_size(size: u64) -> String {
    if size < 1024 {
        format!("{} B", size)
    } else if size < 1024 * 1024 {
        format!("{:.1} KB", size as f64 / 1024.0)
    } else if size < 1024 * 1024 * 1024 {
        format!("{:.1} MB", size as f64 / (1024.0 * 1024.0))
    } else {
        format!("{:.1} GB", size as f64 / (1024.0 * 1024.0 * 1024.0))
    }
}

/// Format modification time.
pub fn format_time(time: &SystemTime) -> String {
    let duration = time
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or_default();
    let secs = duration.as_secs() as i64;

    // Simple date formatting without chrono dependency
    let days = secs / 86400;
    let remaining = secs % 86400;
    let hours = remaining / 3600;
    let minutes = (remaining % 3600) / 60;

    // Calculate year/month/day from days since epoch
    let (year, month, day) = days_to_date(days);

    format!(
        "{:04}-{:02}-{:02} {:02}:{:02}",
        year, month, day, hours, minutes
    )
}

fn days_to_date(mut days: i64) -> (i64, i64, i64) {
    // Algorithm from http://howardhinnant.github.io/date_algorithms.html
    days += 719468;
    let era = if days >= 0 { days } else { days - 146096 } / 146097;
    let doe = days - era * 146097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = if m <= 2 { y + 1 } else { y };
    (y, m, d)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_size() {
        assert_eq!(format_size(0), "0 B");
        assert_eq!(format_size(512), "512 B");
        assert_eq!(format_size(1024), "1.0 KB");
        assert_eq!(format_size(1536), "1.5 KB");
        assert_eq!(format_size(1048576), "1.0 MB");
        assert_eq!(format_size(1073741824), "1.0 GB");
    }

    #[test]
    fn test_dir_entry_from_path() {
        let dir = tempfile::tempdir().unwrap();
        let file_path = dir.path().join("test.md");
        std::fs::write(&file_path, "# Hello").unwrap();

        let entry = DirEntry::from_path(file_path).unwrap();
        assert_eq!(entry.name, "test.md");
        assert!(!entry.is_dir);
        assert!(!entry.is_hidden);
        assert!(entry.is_md());
        assert!(!entry.is_txt());
    }

    #[test]
    fn test_hidden_file() {
        let dir = tempfile::tempdir().unwrap();
        let file_path = dir.path().join(".hidden");
        std::fs::write(&file_path, "").unwrap();

        let entry = DirEntry::from_path(file_path).unwrap();
        assert!(entry.is_hidden);
    }

    #[test]
    fn test_directory_entry() {
        let dir = tempfile::tempdir().unwrap();
        let sub = dir.path().join("subdir");
        std::fs::create_dir(&sub).unwrap();

        let entry = DirEntry::from_path(sub).unwrap();
        assert!(entry.is_dir);
        assert!(entry.extension.is_none());
    }
}
