pub mod command;

pub(crate) mod run;

use anyhow::Result;

pub trait Command {
    fn before(&self) -> Result<()> {
        Ok(())
    }
    fn after(&self) -> Result<()> {
        Ok(())
    }

    fn action(&self) -> Result<()> {
        Ok(())
    }

    fn execute(&self) -> Result<()> {
        self.before()?;
        self.action()?;
        self.after()
    }
}
