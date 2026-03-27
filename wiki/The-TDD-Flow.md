# The TDD Flow

How the automated red-green-refactor workflow works.

## The Cycle

```
┌─────────────┐     Create draft PR      ┌─────────┐
│   Backlog   │ ──────────────────────────▶ │   Red   │
│   (gray)    │                           │  🔴     │
└─────────────┘                           └────┬────┘
                                              │
                                              │ Write code
                                              │ to pass
                                              ▼
┌─────────────┐     Merge PR               ┌─────────┐
│    Done     │ ◀────────────────────────── │  Green  │
│   (purple)  │                           │  🟢     │
└─────────────┘                           └────┬────┘
       ▲                                       │
       │                                       │ Refactor
       │                                       │
       └───────────────────────────────────────┘
```

## Automated Transitions

### Backlog → Red
**Trigger:** Draft PR opened referencing issue

**What happens:**
```yaml
on:
  pull_request:
    types: [opened]
    
if: github.event.pull_request.draft == true

steps:
  - finds issue linked in PR description
  - removes "backlog" label
  - adds "red" label
```

### Red → Green  
**Trigger:** PR marked "Ready for review"

**What happens:**
```yaml
on:
  pull_request:
    types: [ready_for_review]
    
steps:
  - removes "red" label
  - adds "green" label
```

### Green → Refactor
**Optional trigger:**
Can be done manually or by commenting `!refactor` on PR

### Refactor → Done
**Trigger:** PR merged

**What happens:**
```yaml
on:
  pull_request:
    types: [closed]
    
if: github.event.pull_request.merged == true

steps:
  - removes "refactor" label (if present)
  - adds "done" label
  - closes the issue
```

## Manual Labels

You can also move issues manually:

```bash
# Add label
curl -X POST \
  -H "Authorization: token TOKEN" \
  -H "Accept: application/vnd.github+json" \
  https://api.github.com/repos/OWNER/REPO/issues/ISSUE_NUMBER/labels \
  -d '{"labels":["red"]}'

# Remove label  
curl -X DELETE \
  -H "Authorization: token TOKEN" \
  https://api.github.com/repos/OWNER/REPO/issues/ISSUE_NUMBER/labels/backlog
```

Or use the [Kanban Dashboard](https://github.com/rykunk21/kanban-dashboard) drag-and-drop.

## Label Meanings

| Label | Hex Color | State |
|-------|-----------|-------|
| `backlog` | `#8B8B8B` | Not started |
| `red` | `#FF0000` | Failing test written |
| `green` | `#00FF00` | Test passes |
| `refactor` | `#0000FF` | Cleaning up |
| `done` | `#800080` | Complete |

Colors are set in `.github/workflows/kanban-automation.yml`