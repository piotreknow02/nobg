use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "nobg")]
#[command(version, about="A cli for rembg models", long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub cmd: TopLevelSubcommands,
}

#[derive(Subcommand, Debug)]
pub enum TopLevelSubcommands {
    Run {
        model: String,
        input: PathBuf,
        output: PathBuf,
    },
    Model {
        #[command(subcommand)]
        cmd: ModelSubcommands,
    },
    Webui {
        #[clap(short, long, default_value_t = 8080)]
        port: u16,

        #[clap(short, long, action)]
        expose: bool,
    },
}

#[derive(Subcommand, Debug, Clone)]
pub enum ModelSubcommands {
    Pull {
        names: Vec<String>,
    },
    Rm {
        names: Vec<String>,
    },
    Ls {
        #[clap(short = 'v', long, action = clap::ArgAction::Count)]
        verbose: u8,

        #[clap(short, long, action)]
        all: bool,

        #[clap(short, long, action)]
        quiet: bool,
    },
    Prune {
        #[clap(short, long, action)]
        yes: bool,
    },
}
