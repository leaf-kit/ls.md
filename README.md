<p align="center">
  <img src="images/logo.png" alt="lsmd logo" width="240" />
</p>

# lsmd — **L**ist **M**ark**d**own

[![Release](https://img.shields.io/github/v/release/leaf-kit/ls.md?label=release)](https://github.com/leaf-kit/ls.md/releases/latest)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![Homebrew](https://img.shields.io/badge/homebrew-leaf--kit%2Flsmd-yellow.svg)](https://github.com/leaf-kit/homebrew-lsmd)
[![Homebrew install](https://img.shields.io/badge/brew%20install-lsmd-success.svg)](https://github.com/leaf-kit/homebrew-lsmd)

A markdown-aware directory listing tool for the terminal.

> **v0.1.0 Released** — [GitHub Release](https://github.com/leaf-kit/ls.md/releases/tag/v0.1.0)
>
> ```bash
> git clone https://github.com/leaf-kit/ls.md.git
> cd ls.md && ./build.sh
> ```

**lsmd** is like `ls`, but it understands Markdown. It parses YAML frontmatter, extracts headings from `.md` files, previews `.txt` first lines, and shows extension-based icons — all inline with the file listing. Directories are blue, tags get hash-based color badges, and `.txt` previews are dimmed for clean readability.

A must-have terminal tool for anyone working with Markdown-heavy projects — documentation writers, knowledge base managers, and developers who live in the terminal.

## Why "lsmd"?

**lsmd** stands for **List Markdown**. Just like `ls` lists files, **lsmd** lists files — but with Markdown intelligence. It knows the difference between a file with frontmatter, a file with just an H1 heading, and a plain text file. See metadata at a glance, not after opening.

> *Don't just list files. List meaning.*

## Features

- **YAML frontmatter parsing** — show title, date, and tags inline
- **Tag color badges** — hash-based consistent coloring per tag
- **H1 heading fallback** — when no frontmatter, show the first heading
- **`.txt` first-line preview** — dimmed, truncated at 60 chars
- **Extension-based icons** — `.md` 📄, `.py` 🐍, `.rs` ⚙, `.json` {}, and more
- **Directory-first sorting** — directories always listed first, in blue
- **Hidden file support** — `-a` flag to show dotfiles
- **Long format** — `-l` for size, modification time, and metadata
- **Markdown-only filter** — `-m` to show only `.md` and `.txt` files
- **Sorting options** — by name, size, modified time, or type
- **Reverse sort** — `-r` flag
- **Auto color detection** — disables ANSI in non-TTY (pipe-safe)
- **`--no-color` flag** — explicit color disable
- Fast startup — written in Rust, optimized with LTO

## Installation

### Build from Source

```bash
git clone https://github.com/leaf-kit/ls.md.git
cd ls.md
cargo build --release
cp target/release/lsmd /usr/local/bin/
```

Or use the build script (runs tests first):

```bash
./build.sh
```

## Update

```bash
git pull origin main
cargo build --release
cp target/release/lsmd /usr/local/bin/
```

## Uninstall

```bash
rm /usr/local/bin/lsmd
```

## Playground

The repository includes a `playground/` directory with sample Markdown, text, and other files for testing. You can run lsmd against it to see every feature in action.

```bash
git clone https://github.com/leaf-kit/ls.md.git
cd ls.md
cargo build --release
./target/release/lsmd playground
```

## Usage

```
% lsmd
lsmd — the markdown directory utility

List directory contents with inline metadata for .md and .txt files.
Like ls, but understands Markdown frontmatter, headings, and text previews.

Get started with `lsmd` to see the current directory,
or `lsmd -l` for detailed output with metadata summaries.

Usage: lsmd [OPTIONS] [PATH]

Arguments:
  [PATH]
          Directory to list (default: current directory)

          [default: .]

Options:
  -a, --all
          Show hidden files (dotfiles)

  -l, --long
          Long listing format with metadata details

      --no-color
          Disable colored output

  -s, --sort <SORT_BY>
          Sort by: name (default), size, modified, type

          [default: name]

  -r, --reverse
          Reverse sort order

  -m, --md-only
          Show only .md and .txt files

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version

Discussion:
    lsmd is your markdown companion for navigating directories
    with rich document context. It parses YAML frontmatter,
    extracts headings, previews text files, and displays
    extension-based icons — all inline with the listing.

    Get started with `lsmd` to see the current directory,
    or `lsmd -l` for detailed metadata view.
```

## Commands & Output Examples

All examples below are actual outputs from running `lsmd` against the included `playground/` directory.

### 1. Default Listing

```
% lsmd playground
📂 docs/
🐍 app.py
📄 blog-post.md  Getting Started with Rust · 2026-03-15 ·  rust   programming   tutorial
📄 broken-yaml.md
📋 config.yaml
📄 empty.md
📝 empty.txt
📝 long-line.txt  This is a very long first line that should be truncated wit…
📄 meeting-notes.md  Team Meeting Notes · 2026-04-01 ·  meeting   planning
📄 no-frontmatter.md  Simple Document
📝 notes.txt  Quick notes from today's brainstorming session about the ne…
{} sample.json
```

Key behaviors:
- **Directories** listed first (`docs/`)
- **`.md` with frontmatter** shows title · date · tag badges (`blog-post.md`)
- **`.md` without frontmatter** shows H1 heading (`no-frontmatter.md`)
- **`.md` with broken YAML** silently ignored (`broken-yaml.md`)
- **`.md` empty file** no extra info (`empty.md`)
- **`.txt` files** show dimmed first-line preview, truncated at 60 chars
- **Other files** show extension-based icon only

### 2. Long Format (`-l`)

```
% lsmd playground -l
📂  96 B  2026-04-05 01:22  docs/
🐍 152 B  2026-04-05 01:21  app.py
📄 487 B  2026-04-05 01:21  blog-post.md  Getting Started with Rust · 2026-03-15 ·  rust   programming   tutorial
📄 191 B  2026-04-05 01:22  broken-yaml.md
📋  56 B  2026-04-05 01:22  config.yaml
📄   0 B  2026-04-05 01:21  empty.md
📝   0 B  2026-04-05 01:21  empty.txt
📝 180 B  2026-04-05 01:21  long-line.txt  This is a very long first line that should be truncated wit…
📄 395 B  2026-04-05 01:21  meeting-notes.md  Team Meeting Notes · 2026-04-01 ·  meeting   planning
📄 187 B  2026-04-05 01:21  no-frontmatter.md  Simple Document
📝 190 B  2026-04-05 01:21  notes.txt  Quick notes from today's brainstorming session about the ne…
{}  43 B  2026-04-05 01:21  sample.json
```

Shows file size and modification time alongside metadata.

### 3. Show Hidden Files (`-a`)

```
% lsmd playground -a
📂 .hidden-dir/
📂 docs/
📄 .hidden-file.md  Hidden Markdown File
🐍 app.py
📄 blog-post.md  Getting Started with Rust · 2026-03-15 ·  rust   programming   tutorial
...
```

Reveals dotfiles and hidden directories.

### 4. Markdown Only (`-m`)

```
% lsmd playground -m
📂 docs/
📄 blog-post.md  Getting Started with Rust · 2026-03-15 ·  rust   programming   tutorial
📄 broken-yaml.md
📄 empty.md
📝 empty.txt
📝 long-line.txt  This is a very long first line that should be truncated wit…
📄 meeting-notes.md  Team Meeting Notes · 2026-04-01 ·  meeting   planning
📄 no-frontmatter.md  Simple Document
📝 notes.txt  Quick notes from today's brainstorming session about the ne…
```

Filters to show only `.md` and `.txt` files (directories are always included).

### 5. Sort Options

**Sort by size:**
```
% lsmd playground -s size
📂 docs/
📝 empty.txt
📄 empty.md
{} sample.json
📋 config.yaml
🐍 app.py
📝 long-line.txt  This is a very long first line that should be truncated wit…
📄 no-frontmatter.md  Simple Document
📝 notes.txt  Quick notes from today's brainstorming session about the ne…
📄 broken-yaml.md
📄 meeting-notes.md  Team Meeting Notes · 2026-04-01 ·  meeting   planning
📄 blog-post.md  Getting Started with Rust · 2026-03-15 ·  rust   programming   tutorial
```

**Sort by name, reversed:**
```
% lsmd playground -s name -r
📂 docs/
{} sample.json
📝 notes.txt  Quick notes from today's brainstorming session about the ne…
📄 no-frontmatter.md  Simple Document
📄 meeting-notes.md  Team Meeting Notes · 2026-04-01 ·  meeting   planning
📝 long-line.txt  This is a very long first line that should be truncated wit…
📝 empty.txt
📄 empty.md
📋 config.yaml
📄 broken-yaml.md
📄 blog-post.md  Getting Started with Rust · 2026-03-15 ·  rust   programming   tutorial
🐍 app.py
```

| Option | Short | Description |
|--------|-------|-------------|
| `--all` | `-a` | Show hidden files (dotfiles) |
| `--long` | `-l` | Long format with size, date, metadata |
| `--no-color` | | Disable ANSI color output |
| `--sort <FIELD>` | `-s` | Sort by: `name`, `size`, `modified`, `type` |
| `--reverse` | `-r` | Reverse sort order |
| `--md-only` | `-m` | Show only `.md` and `.txt` files |

## File Type Handling

| File Type | Behavior |
|-----------|----------|
| `.md` with frontmatter | title · date · colored tag badges |
| `.md` with H1 only | dimmed heading text |
| `.md` empty / no metadata | icon only, no extra info |
| `.md` broken YAML | silently ignored, no crash |
| `.txt` | dimmed first non-empty line (max 60 chars + ellipsis) |
| `.txt` empty | icon only |
| Other files | extension-based icon (🐍 .py, ⚙ .rs, {} .json, etc.) |
| Directories | 📂 icon, blue name, always listed first |

## Extension Icon Map

| Icon | Extensions |
|------|-----------|
| 📄 | `.md`, `.markdown` |
| 📝 | `.txt` |
| ⚙ | `.rs` |
| 🐍 | `.py` |
| 🟨 | `.js`, `.mjs`, `.cjs` |
| 🔵 | `.ts`, `.tsx` |
| {} | `.json` |
| 📋 | `.yaml`, `.yml` |
| 🔧 | `.toml` |
| 🌐 | `.html`, `.htm` |
| 🎨 | `.css`, `.scss`, `.sass` |
| 📜 | `.sh`, `.bash`, `.zsh` |
| 🐹 | `.go` |
| ☕ | `.java`, `.kt`, `.kts` |
| 💎 | `.rb` |
| 🐦 | `.swift` |
| 🖼 | `.png`, `.jpg`, `.gif`, `.svg`, `.webp` |
| 📕 | `.pdf` |
| 📦 | `.zip`, `.tar`, `.gz`, `.7z` |
| 🎵 | `.mp3`, `.wav`, `.flac` |
| 🎬 | `.mp4`, `.mkv`, `.avi` |
| 🐳 | `Dockerfile` |
| 📁 | Other / unknown |
| 📂 | Directory |

## Pipe Integration

lsmd auto-disables ANSI colors when output is piped, making it safe for use with `grep`, `awk`, `wc`, and other Unix tools.

```bash
# Count markdown files
lsmd -m playground | wc -l

# Find files with specific tags in frontmatter
lsmd playground --no-color | grep "rust"

# List only file names
lsmd playground --no-color | awk '{print $2}'
```

## Edge Cases

lsmd handles these scenarios gracefully:

- **Empty files** — displayed with icon only, no crash
- **Broken YAML frontmatter** — silently ignored
- **Binary files** — shown with generic icon
- **Very long file names** — terminal wrapping handles overflow
- **Non-existent paths** — clear error message
- **Permission errors** — skipped silently
- **Non-directory paths** — clear error message

## License

[MIT](LICENSE)
