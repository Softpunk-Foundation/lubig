use std::{process::{Command,Stdio}, path::{Path, PathBuf}};

use crate::conf::Config;
use crate::util::{LocalStuff, RemoteStuff};

pub struct Execute;
impl Execute {

    pub fn get(url: &String, name: &String) -> std::io::Result<()> {
        let cmd = Config::load_config().unwrap_or_default();

        let path_str = match cmd.get_value("Directories", "sources") {
            Some(p) => p,
            None => {
                eprintln!("Error: Unknown Configuration");
                return Ok(());
            }
        };

        let final_str = format!("{}/{}", path_str, name);
        let path = Path::new(&final_str);

        let status = Command::new("git")
            .arg("clone")
            .arg(url)
            .arg(path)
            .stdout(std::process::Stdio::inherit())
            .stderr(std::process::Stdio::inherit())
            .status()?;

        if status.success() {
            Self::add(&path, name);
            println!("SUCCESS: getting '{}'", name);
        } else {
            eprintln!("ERROR: Failed to clone '{}'", name);
        }

        Ok(())
    }

    pub fn add<P: AsRef<Path>>(path: &P, name: &str) {
        let mut config = Config::load_config().unwrap_or_default();
        let src_path_str = config.get_value("Directories", "sources").unwrap() + "/" + name;
        let src_path = Path::new(&src_path_str);
        if !src_path.exists()
        {
            LocalStuff::generate_path(src_path).ok();
        }
        if !LocalStuff::is_subdir(src_path, path) {
            LocalStuff::move_dir(&path, src_path).ok();
        }

        config.modify_and_save("Added", name, &src_path_str).ok();
        println!("'{}' Added", name)
    }

    pub fn upgrade() {

        let config = Config::load_config().unwrap_or_default();

        for key in config.added.keys() {

            if config.key_exists("Unlocked", key) {
                let path = config.get_value("Added", key).unwrap();
                let branch = config.get_value("Unlocked", key).unwrap();
                RemoteStuff::pull_fast_forward(&path, &branch).ok();
                
                if config.key_exists("Build", key) {
                    Self::build(key).ok();
                }
            }
        }
    }

    pub fn build(name: &str) -> std::io::Result<()> {
        let mut config = Config::load_config().unwrap_or_default();
        
        let ext = if cfg!(windows) { ".bat" } else { ".sh" };

        let src_path_str = config.get_value("Directories", "sources").unwrap();
        let prof_path_str = config.get_value("Directories", "profiles").unwrap();
        let prog_path_str = config.get_value("Directories", "programs").unwrap();

        let mut src_path = PathBuf::from(src_path_str);
        let mut prof_path = PathBuf::from(prof_path_str);
        let mut prog_path = PathBuf::from(prog_path_str);

        src_path = src_path.join(name);
        prof_path = prof_path.join(format!("{}{}", name, ext));

        if prog_path.join(name).exists()
        {
            LocalStuff::delete_dir(prog_path.join(name)).ok();
        }

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

        prog_path = prog_path.join(name);
        config.modify_and_save("Build", name, prog_path.to_str().unwrap()).ok();
        println!("SUCCESS: Build complete: {}", prog_path.to_str().unwrap());

        Ok(())
    }

    pub fn remove(name: &str) {
        let mut config = Config::load_config().unwrap_or_default();
        let src_path_str = config.get_value("Added", name).unwrap().to_string();
        let src_path = Path::new(&src_path_str);

        if config.key_exists("Build", name) {

            let path_str = config.get_value("Build", name).unwrap().to_string();
            let path = Path::new(&path_str);
            
            if path.exists(){
                LocalStuff::delete_dir(path).ok();
            }
            config.remove_and_save("Build", name).ok();
        }

        if config.key_exists("Unlocked", name)
        {
            config.remove_and_save("Unlocked", name).ok();
        }
        
        LocalStuff::remove_script(&src_path_str, name).ok();
        LocalStuff::delete_dir(src_path).ok();
        config.remove_and_save("Added", name).ok();
    }
}