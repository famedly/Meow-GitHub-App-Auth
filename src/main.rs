use clap::Parser;
use jsonwebtoken::{Algorithm, Header};
use reqwest::header::ACCEPT;
use serde::{Deserialize, Serialize};
use time::Duration;

/// GitHub App Auth CLI Arguments
#[derive(Debug, clap::Parser)]
#[clap(author, version, about, long_about = None)]
struct CliArgs {
    /// GitHub app ID
    #[clap(long)]
    app_id: u64,

    /// GitHub app installation ID
    #[clap(long)]
    installation_id: u64,

    /// GitHub app private key path
    #[clap(long)]
    key_path: String,
}

#[derive(Debug, Deserialize)]
struct AppAuthResponse {
    pub token: String,
}

#[derive(Debug, thiserror::Error)]
enum AppAuthError {
    /// Invalid private key provided
    #[error("Invalid Private Key: {0}")]
    InvalidPrivateKey(jsonwebtoken::errors::Error),
    /// JWT encoding failed
    #[error("Failed to make JWT: {0}")]
    JwtEncode(jsonwebtoken::errors::Error),
    /// Reqwest [reqwest::Error]
    #[error("HTTP Error: {0}")]
    Reqwest(#[from] reqwest::Error),
    /// Unable to read private key file
    #[error("Unable to read the private key file: {0}")]
    KeyRead(std::io::Error),
}

#[derive(Debug, Serialize)]
struct AppAuthClaims {
    iss: u64,
    iat: i64,
    exp: i64,
}

#[tokio::main]
async fn main() -> Result<(), AppAuthError> {
    let args: CliArgs = CliArgs::parse();

    let key_content = std::fs::read_to_string(args.key_path).map_err(AppAuthError::KeyRead)?;
    let key = jsonwebtoken::EncodingKey::from_rsa_pem(key_content.as_bytes())
        .map_err(AppAuthError::InvalidPrivateKey)?;

    let claims = AppAuthClaims {
        iss: args.app_id,
        iat: time::OffsetDateTime::now_utc().unix_timestamp(),
        exp: (time::OffsetDateTime::now_utc() + Duration::minutes(2)).unix_timestamp(),
    };

    let jwt = jsonwebtoken::encode(&Header::new(Algorithm::RS256), &claims, &key)
        .map_err(AppAuthError::JwtEncode)?;

    let response: AppAuthResponse = reqwest::Client::default()
        .post(format!(
            "https://api.github.com/app/installations/{}/access_tokens",
            args.installation_id
        ))
        .header("User-Agent", "meow-github-app-auth")
        .header(ACCEPT, "application/vnd.github+json")
        .bearer_auth(jwt)
        .send()
        .await?
        .json()
        .await?;

    println!("{}", response.token);

    Ok(())
}
