# Quick Start

Get the TDD Kanban automation running in 2 minutes.

## Prerequisites

- GitHub account
- A repo created from this template

## Step 1: Install the GitHub App

1. Go to: <https://github.com/apps/autoaxiom>
2. Click **"Install"**
3. Select your repo (or "All repositories")
4. Click **"Install"**

The app now has permission to manage labels on your issues.

## Step 2: Create Your First TDD Issue

```bash
# Clone your repo
git clone https://github.com/YOURNAME/YOURREPO.git
cd YOURREPO

# Create a feature branch
git checkout -b feature/auth-tests

# Write your failing test
echo "# Failing auth test" > test.txt
git add .
git commit -m "Add failing auth test [TEST-001]"
git push origin feature/auth-tests
```

## Step 3: Open Draft PR

1. Go to GitHub → Pull requests → New pull request
2. Select your branch
3. **Important:** Check "Create draft pull request"
4. Add title: `TEST-001: Auth tests`
5. In description, write `Closes #X` (where X is your issue number)
6. Click **"Create draft PR"**

## Step 4: Watch the Magic

Within seconds, the issue will be labeled `red` automatically.

## Step 5: Green Phase

1. Write code to make tests pass
2. Commit and push
3. Click **"Ready for review"** on the PR
4. Issue automatically labeled `green`

## Step 6: Done

1. Get review approval
2. Merge the PR
3. Issue automatically labeled `done`

That's it. You've completed one TDD cycle.

## Next Steps

- Learn [why this uses labels](Why-Labels.md) instead of GitHub Projects
- Set up the [mobile dashboard](https://github.com/rykunk21/kanban-dashboard)
- Read the full [architecture](Architecture.md)