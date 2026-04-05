/// Return a Unicode icon for a file based on name, extension, and type.
/// Uses only widely-supported Unicode symbols — no Nerd Font required.
pub fn icon_for_entry(ext: Option<&str>, is_dir: bool, _name: &str) -> &'static str {
    if is_dir {
        return "\u{25B8}"; // ▸
    }

    // Match by extension
    match ext.map(|e| e.to_lowercase()).as_deref() {
        Some("md" | "markdown") => "\u{25C6}",  // ◆
        Some("txt")             => "\u{25CB}",  // ○
        _                       => "\u{25E6}",  // ◦
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dir_icon() {
        assert_eq!(icon_for_entry(None, true, "mydir"), "\u{25B8}");
    }

    #[test]
    fn test_md_icon() {
        assert_eq!(icon_for_entry(Some("md"), false, "test.md"), "\u{25C6}");
    }

    #[test]
    fn test_txt_icon() {
        assert_eq!(icon_for_entry(Some("txt"), false, "test.txt"), "\u{25CB}");
    }

    #[test]
    fn test_other_icon() {
        assert_eq!(icon_for_entry(Some("py"), false, "app.py"), "\u{25E6}");
    }
}
