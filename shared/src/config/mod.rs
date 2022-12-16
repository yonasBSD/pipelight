use crate::exec::exec;
use crate::types::{Config, Path, Pipeline};
use log::{debug, error, info, trace, warn};
use project_root::get_project_root;
use std::error::Error;

/// Return the config from .ts file inside the working dir.
pub fn load_config() -> Result<Config, Box<dyn Error>> {
    let executable = "ts-node";
    let path = Path {
        folder: get_project_root()?.to_str().unwrap().to_owned(),
        file: "typescript/main.ts".into(),
    };
    let command = format!("{} {}/{}", executable, path.folder, path.file);
    let data = exec(command)?;

    // Typecast Json output
    let config_result = serde_json::from_str::<Config>(&data);

    let config = match config_result {
        Ok(res) => {
            return Ok(res);
        }
        Err(e) => {
            error!("typo in config file {}", e);
            debug!("{}", data);
            return Err(Box::from(e));
        }
    };
}

/// Apply constraints to the Config struct
pub fn check_config(config: Config) -> Result<Config, Box<dyn Error>> {
    let names = config
        .pipelines
        .iter()
        .map(|p| &p.name)
        .cloned()
        .collect::<Vec<String>>();

    //Clone vector and remove duplicates
    let mut dedup = names.clone();
    dedup.sort();
    dedup.dedup();

    let has_duplicate = dedup.len() != names.len();

    if has_duplicate {
        let message = "Duplicate pipeline names in config";
        error!("{}", message);
        Err(Box::from(message))
    } else {
        Ok(config)
    }
}
pub fn get_config() -> Result<Config, Box<dyn Error>> {
    let mut config = load_config()?;
    config = check_config(config)?;
    Ok(config)
}
pub fn get_pipeline(name: String) -> Pipeline {
    let config = get_config().unwrap();
    let pipeline_result = config
        .pipelines
        .iter()
        .filter(|p| p.name == name)
        .cloned()
        .next();
    let pipeline = pipeline_result.expect("Failed to retriev pipeline");
    return pipeline;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::type_of;
    #[test]
    fn internal() {
        let res = load_config();
    }
}