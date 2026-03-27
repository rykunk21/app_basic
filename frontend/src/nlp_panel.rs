//! NLP Panel Component
//! Stub for LLM-powered summary generation

use yew::prelude::*;
use crate::types::KanbanBoardData;

#[derive(Properties, PartialEq)]
pub struct NlpPanelProps {
    pub board: KanbanBoardData,
}

#[function_component(NlpPanel)]
pub fn nlp_panel(props: &NlpPanelProps) -> Html {
    let board = &props.board;

    // Summary statistics
    let total = board.all_issues().len();
    let in_progress = board.red.len() + board.green.len() + board.refactor.len();
    let completed = board.done.len();

    html! {
        <div class="nlp-panel">
            <div class="nlp-header">
                <h3>{ "📊 Kanban Summary" }</h3>
            </div>

            <div class="nlp-content">
                <div class="stats-grid">
                    <div class="stat-box">
                        <span class="stat-value">{ total }</span>
                        <span class="stat-label">{ "Total Issues" }</span>
                    </div>
                    <div class="stat-box">
                        <span class="stat-value">{ in_progress }</span>
                        <span class="stat-label">{ "In Progress" }</span>
                    </div>
                    <div class="stat-box">
                        <span class="stat-value">{ completed }</span>
                        <span class="stat-label">{ "Completed" }</span>
                    </div>
                </div>

                <div class="column-summary">
                    {
                        for board.by_column().into_iter().map(|(name, issues)| {
                            if !issues.is_empty() {
                                html! {
                                    <div class="column-summary-item">
                                        <strong>{ name }</strong>
                                        <span>{ format!(": {} issues", issues.len()) }</span>
                                    </div>
                                }
                            } else {
                                html! {}
                            }
                        })
                    }
                </div>

                <div class="nlp-placeholder">
                    <p>{ "🤖 LLM Integration Coming Soon" }</p>
                    <p class="placeholder-text">
                        { "Future versions will generate AI-powered summaries of blockers, progress, and next actions." }
                    </p>
                </div>
            </div>
        </div>
    }
}