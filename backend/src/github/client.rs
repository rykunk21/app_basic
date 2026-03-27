//! GitHub REST API Client
//! Uses the autoaxiom GitHub App installation token

use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

const GITHUB_API_BASE: &str = "https://api.github.com";
const REPO_OWNER: &str = "rykunk21";
const REPO_NAME: &str = "app_basic";

/// GitHub Issue from REST API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Issue {
    pub number: u32,
    pub title: String,
    pub state: String,
    pub html_url: String,
    pub created_at: String,
    pub updated_at: String,
    pub labels: Vec<Label>,
    pub assignee: Option<Assignee>,
    pub body: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Label {
    pub name: String,
    pub color: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Assignee {
    pub login: String,
    pub avatar_url: String,
}

/// GitHub API Client
pub struct GitHubClient {
    token: String,
    client: reqwest::Client,
}

impl GitHubClient {
    pub fn new(token: String) -> Self {
        Self {
            token,
            client: reqwest::Client::new(),
        }
    }

    /// Get issues filtered by label
    pub async fn get_issues_by_label(&self, label: &str) -> anyhow::Result<Vec<Issue>> {
        let url = format!(
            "{}/repos/{}/{}/issues?state=open&labels={}",
            GITHUB_API_BASE, REPO_OWNER, REPO_NAME, label
        );

        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.token))
            .header("Accept", "application/vnd.github+json")
            .header("User-Agent", "autoaxiom-kanban")
            .header("X-GitHub-Api-Version", "2022-11-28")
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await?;
            return Err(anyhow::anyhow!(
                "GitHub API error: {} - {}",
                status,
                text
            ));
        }

        let issues = response.json::<Vec<Issue>>().await?;
        Ok(issues)
    }

    /// Get all open issues
    pub async fn get_all_issues(&self) -> anyhow::Result<Vec<Issue>> {
        let url = format!(
            "{}/repos/{}/{}/issues?state=open&per_page=100",
            GITHUB_API_BASE, REPO_OWNER, REPO_NAME
        );

        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.token))
            .header("Accept", "application/vnd.github+json")
            .header("User-Agent", "autoaxiom-kanban")
            .header("X-GitHub-Api-Version", "2022-11-28")
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await?;
            return Err(anyhow::anyhow!(
                "GitHub API error: {} - {}",
                status,
                text
            ));
        }

        let issues = response.json::<Vec<Issue>>().await?;
        Ok(issues)
    }

    /// Remove a label from an issue
    pub async fn remove_label(&self, issue_number: u32, label: &str) -> anyhow::Result<()> {
        let url = format!(
            "{}/repos/{}/{}/issues/{}/labels/{}",
            GITHUB_API_BASE, REPO_OWNER, REPO_NAME, issue_number,
            urlencoding::encode(label)
        );

        let response = self
            .client
            .delete(&url)
            .header("Authorization", format!("Bearer {}", self.token))
            .header("Accept", "application/vnd.github+json")
            .header("User-Agent", "autoaxiom-kanban")
            .send()
            .await?;

        // 404 is OK if label didn't exist
        if !response.status().is_success() && response.status() != reqwest::StatusCode::NOT_FOUND {
            let status = response.status();
            let text = response.text().await?;
            return Err(anyhow::anyhow!(
                "Failed to remove label: {} - {}",
                status,
                text
            ));
        }

        Ok(())
    }

    /// Add labels to an issue
    pub async fn add_labels(&self, issue_number: u32, labels: Vec<String>) -> anyhow::Result<()> {
        let url = format!(
            "{}/repos/{}/{}/issues/{}/labels",
            GITHUB_API_BASE, REPO_OWNER, REPO_NAME, issue_number
        );

        let body = serde_json::json!({ "labels": labels });

        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.token))
            .header("Accept", "application/vnd.github+json")
            .header("User-Agent", "autoaxiom-kanban")
            .header("X-GitHub-Api-Version", "2022-11-28")
            .json(&body)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await?;
            return Err(anyhow::anyhow!(
                "Failed to add labels: {} - {}",
                status,
                text
            ));
        }

        Ok(())
    }

    /// Move an issue from one label to another
    pub async fn move_issue(
        &self,
        issue_number: u32,
        old_label: &str,
        new_label: &str,
    ) -> anyhow::Result<()> {
        // Remove old label
        self.remove_label(issue_number, old_label).await?;

        // Add new label
        self.add_labels(issue_number, vec![new_label.to_string()]).await?;

        Ok(())
    }
}

/// Group issues by their kanban column label
pub fn group_issues_by_column(issues: Vec<Issue>) -> KanbanBoard {
    let mut board = KanbanBoard::default();
    let column_order = ["backlog", "red", "green", "refactor", "done"];

    for issue in issues {
        let issue_labels: Vec<String> = issue.labels.iter().map(|l| l.name.clone()).collect();

        // Find the leftmost matching column label
        let mut matched = false;
        for col in &column_order {
            if issue_labels.contains(&col.to_string()) {
                board.add_to_column(col, issue);
                matched = true;
                break;
            }
        }

        // No matching label -> Backlog
        if !matched {
            board.add_to_column("backlog", issue);
        }
    }

    board
}

/// Kanban board state
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct KanbanBoard {
    pub backlog: Vec<Issue>,
    pub red: Vec<Issue>,
    pub green: Vec<Issue>,
    pub refactor: Vec<Issue>,
    pub done: Vec<Issue>,
}

impl KanbanBoard {
    fn add_to_column(&mut self, column: &str, issue: Issue) {
        match column {
            "backlog" => self.backlog.push(issue),
            "red" => self.red.push(issue),
            "green" => self.green.push(issue),
            "refactor" => self.refactor.push(issue),
            "done" => self.done.push(issue),
            _ => self.backlog.push(issue),
        }
    }
}

/// Calculate issue age from created_at
pub fn calculate_age(created_at: &str) -> String {
    use chrono::{DateTime, Utc};

    if let Ok(dt) = DateTime::parse_from_rfc3339(created_at) {
        let now = Utc::now();
        let duration = now.signed_duration_since(dt.with_timezone(&Utc));

        let days = duration.num_days();
        let hours = duration.num_hours() % 24;

        if days > 0 {
            format!("{} days ago", days)
        } else if hours > 0 {
            format!("{} hours ago", hours)
        } else {
            "just now".to_string()
        }
    } else {
        "unknown".to_string()
    }
}