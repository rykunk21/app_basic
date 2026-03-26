# app_basic

Rocket + Yew + SurrealDB full-stack template with automated TDD kanban gate enforcement.

## Quick Start

```bash
docker compose up
```

---

## Kanban Board

The project board lives at:
```
https://github.com/users/rykunk21/projects/PROJECT_NUMBER
```

Open it directly from the terminal:
```bash
gh project view PROJECT_NUMBER --owner "@me" --web
```

---

## Managing Issues

### View issues
```bash
# All open issues
gh issue list --repo rykunk21/app_basic

# Filter by label
gh issue list --repo rykunk21/app_basic --label "red"
gh issue list --repo rykunk21/app_basic --label "story"
gh issue list --repo rykunk21/app_basic --label "epic"

# Filter by milestone
gh issue list --repo rykunk21/app_basic --milestone "M1: Project Infrastructure"

# View a specific issue
gh issue view ISSUE_NUMBER --repo rykunk21/app_basic

# View in browser
gh issue view ISSUE_NUMBER --repo rykunk21/app_basic --web
```

### Create issues
```bash
# Create a story
gh issue create \
  --repo rykunk21/app_basic \
  --title "REQ-###: description" \
  --label "story,backlog" \
  --milestone "M1: Project Infrastructure"

# Create a bug
gh issue create \
  --repo rykunk21/app_basic \
  --title "short description" \
  --label "bug,triage"
```

### Close an issue
```bash
gh issue close ISSUE_NUMBER --repo rykunk21/app_basic
```

---

## Moving Issues Through the Kanban

Each kanban transition is a label swap.

### Backlog → 🔴 Red
```bash
gh issue edit ISSUE_NUMBER --repo rykunk21/app_basic \
  --add-label "red" \
  --remove-label "backlog"
```

### 🔴 Red → 🟢 Green
```bash
gh issue edit ISSUE_NUMBER --repo rykunk21/app_basic \
  --add-label "green" \
  --remove-label "red"
```

### 🟢 Green → 🔵 Refactor
```bash
gh issue edit ISSUE_NUMBER --repo rykunk21/app_basic \
  --add-label "refactor" \
  --remove-label "green"
```

### 🔵 Refactor → ✅ Done
```bash
gh issue edit ISSUE_NUMBER --repo rykunk21/app_basic \
  --add-label "done" \
  --remove-label "refactor"
gh issue close ISSUE_NUMBER --repo rykunk21/app_basic
```

---

## Managing Pull Requests

### Create a PR for a story
```bash
git checkout -b feat/REQ-###-short-description
# ... do work ...
gh pr create \
  --repo rykunk21/app_basic \
  --title "feat(REQ-###): short description" \
  --body "Closes #ISSUE_NUMBER"
```

### View current PR status and CI checks
```bash
gh pr view --repo rykunk21/app_basic
```

### Merge when gate passes
```bash
gh pr merge --repo rykunk21/app_basic --squash --delete-branch
```

---

## Milestones

### View milestone progress
```bash
gh api repos/rykunk21/app_basic/milestones \
  --jq '.[] | "\(.title): \(.open_issues) open, \(.closed_issues) closed"'
```

### View all issues in a milestone
```bash
gh issue list --repo rykunk21/app_basic \
  --milestone "M1: Project Infrastructure"
```

---

## Labels

### View all labels
```bash
gh label list --repo rykunk21/app_basic
```

### Kanban state labels
| Label      | Meaning                        |
|------------|--------------------------------|
| `backlog`  | Not yet started                |
| `red`      | Test written, currently failing|
| `green`    | Tests passing                  |
| `refactor` | Cleaning up                    |
| `done`     | Complete, issue closed         |

### Issue type labels
| Label       | Meaning                        |
|-------------|--------------------------------|
| `epic`      | Tracking issue, groups stories |
| `story`     | One REQ-### unit of work       |
| `bug`       | Unplanned defect               |
| `triage`    | Bug awaiting assessment        |

---

## Documentation

### Build the requirements book
```bash
./scripts/build-docs.sh
```

### Live preview
```bash
mdbook serve doc/
```

### View a specific requirement
```bash
cat doc/requirements/REQ-001.md
```

---

## Development Workflow

```bash
# 1. Pick the next story from backlog, move it to Red
gh issue edit ISSUE_NUMBER --repo rykunk21/app_basic \
  --add-label "red" --remove-label "backlog"

# 2. Create a branch
git checkout main && git pull
git checkout -b feat/REQ-###-short-description

# 3. Create the requirement doc
touch doc/requirements/REQ-###.md

# 4. Write the failing test, commit
git add . && git commit -m "test(REQ-###): add failing test"
git push -u origin feat/REQ-###-short-description

# 5. Open a PR
gh pr create --title "feat(REQ-###): description" --body "Closes #ISSUE_NUMBER"

# 6. Implement until tests pass, push
git add . && git commit -m "feat(REQ-###): implement"
git push

# 7. Move to Green, merge
gh issue edit ISSUE_NUMBER --repo rykunk21/app_basic \
  --add-label "green" --remove-label "red"
gh pr merge --squash --delete-branch
```
