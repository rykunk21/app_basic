# Architecture

System design for the TDD Kanban automation.

## Components

```
┌─────────────────────────────────────────────────────────────┐
│                     GitHub Ecosystem                         │
│                                                              │
│  ┌──────────────┐    HTTP/Github API    ┌──────────────┐  │
│  │  Your Repo   │ ◀───────────────────▶ │  autoaxiom   │  │
│  │              │  (App installation)   │  GitHub App  │  │
│  │  Issues      │                        └──────┬───────┘  │
│  │  PRs         │                               │          │
│  │  Labels      │                               │          │
│  └──────────────┘                               │          │
│         ▲                                      │          │
│         │                                      │          │
│         └──────────────────────────────────────┘          │
│                    Workflow triggers                      │
│                                                           │
└───────────────────────────────────────────────────────────┘
                              │
                              │ REST API
                              ▼
┌───────────────────────────────────────────────────────────┐
│                Kanban Dashboard (Optional)                │
│                                                           │
│  ┌────────────┐  ┌────────────┐  ┌────────────┐          │
│  │   Rocket   │  │    Yew     │  │    Rust    │          │
│  │  Backend   │  │  Frontend  │  │   Client   │          │
│  │            │  │   (WASM)   │  │            │          │
│  └─────┬──────┘  └─────┬──────┘  └────────────┘          │
│        │               │                                  │
│        └───────────────┘                                  │
│              │                                            │
│              ▼                                            │
│        Mobile/Web UI                                     │
│        (PWA-capable)                                      │
│                                                           │
└───────────────────────────────────────────────────────────┘
```

## Repository Template

`app_basic` is the source template. When used:

1. GitHub copies files
2. Labels are **not** copied (must be created manually or via workflow)
3. GitHub App must be installed separately

## GitHub App

**autoaxiom[bot]** is a custom GitHub App with:

- **ID:** 3196906
- **Permissions:**
  - `issues`: write (add/remove labels)
  - `pull_requests`: write (read PR state)
  - `contents`: read (workflow access)

**Authentication flow:**
```
1. JWT generated from private key (RS256)
2. POST /app/installations/{id}/access_tokens
3. Returns 1-hour installation token
4. Token cached and refreshed before expiry
```

## Workflow Automation

File: `.github/workflows/kanban-automation.yml`

**Triggers:**
- `pull_request.opened` - Check if draft
- `pull_request.converted_to_draft` - Label red
- `pull_request.ready_for_review` - Label green
- `pull_request.closed` (merged) - Label done

**Actions:**
```rust
// Simplified logic
match event {
    DraftPR => {
        remove_label("backlog");
        add_label("red");
    }
    ReadyForReview => {
        remove_label("red");
        add_label("green");
    }
    Merged => {
        remove_label("refactor");
        add_label("done");
        close_issue();
    }
}
```

## Label System

**Why 5 labels?**

| Label | Color | Purpose | API |
|-------|-------|---------|-----|
| `backlog` | `#8B8B8B` | Not started | `POST /labels` |
| `red` | `#FF0000` | Failing test exists | `POST /labels` + `DELETE /labels/backlog` |
| `green` | `#00FF00` | Test passes | `POST /labels` + `DELETE /labels/red` |
| `refactor` | `#0000FF` | Cleanup phase | `POST /labels` + `DELETE /labels/green` |
| `done` | `#800080` | Complete | `POST /labels` |

**Constraints:**
- Lowercase only (GitHub label convention)
- Issue can have multiple labels
- Leftmost matching label determines column

## Dashboard (Separate Service)

**kanban-dashboard** aggregates across repos:

```rust
// Concurrent fetch
let repos = ["app_basic", "app_frontend", "app_api"];
let futures = repos.map(|r| fetch_issues(r));
let results = join_all(futures).await;

// Group by label
let board = group_by_column(results);
```

**Architecture:**
- **Backend:** Rocket with REST API
- **Frontend:** Yew compiled to WASM
- **Storage:** None (ephemeral, API-driven)
- **Caching:** In-memory DashMap with TTL

## Security

**Branch protection:**
- Direct pushes to `main` blocked
- All code changes require PR
- App cannot bypass (distinct identity)

**Token handling:**
- Private key never leaves GitHub
- Installation tokens valid 1 hour
- Tokens not logged or exposed

## Performance

**API limits:**
- GitHub REST: 5,000 requests/hour
- Label operations: 2 requests per transition
- Dashboard refresh: 1 request per repo

**Optimization:**
- Token caching (refresh at 55 min)
- Dashboard caching (60 second TTL)
- Concurrent repo fetching

## Future Extensions

**Planned:**
- LLM summaries of blockers
- Burndown charts
- Slack notifications
- Multi-user dashboards