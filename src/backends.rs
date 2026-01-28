use anyhow::Result;
use dioxus::prelude::*;

/// Check the alive backend
#[post("/api/v1/alive1")]
pub async fn alive() -> Result<()> {
    Ok(())
}
