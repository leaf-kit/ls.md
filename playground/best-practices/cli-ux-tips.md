---
title: CLI UX Design Tips
date: 2026-04-03
tags:
  - cli
  - ux
  - design
---

# CLI UX Design Tips

## Principles

1. **Sensible defaults** — zero config for the common case
2. **Progressive disclosure** — simple first, `--verbose` later
3. **Fail loudly** — clear error messages with actionable hints
4. **Respect the terminal** — detect width, color support, pipe mode

## Output Guidelines

- Use color sparingly; always support `--no-color`
- Align columns for scannable output
- Send data to stdout, messages to stderr
- Auto-disable color when piped

## Flag Design

- Short flags for frequent use: `-l`, `-a`, `-r`
- Long flags for clarity: `--recursive`, `--dry-run`
- Boolean flags should default to false
- Provide `--help` with real-world examples
