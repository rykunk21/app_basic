//! Kanban Board Component
//! Drag-and-drop enabled columns for each TDD phase

use yew::prelude::*;
use yew::html::DragEvent;
use gloo_net::http::Request;
use gloo_console::log;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlElement;

use crate::types::*;
use crate::card::IssueCard;
use crate::nlp_panel::NlpPanel;

#[derive(Properties, PartialEq, Clone)]
pub struct KanbanBoardProps {
    pub board: KanbanBoardData,
    pub on_move: Callback<(MoveRequest)>,
}

#[function_component(KanbanBoard)]
pub fn kanban_board(props: &KanbanBoardProps) -> Html {
    let board = props.board.clone();
    let on_move = props.on_move.clone();

    html! {
        <div class="kanban-board">
            <div class="board-columns">
                {
                    for [
                        ("backlog", "Backlog", board.backlog),
                        ("red", "🔴 Red", board.red),
                        ("green", "🟢 Green", board.green),
                        ("refactor", "🔵 Refactor", board.refactor),
                        ("done", "✅ Done", board.done),
                    ].into_iter().map(|(col_id, title, issues)| {
                        html! {
                            <KanbanColumn
                                id={col_id.to_string()}
                                title={title.to_string()}
                                issues={issues}
                                {on_move.clone()}
                            />
                        }
                    })
                }
            </div>
            <NlpPanel board={board.clone()} />
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct ColumnProps {
    id: String,
    title: String,
    issues: Vec<Issue>,
    on_move: Callback<MoveRequest>,
}

#[function_component(KanbanColumn)]
fn kanban_column(props: &ColumnProps) -> Html {
    let drag_over = use_state(|| false);
    let column_id = props.id.clone();

    let on_drag_over = {
        let drag_over = drag_over.clone();
        Callback::from(move |e: DragEvent| {
            e.prevent_default();
            drag_over.set(true);
        })
    };

    let on_drag_leave = {
        let drag_over = drag_over.clone();
        Callback::from(move |_: DragEvent| {
            drag_over.set(false);
        })
    };

    let on_drop = {
        let on_move = props.on_move.clone();
        let column_id = column_id.clone();
        let drag_over = drag_over.clone();
        
        Callback::from(move |e: DragEvent| {
            e.prevent_default();
            drag_over.set(false);

            if let Some(data_transfer) = e.data_transfer() {
                if let Ok(data) = data_transfer.get_data("application/json") {
                    if let Ok(move_req) = serde_json::from_str::<MoveRequest>(&data) {
                        on_move.emit(MoveRequest {
                            issue_number: move_req.issue_number,
                            old_label: move_req.old_label,
                            new_label: column_id.clone(),
                        });
                    }
                }
            }
        })
    };

    let class = if *drag_over {
        "kanban-column drag-over"
    } else {
        "kanban-column"
    };

    html! {
        <div
            class={class}
            ondragover={on_drag_over}
            ondragleave={on_drag_leave}
            ondrop={on_drop}
            id={format!("column-{}", column_id)}
        >
            <div class="column-header">
                <span class="column-title">{ &props.title }</span>
                <span class="column-count">{ props.issues.len() }</span>
            </div>
            <div class="column-content">
                { for props.issues.iter().map(|issue| {
                    html! {
                        <IssueCard
                            issue={issue.clone()}
                            current_column={column_id.clone()}
                        />
                    }
                })}
            </div>
        </div>
    }
}