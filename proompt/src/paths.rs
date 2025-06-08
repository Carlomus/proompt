use std::path::PathBuf;
use std::{env, io};

use crate::cli::Cli;

fn find_root() -> io::Result<PathBuf> {
    let cwd = env::current_dir()?;
    for dir in cwd.ancestors() {
        if dir.join(".git").is_dir() {
            return Ok(dir.to_path_buf());
        }
    }
    Ok(cwd)
}

pub fn get_prompt_root(args: &Cli) -> io::Result<PathBuf> {
    if let Some(path) = &args.path {
        if path.exists() {
            Ok(path.clone())
        } else {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("Supplied path: {:?} does not exist", path),
            ));
        }
    } else {
        find_root()
    }
}
