/// Return an icon for the given file extension.
pub fn icon_for_ext(ext: &str) -> &'static str {
    match ext.to_lowercase().as_str() {
        "md" | "markdown" => "\u{1F4C4}",  // 📄
        "txt" => "\u{1F4DD}",               // 📝
        "rs" => "\u{2699}",                 // ⚙
        "py" => "\u{1F40D}",               // 🐍
        "js" | "mjs" | "cjs" => "\u{1F7E8}", // 🟨
        "ts" | "tsx" => "\u{1F535}",        // 🔵
        "json" => "{}",
        "yaml" | "yml" => "\u{1F4CB}",     // 📋
        "toml" => "\u{1F527}",             // 🔧
        "html" | "htm" => "\u{1F310}",     // 🌐
        "css" | "scss" | "sass" => "\u{1F3A8}", // 🎨
        "sh" | "bash" | "zsh" => "\u{1F4DC}", // 📜
        "go" => "\u{1F439}",               // 🐹
        "java" | "kt" | "kts" => "\u{2615}", // ☕
        "c" | "h" => "C",
        "cpp" | "cc" | "cxx" | "hpp" => "C++",
        "rb" => "\u{1F48E}",               // 💎
        "swift" => "\u{1F426}",            // 🐦
        "lock" => "\u{1F512}",             // 🔒
        "png" | "jpg" | "jpeg" | "gif" | "svg" | "webp" | "ico" => "\u{1F5BC}", // 🖼
        "pdf" => "\u{1F4D5}",              // 📕
        "zip" | "tar" | "gz" | "bz2" | "xz" | "7z" => "\u{1F4E6}", // 📦
        "mp3" | "wav" | "flac" | "ogg" | "aac" => "\u{1F3B5}", // 🎵
        "mp4" | "mkv" | "avi" | "mov" | "webm" => "\u{1F3AC}", // 🎬
        "env" => "\u{1F510}",              // 🔐
        "gitignore" | "gitmodules" | "gitattributes" => "\u{1F500}", // 🔀
        "dockerfile" | "docker" => "\u{1F433}", // 🐳
        "makefile" | "cmake" => "\u{1F3D7}", // 🏗
        _ => "\u{1F4C1}",                  // 📁
    }
}

/// Icon for a directory entry.
pub fn dir_icon() -> &'static str {
    "\u{1F4C2}" // 📂
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_known_extensions() {
        assert_eq!(icon_for_ext("md"), "\u{1F4C4}");
        assert_eq!(icon_for_ext("txt"), "\u{1F4DD}");
        assert_eq!(icon_for_ext("rs"), "\u{2699}");
        assert_eq!(icon_for_ext("py"), "\u{1F40D}");
        assert_eq!(icon_for_ext("json"), "{}");
    }

    #[test]
    fn test_case_insensitive() {
        assert_eq!(icon_for_ext("MD"), icon_for_ext("md"));
        assert_eq!(icon_for_ext("Rs"), icon_for_ext("rs"));
    }

    #[test]
    fn test_unknown_extension() {
        assert_eq!(icon_for_ext("xyz"), "\u{1F4C1}");
    }
}
