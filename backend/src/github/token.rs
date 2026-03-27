//! GitHub App Installation Token Generator
//! Generates short-lived tokens from the GitHub App private key

use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};

const APP_ID: &str = "3196906";
const INSTALLATION_ID: &str = "119295792";
const PRIVATE_KEY_PATH: &str = "/home/axiom/.axiom/keys/autoaxiom.2026-03-26.private-key.pem";

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    /// Issued at
    iat: usize,
    /// Expiration time (10 minutes from now)
    exp: usize,
    /// Issuer (GitHub App ID)
    iss: String,
}

/// Generate a JWT for GitHub App authentication
fn generate_jwt() -> anyhow::Result<String> {
    let private_key = fs::read_to_string(PRIVATE_KEY_PATH)?;

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)?
        .as_secs() as usize;

    let claims = Claims {
        iat: now - 60, // 60 seconds ago (account for clock drift)
        exp: now + 600, // 10 minutes from now
        iss: APP_ID.to_string(),
    };

    let header = Header::new(Algorithm::RS256);
    let encoding_key = EncodingKey::from_rsa_pem(private_key.as_bytes())?;

    let token = encode(&header, &claims, &encoding_key)?;

    Ok(token)
}

#[derive(Debug, Deserialize)]
struct InstallationToken {
    token: String,
    expires_at: String,
}

/// Get an installation access token
/// These tokens are valid for 1 hour
pub async fn get_installation_token() -> anyhow::Result<String> {
    let jwt = generate_jwt()?;

    let client = reqwest::Client::new();
    let url = format!(
        "https://api.github.com/app/installations/{}/access_tokens",
        INSTALLATION_ID
    );

    let response = client
        .post(&url)
        .header("Authorization", format!("Bearer {}", jwt))
        .header("Accept", "application/vnd.github+json")
        .header("User-Agent", "autoaxiom-kanban")
        .header("X-GitHub-Api-Version", "2022-11-28")
        .send()
        .await?;

    if !response.status().is_success() {
        let status = response.status();
        let text = response.text().await?;
        return Err(anyhow::anyhow!(
            "Failed to get installation token: {} - {}",
            status,
            text
        ));
    }

    let token_data: InstallationToken = response.json().await?;

    Ok(token_data.token)
}

/// Cached token storage
use std::sync::{Arc, Mutex};
use std::time::Duration;

pub struct TokenCache {
    token: Arc<Mutex<Option<CachedToken>>>,
}

struct CachedToken {
    token: String,
    expires_at: SystemTime,
}

impl TokenCache {
    pub fn new() -> Self {
        Self {
            token: Arc::new(Mutex::new(None)),
        }
    }

    pub async fn get_token(&self) -> anyhow::Result<String> {
        // Check if we have a cached token that's still valid
        {
            let guard = self.token.lock().unwrap();
            if let Some(ref cached) = *guard {
                // Refresh 5 minutes before expiration
                if cached.expires_at > SystemTime::now() + Duration::from_secs(300) {
                    return Ok(cached.token.clone());
                }
            }
        }

        // Get new token
        let new_token = get_installation_token().await?;

        // Cache it (expires in ~1 hour from GitHub)
        let expires_at = SystemTime::now() + Duration::from_secs(3600);

        {
            let mut guard = self.token.lock().unwrap();
            *guard = Some(CachedToken {
                token: new_token.clone(),
                expires_at,
            });
        }

        Ok(new_token)
    }
}

/// Global token cache
lazy_static::lazy_static! {
    pub static ref TOKEN_CACHE: TokenCache = TokenCache::new();
}