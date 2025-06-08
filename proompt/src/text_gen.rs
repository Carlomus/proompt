use std::path::PathBuf;
use std::string::String;
use std::{fs, io};

use globset::{GlobBuilder, GlobSet, GlobSetBuilder};
use ignore::gitignore::{Gitignore, GitignoreBuilder};

use crate::cli::Cli;

pub fn read_files(root: PathBuf, args: &Cli) -> io::Result<String> {
    let (skip_set, gitignore) = build_skippers(&root, args);
    generate_prompt(&root, &root, &skip_set, gitignore.as_ref())
}

fn build_skippers(root: &PathBuf, args: &Cli) -> (GlobSet, Option<Gitignore>) {
    let mut builder = GlobSetBuilder::new();
    for pat in &args.skip {
        builder.add(
            GlobBuilder::new(pat)
                .literal_separator(true)
                .build()
                .unwrap_or_else(|e| panic!("Invalid skip glob `{}`: {}", pat, e)),
        );
    }
    let skip_set = builder.build().unwrap();

    let gitignore = if !args.include {
        let gi_file = root.join(".gitignore");
        if gi_file.exists() {
            let mut gi_builder = GitignoreBuilder::new(root);
            gi_builder.add(gi_file);
            let gi = gi_builder.build().unwrap();
            Some(gi)
        } else {
            None
        }
    } else {
        None
    };

    (skip_set, gitignore)
}

fn generate_prompt(
    root: &PathBuf,
    og_root: &PathBuf,
    skip_set: &GlobSet,
    gitignore: Option<&Gitignore>,
) -> io::Result<String> {
    let mut prompt = String::new();
    for entry_res in fs::read_dir(&root)? {
        let entry = entry_res?;
        let path = entry.path();

        if skip_path(&path, &og_root, &skip_set, gitignore) {
            continue;
        }

        if path.is_dir() {
            let sub_prompt = generate_prompt(&path, &og_root, &skip_set, gitignore)?;
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

fn skip_path(
    path: &PathBuf,
    og_root: &PathBuf,
    skip_set: &GlobSet,
    gitignore: Option<&Gitignore>,
) -> bool {
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
        if skip_set.is_match(rel) {
            return true;
        }
        if let Some(gi) = gitignore {
            if gi.matched(rel, path.is_dir()).is_ignore() {
                return true;
            }
        }
    }

    return false;
}
