use crate::model::{error::Error, registry::MODELS, types::RembgModel};

pub fn list(verbose: u8, all: bool, quiet: bool) {
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

pub fn pull(models: Vec<String>) -> Result<(), Error> {
    for model_name in &models {
        let target_model = RembgModel::find_model(model_name);
        match target_model {
            Some(m) => match m.pull() {
                Err(Error::ModelAlreadyDownloaded(_)) => continue,
                Err(e) => return Err(e),
                Ok(()) => {}
            },
            None => return Err(Error::ModelNotFound(model_name.clone())),
        }
    }

    Ok(())
}

pub fn rm(models: Vec<String>) -> Result<(), Error> {
    for model_name in &models {
        let target_model = RembgModel::find_model(model_name);
        match target_model {
            Some(m) => m.rm()?,
            None => return Err(Error::ModelNotFound(model_name.clone())),
        }
    }

    Ok(())
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
