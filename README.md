# Red-Green Template

Rocket + Yew + SurrealDB full-stack template with **Axiom Agent** — autonomous TDD Kanban automation.

---

## Overview

This is a **continuously running agent system**. Axiom works 24/7 to:
- Pick issues from backlog
- Plan and validate requirements
- Write failing tests (Red phase)
- Implement until tests pass (Green phase)
- Refactor and open PRs

You act as **project manager** via the GitHub board. Add issues to backlog. Axiom handles the rest.

---

## Architecture

```
┌─────────────────┐     ┌──────────────┐     ┌─────────────┐
│   GitHub Board  │────▶│ Webhook      │────▶│ Axiom Agent │
│   (Kanban)      │     │ Server       │     │ (Daemon)    │
└─────────────────┘     └──────────────┘     └─────────────┘
        ▲                                            │
        │                                            │
        └────────────────────────────────────────────┘
                    (gh CLI commands)
```

### Components

| Component | Purpose | Location |
|-----------|---------|----------|
| **Axiom Agent** | Main decision loop, manages Kanban phases | Higher-order system |
| **Webhook Server** | Receives GitHub events (PR merge, CI, issues) | Higher-order system |
| **Req Validator** | Validates requirement expressivity | Higher-order system |
| **Test Fingerprinter** | Hashes tests to detect Green→Refactor tampering | Higher-order system |

---

## Quick Start

```bash
docker compose up
```

---

## Setup

### 1. Deploy Webhook Server (Railway)

```bash
cd webhook-server
railway login
railway init --name axiom-webhooks
railway up
```

**Environment variables** (Railway dashboard):
- `WEBHOOK_SECRET` — Generate with `openssl rand -hex 32`
- `OPENCLAW_GATEWAY_URL` — Your OpenClaw gateway endpoint
- `OPENCLAW_TOKEN` — Gateway auth token

**Note the webhook URL:** `https://axiom-webhooks.up.railway.app/webhook/github`

### 2. Configure GitHub App

Settings → Developer settings → GitHub Apps → New:
- **Webhook URL:** Your Railway URL + `/webhook/github`
- **Webhook secret:** Same as `WEBHOOK_SECRET`
- **Permissions:**
  - Issues: Read & Write
  - Pull requests: Read & Write
  - Actions: Read (for CI status)
  - Contents: Read (for code inspection)
- **Subscribe to events:** Issues, Pull request, Push, Workflow run

**Generate private key**, download `.pem` file.

### 3. Install GitHub App

Install the app on your repository:
```
https://github.com/settings/apps/YOUR_APP/installations
```

### 4. Deploy Agent Daemon

```bash
# Copy service file
sudo cp systemd/axiom-agent.service /etc/systemd/system/
sudo systemctl daemon-reload

# Set environment
sudo systemctl edit axiom-agent
# Add:
# [Service]
# Environment="REPO=rykunk21/app_basic"
# Environment="PROJECT_NUMBER=5"
# Environment="OPENCLAW_GATEWAY_URL=..."

sudo systemctl enable axiom-agent
sudo systemctl start axiom-agent
sudo systemctl status axiom-agent
```

### 5. Verify

