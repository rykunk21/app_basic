# Why Labels, Not Projects?

The honest answer: **GitHub Projects v2 blocks GitHub Apps.**

## The Problem

We tried Projects v2 first. Here's what happened:

```graphql
# This query works with your token
query {
  user(login: "rykunk21") {
    projectV2(number: 8) {
      title
    }
  }
}
# → Returns project data ✓

# This mutation fails for GitHub Apps
mutation {
  addProjectV2ItemById(input: {projectId: "...", contentId: "..."}) {
    item { id }
  }
}
# → "Resource not accessible by integration" ✗
```

**Root cause:** GitHub Projects v2 has different permissions than the REST API. Apps can read but not write user-level projects.

## Solutions We Considered

| Approach | Pros | Cons |
|----------|------|------|
| Projects v2 API | Native Kanban view | ❌ App can't write |
| Personal Access Token | Full access | ❌ Bypasses branch protection, acts as user |
| Organization projects | App can write | ❌ Costs $4/user/month |
| **Labels** ✅ | Works everywhere, free | Syncs to all views |

## Why Labels Won

**1. Universal support**
Every GitHub issue has labels. They work in:
- Issue lists
- Project boards (as view filter)
- API responses
- GitHub mobile app
- Third-party tools

**2. App-compatible**
```bash
# This works with GitHub App tokens
curl -X DELETE \
  -H "Authorization: Bearer ${{ steps.token.outputs.token }}" \
  https://api.github.com/repos/owner/repo/issues/1/labels/red

curl -X POST \
  -H "Authorization: Bearer ${{ steps.token.outputs.token }}" \
  https://api.github.com/repos/owner/repo/issues/1/labels \
  -d '{"labels":["green"]}'
```

**3. Branch protection preserved**
Using a GitHub App means:
- App has distinct identity (not you)
- Branch protection rules apply
- All changes go through PRs
- Audit trail shows "autoaxiom[bot]" not "you"

**4. Visual parity**
You can still see a Kanban board:

1. Go to GitHub → Projects
2. Create project from template
3. Add Status field matching labels
4. View as board

The labels power the board. Same experience, different implementation.

## The Trade-offs

**What we lose:**
- Built-in Kanban drag-and-drop on GitHub web
- Automatic "Done" column archiving

**What we gain:**
- ✅ Free
- ✅ App-automated
- ✅ Works across repos
- ✅ Branch protection intact
- ✅ Custom dashboard ([kanban-dashboard](https://github.com/rykunk21/kanban-dashboard))

## Bottom Line

Labels are the source of truth. The dashboard is the view. This keeps the workflow:
- Automated by the GitHub App
- Visible on mobile
- Free forever