# navpp - Simple Fuzzy File Explorer

Dead simple fuzzy file/folder navigator in Rust.

## Install

```bash
git clone https://github.com/ivaansrivastavadev/navpp.git
cd navpp
cargo build --release
sudo mv bin-latest/navpp /usr/local/bin/
```

## Usage

```bash
navpp                 # cd into folder (current dir)
navpp nvim            # open file with nvim (all, current dir)
navpp nvim %          # open file with nvim (files only)
navpp -a cat @        # cat from home (all files/folders)
navpp --help          # show help
```

## Types

- `@` - All files and directories (default)
- `%` - Files only
- `[` - Directories only

## Flags

- `-a` - Scan from home directory instead of current
- `--help, -h` - Show help

## Features

- Image preview with chafa
- Directory preview (ls or tree via `$navppdirside`)
- Fast fuzzy search with fzf
- No dependencies except fzf and chafa

## Dependencies

```bash
# Arch
sudo pacman -S fzf chafa

# Ubuntu/Debian
sudo apt install fzf chafa
```

## License

MIT