```bash
# Check webhook health
curl https://axiom-webhooks.up.railway.app/health

# Check agent health  
curl http://localhost:8080/health

# View logs
sudo journalctl -u axiom-agent -f
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

## Gating System (Agent-Driven)

Axiom manages all transitions automatically based on validation gates.

### Column Flow

| Column | Label | Entry Gate | Exit Gate | Agent Action |
|--------|-------|------------|-----------|--------------|
| **Backlog** | `backlog` | Issue created | Requirement validated | Validate, promote to Red |
| **Red** | `red` | Requirement passes validator | Tests written & failing | Write failing test |
| **Green** | `green` | Tests failing | Tests passing + fingerprint match | Implement code |
| **Refactor** | `refactor` | Tests passing | PR opened & CI passes | Open PR, wait for merge |
| **Done** | `done` | PR merged | — | Close issue |

### Gate Details

**Backlog → Red**
- Requirement validator checks:
  - Has acceptance criteria (Given-When-Then)
  - Defines boundaries (edge cases, errors)
  - Is measurable (latencies, throughputs)
  - Is atomic (one requirement per issue)
  - Is testable (no vague terms)
- Failed validation = issue commented with feedback
- Passed = auto-promote to Red

**Red → Green**
- Agent generates REQ-###.md from issue
- Writes failing test matching acceptance criteria
- Runs `cargo test` to confirm failure
- Auto-promotes to Green on test written

**Green → Refactor**
- Agent implements code to pass tests
- Validates tests haven't changed (fingerprint)
- On tampering detected: revert to Red
- On success: promote to Refactor

**Refactor → Done**
- Agent opens PR with squash merge
- Monitors PR via webhooks
- On merge webhook: mark Done, close issue

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

## Requirement Validator

Modular validation system (resides in higher-order system).

### Current Checks

| Check | Looks For | Fail Message |
|-------|-----------|--------------|
| Acceptance Criteria | Given-When-Then, checkboxes, user story | Add acceptance criteria |
| Boundaries | Edge cases, errors, limits | Describe error conditions |
| Measurability | Numbers, units, thresholds | Add specific metrics |
| Atomicity | Length, multiple asks | Split into smaller requirements |
| Testability | Observable terms | Replace vague words |

### Extending Validators

The validator module supports custom checks — add domain-specific validation as needed.

---

## Labels

### View all labels
```bash
gh label list --repo rykunk21/app_basic
```

### Kanban State Labels

| Label      | Meaning                        |
|------------|--------------------------------|
| `backlog`  | Awaiting validator             |
| `red`      | Test written, currently failing|
| `green`    | Tests passing                  |
| `refactor` | Cleaning up                    |
| `done`     | Complete, issue closed         |

### Issue Type Labels

| Label       | Meaning                        |
|-------------|--------------------------------|
| `epic`      | Tracking issue, groups stories |
| `story`     | One REQ-### unit of work       |
| `bug`       | Unplanned defect               |
| `triage`    | Bug awaiting assessment        |
| `qa-generated` | From QA agent           |

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

## QA Agent (Optional)

Run a separate agent instance for continuous testing:

```bash
# Start QA daemon
OPENCLAW_SESSION=qa-agent node qa-daemon.js
```

- Fuzz tests APIs
- Tries to break boundaries
- Adds `bug,triage,qa-generated` issues to backlog
- Independent — no coordination with main agent

---

## Project Manager Usage

Add to backlog via GitHub web or CLI:

```bash
gh issue create \
  --title "REQ-004: User login with JWT" \
  --body "Given a user\nWhen they authenticate\nThen they receive a JWT\n\nEdge cases:\n- Invalid credentials\n- Expired tokens\n\nAcceptance:\n- < 100ms response\n- 99.9% availability" \
  --label "story,backlog"
```

Axiom picks it up automatically. Watch it move through columns.

---

## API Reference

### Webhook Endpoints

| Endpoint | Method | Purpose |
|----------|--------|---------|
| `/webhook/github` | POST | Receive GitHub events |
| `/health` | GET | Service status |

### Agent Daemon Endpoints (localhost)

| Endpoint | Method | Purpose |
|----------|--------|---------|
| `/webhook` | POST | Receive forwarded events |
| `/health` | GET | Current phase, issue |

---

## Troubleshooting

**Webhook not firing**
- Check GitHub App → Advanced → Recent Deliveries
- Verify secret matches
- Check Railway logs: `railway logs`

**Agent stuck in phase**
- Check `sudo journalctl -u axiom-agent -f`
- Review state file: `cat /tmp/axiom-agent-state.json`
- Manual reset: `sudo systemctl restart axiom-agent`

**Requirement validation too strict**
- Adjust validator configuration in higher-order system
- Disable checks or adjust thresholds as needed

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

---

## Summary

**What this creates:** A self-driving development pipeline where you manage requirements and Axiom executes TDD cycles 24/7.

**Your role:** Product owner — write clear backlog items, review PRs.

**Axiom's role:** Autonomous developer — validate, test, implement, ship.

**Expansion:** Add QA agent for adversarial testing, extend validators for domain-specific rules.

**Implementation:** The agent, webhook server, and validator live in a higher-order system. This README serves as the template specification.
