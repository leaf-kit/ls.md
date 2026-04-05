/// Return a type-indicator icon for a file based on extension.
pub fn icon_for_entry(ext: Option<&str>, is_dir: bool) -> &'static str {
    if is_dir {
        "\u{f115}" //
    } else {
        match ext.map(|e| e.to_lowercase()).as_deref() {
            Some("md" | "markdown") => "\u{f48a}",  //
            Some("txt")            => "\u{f15c}",  //
            Some("rs")             => "\u{e7a8}",  //
            Some("py")             => "\u{e73c}",  //
            Some("js" | "mjs")     => "\u{e74e}",  //
            Some("ts" | "tsx")     => "\u{e628}",  //
            Some("json")           => "\u{e60b}",  //
            Some("yaml" | "yml")   => "\u{e6a8}",  //
            Some("toml")           => "\u{e6b2}",  //
            Some("html" | "htm")   => "\u{e736}",  //
            Some("css" | "scss")   => "\u{e749}",  //
            Some("sh" | "bash" | "zsh") => "\u{e795}", //
            Some("go")             => "\u{e626}",  //
            Some("java")           => "\u{e738}",  //
            Some("rb")             => "\u{e739}",  //
            Some("swift")          => "\u{e755}",  //
            Some("c" | "h")        => "\u{e61e}",  //
            Some("cpp" | "cc" | "hpp") => "\u{e61d}", //
            Some("lock")           => "\u{f023}",  //
            Some("png" | "jpg" | "jpeg" | "gif" | "svg" | "webp") => "\u{f1c5}", //
            Some("pdf")            => "\u{f1c1}",  //
            Some("zip" | "tar" | "gz" | "7z") => "\u{f1c6}", //
            Some("mp3" | "wav" | "flac") => "\u{f001}", //
            Some("mp4" | "mkv" | "avi" | "mov") => "\u{f03d}", //
            Some("env")            => "\u{f21b}",  //
            Some("gitignore")      => "\u{e702}",  //
            _                      => "\u{f016}",  //
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dir_icon() {
        assert_eq!(icon_for_entry(None, true), "\u{f115}");
    }

    #[test]
    fn test_md_icon() {
        assert_eq!(icon_for_entry(Some("md"), false), "\u{f48a}");
    }

    #[test]
    fn test_unknown_icon() {
        assert_eq!(icon_for_entry(Some("xyz"), false), "\u{f016}");
    }
}
