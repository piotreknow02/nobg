use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "nobg")]
#[command(version, about="", long_about = None)]
struct Args {
    #[command(subcommand)]
    cmd: Option<TopLevelSumbommands>,

    // model:
    input: PathBuf,
    output: PathBuf,
}

#[derive(Subcommand, Debug)]
enum TopLevelSumbommands {
    Model {
        cmd: ModelSumcommands,
    },
    WebUI {
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
