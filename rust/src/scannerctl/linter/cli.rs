use std::path::{Path, PathBuf};

use scannerlib::nasl::{
    Loader,
    syntax::{LoadError, load_non_utf8_path},
};

use crate::error::CliError;

#[derive(clap::Parser)]
pub struct LinterArgs {
    /// Either a single NASL file or a directory of NASL files on which to run the linter.
    pub path: PathBuf,
}

pub struct NonUtf8Loader;

impl Loader for NonUtf8Loader {
    fn load(&self, key: &str) -> Result<String, LoadError> {
        load_non_utf8_path(key)
    }

    fn root_path(&self) -> Result<String, LoadError> {
        Ok(".".to_owned())
    }
}

pub(super) fn get_files(path: &Path) -> Result<Vec<PathBuf>, CliError> {
    let mut files = vec![];
    if path.is_file() {
        files.push(path.into())
    } else {
        for e in walkdir::WalkDir::new(&path) {
            let e = e.map_err(|e| std::io::Error::from(e))?;
            if let Some("nasl") | Some("inc") = e.path().extension().and_then(|ext| ext.to_str()) {
                files.push(e.path().to_owned())
            }
        }
    }
    Ok(files)
}
