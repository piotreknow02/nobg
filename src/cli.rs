use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "nobg")]
#[command(version, about="A cli for rembg models", long_about = None)]
pub struct Args {
    #[command(subcommand)]
    cmd: Option<TopLevelSumbommands>,

    input: Option<PathBuf>,
    output: Option<PathBuf>,
}

#[derive(Subcommand, Debug)]
enum TopLevelSumbommands {
    Model {
        #[command(subcommand)]
        cmd: ModelSumcommands,
    },
    Webui {
        #[clap(short, long, default_value_t = 8080)]
        port: u16,
    },
}

#[derive(Subcommand, Debug, Clone)]
enum ModelSumcommands {
    Get { name: String },
    Remove { name: String },
    List,
}
