<p align="center">
  <img src="src/icon/lubig.png" alt="LUBIG Logo" width="200">
</p>

# LUBIG  
**Load, Upgrade & Build Interface for Git repositories**

## 1. Purpose
Tired of the hassle of manually compiling and individually updating your favourite Git repositories?  
**LUBIG** is a portable, package‑manager‑like interface designed to list and manage Git repositories.  
It can update and build your repositories **automatically** — once you’ve written your build instructions in a saved script, of course.

## 2. What is LUBIG for?

LUBIG is a command‑line interface to **load**, **update**, and **build** multiple Git repositories in a controlled, repeatable, and portable way.

It is designed for users who:
- Maintain several Git repositories and want to update them all without manual `git pull` in each one.
- Need to run custom build scripts for each repository after updating.
- Prefer to decide exactly **where** sources and builds are stored, and **how** the directory structure is organised.
- Want a workflow that is **reversible**, **auditable**, and free from hidden state or platform‑specific hacks.

LUBIG does **not** impose a build system or package format.  
Instead, it executes the scripts you define, giving you full control over:
- The build commands
- The output location
- The structure of the final artefacts

In short: **LUBIG automates the repetitive parts of managing and building Git repositories, without taking away your decision‑making power.**

## 3. Intentions
Too little control in Windows installations and too much chaos in Linux community package management?  
I dislike disorder and the lack of control over my own workspace. This project was born from the vision of having both control and order, aiming to create an automated build system that produces portable‑like versions of the programs I love to use.

After a long journey through many technologies, I discovered that a universal solution is impossible — there are simply too many differences between the build processes of open‑source projects.  

Instead, I decided to develop **LUBIG**: a minimal, “fake installer” for Git repositories. It seeks the order of an installer‑like system while giving the user full decision‑making power over:
- Where to store their projects
- How to structure the build process
- What the final build layout should be


**Enjoy it!**


## 4. Features

- **Repository Management**  
  Add, list, update, and remove Git repositories from a central configuration.

- **Script‑Driven Builds**  
  Each repository can have its own build script, defined and stored by the user.  
  LUBIG executes exactly what you specify — no hidden steps, no implicit state.

- **Automatic Updates**  
  Pull the latest changes from all tracked repositories in a single command.

- **Portable Structure**  
  Define where sources, build outputs, and binaries are stored.  
  Keep your workspace organised and reproducible across systems.

- **Cross‑Platform**  
  Works on Linux and Windows (with Git installed), respecting your filesystem layout.

- **Reversible & Auditable**  
  All operations are explicit and logged. Nothing happens without a visible command.

---
## 5. LUBIG Users Manual

### Centralized Configuration
LUBIG uses a single `config.toml` file to store:

- Working directories (`sources`, `profiles`, `programs`)
- Registered repositories
- Lock/unlock state and target branch
- Associated build scripts

Example `config.toml`:
```toml
[Directories]
sources = "/path"
profiles = "/path"
programs = "/path"

[Added]
repo = "/sources/repo"

[Unlocked]
repo = "main"

[Build]
repo = "/profiles/repo"
```

---

### Commands

#### `conf`
Configures working directories:
- **src** → source repositories
- **prof** → build profiles
- **prog** → compiled programs

**Examples:**
```bash
lubig conf src /home/user/dev/src
lubig conf prof /home/user/dev/profiles
lubig conf prog /home/user/dev/programs
```

---

#### `get`
Clones a remote Git repository (default branch) and registers it in LUBIG.

- Rejects if the `custom_name` already exists in the registry.

**Example:**
```bash
lubig get https://github.com/user/project.git myproject
```

---

#### `add`
Registers an already cloned local Git repository.

- Validates that the path is a valid Git repository.
- Rejects if the name is already registered.

**Example:**
```bash
lubig add /home/user/dev/project myproject
```

---

#### `lock`
Locks a registered repository against any updates.

- Can only be applied if the repository is currently **unlocked**.

**Example:**
```bash
lubig lock myproject
```

---

#### `unlock`
Unlocks a repository and optionally sets the target branch for future updates.

- Can be used even if already unlocked.
- Does not perform an immediate checkout; only changes the target branch for `update`.

**Examples:**
```bash
lubig unlock myproject           # uses 'main' by default
lubig unlock myproject develop   # sets 'develop' as target branch
```

---

#### `upgrade`
Updates all unlocked repositories using `git pull --ff-only` toward the branch set with `unlock`.

- Ignores locked repositories.
- No merges; fast‑forward only.

**Example:**
```bash
lubig upgrade
```

---

#### `build`
Runs the build script associated with a repository.

- Looks for the script in the `profiles` directory set with `conf prof`.
- Script name must match the registered name + `.sh` or `.bat`.
- Creating the folder and placing the script is manual.
- Fails if no script is found.
- The only argument passed to the script is the output path for the build.

**Example:**
```bash
lubig build myproject
```

---

#### `remove`
Removes everything associated with a registered repository:
1. Build folder
2. Source folder
3. Build script
4. Registry entry

**Example:**
```bash
lubig remove myproject
```

---

#### `list`
Shows the paths of all registered repositories.

**Example:**
```bash
lubig list
```

---

#### `status`
Displays:
- Whether the repository has been built.
- Whether it is open or closed to updates.

**Example:**
```bash
lubig status myproject
```

---

### Additional Notes
- Registered names must be unique.
- Paths set with `conf` are absolute.
- The `config.toml` file is the single source of truth for state and paths.
- Build scripts must have the same name as the registered repository plus `.bat` or `.sh` extension.

---

## General Requirements and Build Process

- [Rust and Cargo](https://www.rust-lang.org/tools/install)  
- Git (to clone the repository)

---

#### Debian / Ubuntu

```bash
# Install dependencies
sudo apt update
sudo apt install -y curl git build-essential

# Install Rust (includes cargo)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Clone and build
git clone https://github.com/<user>/lubig.git
cd lubig
cargo build --release
```

> Binary generated at: `./target/release/lubig`

---

#### Windows (PowerShell)

```powershell
# Install Rust (includes cargo)
Invoke-WebRequest -Uri https://win.rustup.rs -OutFile rustup-init.exe
Start-Process .\rustup-init.exe -Wait

# Clone and build
git clone https://github.com/<user>/lubig.git
cd lubig
cargo build --release
```

> Binary generated at: `.\target\release\lubig.exe`

---

#### MacOS

```bash
# Install Xcode CLI tools (if not already installed)
xcode-select --install

# Install Homebrew (if not already installed)
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"

# Install Git and Rust
brew install git
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Clone and build
git clone https://github.com/<user>/lubig.git
cd lubig
cargo build --release
```

> Binary generated at: `./target/release/lubig`

---

#### Arch Linux

```bash
# Install dependencies
sudo pacman -Syu --noconfirm
sudo pacman -S --noconfirm rust git base-devel

# Clone and build
git clone https://github.com/<user>/lubig.git
cd lubig
cargo build --release
```

> Binary generated at: `./target/release/lubig`

---

## Project Structure

- `src/` — Source modules  
- `Cargo.toml` — Explicit declaration of name, version, and dependencies  
- `target/` — Build artifacts (not versioned)  
- `README.md` — This document

---
