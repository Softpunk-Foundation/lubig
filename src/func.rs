use std::{
    process::{Command, Stdio},
    path::{Path, PathBuf}
};

use crate::conf::Config;
use crate::util::{LocalStuff, RemoteStuff};

/// Core executor for LUBIG operations.
/// Each method corresponds to a high-level command.
pub struct Execute;

impl Execute {

    /// Clone a remote Git repository into the sources directory and register it.
    pub fn get(url: &String, name: &String) -> std::io::Result<()> {
        let cmd = Config::load_config().unwrap_or_default();

        // Retrieve the configured sources directory.
        let path_str = match cmd.get_value("Directories", "sources") {
            Some(p) => p,
            None => {
                eprintln!("Error: Unknown Configuration");
                return Ok(());
            }
        };

        // Build the final target path for the repository.
        let final_str = format!("{}/{}", path_str, name);
        let path = Path::new(&final_str);

        // Execute `git clone` command.
        let status = Command::new("git")
            .arg("clone")
            .arg(url)
            .arg(path)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()?;

        // If clone succeeds, register the repository.
        if status.success() {
            Self::add(&path, name);
            println!("SUCCESS: getting '{}'", name);
        } else {
            eprintln!("ERROR: Failed to clone '{}'", name);
        }

        Ok(())
    }

    /// Register a repository in the configuration.
    /// Moves it into the sources directory if needed.
    pub fn add<P: AsRef<Path>>(path: &P, name: &str) {
        let mut config = Config::load_config().unwrap_or_default();
        let src_path_str = config.get_value("Directories", "sources").unwrap() + "/" + name;
        let src_path = Path::new(&src_path_str);

        // Ensure the target directory exists.
        if !src_path.exists() {
            LocalStuff::generate_path(src_path).ok();
        }

        // Move the repository into the sources directory if it's not already there.
        if !LocalStuff::is_subdir(src_path, path) {
            LocalStuff::move_dir(&path, src_path).ok();
        }

        // Save the registration in the config.
        config.modify_and_save("Added", name, &src_path_str).ok();
        println!("'{}' Added", name)
    }

    /// Upgrade all unlocked repositories.
    /// If a repository is marked for build, rebuild it after upgrade.
    pub fn upgrade() {
        let config = Config::load_config().unwrap_or_default();

        for key in config.added.keys() {
            if config.key_exists("Unlocked", key) {
                let path = config.get_value("Added", key).unwrap();
                let branch = config.get_value("Unlocked", key).unwrap();

                // Pull latest changes from the remote branch.
                RemoteStuff::pull_fast_forward(&path, &branch).ok();
                
                // If build flag exists, rebuild after upgrade.
                if config.key_exists("Build", key) {
                    Self::build(key).ok();
                }
            }
        }
    }

    /// Build a registered repository using its profile script.
    pub fn build(name: &str) -> std::io::Result<()> {
        let mut config = Config::load_config().unwrap_or_default();
        
        // Determine script extension based on OS.
        let ext = if cfg!(windows) { ".bat" } else { ".sh" };

        // Retrieve configured directories.
        let src_path_str = config.get_value("Directories", "sources").unwrap();
        let prof_path_str = config.get_value("Directories", "profiles").unwrap();
        let prog_path_str = config.get_value("Directories", "programs").unwrap();

        let mut src_path = PathBuf::from(src_path_str);
        let mut prof_path = PathBuf::from(prof_path_str);
        let mut prog_path = PathBuf::from(prog_path_str);

        // Append repository name to source path.
        src_path = src_path.join(name);
        // Append script filename to profile path.
        prof_path = prof_path.join(format!("{}{}", name, ext));

        // Remove existing build output if present.
        if prog_path.join(name).exists() {
            LocalStuff::delete_dir(prog_path.join(name)).ok();
        }

        // Execute the build script, passing the programs directory as argument.
        let status = Command::new(&prof_path)
            .arg(prog_path.to_str().unwrap())
            .current_dir(&src_path)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()?;

        if !status.success() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "ERROR: Script failed.",
            ));
        }

        // Mark the repository as built in the config.
        prog_path = prog_path.join(name);
        config.modify_and_save("Build", name, prog_path.to_str().unwrap()).ok();
        println!("SUCCESS: Build complete: {}", prog_path.to_str().unwrap());

        Ok(())
    }

    /// Remove a registered repository and its associated build artifacts.
    pub fn remove(name: &str) {
        let mut config = Config::load_config().unwrap_or_default();
        let src_path_str = config.get_value("Added", name).unwrap().to_string();
        let src_path = Path::new(&src_path_str);

        // Remove build artifacts if they exist.
        if config.key_exists("Build", name) {
            let path_str = config.get_value("Build", name).unwrap().to_string();
            let path = Path::new(&path_str);
            
            if path.exists() {
                LocalStuff::delete_dir(path).ok();
            }
            config.remove_and_save("Build", name).ok();
        }

        // Remove from unlocked list if present.
        if config.key_exists("Unlocked", name) {
            config.remove_and_save("Unlocked", name).ok();
        }
        
        // Remove build script and source directory.
        LocalStuff::remove_script(&src_path_str, name).ok();
        LocalStuff::delete_dir(src_path).ok();

        // Remove from added list.
        config.remove_and_save("Added", name).ok();
    }
}