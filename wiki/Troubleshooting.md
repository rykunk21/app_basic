# Troubleshooting

Fix common issues with the TDD Kanban automation.

## Issue: Labels not updating

**Symptom:** Draft PR created but issue still shows `backlog` label.

**Check:** Is the GitHub App installed?

1. Go to: `https://github.com/YOURNAME/YOURREPO/settings/installations`
2. Look for "autoaxiom"
3. If missing: <https://github.com/apps/autoaxiom> → Install

**Check:** Is the workflow file present?

```bash
ls .github/workflows/kanban-automation.yml
```

If missing, the automation won't run. Copy from template.

**Check:** Does issue have the label?

```bash
curl -H "Authorization: token YOUR_TOKEN" \
  https://api.github.com/repos/YOURNAME/YOURREPO/issues/ISSUE_NUMBER
```

Look for `"labels": [{"name": "red"}]` in response.

## Issue: 403 Forbidden

**Symptom:** Workflow logs show `403 Resource not accessible`.

**Cause:** Token expired or App lacks permissions.

**Fix:**

1. Re-install the GitHub App
2. Grant "Issues" write permission
3. Grant "Pull requests" read permission

## Issue: Branch protection blocks push

**Symptom:** Workflow fails with `Changes must be made through a pull request`.

**This is correct behavior.** The automation only updates labels on issues, not code.

If you're seeing this on code pushes, you're trying to:
- Push directly to `main` ❌
- Push through a branch without PR ❌

**Fix:** Use the TDD flow:
1. Branch → push → open **draft** PR
2. Work on code
3. Mark ready → green
4. Merge

## Issue: Multi-repo dashboard not showing repos

**Symptom:** Dashboard loads but shows no issues.

**Check:** Is App installed on that repo?

1. Go to: <https://github.com/apps/autoaxiom>
2. Click "Configure"
3. Add repository

**Check:** Are there open issues with labels?

```bash
curl -H "Authorization: Bearer TOKEN" \
  "https://api.github.com/repos/OWNER/REPO/issues?state=open&labels=red"
```

## Issue: Token errors locally

**Symptom:** `scripts/get-github-token.js` fails.

**Check:** Private key file exists:

```bash
ls ~/.axiom/keys/autoaxiom.*.private-key.pem
```

If missing, you need the private key from GitHub App settings.

**Check:** App ID and Installation ID are current:

```bash
# Look in scripts/get-github-token.js
const APP_ID = "3196906";          # Must match GitHub
const INSTALLATION_ID = "119295792"; # Must match your install
```

To find your installation ID:

```bash
curl -H "Authorization: Bearer JWT_TOKEN" \
  -H "Accept: application/vnd.github+json" \
  https://api.github.com/app/installations
```

## Debug: Check workflow logs

1. Go to repo → Actions tab
2. Click "Kanban Automation" workflow
3. Click the failed run
4. Expand steps to see error

## Debug: Test manually

```bash
# Get token
TOKEN=$(./scripts/get-github-token.js)

# Test label add
curl -X POST \
  -H "Authorization: Bearer $TOKEN" \
  -H "Accept: application/vnd.github+json" \
  https://api.github.com/repos/OWNER/REPO/issues/1/labels \
  -d '{"labels":["test"]}'
```

## Still broken?

Open an issue with:
- Repository name
- Exact error message
- Link to failed workflow run