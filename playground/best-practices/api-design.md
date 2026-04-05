---
title: RESTful API Design Principles
date: 2026-03-15
tags:
  - api
  - rest
  - design
---

# RESTful API Design Principles

## URL Structure

- Use nouns, not verbs: `/users` not `/getUsers`
- Use plural: `/users/123` not `/user/123`
- Nest for relationships: `/users/123/orders`

## HTTP Methods

| Method | Action | Idempotent |
|--------|--------|------------|
| GET | Read | Yes |
| POST | Create | No |
| PUT | Replace | Yes |
| PATCH | Update | No |
| DELETE | Remove | Yes |

## Status Codes

- `200` Success
- `201` Created
- `400` Bad Request
- `401` Unauthorized
- `404` Not Found
- `500` Internal Server Error

## Pagination

```json
{
  "data": [...],
  "meta": {
    "page": 1,
    "per_page": 20,
    "total": 142
  }
}
```
