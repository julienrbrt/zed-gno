use zed_extension_api::{self as zed, LanguageServerId, Result, Worktree};

/// The name of the LSP binary.
const BINARY_NAME: &str = "gnopls";

struct GnoExtension {
    cached_binary_path: Option<String>,
}

impl GnoExtension {
    /// Returns the path to the gnopls binary.
    ///
    /// Looks for `gnopls` on PATH. If not found, returns an error with
    /// installation instructions.
    fn language_server_binary_path(&mut self, worktree: &Worktree) -> Result<String> {
        // Check cache first
        if let Some(path) = &self.cached_binary_path {
            if std::fs::metadata(path).is_ok() {
                return Ok(path.clone());
            }
        }

        // Look for gnopls on PATH
        if let Some(path) = worktree.which(BINARY_NAME) {
            self.cached_binary_path = Some(path.clone());
            return Ok(path);
        }

        Err(format!(
            "gnopls not found on PATH. Install it with:\n  go install github.com/gnoverse/gnopls@latest"
        ))
    }
}

impl zed::Extension for GnoExtension {
    fn new() -> Self {
        Self {
            cached_binary_path: None,
        }
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> Result<zed::Command> {
        let binary_path = self.language_server_binary_path(worktree)?;

        Ok(zed::Command {
            command: binary_path,
            args: vec![],
            env: worktree.shell_env(),
        })
    }
}

zed::register_extension!(GnoExtension);
