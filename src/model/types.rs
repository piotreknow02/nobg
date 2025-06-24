use crate::model::{error::Error, registry::MODELS};
use chrono::{DateTime, Local};
use directories::BaseDirs;
use fmtsize::{Conventional, FmtSize};
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::blocking::Client;
use reqwest::header::CONTENT_LENGTH;
use std::{
    fs::File,
    fs::metadata,
    io::{Read, Write},
    path::PathBuf,
    time::SystemTime,
};
use url::Url;

#[derive(Debug)]
pub struct RembgModel {
    pub name: &'static str,
    pub remote_url: &'static str,
    pub description: Option<&'static str>,
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

    fn create_config_if_not_exists() -> Result<(), Error> {
        let config_dir = Self::get_config_dir()?;
        if !config_dir.exists() {
            std::fs::create_dir_all(&config_dir)?;
        }
        Ok(())
    }

    pub fn check_exists(&self) -> bool {
        match &self.get_path() {
            Ok(v) => v.exists(),
            Err(_) => false,
        }
    }

    fn get_path(&self) -> Result<PathBuf, Error> {
        let config_dir = Self::get_config_dir()?;
        let filename = self.get_filename();
        Ok(config_dir.join(filename))
    }

    fn get_filename(&self) -> String {
        let parsed_url = match Url::parse(self.remote_url) {
            Ok(u) => u,
            Err(_) => unreachable!(),
        };
        parsed_url
            .path_segments()
            .and_then(|segments| segments.last())
            .unwrap_or((self.name.to_owned() + ".onnx").as_ref())
            .to_owned()
    }

    fn get_download_time(&self) -> Option<String> {
        let file_path = match self.get_path() {
            Ok(path) => path,
            Err(_) => return None,
        };
        let metadata = match metadata(file_path) {
            Ok(m) => m,
            Err(_) => return None,
        };
        let time = match metadata.created() {
            Ok(t) => t,
            Err(_) => return None,
        };
        Some(Self::time_ago(time))
    }

    fn time_ago(past: SystemTime) -> String {
        let now = SystemTime::now();

        let now_chrono: DateTime<Local> = DateTime::from(now);
        let past_chrono: DateTime<Local> = DateTime::from(past);
        let delta = now_chrono.signed_duration_since(past_chrono);

        let seconds = delta.num_seconds();
        let minutes = delta.num_minutes();
        let hours = delta.num_hours();
        let days = delta.num_days();
        let months = days / 30;
        let years = days / 365;

        if seconds < 60 {
            format!("{} seconds ago", seconds)
        } else if minutes < 60 {
            format!("{} minutes ago", minutes)
        } else if hours < 24 {
            format!("{} hours ago", hours)
        } else if days < 30 {
            format!("{} days ago", days)
        } else if days < 365 {
            format!("{} months ago", months)
        } else {
            format!("{} years ago", years)
        }
    }

    fn get_file_size(&self) -> Option<String> {
        let file_path = match self.get_path() {
            Ok(path) => path,
            Err(_) => return None,
        };

        let metadata = match metadata(&file_path) {
            Ok(m) => m,
            Err(_) => return None,
        };

        if metadata.is_file() {
            Some(metadata.len().fmt_size(Conventional).to_string())
        } else {
            None
        }
    }

    pub fn find_model(name: &str) -> Option<&Self> {
        for m in MODELS {
            if m.name == name {
                return Some(m);
            }
        }
        None
    }

    pub fn print(&self, quiet: bool, verbose: bool) {
        if quiet {
            self.print_quiet();
        } else {
            if verbose {
                self.print_verbose();
            } else {
                self.print_standard();
            }
        }
    }

    fn print_standard(&self) {
        println!(
            "{0:<25} {1:<10} {2:<10}",
            self.name,
            self.get_download_time().unwrap_or("N/a".to_owned()),
            self.get_file_size().unwrap_or("N/a".to_owned()),
        )
    }

    fn print_verbose(&self) {
        let install_status_symbol = if self.check_exists() { " ✅" } else { " " };
        println!(
            "{0:<3} {1:<25} {2:<15} {3:<15} - {4}",
            install_status_symbol,
            self.name,
            self.get_download_time().unwrap_or("N/a".to_owned()),
            self.get_file_size().unwrap_or("N/a".to_owned()),
            self.description.unwrap(),
        )
    }

    fn print_quiet(&self) {
        println!("{}", self.name);
    }

    pub fn pull(&self) -> Result<(), Error> {
        println!("Pulling model {}...", self.name);
        Self::create_config_if_not_exists()?;

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

    pub fn rm(&self) -> Result<(), Error> {
        let model_path = self.get_path()?;
        match std::fs::remove_file(&model_path) {
            Ok(_) => {
                println!("Model {} was removed", self.name);
                Ok(())
            }
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Err(Error::ModelNotFound(
                model_path.to_str().unwrap().to_owned(),
            )),
            Err(e) => Err(Error::ModelIOError(e)),
        }
    }
}
