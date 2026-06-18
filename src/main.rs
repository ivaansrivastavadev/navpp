use std::env;
use std::process::Command;

const ALL_TYPE: &str = "@";
const FILE_TYPE: &str = "%";
const FOLDER_TYPE: &str = "[";

fn main() {
    let args: Vec<String> = env::args().collect();

    // Default: navpp -> cd into folder
    if args.len() == 1 {
        execute_navpp("cd", FOLDER_TYPE, ".");
        return;
    }

    // Parse flags and command
    let mut scan_dir = ".".to_string();
    let mut arg_idx = 1;

    // Check for -a flag (scan from home)
    if args.len() > 1 && args[1] == "-a" {
        scan_dir = env::var("HOME").unwrap_or_else(|_| ".".to_string());
        arg_idx = 2;
    }

    // Check for help
    if arg_idx < args.len() && (args[arg_idx] == "--help" || args[arg_idx] == "-h") {
        show_help();
        return;
    }

    // Validate command exists
    if arg_idx >= args.len() {
        eprintln!("navpp: command required. Use 'navpp --help' for usage.");
        return;
    }

    // Parse command and type
    let cmd = &args[arg_idx];
    let file_type = if arg_idx + 1 < args.len() {
        args[arg_idx + 1].as_str()
    } else {
        ALL_TYPE
    };

    execute_navpp(cmd, file_type, &scan_dir);
}

fn show_help() {
    println!(
        r#"
╔══════════════════════════════════════════════════════════════╗
║          navpp - Simple Fuzzy File Explorer v1.0             ║
║                                                               ║
║ USAGE:                                                        ║
║   navpp [FLAGS] [COMMAND] [TYPE]                             ║
║                                                               ║
║ COMMANDS:                                                     ║
║   cd, nvim, nano, cat, less, vim, vi, zeditor, code, gedit  ║
║                                                               ║
║ TYPES:                                                        ║
║   @  - All files and directories (default)                   ║
║   %  - Files only                                            ║
║   [  - Directories only                                      ║
║                                                               ║
║ FLAGS:                                                        ║
║   -a           Scan from home directory (not current)        ║
║   --help, -h   Show this help                                ║
║                                                               ║
║ EXAMPLES:                                                     ║
║   navpp                  # cd into folder (current)          ║
║   navpp -a              # cd into folder (from home)         ║
║   navpp nvim            # open file (all, current)           ║
║   navpp nvim %          # open file (files only)             ║
║   navpp -a cat @        # cat from home (all)                ║
║                                                               ║
║ ENV VARS:                                                     ║
║   navppdirside=tree     # show tree for dirs (default: ls)   ║
║                                                               ║
╚══════════════════════════════════════════════════════════════╝
"#
    );
}

fn get_fd_type(file_type: &str) -> &str {
    match file_type {
        "%" => "-t f",
        "[" => "-t d",
        "@" => "",
        _ => "",
    }
}

fn get_preview_cmd(file_type: &str) -> &str {
    match file_type {
        "%" => "if file {} | grep -qi image; then chafa {} --size 30x20; else head -20 {}; fi",
        "[" => {
            if env::var("navppdirside").as_deref() == Ok("tree") {
                "tree -L 2 {} 2>/dev/null || ls -lah {}"
            } else {
                "ls -lah {}"
            }
        }
        "@" => "if [ -d {} ]; then ls -lah {}; elif file {} | grep -qi image; then chafa {} --size 30x20; else head -20 {}; fi",
        _ => "head -20 {}",
    }
}

fn execute_navpp(cmd: &str, file_type: &str, scan_dir: &str) {
    let fd_type = get_fd_type(file_type);
    let preview = get_preview_cmd(file_type);

    // Build fd command
    let fd_cmd = if fd_type.is_empty() {
        format!("fd -H . '{}'", scan_dir)
    } else {
        format!("fd {} -H . '{}'", fd_type, scan_dir)
    };

    // Build fzf command
    let fzf_cmd = format!("{} | fzf --preview '{}'", fd_cmd, preview);

    // Run fzf and capture selection
    let output = Command::new("sh")
        .arg("-c")
        .arg(&fzf_cmd)
        .output()
        .expect("Failed to run fzf");

    // User cancelled
    if !output.status.success() {
        return;
    }

    let selection = String::from_utf8_lossy(&output.stdout).trim().to_string();

    if selection.is_empty() {
        return;
    }

    // Execute the selected command
    if cmd == "cd" {
        // Change directory using libc
        let c_path = std::ffi::CString::new(selection.clone())
            .expect("Failed to convert path");

        let result = unsafe { libc::chdir(c_path.as_ptr()) };

        if result == 0 {
            // Success - print new directory
            if let Ok(cwd) = env::current_dir() {
                println!("{}", cwd.display());
            }
        } else {
            eprintln!("navpp: failed to change directory");
            std::process::exit(1);
        }
    } else {
        // Execute other commands
        let status = Command::new(cmd)
            .arg(&selection)
            .spawn()
            .unwrap_or_else(|_| {
                eprintln!("navpp: command '{}' not found", cmd);
                std::process::exit(1);
            })
            .wait()
            .expect("Failed to wait for command");

        // Exit with same code as child process
        if let Some(code) = status.code() {
            std::process::exit(code);
        }
    }
}
