use crate::model::{error::Error, registry::MODELS, types::RembgModel};

pub fn list(verbose: bool, all: bool, quiet: bool) {
    if all {
        for m in MODELS {
            m.print(quiet, verbose);
        }
    } else {
        for m in MODELS {
            if m.check_exists() {
                m.print(quiet, verbose);
            }
        }
    }
}

pub fn pull(name: &str) -> Result<(), Error> {
    let target_model = RembgModel::find_model(name);
    match target_model {
        Some(m) => m.pull(),
        None => Err(Error::ModelNotFound(name.to_owned())),
    }
}

pub fn rm(name: &str) -> Result<(), Error> {
    let target_model = RembgModel::find_model(name);
    match target_model {
        Some(m) => m.rm(),
        None => Err(Error::ModelNotFound(name.to_owned())),
    }
}

pub fn prune(yes: bool) {
    if !yes {
        println!("This will remove all downloaded models. Would you like to continue? [y/N]");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).ok();
        if !input.trim().eq_ignore_ascii_case("y") {
            println!("Aborted.");
            return;
        }
    }

    for m in MODELS {
        let _ = m.rm();
    }
}
