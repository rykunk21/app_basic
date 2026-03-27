# TDD Kanban Template

A GitHub template for **red-green-refactor development workflows** with automated Kanban board management.

## What This Does

Turns your GitHub repo into a TDD machine:

1. **Create a draft PR** → Issue labeled `red` (write failing test)
2. **Mark PR ready** → Issue labeled `green` (make it pass)
3. **Merge the PR** → Issue labeled `done` (refactor complete)

No manual board updates. The workflow handles labels automatically.

## Quick Start

```bash
# 1. Use this template
git clone https://github.com/rykunk21/app_basic.git my-project

# 2. Install the GitHub App
# Go to: https://github.com/apps/autoaxiom
# Click "Install" → Select your repo

# 3. Push a branch and open a draft PR
git checkout -b feature/my-test
git push origin feature/my-test
# Open PR as draft

# 4. Watch the label change automatically
```

## Architecture

```
Your Repo
├── Issue #12 (backlog)
└── Draft PR #13 ──→ autoaxiom labels issue "red"
    ↓
Mark ready ───────→ autoaxiom labels issue "green"  
    ↓
Merge ────────────→ autoaxiom labels issue "done"
```

## Key Concepts

| Label | Meaning | Phase |
|-------|---------|-------|
| `backlog` | Not started | Gray |
| `red` | Write failing test | 🔴 |
| `green` | Make test pass | 🟢 |
| `refactor` | Clean up | 🔵 |
| `done` | Complete | 🟣 |

## Mobile Dashboard

Track all repos on your phone: [kanban-dashboard](https://github.com/rykunk21/kanban-dashboard)

## Documentation

- [Quick Start](Quick-Start.md) - Get running in 2 minutes
- [The TDD Flow](The-TDD-Flow.md) - How the automation works
- [Why Labels Not Projects?](Why-Labels.md) - The technical reason
- [Troubleshooting](Troubleshooting.md) - Fix common issues
- [Architecture](Architecture.md) - System design

## Support

Problems? Check [Troubleshooting](Troubleshooting.md) or open an issue.