<div align="center">
 <img src="https://upload.wikimedia.org/wikipedia/en/b/ba/Hytale_logo.png" width="250">

 <img src="https://upload.wikimedia.org/wikipedia/commons/thumb/2/20/Rustacean-orig-noshadow.svg/1200px-Rustacean-orig-noshadow.svg.png" width="100">
 <h1>Rust Hytale Utility</h1>
</div>

**rhu** is a universal command-line utility for **Hytale**, designed to help you manage your worlds, mods, and other game data efficiently. With Hle, you can easily export/import worlds, delete worlds, manage mods, and perform other administrative tasks — all from the terminal.

--- 

## Features

- **World Management**
  - List all worlds
  - Export worlds to ZIP *(in work)*
  - Import worlds from ZIP *(in work)*
  - Delete worlds safely

- **Mod Management**
  - List installed mods *(in work)*
  - Enable or disable mods *(in work)*
  - Install or remove mods (in work)

- **Configuration & Utilities**
  - Manage configuration files *(in work)*

---

## Installation

Clone the repository and build with Cargo:

```bash
git clone https://github.com/musdev13/rhu.git
cd rhu
cargo build --release
```

The compiled binary will be located in `target/release/rhu`

---

## Usage

Hle uses a modular CLI. Each main category of commands has its own subcommands:
```bash
hle worlds list           # List all Hytale worlds
hle worlds export <name>  # Export a world to ZIP
hle worlds import <file>  # Import a world from ZIP
hle worlds delete <name>  # Delete a world

hle mods list             # List all installed mods
hle mods enable <mod>     # Enable a mod
hle mods disable <mod>    # Disable a mod
hle mods install <mod>    # Install a new mod
hle mods remove <mod>     # Remove a mod
```

### Example

```bash
# List all worlds
hle worlds list

# Export a world called "MyWorld"
hle worlds export MyWorld

# Import a world from a ZIP file
hle worlds import MyWorld.zip

# Delete a world
hle worlds delete MyWorld
```
