use std::path::PathBuf;

use clap::Parser;

use crate::{
    cli::{ModelSumcommands, TopLevelSumbommands},
    error::Error,
};

mod cli;
mod error;
mod inference;
mod model;

fn main() -> Result<(), Error> {
    let args = cli::Args::parse();
    match args.cmd {
        TopLevelSumbommands::Model { cmd } => model_subcommand(cmd),
        TopLevelSumbommands::Run {
            model,
            input,
            output,
        } => run_model(model, input, output),
        TopLevelSumbommands::Webui { port, expose } => run_webui(port, expose),
    }
}

fn model_subcommand(command: ModelSumcommands) -> Result<(), Error> {
    let result = match command {
        ModelSumcommands::Pull { name } => model::commands::pull(name.as_str()),
        ModelSumcommands::Rm { name } => model::commands::rm(name.as_str()),
        ModelSumcommands::Ls {
            verbose,
            all,
            quiet,
        } => Ok(model::commands::list(verbose, all, quiet)),
        ModelSumcommands::Prune => Ok(model::commands::prune()),
    };
    match result {
        Ok(_) => Ok(()),
        Err(e) => Err(Error::ModelError(e)),
    }
}

fn run_model(model: String, input: PathBuf, output: PathBuf) -> Result<(), Error> {
    inference::commands::cli_run(
        model,
        input.to_string_lossy().to_string(),
        output.to_string_lossy().to_string(),
    )
}

fn run_webui(_port: u16, _expose: bool) -> Result<(), Error> {
    todo!()
}
