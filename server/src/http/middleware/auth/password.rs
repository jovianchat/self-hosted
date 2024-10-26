use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use serde::{Deserialize, Serialize};

use crate::{Context, Error, Result};

pub async fn hash(password: String) -> Result<String> {
    // Argon2 hashing is designed to be computationally intensive,
    // so we need to do this on a blocking thread.
    Ok(tokio::task::spawn_blocking(move || -> Result<String> {
        let salt = SaltString::generate(&mut OsRng);
        Ok(Argon2::default()
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| anyhow::anyhow!("failed to generate password hash: {}", e))?
            .to_string())
    })
    .await
    .context("panic in generating password hash")??)
}
pub async fn verify(password: String, password_hash: String) -> Result<()> {
    // Argon2 hashing is designed to be computationally intensive,
    // so we need to do this on a blocking thread.
    Ok(tokio::task::spawn_blocking(move || -> Result<()> {
        let parsed_hash = PasswordHash::new(&password_hash)
            .map_err(|e| anyhow::anyhow!("invalid password hash: {}", e))?;

        Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .map_err(|e| match e {
                argon2::password_hash::Error::Password => Error::Unauthorized,
                _ => anyhow::anyhow!("failed to verify password hash: {}", e).into(),
            })
    })
    .await
    .context("panic in verifying password hash")??)
}

#[derive(Serialize, Deserialize)]
struct Pass {
    password_hash: String,
}
pub fn load_admin() -> String {
    let content = std::fs::read_to_string("admin/user-config/pass.toml")
        .expect("Failed to read admin password from file");
    let toml: Pass =
        toml::from_str(&content).expect("Failed to parse admin password from toml file");
    toml.password_hash
}
pub fn save_admin(password_hash: String) -> Result<()> {
    let password_hash = Pass { password_hash };
    let content =
        toml::to_string(&password_hash).context("Failed to parse admin password to toml")?;
    std::fs::write("admin/user-config/pass.toml", content)
        .context("Failed to write admin password to file")?;
    Ok(())
}
// pub async fn generate_admin_password_on_first_run() -> Result<()> {
//     if !std::path::Path::new("/admin/user-config/pass.toml").exists() {
//         let hash = hash_password("admin".to_string()).await?;
//         tracing::info!("Generated admin password hash: {}", hash);
//         save_admin_password(hash).unwrap();
//     }
//     Ok(())
// }
