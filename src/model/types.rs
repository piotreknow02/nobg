use crate::model::{error::Error, registry::MODELS};
use chrono::{DateTime, Local};
use colored::Colorize;
use directories::BaseDirs;
use fmtsize::{Conventional, FmtSize};
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::blocking::Client;
use reqwest::header::CONTENT_LENGTH;
use sha2::{Digest, Sha256};
use std::{
    fs::{File, metadata},
    io::{Read, Write},
    path::PathBuf,
    sync::OnceLock,
    sync::atomic::{AtomicU8, Ordering},
    time::SystemTime,
};

static DOWNLOAD_RETRIES: AtomicU8 = AtomicU8::new(0);

#[derive(Debug, Clone)]
pub struct RembgModel {
    pub name: &'static str,
    pub resolution: (u32, u32),
    pub remote_url: &'static str,
    pub description: Option<&'static str>,
    pub checksum: Option<&'static str>,
}

impl RembgModel {
    fn get_config_dir<'a>() -> Result<&'a PathBuf, Error> {
        static CACHED_CONFIG: OnceLock<Option<PathBuf>> = OnceLock::new();

        let config_dir = CACHED_CONFIG.get_or_init(|| -> Option<PathBuf> {
            if let Some(base_dirs) = BaseDirs::new() {
                let config_dir = if cfg!(target_os = "windows") {
                    base_dirs.data_dir().join("nobg")
                } else {
                    base_dirs.home_dir().join(".nobg")
                };
                Some(config_dir)
            } else {
                None
            }
        });

        match config_dir {
            Some(c) => Ok(c),
            None => Err(Error::FailedToGetBaseDir),
        }
    }

    fn create_config_if_not_exists() -> Result<(), Error> {
        let config_dir = Self::get_config_dir()?;
        if !config_dir.exists() {
            std::fs::create_dir_all(config_dir)?;
        }
        Ok(())
    }

    pub fn check_exists(&self) -> bool {
        match &self.get_path() {
            Ok(v) => v.exists(),
            Err(_) => false,
        }
    }

    pub fn get_path(&self) -> Result<PathBuf, Error> {
        let config_dir = Self::get_config_dir()?;
        let filename = self.get_filename();
        Ok(config_dir.join(filename))
    }

    fn get_filename(&self) -> String {
        format!("{}{}", self.name, ".onnx")
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
        MODELS.iter().find(|&m| m.name == name).map(|v| v as _)
    }

    pub fn print(&self, quiet: bool, verbose: u8) {
        if quiet {
            self.print_quiet();
        } else {
            match verbose {
                0 => self.print_default(),
                1 => self.print_standard(),
                _ => self.print_verbose(),
            }
        }
    }

    fn print_default(&self) {
        println!("{}", self.name);
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
        DOWNLOAD_RETRIES.store(1, Ordering::SeqCst);
        println!("Pulling model {}...", self.name);
        Self::create_config_if_not_exists()?;

        if self.check_exists() {
            let file_path = self.get_path()?;
            if !self.check_checksum()? {
                eprintln!(
                    "Checksum mismatch for model {}, removing corrupted file and re-downloading...",
                    self.name
                );
                std::fs::remove_file(&file_path)?;
                return self.pull();
            }
            return Err(Error::ModelAlreadyDownloaded(self.name.to_owned()));
        }

        let _output_path = self.download_and_verify()?;

        Ok(())
    }

    fn download_and_verify(&self) -> Result<PathBuf, Error> {
        let client = Client::new();
        let output_path = self.get_path()?;
        let mut downloaded: u64 = 0;
        let mut buffer = [0; 8192];

        let mut response = match client.get(self.remote_url).send() {
            Ok(res) => res,
            Err(e) => return Err(Error::Reqwest(e)),
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

        let mut file = File::create(&output_path)?;
        while let Ok(n) = response.read(&mut buffer) {
            if n == 0 {
                break;
            }
            file.write_all(&buffer[..n])?;
            downloaded += n as u64;
            pb.set_position(downloaded);
        }

        pb.finish_with_message("Download complete");

        let checksum_valid = self.check_checksum()?;
        if !checksum_valid {
            std::fs::remove_file(&output_path)?;
            eprintln!(
                "{}",
                format!(
                    "[WARN] Checksum mismatch for model {}, removing corrupted file and re-downloading...",
                    self.name
                ).yellow(),
            );
            let retries = DOWNLOAD_RETRIES.fetch_add(1, Ordering::SeqCst) + 1;
            if retries > 3 {
                return Err(Error::ChecksumMismatch(self.name.to_owned()));
            }
            let _output_path = self.download_and_verify()?;
            return Ok(_output_path);
        }

        Ok(output_path)
    }

    pub fn check_checksum(&self) -> Result<bool, Error> {
        let expected_checksum = match self.checksum {
            Some(c) => c,
            None => {
                eprintln!(
                    "{}",
                    format!(
                        "[WARN] No checksum provided for model {}, skipping verification",
                        self.name
                    )
                    .yellow(),
                );
                return Ok(true);
            }
        };
        let path = self.get_path()?;
        let mut file = File::open(path)?;
        let mut hasher = Sha256::new();
        let mut buffer = [0; 8192];

        loop {
            match file.read(&mut buffer) {
                Ok(0) => break,
                Ok(n) => hasher.update(&buffer[..n]),
                Err(e) => return Err(Error::ModelIO(e)),
            }
        }

        let calculated_checksum = format!("{:x}", hasher.finalize());
        Ok(calculated_checksum.eq_ignore_ascii_case(expected_checksum))
    }

    pub fn rm(&self) -> Result<(), Error> {
        let model_path = self.get_path()?;
        match std::fs::remove_file(&model_path) {
            Ok(()) => {
                println!("Model {} was removed", self.name);
                Ok(())
            }
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Err(Error::ModelNotFound(
                model_path.to_string_lossy().to_string(),
            )),
            Err(e) => Err(Error::ModelIO(e)),
        }
    }
}
