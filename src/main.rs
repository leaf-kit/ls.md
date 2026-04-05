mod entry;
mod frontmatter;
mod icon;
mod output;

use clap::Parser;
use std::path::PathBuf;

/// lsmd — the markdown directory utility
///
/// List directory contents with inline metadata for .md and .txt files.
/// Like ls, but understands Markdown frontmatter, headings, and text previews.
///
/// Get started with `lsmd` to see the current directory,
/// or `lsmd -l` for detailed output with metadata summaries.
#[derive(Parser, Debug)]
#[command(name = "lsmd", version, about, long_about)]
pub struct Cli {
    /// Directory to list (default: current directory)
    #[arg(default_value = ".")]
    pub path: PathBuf,

    /// Show hidden files (dotfiles)
    #[arg(short = 'a', long = "all")]
    pub show_hidden: bool,

    /// Long listing format with metadata details
    #[arg(short = 'l', long = "long")]
    pub long_format: bool,

    /// Disable colored output
    #[arg(long = "no-color")]
    pub no_color: bool,

    /// Sort by: name (default), size, modified, type
    #[arg(short = 's', long = "sort", default_value = "name")]
    pub sort_by: String,

    /// Reverse sort order
    #[arg(short = 'r', long = "reverse")]
    pub reverse: bool,

    /// Show only .md and .txt files
    #[arg(short = 'm', long = "md-only")]
    pub md_only: bool,
}

fn main() {
    let cli = Cli::parse();

    if cli.no_color {
        colored::control::set_override(false);
    }

    match output::list_directory(&cli) {
        Ok(()) => {}
        Err(e) => {
            eprintln!("lsmd: {}", e);
            std::process::exit(1);
        }
    }
}
