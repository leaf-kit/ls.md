use std::fs;
use std::os::unix::fs::PermissionsExt;
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
    pub mode: u32,
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
        let mode = metadata.permissions().mode();

        Some(DirEntry {
            name,
            path,
            is_dir,
            is_hidden,
            extension,
            size,
            modified,
            mode,
        })
    }

    pub fn is_md(&self) -> bool {
        matches!(self.extension.as_deref(), Some("md" | "markdown"))
    }

    pub fn is_txt(&self) -> bool {
        matches!(self.extension.as_deref(), Some("txt"))
    }

    pub fn is_executable(&self) -> bool {
        !self.is_dir && (self.mode & 0o111) != 0
    }
}

/// Format file size in human-readable form (lsd style: value and unit separate).
pub fn format_size(size: u64) -> (String, String) {
    if size == 0 {
        ("-".to_string(), "".to_string())
    } else if size < 1024 {
        (format!("{}", size), "B".to_string())
    } else if size < 1024 * 1024 {
        let v = size as f64 / 1024.0;
        if v < 10.0 {
            (format!("{:.1}", v), "KB".to_string())
        } else {
            (format!("{:.0}", v), "KB".to_string())
        }
    } else if size < 1024 * 1024 * 1024 {
        let v = size as f64 / (1024.0 * 1024.0);
        if v < 10.0 {
            (format!("{:.1}", v), "MB".to_string())
        } else {
            (format!("{:.0}", v), "MB".to_string())
        }
    } else {
        let v = size as f64 / (1024.0 * 1024.0 * 1024.0);
        if v < 10.0 {
            (format!("{:.1}", v), "GB".to_string())
        } else {
            (format!("{:.0}", v), "GB".to_string())
        }
    }
}

/// Format size as a single string (for footer etc).
pub fn format_size_simple(size: u64) -> String {
    let (v, u) = format_size(size);
    if u.is_empty() { v } else { format!("{} {}", v, u) }
}

/// Format rwx permissions string from mode.
pub fn format_permissions(mode: u32, is_dir: bool) -> String {
    let mut s = String::with_capacity(10);
    s.push(if is_dir { 'd' } else { '.' });
    s.push(if mode & 0o400 != 0 { 'r' } else { '-' });
    s.push(if mode & 0o200 != 0 { 'w' } else { '-' });
    s.push(if mode & 0o100 != 0 { 'x' } else { '-' });
    s.push(if mode & 0o040 != 0 { 'r' } else { '-' });
    s.push(if mode & 0o020 != 0 { 'w' } else { '-' });
    s.push(if mode & 0o010 != 0 { 'x' } else { '-' });
    s.push(if mode & 0o004 != 0 { 'r' } else { '-' });
    s.push(if mode & 0o002 != 0 { 'w' } else { '-' });
    s.push(if mode & 0o001 != 0 { 'x' } else { '-' });
    s
}

/// Format modification time.
pub fn format_time(time: &SystemTime) -> String {
    let duration = time
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or_default();
    let secs = duration.as_secs() as i64;

    let days = secs / 86400;
    let remaining = secs % 86400;
    let hours = remaining / 3600;
    let minutes = (remaining % 3600) / 60;

    let (year, month, day) = days_to_date(days);

    format!(
        "{:04}-{:02}-{:02} {:02}:{:02}",
        year, month, day, hours, minutes
    )
}

fn days_to_date(mut days: i64) -> (i64, i64, i64) {
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
        assert_eq!(format_size(0), ("-".to_string(), "".to_string()));
        assert_eq!(format_size(512), ("512".to_string(), "B".to_string()));
        assert_eq!(format_size(1024), ("1.0".to_string(), "KB".to_string()));
        assert_eq!(format_size(1048576), ("1.0".to_string(), "MB".to_string()));
    }

    #[test]
    fn test_format_permissions() {
        assert_eq!(format_permissions(0o755, true), "drwxr-xr-x");
        assert_eq!(format_permissions(0o644, false), ".rw-r--r--");
        assert_eq!(format_permissions(0o700, false), ".rwx------");
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
