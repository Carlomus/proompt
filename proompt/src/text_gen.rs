use std::path::PathBuf;
use std::string::String;
use std::{fs, io};

use globset::GlobSet;

pub fn read_files(root: PathBuf, skip_set: &GlobSet) -> io::Result<String> {
    let mut prompt = String::new();

    for entry_res in fs::read_dir(&root)? {
        let entry = entry_res?;
        let path = entry.path();

        if skip_path(&path, &skip_set) {
            continue;
        }

        if path.is_dir() {
            let sub_prompt = read_files(path, &skip_set)?;
            prompt.push_str(&sub_prompt);
        } else {
            let contents = match fs::read_to_string(&path) {
                Ok(s) => s,
                Err(_) => continue,
            };
            let filename = path.to_string_lossy();

            prompt.push_str("\n\nFILE:");
            prompt.push_str(&filename);
            prompt.push('\n');
            prompt.push_str(&contents);
        }
    }
    Ok(prompt)
}

fn skip_path(path: &PathBuf, skip_set: &GlobSet) -> bool {
    let path_str = path.file_name().and_then(|s| s.to_str());
    let ext_str = path.extension().and_then(|s| s.to_str());
    if path_str == Some(".git")
        || path_str == Some("target")
        || path_str == Some("LICENSE")
        || path_str == Some(".gitignore")
        || ext_str == Some("lock")
    {
        return true;
    } else if skip_set.is_match(&path) {
        return true;
    }
    return false;
}
