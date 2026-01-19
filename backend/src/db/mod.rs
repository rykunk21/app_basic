use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::Surreal;

pub mod error;
pub mod types;
///
/// TYPES BEING STORED
pub mod ex {
    use serde::{Deserialize, Serialize};
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Ex {
        x: u8,
        y: u8,
    }
}

/// API for docker surrealdb service [deprecated?]
#[allow(unused)]
async fn connect() -> surrealdb::Result<Surreal<Client>> {
    // Create the WebSocket connection using the correct type
    let db = Surreal::new::<Ws>("127.0.0.1:8000").await?;

    // Sign in with root credentials (or use your credentials)
    db.signin(surrealdb::opt::auth::Root {
        username: "root",
        password: "root",
    })
    .await?;

    // Switch to the desired namespace and database
    db.use_ns("test").use_db("test").await?;

    // Your DB logic here
    println!("Connected to SurrealDB!");

    Ok(db)
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};

    use super::*;

    #[tokio::test]
    pub async fn test_connection() {
        let _ = connect().await.expect("Failed");
    }

    #[tokio::test]
    pub async fn test_example_query() {
        #[derive(Debug, Serialize, Deserialize, PartialEq)]
        pub struct Ex {
            x: u8,
            y: u8,
        }

        let db = connect().await.expect("Failed to connect");

        let ex = Ex { x: 8, y: 16 };

        // Create a new person with a random id
        let res: Ex = db
            .create("example")
            .content(Ex { x: 8, y: 16 })
            .await
            .expect("failed to create Ex")
            .unwrap();

        assert_eq!(res, ex);

        db.query("DELETE example").await.expect("failed to delete!");
    }

    #[tokio::test]
    pub async fn test_locking() {
        use std::sync::LazyLock;
        use surrealdb::engine::remote::ws::Client;
        use surrealdb::Surreal;

        let db: LazyLock<Surreal<Client>> = LazyLock::new(Surreal::init);
        db.connect::<Ws>("127.0.0.1:8080")
            .await
            .expect("failed to connect");
    }
}
