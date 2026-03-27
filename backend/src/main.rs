//! Kanban Dashboard Server
//! Rocket backend serving API and static files

mod github;

use rocket::{
    fs::FileServer,
    serde::json::Json,
    Config, State,
};
use std::{
    env,
    net::{IpAddr, Ipv4Addr},
    path::PathBuf,
    sync::Arc,
};
use rocket_cors::{CorsOptions, AllowedOrigins};

/// State shared between requests
pub struct KanbanState {
    token_cache: Arc<github::token::TokenCache>,
}

/// Get the kanban board data
#[get("/api/issues")]
async fn get_kanban(state: &State<KanbanState>) -> Json<github::client::KanbanBoard> {
    let token = match state.token_cache.get_token().await {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Failed to get token: {}", e);
            return Json(github::client::KanbanBoard::default());
        }
    };

    let client = github::client::GitHubClient::new(token);

    match client.get_all_issues().await {
        Ok(issues) => {
            let board = github::client::group_issues_by_column(issues);
            Json(board)
        }
        Err(e) => {
            eprintln!("Failed to fetch issues: {}", e);
            Json(github::client::KanbanBoard::default())
        }
    }
}

/// Move an issue to a new column
#[post("/api/issues/<issue_number>/move", data = "<move_req>")]
async fn move_issue(
    state: &State<KanbanState>,
    issue_number: u32,
    move_req: Json<MoveRequest>,
) -> Result<Json<MoveResponse>>, rocket::http::Status> {
    let token = match state.token_cache.get_token().await {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Failed to get token: {}", e);
            return Err(rocket::http::Status::InternalServerError);
        }
    };

    let client = github::client::GitHubClient::new(token);

    // Move the issue
    if let Err(e) = client
        .move_issue(issue_number, &move_req.old_label, &move_req.new_label)
        .await
    {
        eprintln!("Failed to move issue: {}", e);
        return Err(rocket::http::Status::InternalServerError);
    }

    Ok(Json(MoveResponse {
        success: true,
        issue_number,
        old_label: move_req.old_label.clone(),
        new_label: move_req.new_label.clone(),
    }))
}

#[derive(serde::Deserialize)]
struct MoveRequest {
    old_label: String,
    new_label: String,
}

#[derive(serde::Serialize)]
struct MoveResponse {
    success: bool,
    issue_number: u32,
    old_label: String,
    new_label: String,
}

/// Health check
#[get("/api/health")]
fn health() -> &'static str {
    "OK"
}

#[rocket::main]
async fn main() -> anyhow::Result<()> {
    // CORS configuration
    let cors = CorsOptions {
        allowed_origins: AllowedOrigins::All,
        allowed_methods: vec![
            rocket::http::Method::Get,
            rocket::http::Method::Post,
            rocket::http::Method::Patch,
            rocket::http::Method::Delete,
        ]
        .into_iter()
        .map(From::from)
        .collect(),
        allowed_headers: rocket_cors::AllowedHeaders::All,
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()?;

    // Use Railway's dynamic PORT
    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse()
        .expect("PORT must be a number");

    let state = KanbanState {
        token_cache: Arc::new(github::token::TokenCache::new()),
    };

    // Determine static files path
    let static_path = env::var("STATIC_PATH")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("../frontend/dist"));

    // Build Rocket with config
    let _rocket = rocket::custom(Config {
        port,
        address: IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
        ..Config::default()
    })
    .attach(cors)
    .manage(state)
    .mount("/", FileServer::from(static_path))
    .mount("/", routes![get_kanban, move_issue, health])
    .launch()
    .await?;

    Ok(())
}