pub enum Error {
    #[error(format!("Cannot access the app directory {}", if cfg!("windows") {"~/AppData/nobg/"} else {"~/.nobg/"}))]
    FailedToGetBaseDir,

    ModelAllreadyDownloaded,
}
