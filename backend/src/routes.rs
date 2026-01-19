use serde::{Deserialize, Serialize};
use std::env;
use std::sync::LazyLock;
use std::time::Duration;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::Surreal;
use tokio::time::sleep;

use rocket::serde::json::Json;

use crate::db::{error::Error, ex::Ex};

static DB: LazyLock<Surreal<Client>> = LazyLock::new(Surreal::init);

// LOGIN INFO
const MAX_RETRIES: i8 = 5;
const DELAY: Duration = Duration::from_secs(5);

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Payload {
    Ex(Ex),
}

pub async fn init() -> Result<(), surrealdb::Error> {
    #[derive(Debug, Serialize, Deserialize)]
    struct Config {}

    let namespace = env::var("DATABASE_NS").expect("DATABASE_NS must be set");
    let database_name = env::var("DATABASE_NAME").expect("DATABASE_NAME must be set");

    connect().await?;

    DB.signin(surrealdb::opt::auth::Root {
        username: "root",
        password: "root",
    })
    .await?;

    // Switch to the desired namespace and database
    DB.use_ns(namespace).use_db(database_name).await?;

    eprintln!("Connected to SurrealDB!");

    if let Some(config) = DB
        .update::<Option<Config>>(("config", "config"))
        .content(Config {})
        .await?
    {
        rocket::info!("Config loaded: {:?}", config);
    }

    Ok(())
}

async fn connect() -> Result<(), surrealdb::Error> {
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set")
        .trim()
        .to_string();

    eprintln!("🔍 DATABASE_URL = '{}'", database_url);

    let mut failures = 0;

    loop {
        let res = DB.connect::<Ws>(&database_url).await;
        match res {
            Ok(_) => break,
            Err(e) => {
                eprintln!("{:?}", e);

                if failures >= MAX_RETRIES {
                    return Err(e);
                }
                failures += 1;

                sleep(DELAY).await;
                continue;
            }
        }
    }
    Ok(())
}

#[post("/<table>/<id>", data = "<data>")]
pub async fn create(
    table: &str,
    id: &str,
    data: Json<Payload>,
) -> Result<Json<Option<Payload>>, Error> {
    match data.into_inner() {
        Payload::Ex(ex) => {
            if let Some(res) = DB.create((&*table, &*id)).content(ex).await? {
                Ok(Json(Some(Payload::Ex(res))))
            } else {
                Ok(Json(None))
            }
        }
    }
}

#[get("/<table>/<id>")]
pub async fn read(table: &str, id: &str) -> Result<Json<Option<Ex>>, Error> {
    let res = DB.select((&*table, &*id)).await?;
    Ok(Json(res))
}

#[put("/<table>/<id>", data = "<data>")]
pub async fn update(
    table: &str,
    id: &str,
    data: Json<Payload>,
) -> Result<Json<Option<Payload>>, Error> {
    match data.into_inner() {
        Payload::Ex(ex) => {
            let res = DB.update((&*table, &*id)).content(ex).await?;
            Ok(Json(res))
        }
    }
}

#[delete("/<table>/<id>")]
pub async fn delete(table: &str, id: &str) -> Result<Json<Option<Payload>>, Error> {
    let _: Option<Payload> = DB.delete((&*table, &*id)).await?;

    Ok(Json(None))
}
