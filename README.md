# proompt

`proompt` is a lightweight Rust CLI tool that gathers source files from a project and copies them into your system clipboard (or prints them) in a format suitable for feeding into AI models. It traverses your project directory (by default, the Git repository root) and concatenates file contents, respecting `.gitignore`, custom skip patterns, and other filters.

## Features

* **Auto-detect project root**: walks up the directory tree to find the `.git` folder.
* **Clipboard integration**: copies the generated prompt directly to the system clipboard (Wayland support via `copypasta-ext`).
* **Filtering**:

  * Respect your `.gitignore` (unless `--include` is set).
  * Skip files or directories via  (`--skip`).
* **Flexible output**:

  * Print to stdout with `-p`/`--print`.
  * Optionally include files normally ignored by Git.

## Installation

Make sure you have Rust and Cargo installed. Then:

```bash
cargo install --path .
```

This will build and install the `proompt` binary into your Cargo bin directory (e.g., `~/.cargo/bin`).

## Usage

```bash
proompt [OPTIONS]
```

### Options

* `--path <PATH>`

  Specify an explicit root directory to scan. If omitted, `proompt` locates the closest parent directory containing a `.git` folder.

* `-p, --print`

  Print the composed prompt to stdout.

* `-i, --include`

  Include files in your `.gitignore` rather than skipping them.

* `--skip <PATTERN>...`

  One or more glob patterns to skip. Patterns are matched relative to the project root. For example:

  ```bash
  proompt --skip "target/*" "*.lock"
  ```

### Examples

* Copy all tracked sources into clipboard:

  ```bash
  proompt
  ```

* Print the prompt and include ignored files:

  ```bash
  proompt --print --include
  ```

* Scan a custom directory and skip build artifacts:

  ```bash
  proompt --path /my/project --skip "target/*" "build/*"
  ```

## Upcoming Features

* **Multi-directory **\`\`** support**: respect nested ignore files in subdirectories.
* **Forced include files**: allow overruling ignore/skips for specific files.
* **Single-file or few-files mode**: pick and include just one or a small number of files by name or pattern.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
