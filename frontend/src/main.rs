//! Main Frontend Entry Point
//! Yew app with auto-refreshing Kanban board

use yew::prelude::*;
use gloo_net::http::Request;
use gloo_timers::callback::Interval;
use gloo_console::{log, error};
use wasm_bindgen_futures::spawn_local;

mod board;
mod card;
mod nlp_panel;
mod types;

use board::KanbanBoard;
use types::*;

enum Msg {
    FetchBoard,
    SetBoard(KanbanBoardData),
    MoveIssue(MoveRequest),
    Refresh,
}

struct App {
    board: KanbanBoardData,
    _interval: Interval,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        // Auto-refresh every 60 seconds
        let link = ctx.link().clone();
        let interval = Interval::new(60_000, move || {
            link.send_message(Msg::FetchBoard);
        });

        // Initial fetch
        ctx.link().send_message(Msg::FetchBoard);

        Self {
            board: KanbanBoardData::default(),
            _interval: interval,
        }
    }

    fn update(&mut self,
        ctx: &Context<Self>,
        msg: Self::Message,
    ) -> bool {
        match msg {
            Msg::FetchBoard => {
                let link = ctx.link().clone();
                spawn_local(async move {
                    match fetch_board().await {
                        Ok(board) => {
                            log!("Board fetched successfully");
                            link.send_message(Msg::SetBoard(board));
                        }
                        Err(e) => {
                            error!(format!("Failed to fetch board: {}", e));
                        }
                    }
                });
                false
            }
            Msg::SetBoard(board) => {
                self.board = board;
                true
            }
            Msg::MoveIssue(move_req) => {
                let link = ctx.link().clone();
                spawn_local(async move {
                    match move_issue(move_req).await {
                        Ok(_) => {
                            log!("Issue moved successfully");
                            // Refresh board after move
                            link.send_message(Msg::FetchBoard);
                        }
                        Err(e) => {
                            error!(format!("Failed to move issue: {}", e));
                        }
                    }
                });
                false
            }
            Msg::Refresh => {
                ctx.link().send_message(Msg::FetchBoard);
                false
            }
        }
    }

    fn view(&self,
        ctx: &Context<Self>,
    ) -> Html {
        let on_move = ctx.link().callback(Msg::MoveIssue);
        let on_refresh = ctx.link().callback(|_| Msg::Refresh);

        html! {
            <div class="app">
                <header class="app-header">
                    <h1>{ "🎯 TDD Kanban Dashboard" }</h1>
                    <button class="refresh-btn" onclick={on_refresh}>
                        { "🔄 Refresh" }
                    </button>
                </header>

                <main class="app-main">
                    <KanbanBoard
                        board={self.board.clone()}
                        {on_move}
                    />
                </main>

                <footer class="app-footer">
                    <p>{ "Auto-refreshes every 60 seconds • Drag cards to move issues" }</p>
                </footer>
            </div>
        }
    }
}

async fn fetch_board() -> anyhow::Result<KanbanBoardData> {
    let response = Request::get("/api/issues")
        .send()
        .await?;

    if !response.ok() {
        return Err(anyhow::anyhow!(
            "Failed to fetch board: {}",
            response.status()
        ));
    }

    let board: KanbanBoardData = response.json().await?;
    Ok(board)
}

async fn move_issue(move_req: MoveRequest) -> anyhow::Result<()> {
    let response = Request::post(&format!("/api/issues/{}/move", move_req.issue_number))
        .json(&move_req)?
        .send()
        .await?;

    if !response.ok() {
        return Err(anyhow::anyhow!(
            "Failed to move issue: {}",
            response.status()
        ));
    }

    Ok(())
}

fn main() {
    yew::Renderer::<App>::new().render();
}