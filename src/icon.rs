/// Return a Nerd Font icon for a file based on name, extension, and type.
pub fn icon_for_entry(ext: Option<&str>, is_dir: bool, name: &str) -> &'static str {
    if is_dir {
        return match name.to_lowercase().as_str() {
            ".git"         => "\u{e5fb}",
            ".github"      => "\u{e5fd}",
            ".cargo"       => "\u{e68b}",
            ".config" | "config" => "\u{e5fc}",
            "node_modules" => "\u{e5fa}",
            "src" | "source" => "\u{e5fe}",
            "build" | "dist" | "out" | "target" => "\u{f487}",
            "test" | "tests" | "spec" => "\u{f488}",
            "docs" | "doc" | "documentation" => "\u{f02d}",
            "images" | "img" | "assets" => "\u{f03e}",
            _              => "\u{f115}",
        };
    }

    // Match by exact file name first
    match name.to_lowercase().as_str() {
        "license" | "license.md" | "licence" => return "\u{e60a}",
        "readme" | "readme.md"               => return "\u{e609}",
        "makefile" | "cmake" | "cmakelists.txt" => return "\u{e615}",
        "dockerfile"                          => return "\u{f308}",
        "docker-compose.yml" | "docker-compose.yaml" => return "\u{f308}",
        ".gitignore" | ".gitattributes"       => return "\u{f1d3}",
        ".env" | ".env.local"                 => return "\u{f21b}",
        "package.json"                        => return "\u{e718}",
        "cargo.toml" | "cargo.lock"           => return "\u{e68b}",
        _ => {}
    }

    // Match by extension
    match ext.map(|e| e.to_lowercase()).as_deref() {
        Some("md" | "markdown") => "\u{e73e}",
        Some("txt")             => "\u{f0219}",
        Some("rs")              => "\u{e7a8}",
        Some("py")              => "\u{e73c}",
        Some("js" | "mjs" | "cjs") => "\u{e74e}",
        Some("ts" | "tsx")      => "\u{e628}",
        Some("json")            => "\u{e60b}",
        Some("yaml" | "yml")    => "\u{e6a8}",
        Some("toml")            => "\u{e6b2}",
        Some("html" | "htm")    => "\u{e736}",
        Some("css")             => "\u{e749}",
        Some("scss" | "sass")   => "\u{e74b}",
        Some("sh" | "bash" | "zsh" | "fish") => "\u{e795}",
        Some("go")              => "\u{e626}",
        Some("java")            => "\u{e738}",
        Some("kt" | "kts")     => "\u{e634}",
        Some("rb")              => "\u{e739}",
        Some("swift")           => "\u{e755}",
        Some("c")               => "\u{e61e}",
        Some("h")               => "\u{e61e}",
        Some("cpp" | "cc" | "cxx") => "\u{e61d}",
        Some("hpp")             => "\u{e61d}",
        Some("lock")            => "\u{f023}",
        Some("png" | "jpg" | "jpeg" | "gif" | "bmp") => "\u{f1c5}",
        Some("svg")             => "\u{f1c5}",
        Some("webp" | "ico")   => "\u{f1c5}",
        Some("pdf")             => "\u{f1c1}",
        Some("zip" | "tar" | "gz" | "bz2" | "xz" | "7z" | "rar") => "\u{f1c6}",
        Some("mp3" | "wav" | "flac" | "ogg" | "aac") => "\u{f001}",
        Some("mp4" | "mkv" | "avi" | "mov" | "webm")  => "\u{f03d}",
        Some("env")             => "\u{f21b}",
        Some("gitignore")       => "\u{f1d3}",
        Some("xml")             => "\u{e619}",
        Some("sql")             => "\u{e706}",
        Some("graphql" | "gql") => "\u{e662}",
        Some("vue")             => "\u{e6a0}",
        Some("svelte")          => "\u{e697}",
        Some("ex" | "exs")     => "\u{e62d}",
        Some("erl")             => "\u{e7b1}",
        Some("hs")              => "\u{e61f}",
        Some("lua")             => "\u{e620}",
        Some("r")               => "\u{e68a}",
        Some("php")             => "\u{e73d}",
        Some("pl" | "pm")      => "\u{e769}",
        Some("scala")           => "\u{e737}",
        Some("clj" | "cljs")   => "\u{e768}",
        Some("dart")            => "\u{e798}",
        Some("zig")             => "\u{e6a9}",
        Some("nim")             => "\u{e677}",
        Some("wasm")            => "\u{e6a1}",
        _                       => "\u{f016}",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dir_icon() {
        assert_eq!(icon_for_entry(None, true, "mydir"), "\u{f115}");
    }

    #[test]
    fn test_special_dir_icon() {
        assert_eq!(icon_for_entry(None, true, ".git"), "\u{e5fb}");
        assert_eq!(icon_for_entry(None, true, "src"), "\u{e5fe}");
        assert_eq!(icon_for_entry(None, true, "docs"), "\u{f02d}");
    }

    #[test]
    fn test_special_file_icon() {
        assert_eq!(icon_for_entry(Some("md"), false, "README.md"), "\u{e609}");
        assert_eq!(icon_for_entry(None, false, "Makefile"), "\u{e615}");
        assert_eq!(icon_for_entry(None, false, "Dockerfile"), "\u{f308}");
    }

    #[test]
    fn test_ext_icon() {
        assert_eq!(icon_for_entry(Some("rs"), false, "main.rs"), "\u{e7a8}");
        assert_eq!(icon_for_entry(Some("py"), false, "app.py"), "\u{e73c}");
        assert_eq!(icon_for_entry(Some("json"), false, "data.json"), "\u{e60b}");
    }

    #[test]
    fn test_unknown_icon() {
        assert_eq!(icon_for_entry(Some("xyz"), false, "data.xyz"), "\u{f016}");
    }
}
