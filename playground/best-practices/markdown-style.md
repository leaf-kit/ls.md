---
title: Markdown Writing Style Guide
date: 2026-03-10
tags:
  - markdown
  - writing
  - documentation
---

# Markdown Writing Style Guide

## Structure

- One H1 per document (the title)
- Use H2 for major sections, H3 for subsections
- Keep paragraphs short (3-5 sentences)
- Use lists for scannable content

## Formatting

- **Bold** for key terms on first mention
- `code` for commands, file paths, and identifiers
- > Blockquotes for callouts and important notes

## Frontmatter

Always include at minimum:

```yaml
---
title: Document Title
date: 2026-01-01
tags:
  - topic
---
```

## Links

- Use relative paths for internal links: `[Guide](./guide.md)`
- Add descriptive text: `[API docs](api.md)` not `[click here](api.md)`
