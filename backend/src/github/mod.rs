//! GitHub API module
//! Handles authentication and REST API calls

pub mod client;
pub mod token;

pub use client::{GitHubClient, KanbanBoard, Issue, group_issues_by_column, calculate_age};
pub use token::get_installation_token;