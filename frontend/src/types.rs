//! Shared Types for Kanban Dashboard
//! Mirror of backend types for frontend

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Label {
    pub name: String,
    pub color: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Assignee {
    pub login: String,
    pub avatar_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct KanbanBoardData {
    pub backlog: Vec<Issue>,
    pub red: Vec<Issue>,
    pub green: Vec<Issue>,
    pub refactor: Vec<Issue>,
    pub done: Vec<Issue>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MoveRequest {
    pub issue_number: u32,
    pub old_label: String,
    pub new_label: String,
}

impl KanbanBoardData {
    pub fn all_issues(&self) -> Vec<&Issue> {
        let mut all = Vec::new();
        all.extend(&self.backlog);
        all.extend(&self.red);
        all.extend(&self.green);
        all.extend(&self.refactor);
        all.extend(&self.done);
        all
    }

    pub fn by_column(&self) -> Vec<(&str, &Vec<Issue>)> {
        vec![
            ("Backlog", &self.backlog),
            ("Red", &self.red),
            ("Green", &self.green),
            ("Refactor", &self.refactor),
            ("Done", &self.done),
        ]
    }
}