# 📋 lfd - Line For Do

**Execute a command for each line in a file**

`lfd` is a simple yet powerful command-line tool that reads a file line by line and executes a specified command for each line, replacing a variable placeholder with the line content.

## ✨ Features

- 🚀 **Fast and efficient** - Written in Rust
- 🎨 **Colorful output** - Easy to read execution logs
- 🔇 **Quiet mode** - Show only command output when needed
- 📊 **Detailed statistics** - Track success and failures
- 🛡️ **Error handling** - Graceful error management
- 💡 **Simple syntax** - Intuitive command structure

## 📦 Installation

### From Source

```bash
# Clone the repository
git clone https://github.com/yourusername/lfd.git
cd lfd

# Build with cargo
cargo build --release

# Install binary
sudo cp target/release/lfd /usr/local/bin/

# Or just compile directly
rustc lfd.rs -o lfd
sudo mv lfd /usr/local/bin/
```

### Shell Completion (Optional)

```bash
# Bash
sudo cp completions/lfd-completion.bash /etc/bash_completion.d/lfd
source ~/.bashrc

# Zsh
mkdir -p ~/.zsh/completions
cp completions/lfd-completion.zsh ~/.zsh/completions/_lfd
echo 'fpath=(~/.zsh/completions $fpath)' >> ~/.zshrc
source ~/.zshrc

# Fish
cp completions/lfd.fish ~/.config/fish/completions/
```

## 🚀 Usage

```bash
lfd [-q] <file> <variable> <command> [args...]
```

### Arguments

- **file** - Path to the input file (one item per line)
- **variable** - Placeholder name to be replaced in the command
- **command** - Command to execute with its arguments

### Options

- `-q, --quiet` - Quiet mode (only show command output)
- `-h, --help` - Show help message
- `-v, --version` - Show version

## 📝 Examples

### Download URLs from a file

```bash
# urls.txt contains:
# https://example.com
# https://rust-lang.org
# https://github.com

lfd urls.txt URL curl -s -o URL.html URL
```

### Create multiple directories

```bash
# dirs.txt contains:
# project1
# project2
# project3

lfd dirs.txt DIR mkdir -p /tmp/DIR
```

### Convert images in batch

```bash
# images.txt contains:
# photo1.jpg
# photo2.jpg
# photo3.jpg

lfd images.txt IMG convert IMG IMG.png
```

### Search for terms in files

```bash
# terms.txt contains:
# error
# warning
# critical

lfd terms.txt TERM grep -i TERM /var/log/*.log
```

### Process files with multiple replacements

```bash
# files.txt contains:
# document.txt
# report.txt

# The variable can appear multiple times
lfd files.txt FILE cp FILE FILE.bak
```

## 🔇 Quiet Mode

Use `-q` or `--quiet` to suppress lfd's own output and only show the command's output:

```bash
# Normal mode - shows lfd progress and status
lfd urls.txt URL curl -s URL

# Quiet mode - only shows curl output
lfd -q urls.txt URL curl -s URL

# Perfect for piping
lfd -q files.txt FILE cat FILE | grep "pattern"

# Or redirecting to a file
lfd -q urls.txt URL curl -s URL > combined_output.html
```

## 🎨 Output Example

**Normal Mode:**
```
🚀 Starting lfd

📄 File:     urls.txt
🔤 Variable: URL
⚙️  Command:  curl -s -o URL.html URL
────────────────────────────────────────────────────────────

[Line 1] ▶️  https://example.com
   $ curl -s -o https://example.com.html https://example.com
   ✅ Success

[Line 2] ▶️  https://rust-lang.org
   $ curl -s -o https://rust-lang.org.html https://rust-lang.org
   ✅ Success

────────────────────────────────────────────────────────────
📊 Summary
📝 Lines processed: 2
✅ Successful:     2
❌ Errors:         0

🎉 All commands executed successfully!
```

## ⚙️ How It Works

1. **Reads** the input file line by line
2. **Skips** empty lines automatically
3. **Replaces** the variable placeholder with each line's content
4. **Executes** the command with the replaced values
5. **Reports** success or failure for each execution
6. **Summarizes** the results at the end

## 🛠️ Building from Source

Requirements:
- Rust 1.70 or higher

```bash
# Clone the repository
git clone https://github.com/yourusername/lfd.git
cd lfd

# Run tests (if available)
cargo test

# Build release version
cargo build --release

# Binary will be at target/release/lfd
```

## 📄 License

MIT License - feel free to use and modify as needed.

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## 💡 Tips

- **Empty lines** in the input file are automatically skipped
- The **variable name** can appear multiple times in the command
- Use **quotes** around commands with spaces: `lfd file.txt X sh -c "echo X && ls"`
- Combine with **pipes** and redirections in quiet mode for powerful workflows
- Check the **exit status** of lfd to detect if any commands failed

## 🐛 Troubleshooting

**Command not found:**
- Make sure the command is in your PATH
- Try using the full path: `/usr/bin/curl` instead of `curl`

**Permission denied:**
- Check file permissions with `ls -l`
- Use `sudo` if needed (but be careful!)

**Variable not replaced:**
- Make sure the variable name exactly matches
- Variable names are case-sensitive

## 📚 Similar Tools

- `xargs` - Standard Unix tool for building command lines
- `parallel` - GNU parallel execution tool
- `fd` + `xargs` - Modern find alternative with xargs

**Why lfd?** Simple, colorful, and intuitive syntax specifically designed for line-by-line file processing.

---

Made with ❤️ and 🦀 Rust
