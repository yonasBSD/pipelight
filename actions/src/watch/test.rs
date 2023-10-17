#[cfg(test)]
mod watcher {
    // Globals
    use crate::watch::{Watcher,build};
    use utils::teleport::Portal;
    // Error handling
    use miette::{Diagnostic, IntoDiagnostic, Result};
    use thiserror::Error;

    #[tokio::test]
    async fn builder() -> Result<()> {
        // Teleport
        Portal::new()?.seed("test.pipelight").search()?.teleport()?;
        // Build watcher
        let res = build();
        assert!(res.is_ok());
        Ok(())
    }
    // #[tokio::test]
    async fn try_start() -> Result<()> {
        // Teleport
        Portal::new()?.seed("test.pipelight").search()?.teleport()?;
        // Watcher::start()?;
        let (we, runtime) = build()?;
        we.main().await.into_diagnostic()?;
        Ok(())
    }
}
