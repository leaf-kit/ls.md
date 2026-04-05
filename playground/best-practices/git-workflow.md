---
title: Git Workflow Guide
date: 2026-03-20
tags:
  - git
  - workflow
  - collaboration
---

# Git Workflow Guide

## Branch Naming

| Prefix | Purpose | Example |
|--------|---------|---------|
| `feat/` | New feature | `feat/user-auth` |
| `fix/` | Bug fix | `fix/login-crash` |
| `docs/` | Documentation | `docs/api-reference` |
| `refactor/` | Code refactor | `refactor/db-layer` |

## Commit Message Convention

```
<type>: <short summary>

<optional body>
```

Types: `feat`, `fix`, `docs`, `refactor`, `test`, `chore`

## Code Review Checklist

- [ ] Does it solve the stated problem?
- [ ] Are edge cases handled?
- [ ] Are tests included?
- [ ] Is the code readable without comments?
