use std::cmp::Ordering;
use std::fs;

use colored::Colorize;
use terminal_size::{Width, terminal_size};

use crate::Cli;
use crate::entry::{DirEntry, format_size, format_time};
use crate::frontmatter::{format_md_summary, parse_md, read_txt_preview};
use crate::icon::{dir_icon, icon_for_ext};

/// List directory contents according to CLI options.
pub fn list_directory(cli: &Cli) -> Result<(), String> {
    let path = &cli.path;

    if !path.exists() {
        return Err(format!(
            "cannot access '{}': No such file or directory",
            path.display()
        ));
    }

    if !path.is_dir() {
        return Err(format!("'{}' is not a directory", path.display()));
    }

    let mut entries = collect_entries(cli)?;
    sort_entries(&mut entries, &cli.sort_by, cli.reverse);

    if cli.long_format {
        print_long(&entries);
    } else {
        print_default(&entries);
    }

    Ok(())
}

/// Collect directory entries based on CLI filters.
fn collect_entries(cli: &Cli) -> Result<Vec<DirEntry>, String> {
    let read_dir = fs::read_dir(&cli.path)
        .map_err(|e| format!("cannot open '{}': {}", cli.path.display(), e))?;

    let mut entries: Vec<DirEntry> = Vec::new();

    for item in read_dir {
        let item = match item {
            Ok(i) => i,
            Err(_) => continue,
        };

        let entry = match DirEntry::from_path(item.path()) {
            Some(e) => e,
            None => continue,
        };

        // Filter hidden files
        if entry.is_hidden && !cli.show_hidden {
            continue;
        }

        // Filter md-only mode
        if cli.md_only && !entry.is_dir && !entry.is_md() && !entry.is_txt() {
            continue;
        }

        entries.push(entry);
    }

    Ok(entries)
}

/// Sort entries by the specified field.
fn sort_entries(entries: &mut [DirEntry], sort_by: &str, reverse: bool) {
    entries.sort_by(|a, b| {
        // Directories always first
        let dir_ord = b.is_dir.cmp(&a.is_dir);
        if dir_ord != Ordering::Equal {
            return dir_ord;
        }

        let ord = match sort_by {
            "size" => a.size.cmp(&b.size),
            "modified" => a.modified.cmp(&b.modified),
            "type" => a.extension.cmp(&b.extension),
            _ => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
        };

        if reverse { ord.reverse() } else { ord }
    });
}

/// Default (compact) output format.
fn print_default(entries: &[DirEntry]) {
    let term_width = terminal_size()
        .map(|(Width(w), _)| w as usize)
        .unwrap_or(80);

    for entry in entries {
        let icon = if entry.is_dir {
            dir_icon()
        } else {
            entry
                .extension
                .as_deref()
                .map(icon_for_ext)
                .unwrap_or("\u{1F4C1}")
        };

        let name = format_name(entry);
        let summary = get_summary(entry);

        let line = if summary.is_empty() {
            format!("{} {}", icon, name)
        } else {
            format!("{} {}  {}", icon, name, summary)
        };

        // Truncate to terminal width (approximate, ANSI codes make exact hard)
        println!("{}", truncate_visible(&line, term_width));
    }
}

/// Long format output with details.
fn print_long(entries: &[DirEntry]) {
    let term_width = terminal_size()
        .map(|(Width(w), _)| w as usize)
        .unwrap_or(80);

    // Calculate column widths
    let max_size_len = entries
        .iter()
        .map(|e| format_size(e.size).len())
        .max()
        .unwrap_or(4);

    for entry in entries {
        let icon = if entry.is_dir {
            dir_icon()
        } else {
            entry
                .extension
                .as_deref()
                .map(icon_for_ext)
                .unwrap_or("\u{1F4C1}")
        };

        let name = format_name(entry);
        let size = format_size(entry.size);
        let time = entry
            .modified
            .as_ref()
            .map(format_time)
            .unwrap_or_else(|| "                ".to_string());

        let summary = get_summary(entry);

        let line = if summary.is_empty() {
            format!(
                "{} {:>width$}  {}  {}",
                icon,
                size.dimmed(),
                time.dimmed(),
                name,
                width = max_size_len
            )
        } else {
            format!(
                "{} {:>width$}  {}  {}  {}",
                icon,
                size.dimmed(),
                time.dimmed(),
                name,
                summary,
                width = max_size_len
            )
        };

        println!("{}", truncate_visible(&line, term_width));
    }
}

/// Format the entry name with color.
fn format_name(entry: &DirEntry) -> String {
    if entry.is_dir {
        format!("{}/", entry.name.blue().bold())
    } else if entry.is_hidden {
        entry.name.dimmed().to_string()
    } else if entry.is_md() {
        entry.name.green().to_string()
    } else if entry.is_txt() {
        entry.name.white().to_string()
    } else {
        entry.name.normal().to_string()
    }
}

