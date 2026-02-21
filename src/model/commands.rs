use std::path::PathBuf;

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

pub fn prune() {
    // todo!("Add conformation message here, don't display if -y flag");
    for m in MODELS {
        let _ = m.rm();
    }
}

#[allow(dead_code)]
pub fn get_path(name: &str) -> Result<PathBuf, Error> {
    let target_model = RembgModel::find_model(name);
    match target_model {
        Some(m) => {
            let path = m.get_path()?;
            if m.check_exists() {
                Ok(path)
            } else {
                Err(Error::ModelNotFound(path.to_str().unwrap().to_owned()))
            }
        }
        None => Err(Error::ModelNotFound(name.to_owned())),
    }
}
