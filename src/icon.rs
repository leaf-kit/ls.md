/// Icon for a directory entry.
pub fn dir_icon() -> &'static str {
    "\u{1F4C2}" // 📂
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dir_icon() {
        assert_eq!(dir_icon(), "\u{1F4C2}");
    }
}
