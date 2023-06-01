pub mod command;

pub(crate) mod run;

use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait Command {
    async fn before(&self) -> Result<()> {
        Ok(())
    }
    async fn after(&self) -> Result<()> {
        Ok(())
    }

    async fn action(&self) -> Result<()> {
        Ok(())
    }

    async fn execute(&self) -> Result<()> {
        self.before().await?;
        self.action().await?;
        self.after().await
    }
}
