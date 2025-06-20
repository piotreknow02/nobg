use crate::model::error::Error;
use directories::BaseDirs;
use std::path::PathBuf;

#[derive(Debug)]
pub struct RembgModel {
    name: &'static str,
    remote_url: &'static str,
}

impl RembgModel {
    fn get_config_dir() -> Result<PathBuf, Error> {
        if let Some(base_dirs) = BaseDirs::new() {
            let config_dir = if cfg!(target_os = "windows") {
                base_dirs.data_dir().join("nobg")
            } else {
                base_dirs.home_dir().join(".nobg")
            };
            Ok(config_dir)
        } else {
            Err(Error::FailedToGetBaseDir)
        }
    }

    fn get_path(&self) -> Result<PathBuf, Error> {
        let config_dir = Self::get_config_dir()?;
        Ok(config_dir.join(self.name))
    }

    // pub fn download(&self) -> Result<(), Error> {
    //     Ok(())
    // }

    fn check_exists(&self) -> bool {
        match &self.get_path() {
            Ok(v) => v.exists(),
            Err(_) => false,
        }
    }
}
