use crate::types::{Path, GIT_HOOKS};
use std::error::Error;
use std::fs;

/// Create/Ensure git hooks file trees
pub fn is_hook() -> Result<bool, Box<dyn Error>> {
    Ok(true)
}

///Create directories
pub fn ensure_folders() -> Result<(), Box<dyn Error>> {
    let folder = ".git/hooks".to_owned();
    let extension = ".d";

    let path = Path {
        folder: folder.clone(),
        file: "typescript/scripts/main.ts".to_owned(),
    };
    for hook in GIT_HOOKS {
        fs::create_dir(format!("{}/{}{}", path.folder, hook, extension))?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn internal() -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}