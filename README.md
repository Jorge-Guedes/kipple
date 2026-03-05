## 💻 Technologies

[![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org)
[![Linux](https://img.shields.io/badge/Linux-FCC624?style=for-the-badge&logo=linux&logoColor=black)](https://www.linux.org)
[![Windows](https://img.shields.io/badge/Windows-0078D6?style=for-the-badge&logo=windows&logoColor=white)](https://www.microsoft.com/windows)

---

<div align="center">
<h1>Kipple - Terminal File Organizer</h1>
  <img src="assets/kipple-logo.png" alt="Kipple Logo" width="300" height="300">

### 🌍 Choose Language
[**English**](README.md) • [**Español**](README.es.md)
</div>

---

## 🚀 Description

Kipple is a command-line tool written in Rust that automatically organizes your files by extension into categorized folders. One command and the chaos in your downloads disappears.

## ✨ Features

- 📁 **Automatic Organization** - Sorts files by extension into folders (Pictures, Documents, Music, etc.)
- 🔍 **Dry Run Mode** - Preview what will happen without moving anything (`--dry-run`)
- 🎯 **Custom Directory** - Organize any folder, not just the current one (`-d`)
- 📂 **Recursive Processing** - Includes subfolders with `--include-dirs`
- 🗑️ **Automatic Cleanup** - Removes empty folders after organizing
- 🔄 **Duplicate Handling** - Renames files with the same name (`file(1).txt`)
- ⚡ **Force Mode** - Overwrites existing files with `--force`
- 🎭 **Verbose Mode** - See exactly what's happening under the hood with `-v`
- 🎨 **Colorful Output** - Colored messages for errors, warnings and successes

## 📦 Installation

### From Source
```bash
git clone https://github.com/Jorge-Guedes/kipple.git
cd kipple
cargo build --release
cp target/release/kipple ~/.local/bin/  # or /usr/local/bin/ with sudo
```

### Windows Binary Installation

```bash
# Windows (after compiling)
copy target\release\kipple.exe %USERPROFILE%\bin\
```

### Using Cargo
```bash
cargo install --git https://github.com/Jorge-Guedes/kipple.git
```

## 🚀 Usage

```bash
# Organize current directory
kipple

# Organize a specific folder
kipple -d ~/Downloads

# Preview without moving
kipple --dry-run

# With detailed output
kipple -v

# Include subfolders
kipple --include-dirs

# Overwrite duplicate files
kipple --force

# Combine options
kipple -d ~/Downloads --include-dirs -v
```

## ⚙️ Options

| Option | Description | Default |
|--------|-------------|---------|
| `-d, --directory` | Directory to organize | Current directory |
| `--dry-run` | Show what would be done without moving anything | `false` |
| `--include-dirs` | Process subfolders recursively and remove empty ones | `false` |
| `--force` | Overwrite files instead of renaming | `false` |
| `-v, --verbose` | Show detailed progress messages | `false` |
| `-h, --help` | Show help | - |
| `-V, --version` | Show version | - |

## 📁 How It Works

Files are organized by extension into these folders:

| Folder | Extensions |
|--------|------------|
| **📁 Pictures** | `.jpg`, `.jpeg`, `.png`, `.gif`, `.bmp` |
| **📁 Documents** | `.pdf`, `.txt`, `.doc`, `.docx`, `.odt` |
| **📁 Music** | `.mp3`, `.wav`, `.flac`, `.aac` |
| **📁 Videos** | `.mp4`, `.avi`, `.mkv`, `.mov` |
| **📁 Archives** | `.zip`, `.rar`, `.7z`, `.tar`, `.gz` |
| **📁 Code** | `.rs`, `.py`, `.js`, `.ts`, `.html`, `.css`, `.jsx`, `.tsx`, `.json`, `.sql`, `.yml` |
| **📁 Others** | Any other extension or files without extension |

## 🔧 Example

```bash
$ kipple -d ~/Downloads --include-dirs -v

PATH SELECTED: /home/user/Downloads
--------------------------------------------------------------------------------------------------------------
DIR: 📁 /home/user/Downloads/project
--------------------------------------------------------------------------------------------------------------
EVALUATING: photo.jpg (.jpg)
CLASSIFIED: photo.jpg → 📁 pictures
--------------------------------------------------------------------------------------------------------------
MOVED: photo.jpg → /home/user/Downloads/Pictures
--------------------------------------------------------------------------------------------------------------
EVALUATING: document.pdf (.pdf)
CLASSIFIED: document.pdf → 📁 documents
--------------------------------------------------------------------------------------------------------------
MOVED: document.pdf → /home/user/Downloads/Documents
--------------------------------------------------------------------------------------------------------------
WARNING: File with no extension: "/home/user/Downloads/README"
--------------------------------------------------------------------------------------------------------------
DUPLICATE: song.mp3 already exists, looking for alternative...
--------------------------------------------------------------------------------------------------------------
RENAMED: song.mp3 → song(1).mp3
--------------------------------------------------------------------------------------------------------------
MOVED: song(1).mp3 → /home/user/Downloads/Music
--------------------------------------------------------------------------------------------------------------
REMOVED: /home/user/Downloads/project
--------------------------------------------------------------------------------------------------------------
SUCCESS: Files organized in /home/user/Downloads
```


## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.