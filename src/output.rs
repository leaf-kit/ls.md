use std::cmp::Ordering;
use std::fs;
use std::path::Path;

use colored::Colorize;
use terminal_size::{Width, terminal_size};

use crate::Cli;
use crate::entry::{DirEntry, format_permissions, format_size, format_size_simple, format_time};
use crate::frontmatter::{extract_first_heading, format_md_summary, parse_md, read_txt_preview};
use crate::icon::icon_for_entry;

const NAME_COL_WIDTH: usize = 22;

// ─── Public API ─────────────────────────────────────────────

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

    print_header(path, &entries);

    if cli.long_format {
        print_long(&entries, cli.title_only);
    } else {
        print_default(&entries, cli.title_only);
    }

    print_footer(&entries);

    Ok(())
}

// ─── Header / Footer ───────────────────────────────────────

/// Print summary header: path + counts.
fn print_header(path: &Path, entries: &[DirEntry]) {
    let dir_count = entries.iter().filter(|e| e.is_dir).count();
    let file_count = entries.iter().filter(|e| !e.is_dir).count();
    let md_count = entries.iter().filter(|e| e.is_md()).count();
    let txt_count = entries.iter().filter(|e| e.is_txt()).count();

    let path_str = path
        .canonicalize()
        .unwrap_or_else(|_| path.to_path_buf())
        .display()
        .to_string();

    let mut parts: Vec<String> = Vec::new();
    if dir_count > 0 {
        parts.push(format!("{} dirs", dir_count));
    }
    parts.push(format!("{} files", file_count));
    if md_count > 0 {
        parts.push(format!("{} md", md_count));
    }
    if txt_count > 0 {
        parts.push(format!("{} txt", txt_count));
    }

    println!(
        " {}  {}",
        path_str.bright_blue().bold(),
        format!("({})", parts.join(", ")).bright_black()
    );
    println!();
}

/// Print footer with totals.
fn print_footer(entries: &[DirEntry]) {
    let total_size: u64 = entries.iter().map(|e| e.size).sum();
    println!(
        "\n {}",
        format!(
            "Total: {} items, {}",
            entries.len(),
            format_size_simple(total_size)
        )
        .bright_black()
    );
}

// ─── Name Column ────────────────────────────────────────────

/// Format name into a fixed-width column. Returns (colored_string, visible_width).
fn format_name_column(entry: &DirEntry) -> (String, usize) {
    let raw_name = &entry.name;
    let is_dir_entry = entry.is_dir;

    let display_raw = if is_dir_entry {
        raw_name.clone()
    } else {
        raw_name.clone()
    };

    let visible_len = display_raw.chars().count();

    if visible_len <= NAME_COL_WIDTH {
        let colored = colorize_name(&display_raw, entry);
        let pad = NAME_COL_WIDTH - visible_len;
        (format!("{}{}", colored, " ".repeat(pad)), NAME_COL_WIDTH)
    } else {
        let truncated = if is_dir_entry {
            let t: String = raw_name.chars().take(NAME_COL_WIDTH - 1).collect();
            format!("{}…", t)
        } else if let Some(dot_pos) = raw_name.rfind('.') {
            let ext = &raw_name[dot_pos..];
            let ext_len = ext.chars().count();
            if ext_len + 2 < NAME_COL_WIDTH {
                let base_budget = NAME_COL_WIDTH - ext_len - 1;
                let base: String = raw_name.chars().take(base_budget).collect();
                format!("{}…{}", base, ext)
            } else {
                let t: String = raw_name.chars().take(NAME_COL_WIDTH - 1).collect();
                format!("{}…", t)
            }
        } else {
            let t: String = raw_name.chars().take(NAME_COL_WIDTH - 1).collect();
            format!("{}…", t)
        };

        let colored = colorize_name(&truncated, entry);
        (colored, NAME_COL_WIDTH)
    }
}

// ─── Icon ───────────────────────────────────────────────────

/// Get colored icon string for an entry.
fn format_icon(entry: &DirEntry) -> String {
    let icon = icon_for_entry(entry.extension.as_deref(), entry.is_dir, &entry.name);

    if entry.is_dir {
        icon.bright_blue().bold().to_string()
    } else if entry.is_hidden {
        icon.bright_black().to_string()
    } else if entry.is_md() {
        icon.green().to_string()
    } else if entry.is_txt() {
        icon.yellow().to_string()
    } else {
        icon.bright_black().to_string()
    }
}

// ─── Name Coloring ──────────────────────────────────────────

/// Apply color to a name string based on entry type (lsd palette).
fn colorize_name(name: &str, entry: &DirEntry) -> String {
    if entry.is_dir {
        name.bright_blue().bold().to_string()
    } else if entry.is_hidden {
        name.bright_black().to_string()
    } else if entry.is_md() {
        name.green().to_string()
    } else if entry.is_txt() {
        name.yellow().to_string()
    } else if entry.is_executable() {
        name.bright_green().bold().to_string()
    } else {
        name.normal().to_string()
    }
}

// ─── Permissions Coloring ───────────────────────────────────