/// Get inline summary for an entry.
fn get_summary(entry: &DirEntry) -> String {
    if entry.is_dir {
        return String::new();
    }

    if entry.is_md() {
        let meta = parse_md(&entry.path);
        let summary = format_md_summary(&meta);
        if !summary.is_empty() {
            return summary;
        }
    } else if entry.is_txt() {
        if let Some(preview) = read_txt_preview(&entry.path) {
            return preview;
        }
    }

    String::new()
}

/// Truncate a string to approximate visible width.
/// This is approximate since ANSI escape codes are not counted.
fn truncate_visible(s: &str, max_width: usize) -> &str {
    // For now, don't truncate — terminal wrapping handles it.
    // Exact ANSI-aware truncation is complex; let terminal handle overflow.
    let _ = max_width;
    s
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn make_cli(path: &str) -> Cli {
        Cli {
            path: path.into(),
            show_hidden: false,
            long_format: false,
            no_color: true,
            sort_by: "name".to_string(),
            reverse: false,
            md_only: false,
        }
    }

    #[test]
    fn test_list_nonexistent() {
        let cli = make_cli("/nonexistent/path");
        assert!(list_directory(&cli).is_err());
    }

    #[test]
    fn test_list_file_not_dir() {
        let dir = tempfile::tempdir().unwrap();
        let file = dir.path().join("file.txt");
        fs::write(&file, "hello").unwrap();

        let cli = make_cli(file.to_str().unwrap());
        assert!(list_directory(&cli).is_err());
    }

    #[test]
    fn test_list_empty_dir() {
        let dir = tempfile::tempdir().unwrap();
        let cli = make_cli(dir.path().to_str().unwrap());
        assert!(list_directory(&cli).is_ok());
    }

    #[test]
    fn test_list_with_files() {
        let dir = tempfile::tempdir().unwrap();
        fs::write(dir.path().join("readme.md"), "# Hello\nWorld").unwrap();
        fs::write(dir.path().join("notes.txt"), "First line of notes").unwrap();
        fs::write(dir.path().join("app.py"), "print('hello')").unwrap();

        let cli = make_cli(dir.path().to_str().unwrap());
        assert!(list_directory(&cli).is_ok());
    }

    #[test]
    fn test_hidden_files_filtered() {
        let dir = tempfile::tempdir().unwrap();
        fs::write(dir.path().join(".hidden"), "").unwrap();
        fs::write(dir.path().join("visible.txt"), "text").unwrap();

        let cli = make_cli(dir.path().to_str().unwrap());
        let entries = collect_entries(&cli).unwrap();
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].name, "visible.txt");
    }

    #[test]
    fn test_hidden_files_shown_with_flag() {
        let dir = tempfile::tempdir().unwrap();
        fs::write(dir.path().join(".hidden"), "").unwrap();
        fs::write(dir.path().join("visible.txt"), "text").unwrap();

        let mut cli = make_cli(dir.path().to_str().unwrap());
        cli.show_hidden = true;
        let entries = collect_entries(&cli).unwrap();
        assert_eq!(entries.len(), 2);
    }

    #[test]
    fn test_md_only_filter() {
        let dir = tempfile::tempdir().unwrap();
        fs::write(dir.path().join("readme.md"), "# Hello").unwrap();
        fs::write(dir.path().join("notes.txt"), "text").unwrap();
        fs::write(dir.path().join("app.py"), "code").unwrap();
        fs::create_dir(dir.path().join("subdir")).unwrap();

        let mut cli = make_cli(dir.path().to_str().unwrap());
        cli.md_only = true;
        let entries = collect_entries(&cli).unwrap();
        // Should include: readme.md, notes.txt, subdir (dirs always included)
        let names: Vec<&str> = entries.iter().map(|e| e.name.as_str()).collect();
        assert!(names.contains(&"readme.md"));
        assert!(names.contains(&"notes.txt"));
        assert!(names.contains(&"subdir"));
        assert!(!names.contains(&"app.py"));
    }

    #[test]
    fn test_sort_by_name() {
        let mut entries = vec![
            DirEntry::from_path(create_temp_file("b.md", "")).unwrap(),
            DirEntry::from_path(create_temp_file("a.md", "")).unwrap(),
        ];
        sort_entries(&mut entries, "name", false);
        assert_eq!(entries[0].name, "a.md");
        assert_eq!(entries[1].name, "b.md");
    }

    fn create_temp_file(name: &str, content: &str) -> std::path::PathBuf {
        let dir = std::env::temp_dir().join("lsmd_test");
        fs::create_dir_all(&dir).ok();
        let path = dir.join(name);
        fs::write(&path, content).unwrap();
        path
    }
}
