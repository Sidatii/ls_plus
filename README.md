# LS_PLUS

- Author: Sidati NOUHI - 2025


A modern replacement for the classic `ls` command, written in Rust. `ls_plus` provides enhanced directory listing capabilities with improved performance and additional features.

## Features

- Fast and efficient directory listing
- User-friendly output
- Safe: does not overwrite system `ls` by default

## Installation

0. you can use the install script or manually install using the steps below.

```bash
cd ls_plus
./install.sh
```



1. **Build from source:**
   ```sh
   git clone git@github.com:Sidatii/ls_plus.git
   cd ls_plus
   cargo build --release
   ```

2. **Install for your user:**
   ```sh
   mkdir -p ~/bin # do this if the directory does not already exist.
   cp target/release/ls_plus ~/bin/
   ```

3. **Add to your PATH (if not already):**
   Add this to your `~/.bashrc` or `~/.zshrc`:
   ```sh
   export PATH="$HOME/bin:$PATH"
   ```

4. **(Optional) Alias `ls` to `ls_plus`:**
   Add this to your shell config:
   ```sh
   alias ls='ls_plus'
   ```

## Usage

```sh
ls_plus [OPTIONS] [PATH]
```

## License

OPEN