/// Color each character of a permissions string (lsd style).
fn color_permissions(perm: &str) -> String {
    perm.chars()
        .map(|c| match c {
            'd' => "d".bright_blue().bold().to_string(),
            '.' => ".".bright_black().to_string(),
            'r' => "r".green().to_string(),
            'w' => "w".yellow().to_string(),
            'x' => "x".red().to_string(),
            '-' => "-".bright_black().to_string(),
            _ => c.to_string(),
        })
        .collect()
}

// ─── Size Coloring ──────────────────────────────────────────

/// Color size value + unit by magnitude (lsd style).
fn color_size_parts(val: &str, unit: &str, size: u64) -> String {
    if size == 0 {
        format!("{}", val.bright_black())
    } else if size < 1024 {
        format!("{} {}", val.green(), unit.green())
    } else if size < 1024 * 1024 {
        format!("{} {}", val.yellow(), unit.yellow())
    } else if size < 1024 * 1024 * 1024 {
        format!("{} {}", val.red(), unit.red())
    } else {
        format!("{} {}", val.bright_red().bold(), unit.bright_red().bold())
    }
}

// ─── Date Coloring ──────────────────────────────────────────

/// Color date string (lsd uses cyan tones).
fn color_date(time_str: &str) -> String {
    time_str.bright_black().to_string()
}

// ─── Collect & Sort ─────────────────────────────────────────

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

        if entry.is_hidden && !cli.show_hidden {
            continue;
        }

        if cli.md_only && !entry.is_dir && !entry.is_md() && !entry.is_txt() {
            continue;
        }

        entries.push(entry);
    }

    Ok(entries)
}

fn sort_entries(entries: &mut [DirEntry], sort_by: &str, reverse: bool) {
    entries.sort_by(|a, b| {
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

// ─── Default Output ─────────────────────────────────────────

fn print_default(entries: &[DirEntry], title_only: bool) {
    let term_width = terminal_size()
        .map(|(Width(w), _)| w as usize)
        .unwrap_or(80);

    for entry in entries {
        let icon = format_icon(entry);
        let (name_col, _) = format_name_column(entry);
        let summary = get_summary(entry, title_only);

        let line = if summary.is_empty() {
            format!(" {} {}", icon, name_col.trim_end())
        } else {
            format!(" {} {}  {}", icon, name_col, summary)
        };

        println!("{}", truncate_visible(&line, term_width));
    }
}

// ─── Long Output ────────────────────────────────────────────

fn print_long(entries: &[DirEntry], title_only: bool) {
    let term_width = terminal_size()
        .map(|(Width(w), _)| w as usize)
        .unwrap_or(80);

    // Calculate max widths for alignment
    let max_size_val_len = entries
        .iter()
        .map(|e| {
            let (v, _) = format_size(e.size);
            v.len()
        })
        .max()
        .unwrap_or(1);

    let max_size_unit_len = entries
        .iter()
        .map(|e| {
            let (_, u) = format_size(e.size);
            u.len()
        })
        .max()
        .unwrap_or(1);

    for entry in entries {
        let icon = format_icon(entry);
        let (name_col, _) = format_name_column(entry);

        // Permissions
        let perm_str = format_permissions(entry.mode, entry.is_dir);
        let colored_perm = color_permissions(&perm_str);

        // Size
        let (size_val, size_unit) = format_size(entry.size);
        let colored_size = color_size_parts(&size_val, &size_unit, entry.size);
        let size_pad_val = max_size_val_len - size_val.len();
        let size_pad_unit = max_size_unit_len - size_unit.len();
        let size_field = format!(
            "{}{}{}{}",
            " ".repeat(size_pad_val),
            colored_size,
            " ".repeat(size_pad_unit),
            ""
        );

        // Date
        let time_str = entry
            .modified
            .as_ref()
            .map(format_time)
            .unwrap_or_else(|| "                ".to_string());
        let colored_date = color_date(&time_str);

        let summary = get_summary(entry, title_only);

        let line = if summary.is_empty() {
            format!(
                " {}  {} {} {}  {}",
                colored_perm, icon, size_field, colored_date, name_col.trim_end()
            )
        } else {
            format!(
                " {}  {} {} {}  {}  {}",
                colored_perm, icon, size_field, colored_date, name_col, summary
            )
        };

        println!("{}", truncate_visible(&line, term_width));
    }
}

// ─── Summary ────────────────────────────────────────────────

fn get_summary(entry: &DirEntry, title_only: bool) -> String {
    if entry.is_dir {
        return String::new();
    }

    if entry.is_md() {
        if title_only {
            // --title mode: show only the first # heading
            if let Some(heading) = extract_first_heading(&entry.path) {
                use colored::Colorize;
                return heading.dimmed().to_string();
            }
        } else {
            let meta = parse_md(&entry.path);
            let summary = format_md_summary(&meta);
            if !summary.is_empty() {
                return summary;
            }
        }
    } else if entry.is_txt() {
        if let Some(preview) = read_txt_preview(&entry.path) {
            return preview;
        }
    }

    String::new()
}

fn truncate_visible(s: &str, max_width: usize) -> &str {
    let _ = max_width;
    s
}

// ─── Tests ──────────────────────────────────────────────────

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
            title_only: false,
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
