mod cli;
mod paths;
mod text_gen;

use clap::Parser;
use cli::Cli;
use paths::get_prompt_root;
use text_gen::read_files;

use copypasta_ext;
use copypasta_ext::ClipResult;

use std::io::{self, ErrorKind};

fn copy_to_clipboard(s: String) -> ClipResult<()> {
    let mut ctx = copypasta_ext::try_context()
        .ok_or_else(|| io::Error::new(ErrorKind::Other, "No clipboard context available"))?;
    ctx.set_contents(s)?;
    Ok(())
}

fn main() -> io::Result<()> {
    let args = Cli::parse();

    let root = get_prompt_root(&args)?;
    let prompt = read_files(root, &args)?;
    if args.print {
        println!("{}", prompt);
    }
    let _outcome = match copy_to_clipboard(prompt) {
        Ok(f) => f,
        Err(error) => panic!("Could not add prompt to clipboard: {error:?}"),
    };
    Ok(())
}
