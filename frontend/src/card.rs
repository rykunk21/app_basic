//! Issue Card Component
//! Draggable card showing issue details

use yew::prelude::*;
use gloo_console::log;
use web_sys::{HtmlElement, HtmlAnchorElement};

use crate::types::*;

#[derive(Properties, PartialEq)]
pub struct IssueCardProps {
    pub issue: Issue,
    pub current_column: String,
}

#[function_component(IssueCard)]
pub fn issue_card(props: &IssueCardProps) -> Html {
    let issue = props.issue.clone();
    let current_column = props.current_column.clone();

    let on_drag_start = {
        let issue_number = issue.number;
        let current_column = current_column.clone();
        
        Callback::from(move |e: DragEvent| {
            let move_req = MoveRequest {
                issue_number,
                old_label: current_column.clone(),
                new_label: String::new(), // Will be set on drop
            };

            if let Some(data_transfer) = e.data_transfer() {
                let json = serde_json::to_string(&move_req).unwrap_or_default();
                let _ = data_transfer.set_data("application/json", &json);
                data_transfer.set_effect_allowed("move");
            }

            log!(format!("Started dragging issue #{}", issue_number));
        })
    };

    let on_click = {
        let html_url = issue.html_url.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            if let Some(window) = web_sys::window() {
                let _ = window.open_with_url_and_target(&html_url, "_blank");
            }
        })
    };

    // Calculate age
    let age = calculate_age(&issue.created_at);

    // Avatar URL (default to placeholder if no assignee)
    let avatar = issue.assignee.as_ref().map(|a| a.avatar_url.clone())
        .unwrap_or_else(|| "https://github.com/ghost.png".to_string());

    // Get first label color
    let color = issue.labels.first().map(|l| format!("#{}", l.color))
        .unwrap_or_else(|| "#8B8B8B".to_string());

    html! {
        <div
            class="issue-card"
            draggable="true"
            ondragstart={on_drag_start}
            onclick={on_click}
        >
            <div class="card-header">
                <span class="issue-number">{ format!("#{}", issue.number) }</span>
                <span class="issue-age">{ age }</span>
            </div>
            <div class="card-title">{ &issue.title }</div>
            
            <div class="card-footer">
                <img
                    class="assignee-avatar"
                    src={avatar}
                    alt="Assignee"
                />
                <div class="card-labels">
                    { for issue.labels.iter().map(|label| html! {
                        <span
                            class="label-badge"
                            style={format!("background-color: #{}", label.color)}
                        >
                            { &label.name }
                        </span>
                    })}
                </div>
            </div>
        </div>
    }
}

fn calculate_age(created_at: &str) -> String {
    use chrono::{DateTime, Utc};

    DateTime::parse_from_rfc3339(created_at)
        .ok()
        .map(|dt| {
            let now = Utc::now();
            let duration = now.signed_duration_since(dt.with_timezone(&Utc));
            
            let days = duration.num_days();
            let hours = duration.num_hours() % 24;

            if days > 0 {
                format!("{}d", days)
            } else if hours > 0 {
                format!("{}h", hours)
            } else {
                "now".to_string()
            }
        })
        .unwrap_or_else(|| "?".to_string())
}