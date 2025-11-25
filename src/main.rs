use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process::{Command, exit};

// ANSI color codes
const RESET: &str = "\x1b[0m";
const BOLD: &str = "\x1b[1m";
const RED: &str = "\x1b[31m";
const GREEN: &str = "\x1b[32m";
const YELLOW: &str = "\x1b[33m";
const BLUE: &str = "\x1b[34m";
const MAGENTA: &str = "\x1b[35m";
const CYAN: &str = "\x1b[36m";

fn print_help(program_name: &str) {
    println!("{}{}📋 lfd - Line For Do{}", BOLD, CYAN, RESET);
    println!("{}Execute a command for each line in a file{}\n", BLUE, RESET);
    
    println!("{}Usage:{}", BOLD, RESET);
    println!("  {} {}[-q] <file> <variable> <command> [args...]{}\n", 
             program_name, YELLOW, RESET);
    
    println!("{}Arguments:{}", BOLD, RESET);
    println!("  {}file{}      📄 Path to the file containing data (one per line)", CYAN, RESET);
    println!("  {}variable{}  🔤 Variable name to replace in the command", CYAN, RESET);
    println!("  {}command{}   ⚙️  Command to execute with its arguments", CYAN, RESET);
    
    println!("\n{}Options:{}", BOLD, RESET);
    println!("  {}-q, --quiet{}   🔇 Quiet mode (only show command output)", CYAN, RESET);
    println!("  {}-h, --help{}    📖 Show this help", CYAN, RESET);
    println!("  {}-v, --version{} 🔖 Show version", CYAN, RESET);
    
    println!("\n{}Examples:{}", BOLD, RESET);
    println!("  {}💾 Download URLs:{}", GREEN, RESET);
    println!("    {} urls.txt URL curl -s -o URL.html URL", program_name);
    
    println!("\n  {}📁 Create directories:{}", GREEN, RESET);
    println!("    {} dirs.txt DIR mkdir -p /tmp/DIR", program_name);
    
    println!("\n  {}🖼️  Convert images:{}", GREEN, RESET);
    println!("    {} images.txt IMG convert IMG IMG.png", program_name);
    
    println!("\n  {}🔍 Search in files:{}", GREEN, RESET);
    println!("    {} terms.txt TERM grep -i TERM *.txt", program_name);
    
    println!("\n{}Note:{} Empty lines are automatically ignored.", YELLOW, RESET);
}

fn print_version() {
    println!("{}lfd{} version {}1.0.0{} 🚀", BOLD, RESET, GREEN, RESET);
    println!("Line For Do - Batch command executor");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    // Check for help and version options
    if args.len() >= 2 {
        match args[1].as_str() {
            "-h" | "--help" => {
                print_help(&args[0]);
                exit(0);
            }
            "-v" | "--version" => {
                print_version();
                exit(0);
            }
            _ => {}
        }
    }

    // Check for quiet mode
    let mut quiet = false;
    let mut arg_offset = 1;
    
    if args.len() >= 2 && (args[1] == "-q" || args[1] == "--quiet") {
        quiet = true;
        arg_offset = 2;
    }

    if args.len() < arg_offset + 3 {
        eprintln!("{}❌ Error: Missing arguments{}\n", RED, RESET);
        print_help(&args[0]);
        exit(1);
    }

    let filepath = &args[arg_offset];
    let variable = &args[arg_offset + 1];
    let cmd_args = &args[arg_offset + 2..];

    if !quiet {
        println!("\n{}{}🚀 Starting lfd{}\n", BOLD, MAGENTA, RESET);
        println!("{}📄 File:{}     {}", CYAN, RESET, filepath);
        println!("{}🔤 Variable:{} {}", CYAN, RESET, variable);
        println!("{}⚙️  Command:{}  {}", CYAN, RESET, cmd_args.join(" "));
        println!("{}", "─".repeat(60));
    }

    // Open and read the file
    let file = match File::open(filepath) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("\n{}❌ Error opening file '{}': {}{}", 
                     RED, filepath, e, RESET);
            exit(1);
        }
    };

    let reader = BufReader::new(file);
    let mut line_num = 0;
    let mut success_count = 0;
    let mut error_count = 0;

    // For each line in the file
    for line in reader.lines() {
        line_num += 1;
        
        let line = match line {
            Ok(l) => l.trim().to_string(),
            Err(e) => {
                if !quiet {
                    eprintln!("{}⚠️  Error line {}: {}{}", YELLOW, line_num, e, RESET);
                }
                error_count += 1;
                continue;
            }
        };

        // Skip empty lines
        if line.is_empty() {
            continue;
        }

        // Replace the variable with the line value in all arguments
        let replaced_args: Vec<String> = cmd_args
            .iter()
            .map(|arg| arg.replace(variable, &line))
            .collect();

        if replaced_args.is_empty() {
            eprintln!("{}❌ No command specified{}", RED, RESET);
            exit(1);
        }

        let command = &replaced_args[0];
        let args = &replaced_args[1..];

        if !quiet {
            println!("\n{}[{}Line {}{}] {}▶️  {}{}",
                     BLUE, BOLD, line_num, RESET, BLUE, line, RESET);
            println!("{}   {}$ {} {}{}", 
                     BLUE, RESET, command, args.join(" "), RESET);
        }

        // Execute the command
        let result = if quiet {
            // In quiet mode, inherit stdout/stderr to show command output directly
            Command::new(command)
                .args(args)
                .status()
        } else {
            // In verbose mode, capture output (command runs silently)
            Command::new(command)
                .args(args)
                .status()
        };

        match result {
            Ok(status) => {
                if status.success() {
                    if !quiet {
                        println!("{}   ✅ Success{}", GREEN, RESET);
                    }
                    success_count += 1;
                } else {
                    if !quiet {
                        eprintln!("{}   ❌ Failed (code: {:?}){}", 
                                 RED, status.code(), RESET);
                    }
                    error_count += 1;
                }
            }
            Err(e) => {
                if !quiet {
                    eprintln!("{}   ❌ Execution error: {}{}", RED, e, RESET);
                }
                error_count += 1;
            }
        }
    }

    // Final summary
    if !quiet {
        println!("\n{}", "─".repeat(60));
        println!("{}{}📊 Summary{}", BOLD, MAGENTA, RESET);
        println!("{}📝 Lines processed:{} {}", CYAN, RESET, line_num);
        println!("{}✅ Successful:{}     {}{}{}", CYAN, RESET, GREEN, success_count, RESET);
        println!("{}❌ Errors:{}         {}{}{}", CYAN, RESET, RED, error_count, RESET);
        
        if error_count == 0 {
            println!("\n{}🎉 All commands executed successfully!{}", GREEN, RESET);
        } else {
            println!("\n{}⚠️  Some commands failed.{}", YELLOW, RESET);
        }
        println!();
    }
}
