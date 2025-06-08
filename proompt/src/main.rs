mod cli;
mod paths;
mod text_gen;

use clap::Parser;
use cli::Cli;
use globset::{Glob, GlobSetBuilder};
use paths::get_prompt_root;
use text_gen::read_files;

use arboard::Clipboard;
use std::io;

fn copy_to_clipboard(s: &str) {
    let mut ctx = Clipboard::new().expect("could not initialize clipboard context");
    ctx.set_text(s.to_string())
        .expect("failed to copy text to clipboard");
}

fn main() -> io::Result<()> {
    let args = Cli::parse();

    let mut builder = GlobSetBuilder::new();
    for pat in &args.skip {
        // This will panic immediately if `pat` is not a valid glob
        builder
            .add(Glob::new(pat).unwrap_or_else(|e| panic!("Invalid --skip glob `{}`: {}", pat, e)));
    }

    let skip_set = builder.build().unwrap();
    let root = get_prompt_root(&args)?;
    let prompt = read_files(root, &skip_set)?;
    if args.print {
        println!("{}", prompt);
    }
    copy_to_clipboard(&prompt);
    Ok(())
}
