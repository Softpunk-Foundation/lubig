// Module declarations: these bring in the other source files.
mod text;   // User-facing messages and help text
mod util;   // Local and remote utility functions
mod conf;   // Configuration management
mod func;   // Core functional operations

use std::{env, path::Path};

use text::Text;
use util::LocalStuff;
use conf::Config;
use func::Execute;

/// Main entry point for the LUBIG CLI.
fn main() {
    // Collect all command-line arguments into a vector.
    let args: Vec<String> = env::args().collect();

    // Ensure a config file exists; generate a default one if missing.
    if !conf::conf_path().exists() {
        conf::generate_config().ok();
    }

    // Match the first argument (command) and route to the appropriate handler.
    match args.get(1).map(|s| s.as_str()) {
        Some("conf")    => Validate::conf(args),
        Some("get")     => Validate::get(args),
        Some("add")     => Validate::add(args),
        Some("lock")    => Validate::lock(args),
        Some("unlock")  => Validate::unlock(args),
        Some("upgrade") => Validate::upgrade(args),
        Some("build")   => Validate::build(args),
        Some("list")    => Validate::list(args),
        Some("status")  => Validate::status(args),
        Some("remove")  => Validate::remove(args),
        Some("help")    => Validate::help(args),
        Some(_)         => Text::general_error(), // Unknown command
        None            => Text::need_args(),     // No command provided
    }
}

/// Command validator and dispatcher.
/// Each method checks argument count/validity before calling the core logic.
pub struct Validate;

impl Validate {
    /// Configure directory paths for sources, profiles, or programs.
    pub fn conf(cmd: Vec<String>){
        if !LocalStuff::cmd_len(&cmd, 4) { return; }

        // Map shorthand to config keys.
        let next = match cmd.get(2).map(|s| s.as_str()) {
            Some("src")  => "sources",
            Some("prof") => "profiles",
            Some("prog") => "programs",
            Some(_)      => { Text::general_error(); return; },
            None         => { Text::need_args(); return; },
        };

        // Validate the provided path.
        if !LocalStuff::usable_path(&cmd[3]) {
            return;
        }

        // Update configuration.
        let mut config = Config::load_config().unwrap_or_default();
        config.modify_and_save("Directories", next, &cmd[3]).ok();
    }

    /// Clone a remote Git repository and register it.
    pub fn get(cmd: Vec<String>){
        if !LocalStuff::cmd_len(&cmd, 4) { return; }

        let config = Config::load_config().unwrap_or_default();

        // Prevent duplicate registration.
        if config.key_exists("Added", &cmd[3]) {
            Text::key_exists(&cmd[3]);
            return;
        }

        Execute::get(&cmd[2], &cmd[3]).ok();
    }

    /// Register an existing local Git repository.
    pub fn add(cmd: Vec<String>){
        if !LocalStuff::cmd_len(&cmd, 4) { return; }

        // Validate path existence.
        if !Path::new(&cmd[2]).exists() {
            Text::error_dir(&cmd[2]);
            return;
        }

        // Ensure it's a Git repository.
        if !LocalStuff::is_git_repo(&cmd[2]) {
            Text::no_git_repo(&cmd[2]);
            return;
        }

        let config = Config::load_config().unwrap_or_default();

        // Prevent duplicate registration.
        if config.key_exists("Added", &cmd[3]) {
            Text::key_exists(&cmd[3]);
            return;
        }

        Execute::add(&cmd[2], &cmd[3]);
    }

    /// Lock a registered repository to prevent updates.
    pub fn lock(cmd: Vec<String>) {
        if !LocalStuff::cmd_len(&cmd, 3) { return; }

        let mut config = Config::load_config().unwrap_or_default();

        if !config.key_exists("Added", &cmd[2]) {
            Text::key_doesnt_exists(&cmd[2]);
            return;
        }

        // If currently unlocked, remove from unlocked list.
        if config.key_exists("Unlocked", &cmd[2]) {
            config.remove_and_save("Unlocked", &cmd[2]).ok();
            println!("SUCCESS: '{}' was lock for updates.", &cmd[2]);
        } else {
            println!("Error: '{}' is already locked for updates.", &cmd[2]);
        }
    }

    /// Unlock a repository for updates, optionally specifying a branch.
    pub fn unlock(cmd: Vec<String>) {
        if cmd.len() < 3 {
            Text::need_args(); 
            return; 
        } else if cmd.len() > 4 {
            Text::exceed_args();
            return;
        }

        // Default branch is "main" if not provided.
        let branch = match cmd.get(3).map(|s| s.as_str()) {
            Some(_) => &cmd[3],
            None    => "main",
        };

        let mut config = Config::load_config().unwrap_or_default();

        if !config.key_exists("Added", &cmd[2]) {
            Text::key_doesnt_exists(&cmd[2]);
            return;
        }

        config.modify_and_save("Unlocked", &cmd[2], branch).ok();
        println!("SUCCESS: '{}' was unlock for updates. From branch: '{}'", &cmd[2], branch);
    }

    /// Upgrade all unlocked repositories.
    pub fn upgrade(cmd: Vec<String>){
        if !LocalStuff::cmd_len(&cmd, 2) { return; }
        Execute::upgrade();
    }

    /// Build a specific registered repository.
    pub fn build(cmd: Vec<String>){
        if !LocalStuff::cmd_len(&cmd, 3) { return; }

        let config = Config::load_config().unwrap_or_default();

        if !config.key_exists("Added", &cmd[2]) {
            Text::key_doesnt_exists(&cmd[2]);
            return;
        }

        Execute::build(&cmd[2]).ok();
    }

    /// List all registered repositories.
    pub fn list(cmd: Vec<String>){
        if !LocalStuff::cmd_len(&cmd, 2) { return; }
        
        let config = Config::load_config().unwrap_or_default();
        
        for key in config.added.keys() {
            println!("{}", key);
        }
    }

    /// Show the lock/build status of a repository.
    pub fn status(cmd: Vec<String>){
        if !LocalStuff::cmd_len(&cmd, 3) { return; }
        
        let config = Config::load_config().unwrap_or_default();
        
        if !config.key_exists("Added", &cmd[2]) {
            Text::key_doesnt_exists(&cmd[2]);
            return;
        }

        // Determine lock state.
        let locked = if config.key_exists("Unlocked", &cmd[2]) {
            "false"
        } else {
            "true"
        };

        // Determine build state.
        let build = if config.key_exists("Build", &cmd[2]) {
            "true"
        } else {
            "false"
        };

        println!("'{}' state is: lock = {}, build = {}", &cmd[2], locked, build);
    }

    /// Remove a registered repository and its builds.
    pub fn remove(cmd: Vec<String>){
        let config = Config::load_config().unwrap_or_default();

        if !LocalStuff::cmd_len(&cmd, 3) { return; }

        if !config.key_exists("Added", &cmd[2]) {
            Text::key_doesnt_exists(&cmd[2]);
            return;
        }

        Execute::remove(&cmd[2]);
    }

    /// Display the help text.
    pub fn help(cmd: Vec<String>){
        if !LocalStuff::cmd_len(&cmd, 2) { return; }
        Text::help();
    }
}