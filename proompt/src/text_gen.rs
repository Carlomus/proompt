use std::path::PathBuf;
use std::string::String;
use std::{fs, io};

use ignore::gitignore::{Gitignore, GitignoreBuilder};

use crate::cli::Cli;

pub fn read_files(root: PathBuf, args: &Cli) -> io::Result<String> {
    let gitignore = build_skippers(&root, args)?;
    generate_prompt(&root, &root, &gitignore)
}

fn build_skippers(root: &PathBuf, args: &Cli) -> io::Result<Gitignore> {
    let mut gi_builder = GitignoreBuilder::new(root);
    if !args.include {
        let gi_file = root.join(".gitignore");
        if gi_file.exists() {
            gi_builder.add(gi_file);
        }
    }
    for pat in &args.skip {
        gi_builder.add_line(None, pat).map_err(|e| {
            io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("Invalid skip pattern `{}`: {}", pat, e),
            )
        })?;
    }
    gi_builder.build().map_err(|e| {
        io::Error::new(
            io::ErrorKind::Other,
            format!("Failed to build gitignore matcher: {}", e),
        )
    })
}

fn generate_prompt(root: &PathBuf, og_root: &PathBuf, gitignore: &Gitignore) -> io::Result<String> {
    let mut prompt = String::new();
    for entry_res in fs::read_dir(&root)? {
        let entry = entry_res?;
        let path = entry.path();

        if skip_path(&path, og_root, gitignore) {
            continue;
        }

        if path.is_dir() {
            let sub_prompt = generate_prompt(&path, &og_root, gitignore)?;
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

fn skip_path(path: &PathBuf, og_root: &PathBuf, gi: &Gitignore) -> bool {
    let path_str = path.file_name().and_then(|s| s.to_str());
    let ext_str = path.extension().and_then(|s| s.to_str());
    if path_str == Some(".git")
        || path_str == Some("LICENSE")
        || path_str == Some(".gitignore")
        || ext_str == Some("lock")
    {
        return true;
    }

    if let Ok(rel) = path.strip_prefix(og_root) {
        // use the two-arg API: (relative_path, is_dir)
        if gi.matched(rel, path.is_dir()).is_ignore() {
            return true;
        }
    }

    return false;
}
