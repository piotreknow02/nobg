use std::path::PathBuf;

use clap::Parser;

use crate::{cli::TopLevelSubcommands, error::Error};

mod cli;
mod error;
mod inference;
mod model;
mod webui;
mod webui_assets;

fn main() -> Result<(), Error> {
    let args = cli::Args::parse();
    match args.cmd {
        TopLevelSubcommands::Pull { models } => Ok(model::commands::pull(models)?),
        TopLevelSubcommands::Run {
            model,
            input,
            output,
        } => Ok(run_model(model, input, output)?),
        TopLevelSubcommands::Rm { models } => Ok(model::commands::rm(models)?),
        TopLevelSubcommands::Ls {
            verbose,
            all,
            quiet,
        } => Ok(model::commands::list(verbose, all, quiet)?),
        TopLevelSubcommands::Prune { yes } => Ok(model::commands::prune(yes)?),
        TopLevelSubcommands::Webui { port, expose } => Ok(run_webui(port, expose)?),
    }
}

fn run_model(model: String, input: PathBuf, output: Option<PathBuf>) -> Result<(), Error> {
    let output_path = if let Some(out) = output {
        out.clone()
    } else {
        let input_filename = input.file_name().unwrap_or_default().to_string_lossy();
        let mut output_buf = PathBuf::new();
        if let Some(parent) = input.parent() {
            output_buf.push(parent);
        }
        output_buf.push(format!("{}_nobg", input_filename));
        output_buf
    };
    inference::commands::cli_run(
        model,
        input.to_string_lossy().to_string(),
        output_path.to_string_lossy().to_string(),
    )
}

fn run_webui(port: u16, _expose: bool) -> Result<(), Error> {
    tokio::runtime::Runtime::new()
        .map_err(|e| Error::WebUi(format!("Failed to create runtime: {}", e)))?
        .block_on(webui::start(port))
        .map_err(|e| Error::WebUi(format!("WebUI error: {}", e)))
}
