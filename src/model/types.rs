use crate::model::error::Error;
use directories::BaseDirs;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::blocking::Client;
use reqwest::header::CONTENT_LENGTH;
use std::{
    fs::File,
    io::{Read, Write},
    path::PathBuf,
};

#[derive(Debug)]
pub struct RembgModel {
    pub name: &'static str,
    pub remote_url: &'static str,
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

    pub fn download(&self) -> Result<(), Error> {
        if self.check_exists() {
            return Err(Error::ModelAllreadyDownloaded(self.name.to_owned()));
        }

        let client = Client::new();
        let mut response = match client.get(self.remote_url).send() {
            Ok(res) => res,
            Err(e) => return Err(Error::ReqwestError(e)),
        };

        let total_size = response
            .headers()
            .get(CONTENT_LENGTH)
            .and_then(|ct_len| ct_len.to_str().ok())
            .and_then(|ct_len_str| ct_len_str.parse().ok())
            .unwrap_or(0);

        let pb = ProgressBar::new(total_size);
        pb.set_style(
                ProgressStyle::default_bar()
                    .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")
                    .unwrap()
                    .progress_chars("#>-"),
            );

        let output_path = self.get_path()?;
        let mut file = File::create(output_path)?;
        let mut downloaded: u64 = 0;
        let mut buffer = [0; 8192];

        while let Ok(n) = response.read(&mut buffer) {
            if n == 0 {
                break;
            }
            file.write_all(&buffer[..n])?;
            downloaded += n as u64;
            pb.set_position(downloaded);
        }

        pb.finish_with_message("Download complete");

        Ok(())
    }

    fn check_exists(&self) -> bool {
        match &self.get_path() {
            Ok(v) => v.exists(),
            Err(_) => false,
        }
    }
}
