<p align="center">
  <img src="images/logo.png" alt="lsmd logo" width="240" />
</p>

# lsmd — **L**ist **M**ark**d**own

[![Release](https://img.shields.io/github/v/release/leaf-kit/ls.md?label=release)](https://github.com/leaf-kit/ls.md/releases/latest)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![GitHub Stars](https://img.shields.io/github/stars/leaf-kit/ls.md?style=social)](https://github.com/leaf-kit/ls.md/stargazers)
[![GitHub Forks](https://img.shields.io/github/forks/leaf-kit/ls.md?style=social)](https://github.com/leaf-kit/ls.md/network/members)
[![GitHub Downloads](https://img.shields.io/github/downloads/leaf-kit/ls.md/total?label=downloads)](https://github.com/leaf-kit/ls.md/releases)
[![Homebrew](https://img.shields.io/badge/homebrew-leaf--kit%2Flsmd-yellow.svg)](https://github.com/leaf-kit/homebrew-lsmd)
[![brew install](https://img.shields.io/badge/brew%20install-lsmd-success.svg)](https://github.com/leaf-kit/homebrew-lsmd)

Production-ready, structure-aware directory listing for Markdown-heavy workflows.

> **v0.2.0 Released** — [GitHub Release](https://github.com/leaf-kit/ls.md/releases/tag/v0.2.0) | [Homebrew Tap](https://github.com/leaf-kit/homebrew-lsmd)
>
> ```bash
> brew tap leaf-kit/lsmd && brew install lsmd
> ```

**lsmd** is a drop-in companion to `ls`, purpose-built for developers, technical writers, and PKM practitioners who work with Markdown daily. It parses YAML frontmatter, extracts headings, previews text files, and renders colored tag badges — all inline, in a single command. Designed for real-world document collections from dozens to thousands of files.

Built with Rust for speed and safety. Optimized with LTO. Zero runtime dependencies. Ships as a single static binary.

## Why lsmd?

### The Problem

With `ls`, all you see is a list of file names. To know what `meeting-2026-03.md` contains, you have to open it. When you have 50, 100, or 500 markdown files, this means opening files one by one just to find what you're looking for.

```
% ls
api-design.md    debugging-checklist.txt  markdown-style.md     quick-reference.txt
cli-ux-tips.md   git-workflow.md          project-kickoff.md    rust-error-handling.md
```

File names alone tell you almost nothing. What tags does `api-design.md` have? When was `project-kickoff.md` written? What is `quick-reference.txt` about?

### The Solution

**lsmd** reads the content and shows you the answers — without opening a single file.

- **`.md` files**: Parses YAML frontmatter and displays **title**, **date**, and **tags** as colored badges inline. If there's no frontmatter, it falls back to the first `# H1` heading, then to the first meaningful body line.
- **`.txt` files**: Shows the **first line** as a dimmed preview, so you instantly know what the file is about.

```
% lsmd
  api-design.md         RESTful API Design Principles · 2026-03-15 ·  api   rest   design
  cli-ux-tips.md        CLI UX Design Tips · 2026-04-03 ·  cli   ux   design
  debugging-check….txt  Step-by-step debugging checklist for production incidents
  git-workflow.md       Git Workflow Guide · 2026-03-20 ·  git   workflow   collaboration
  project-kickoff.md    Project Kickoff Checklist · 2026-04-01 ·  project   checklist   onboarding
  quick-reference.txt   Common terminal shortcuts and commands for daily developmen…
  rust-error-handl….md  Rust Error Handling Patterns · 2026-03-28 ·  rust   error-handling   patterns
```

Now you can see at a glance: what each document is about, when it was written, and what topics it covers — all from a single command.

### Built for PKM (Personal Knowledge Management)

If you manage a personal knowledge base with tools like Obsidian, Logseq, Dendron, or plain markdown files, **lsmd** is your terminal companion. PKM workflows rely heavily on **frontmatter tags** and **titles** to organize knowledge — but `ls` ignores all of that.

With lsmd:

- **Navigate your vault from the terminal** — see titles, dates, and tags without launching a GUI
- **Find notes by tag** — pipe to `grep` to instantly locate all notes tagged `#rust` or `#meeting`
- **Scan before you open** — know what's inside 100 files without opening any of them
- **Review at a glance** — quickly check if your notes have proper frontmatter and tags
- **Combine with Unix tools** — `grep`, `awk`, `sort` for powerful ad-hoc queries across your knowledge base

Trusted by developers and writers who manage documentation repositories, Zettelkasten vaults, and engineering knowledge bases from the terminal.

> *Don't just list files. List meaning.*

<p align="center">
  <img src="images/lsmd.png" alt="lsmd screenshot" width="100%" />
</p>

## Features

- **YAML frontmatter parsing** — show title, date, and tags inline
- **Tag color badges** — hash-based consistent coloring per tag
- **H1 heading fallback** — when no frontmatter, show the first heading
- **`.txt` first-line preview** — dimmed, truncated at 60 chars
- **Nerd Font icons** — file-type specific icons with extension-based coloring (`.rs` red, `.py` yellow, `.json` yellow, `.yaml` magenta, etc.)
- **Column-aligned output** — file names fit a fixed 22-char column; long names truncated with `…` preserving extension
- **Header & footer** — directory path with summary counts (dirs, files, md, txt) and total size
- **Size coloring** — green (bytes), yellow (KB), red (MB+) in long format
- **Directory-first sorting** — directories always listed first, bright blue bold
- **Hidden file support** — `-a` flag to show dotfiles
- **Long format** — `-l` for size, modification time, and metadata
- **Markdown-only filter** — `-m` to show only `.md` and `.txt` files
- **Sorting options** — by name, size, modified time, or type
- **Reverse sort** — `-r` flag
- **Auto color detection** — disables ANSI in non-TTY (pipe-safe)
- **`--no-color` flag** — explicit color disable
- Fast startup — written in Rust, optimized with LTO

## Installation

### Homebrew (macOS)

```bash
brew tap leaf-kit/lsmd
brew install lsmd
```

### Build from Source

```bash
git clone https://github.com/leaf-kit/ls.md.git
cd ls.md
cargo build --release
cp target/release/lsmd /usr/local/bin/
```

Or use the interactive build script (runs tests before release):

```bash
./build.sh
```

The build script provides a menu with options for debug/release builds, local install, tests, clippy, packaging, and Homebrew deployment.

## Update

### Homebrew

```bash
brew upgrade lsmd
```

### From Source

```bash
git pull origin main
cargo build --release
cp target/release/lsmd /usr/local/bin/
```

## Uninstall

### Homebrew

```bash
brew uninstall lsmd
brew untap leaf-kit/lsmd
```

### Manual (source install)

```bash
rm /usr/local/bin/lsmd
```

## Playground

The repository includes a `playground/` directory with sample files for testing every feature. It contains mixed file types, edge cases, and a curated `best-practices/` subdirectory showcasing rich frontmatter with tags.

```bash
git clone https://github.com/leaf-kit/ls.md.git
cd ls.md
cargo build --release
./target/release/lsmd playground
./target/release/lsmd playground/best-practices
```

### Playground Structure

```
playground/
├── best-practices/          # Curated examples with rich frontmatter & tags
│   ├── api-design.md        # tags: api, rest, design
│   ├── cli-ux-tips.md       # tags: cli, ux, design
│   ├── git-workflow.md      # tags: git, workflow, collaboration
│   ├── markdown-style.md    # tags: markdown, writing, documentation
│   ├── project-kickoff.md   # tags: project, checklist, onboarding
│   ├── rust-error-handling.md # tags: rust, error-handling, patterns
│   ├── debugging-checklist.txt
│   └── quick-reference.txt
├── docs/
│   └── guide.md
├── blog-post.md             # Frontmatter with title + date + tags
├── meeting-notes.md          # Frontmatter with title + date + tags
├── no-frontmatter.md         # H1 heading only (fallback test)
├── broken-yaml.md            # Broken YAML (silent error test)
├── empty.md                  # Empty file (edge case)
├── notes.txt                 # .txt first-line preview
├── long-line.txt             # .txt truncation test (>60 chars)
├── empty.txt                 # Empty .txt (edge case)
├── app.py                    # Non-markdown file
├── config.yaml               # Non-markdown file
└── sample.json               # Non-markdown file
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
    colored tag badges — all inline with the listing.

    Get started with `lsmd` to see the current directory,
    or `lsmd -l` for detailed metadata view.
```

## Commands & Output Examples

All examples below are actual outputs from running `lsmd` against the included `playground/` directory.

### 1. Default Listing

```
% lsmd playground
  /path/to/playground  (2 dirs, 13 files, 7 md, 3 txt)

   best-practices/
   docs/
   app.py
   blog-post.md            Getting Started with Rust · 2026-03-15 ·  rust   programming   tutorial
   broken-yaml.md          Broken YAML Frontmatter Test
   config.yaml
   empty.md
   empty.txt
   long-line.txt           This is a very long first line that should be truncated wit…
   meeting-notes.md        Team Meeting Notes · 2026-04-01 ·  meeting   planning
   no-frontmatter.md       Simple Document
   notes.txt               Quick notes from today's brainstorming session about the ne…
   sample.json
   short.md                Short
   very-long-filename….md  Long Name Test · 2026-04-05 ·  test

  Total: 15 items, 2.4 KB
```

Key behaviors:
- **Header** shows directory path with summary counts (dirs, files, md, txt)
- **Nerd Font icons** per file type — colored by extension (requires [Nerd Fonts](https://www.nerdfonts.com/))
- **Directories** listed first with folder icon in yellow
- **`.md` files** in green with markdown icon and frontmatter summary
- **`.md` without frontmatter** shows H1 heading or first body line as fallback
- **`.md` with broken YAML** falls back to body text
- **`.txt` files** in white with dimmed first-line preview (sanitized, 60 char truncation)
- **Other files** with extension-specific icon and color
- **Footer** shows total items and combined size
- **22-char name column** — long names truncated with `…`, preserving extension

### 2. Long Format (`-l`)

<p align="center">
  <img src="images/lsmd-l.png" alt="lsmd -l screenshot" width="100%" />
</p>

```
% lsmd playground -l
📂 320 B  2026-04-05 02:09  best-practices/
📂  96 B  2026-04-05 01:22  docs/
  152 B  2026-04-05 01:21  app.py
  487 B  2026-04-05 01:21  blog-post.md  Getting Started with Rust · 2026-03-15 ·  rust   programming   tutorial
  191 B  2026-04-05 01:22  broken-yaml.md
   56 B  2026-04-05 01:22  config.yaml
    0 B  2026-04-05 01:21  empty.md
    0 B  2026-04-05 01:21  empty.txt
  180 B  2026-04-05 01:21  long-line.txt  This is a very long first line that should be truncated wit…
  395 B  2026-04-05 01:21  meeting-notes.md  Team Meeting Notes · 2026-04-01 ·  meeting   planning
  187 B  2026-04-05 01:21  no-frontmatter.md  Simple Document
  190 B  2026-04-05 01:21  notes.txt  Quick notes from today's brainstorming session about the ne…
   43 B  2026-04-05 01:21  sample.json
```

Shows file size and modification time alongside metadata.

### 3. Show Hidden Files (`-a`)

```
% lsmd playground -a
📂 .hidden-dir/
📂 best-practices/
📂 docs/
  .hidden-file.md  Hidden Markdown File
  app.py
  blog-post.md  Getting Started with Rust · 2026-03-15 ·  rust   programming   tutorial
  broken-yaml.md
  ...
```

Reveals dotfiles and hidden directories.

### 4. Markdown Only (`-m`)

```
% lsmd playground -m
📂 best-practices/
📂 docs/
  blog-post.md  Getting Started with Rust · 2026-03-15 ·  rust   programming   tutorial
  broken-yaml.md
  empty.md
  empty.txt
  long-line.txt  This is a very long first line that should be truncated wit…
  meeting-notes.md  Team Meeting Notes · 2026-04-01 ·  meeting   planning
  no-frontmatter.md  Simple Document
  notes.txt  Quick notes from today's brainstorming session about the ne…
```

Filters to show only `.md` and `.txt` files (directories are always included).

### 5. Sort Options

**Sort by size:**
```
% lsmd playground -s size
📂 docs/
📂 best-practices/
  empty.txt
  empty.md
  sample.json
  config.yaml
  app.py
  long-line.txt  This is a very long first line that should be truncated wit…
  no-frontmatter.md  Simple Document
  notes.txt  Quick notes from today's brainstorming session about the ne…
  broken-yaml.md
  meeting-notes.md  Team Meeting Notes · 2026-04-01 ·  meeting   planning
  blog-post.md  Getting Started with Rust · 2026-03-15 ·  rust   programming   tutorial
```

**Sort by name, reversed:**
```
% lsmd playground -s name -r
📂 docs/
📂 best-practices/
  sample.json
  notes.txt  Quick notes from today's brainstorming session about the ne…
  no-frontmatter.md  Simple Document
  meeting-notes.md  Team Meeting Notes · 2026-04-01 ·  meeting   planning
  long-line.txt  This is a very long first line that should be truncated wit…
  empty.txt
  empty.md
  config.yaml
  broken-yaml.md
  blog-post.md  Getting Started with Rust · 2026-03-15 ·  rust   programming   tutorial
  app.py
```

### 6. Best Practices — Curated Examples

The `playground/best-practices/` directory contains well-structured Markdown documents with rich frontmatter, demonstrating how lsmd displays titles, dates, and colored tag badges at a glance.

**Default listing:**
```
% lsmd playground/best-practices
  api-design.md  RESTful API Design Principles · 2026-03-15 ·  api   rest   design
  cli-ux-tips.md  CLI UX Design Tips · 2026-04-03 ·  cli   ux   design
  debugging-checklist.txt  Step-by-step debugging checklist for production incidents
  git-workflow.md  Git Workflow Guide · 2026-03-20 ·  git   workflow   collaboration
  markdown-style.md  Markdown Writing Style Guide · 2026-03-10 ·  markdown   writing   documentation
  project-kickoff.md  Project Kickoff Checklist · 2026-04-01 ·  project   checklist   onboarding
  quick-reference.txt  Common terminal shortcuts and commands for daily developmen…
  rust-error-handling.md  Rust Error Handling Patterns · 2026-03-28 ·  rust   error-handling   patterns
```

**Long format:**
```
% lsmd playground/best-practices -l
  744 B  2026-04-05 02:08  api-design.md  RESTful API Design Principles · 2026-03-15 ·  api   rest   design
  773 B  2026-04-05 02:09  cli-ux-tips.md  CLI UX Design Tips · 2026-04-03 ·  cli   ux   design
  403 B  2026-04-05 02:09  debugging-checklist.txt  Step-by-step debugging checklist for production incidents
  697 B  2026-04-05 02:08  git-workflow.md  Git Workflow Guide · 2026-03-20 ·  git   workflow   collaboration
  737 B  2026-04-05 02:09  markdown-style.md  Markdown Writing Style Guide · 2026-03-10 ·  markdown   writing   documentation
  532 B  2026-04-05 02:08  project-kickoff.md  Project Kickoff Checklist · 2026-04-01 ·  project   checklist   onboarding
  381 B  2026-04-05 02:09  quick-reference.txt  Common terminal shortcuts and commands for daily developmen…
  836 B  2026-04-05 02:08  rust-error-handling.md  Rust Error Handling Patterns · 2026-03-28 ·  rust   error-handling   patterns
```

**Markdown only (`-m`):**
```
% lsmd playground/best-practices -m
  api-design.md  RESTful API Design Principles · 2026-03-15 ·  api   rest   design
  cli-ux-tips.md  CLI UX Design Tips · 2026-04-03 ·  cli   ux   design
  debugging-checklist.txt  Step-by-step debugging checklist for production incidents
  git-workflow.md  Git Workflow Guide · 2026-03-20 ·  git   workflow   collaboration
  markdown-style.md  Markdown Writing Style Guide · 2026-03-10 ·  markdown   writing   documentation
  project-kickoff.md  Project Kickoff Checklist · 2026-04-01 ·  project   checklist   onboarding
  quick-reference.txt  Common terminal shortcuts and commands for daily developmen…
  rust-error-handling.md  Rust Error Handling Patterns · 2026-03-28 ·  rust   error-handling   patterns
```

**Sort by modification date (`-s modified`):**
```
% lsmd playground/best-practices -s modified
  project-kickoff.md  Project Kickoff Checklist · 2026-04-01 ·  project   checklist   onboarding
  rust-error-handling.md  Rust Error Handling Patterns · 2026-03-28 ·  rust   error-handling   patterns
  git-workflow.md  Git Workflow Guide · 2026-03-20 ·  git   workflow   collaboration
  api-design.md  RESTful API Design Principles · 2026-03-15 ·  api   rest   design
  cli-ux-tips.md  CLI UX Design Tips · 2026-04-03 ·  cli   ux   design
  markdown-style.md  Markdown Writing Style Guide · 2026-03-10 ·  markdown   writing   documentation
  quick-reference.txt  Common terminal shortcuts and commands for daily developmen…
  debugging-checklist.txt  Step-by-step debugging checklist for production incidents
```

## Options Reference

| Option | Short | Description |
|--------|-------|-------------|
| `--all` | `-a` | Show hidden files (dotfiles) |
| `--long` | `-l` | Long format with size, date, metadata |
| `--no-color` | | Disable ANSI color output |
| `--sort <FIELD>` | `-s` | Sort by: `name`, `size`, `modified`, `type` |
| `--reverse` | `-r` | Reverse sort order |
| `--md-only` | `-m` | Show only `.md` and `.txt` files |

## Content Preview Policy

lsmd extracts a one-line summary from `.md` and `.txt` files. The preview text is **sanitized** — special characters (`*`, `[`, `]`, `` ` ``, `#`, `|`, `{`, `}`, `<`, `>`, etc.) are stripped, keeping only readable text: alphanumeric characters, Korean, Chinese, Japanese, and basic punctuation (`.` `,` `!` `?` `:` `-`). This ensures clean, scannable output.

### `.md` file preview priority

1. **YAML frontmatter** — if `title`, `date`, `tags` fields exist, display them inline with colored tag badges
2. **`# H1` heading** — if no frontmatter, show the first H1 heading text
3. **First content line** — if no frontmatter and no H1, show the first meaningful body text (skipping blank lines, code fences, horizontal rules)
4. **Broken YAML** — if frontmatter exists but fails to parse, fall through to H1 or content fallback (no crash)
5. **Empty file** — file name only

### `.txt` file preview

1. **First meaningful line** — the first non-blank line after sanitization, truncated at 60 characters with ellipsis
2. **Empty file** — file name only

## File Type Handling

| File Type | Behavior |
|-----------|----------|
| `.md` with frontmatter | title · date · colored tag badges |
| `.md` with H1 only | dimmed heading text (sanitized) |
| `.md` with body text only | dimmed first content line (sanitized) |
| `.md` broken YAML | falls back to H1 or body text |
| `.md` empty | file name only |
| `.txt` | dimmed first meaningful line (sanitized, max 60 chars) |
| `.txt` empty | file name only |
| Other files | extension-colored Nerd Font icon, normal name |
| Directories | yellow folder icon, bright blue bold name, always listed first |

## Pipe Integration (`|`)

lsmd auto-disables ANSI colors when output is piped, making it safe for use with `grep`, `awk`, `wc`, `sort`, `sed`, `xargs`, and other Unix tools. All examples below were tested against `playground/best-practices/`.

### Useful Pipe Recipes

**Find files by tag** — search for a specific tag across the listing:

```
% lsmd playground/best-practices | grep "rust"
  rust-error-handling.md  Rust Error Handling Patterns · 2026-03-28 ·  rust   error-handling   patterns
```

**Find files sharing a tag** — find all documents tagged with "design":

```
% lsmd playground/best-practices | grep "design"
  api-design.md  RESTful API Design Principles · 2026-03-15 ·  api   rest   design
  cli-ux-tips.md  CLI UX Design Tips · 2026-04-03 ·  cli   ux   design
```

**Case-insensitive multi-pattern search** — find API or REST related files:

```
% lsmd playground/best-practices | grep -iE "api|rest"
  api-design.md  RESTful API Design Principles · 2026-03-15 ·  api   rest   design
```

**Count markdown files** — quick total with `wc -l`:

```
% lsmd playground/best-practices | grep "\.md" | wc -l
6
```

**Count files with frontmatter** — files that have title/date/tags:

```
% lsmd playground/best-practices | grep -c "·"
6
```

**Extract file names only** — clean list for scripting:

```
% lsmd playground/best-practices | awk '{print $1}'
api-design.md
cli-ux-tips.md
debugging-checklist.txt
git-workflow.md
markdown-style.md
project-kickoff.md
quick-reference.txt
rust-error-handling.md
```

**Extract document titles** — parse titles from frontmatter output:

```
% lsmd playground/best-practices | grep "·" | cut -d'·' -f1 | sed 's/^[[:space:]]*[^ ]* *//'
RESTful API Design Principles
CLI UX Design Tips
Git Workflow Guide
Markdown Writing Style Guide
Project Kickoff Checklist
Rust Error Handling Patterns
```

**Tag frequency analysis** — find the most-used tags across all documents:

```
% lsmd playground/best-practices | grep "·" | sed 's/.*·//' | grep -oE '[a-z][-a-z]*' | sort | uniq -c | sort -rn
   2 design
   1 writing
   1 workflow
   1 ux
   1 rust
   1 rest
   1 project
   1 patterns
   1 onboarding
   1 markdown
   1 git
   1 error-handling
   1 documentation
   1 collaboration
   1 cli
   1 checklist
   1 api
```

**Exclude txt files** — show only markdown results:

```
% lsmd playground/best-practices | grep -v "\.txt"
  api-design.md  RESTful API Design Principles · 2026-03-15 ·  api   rest   design
  cli-ux-tips.md  CLI UX Design Tips · 2026-04-03 ·  cli   ux   design
  git-workflow.md  Git Workflow Guide · 2026-03-20 ·  git   workflow   collaboration
  markdown-style.md  Markdown Writing Style Guide · 2026-03-10 ·  markdown   writing   documentation
  project-kickoff.md  Project Kickoff Checklist · 2026-04-01 ·  project   checklist   onboarding
  rust-error-handling.md  Rust Error Handling Patterns · 2026-03-28 ·  rust   error-handling   patterns
```

**Sort long output by file size** — combine `-l` with `sort`:

```
% lsmd playground/best-practices -l | sort -n -k1
  381 B  2026-04-05 02:09  quick-reference.txt  Common terminal shortcuts and commands for daily developmen…
  403 B  2026-04-05 02:09  debugging-checklist.txt  Step-by-step debugging checklist for production incidents
  532 B  2026-04-05 02:08  project-kickoff.md  Project Kickoff Checklist · 2026-04-01 ·  project   checklist   onboarding
  697 B  2026-04-05 02:08  git-workflow.md  Git Workflow Guide · 2026-03-20 ·  git   workflow   collaboration
  737 B  2026-04-05 02:09  markdown-style.md  Markdown Writing Style Guide · 2026-03-10 ·  markdown   writing   documentation
  744 B  2026-04-05 02:08  api-design.md  RESTful API Design Principles · 2026-03-15 ·  api   rest   design
  773 B  2026-04-05 02:09  cli-ux-tips.md  CLI UX Design Tips · 2026-04-03 ·  cli   ux   design
  836 B  2026-04-05 02:08  rust-error-handling.md  Rust Error Handling Patterns · 2026-03-28 ·  rust   error-handling   patterns
```

**Line count per file** — combine with `xargs wc`:

```
% lsmd playground/best-practices | awk '{print $1}' | sed 's|^|playground/best-practices/|' | xargs wc -l
      48 playground/best-practices/api-design.md
      31 playground/best-practices/cli-ux-tips.md
       9 playground/best-practices/debugging-checklist.txt
      36 playground/best-practices/git-workflow.md
      41 playground/best-practices/markdown-style.md
      30 playground/best-practices/project-kickoff.md
      11 playground/best-practices/quick-reference.txt
      42 playground/best-practices/rust-error-handling.md
     248 total
```

### Pipe Usage Notes

> **Important caveats when using lsmd with pipes:**

1. **Colored output is auto-disabled in pipes.** The `colored` library auto-detects non-TTY output and disables ANSI codes, so pipe output is clean plain text.

2. **Use `--no-color` for explicit control.** When scripting, add `--no-color` to guarantee plain text regardless of environment.

3. **File names are the first field.** Use `awk '{print $1}'` to extract file names from default output.

4. **Frontmatter fields are separated by `·`.** Use `cut -d'·'` to split title, date, and tag sections.

5. **Encoding: lsmd outputs UTF-8.** If piping to tools that expect ASCII, use `LC_ALL=en_US.UTF-8` if needed.

## Edge Cases

lsmd handles these scenarios gracefully:

- **Empty files** — displayed with name only, no crash
- **Broken YAML frontmatter** — silently ignored
- **Very long file names** — terminal wrapping handles overflow
- **Non-existent paths** — clear error message
- **Permission errors** — skipped silently
- **Non-directory paths** — clear error message

## Related Projects

lsmd complements existing terminal tools in the Markdown ecosystem:

- [**gmd**](https://github.com/leaf-kit/g.md) — Grep Markdown. Structure-aware search across Markdown documents.
- [**lsd**](https://github.com/lsd-rs/lsd) — LSDeluxe. A modern `ls` with icons and colors (file-type aware, not content-aware).
- [**exa/eza**](https://github.com/eza-community/eza) — A modern replacement for `ls` (metadata-focused, not Markdown-aware).

lsmd fills the gap: it is the only `ls`-style tool that **reads inside** `.md` and `.txt` files to surface structured metadata inline.

## Feedback & Contributing

Contributions, issues, and feature requests are welcome. If lsmd is useful in your workflow, consider starring the repo — it helps others discover the project.

[github.com/leaf-kit/ls.md/issues](https://github.com/leaf-kit/ls.md/issues)

## License

[MIT](LICENSE)
