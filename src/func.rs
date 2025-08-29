use git2::Repository;
use std::{process::{Command,Stdio}, path::{Path, PathBuf}};

use crate::conf::Config;
use crate::util::{LocalStuff, RemoteStuff};

pub struct Execute;
impl Execute {
    pub fn get(url: &String, name: &String) -> Result<(), git2::Error> {
        let cmd = Config::load_config().unwrap_or_default();

        if let Some(path_str) = cmd.get_value("Directories", "sources") {
            let final_str = &format!("{}/{}", path_str, name);
            let path = Path::new(final_str);

            match Repository::clone(url, path) {
                Ok(_) => {
                    Self::add(&path, name);
                    println!("SUCCESS: getting '{}'", name);
                }
                Err(e) => {
                    println!("ERROR:'{}': {}", name, e.message());
                }
            }

        } else {
            eprintln!("Error: Unknown Configuration");
        }

        Ok(())
    }

    pub fn add<P: AsRef<Path>>(path: &P, name: &str) {
        let config = Config::load_config().unwrap_or_default();
        let src_path_str = config.get_value("Directories", "sources").unwrap() + "/" + name;
        let src_path = Path::new(&src_path_str);
        if !src_path.exists()
        {
            LocalStuff::generate_path(src_path).ok();
        }
        if !LocalStuff::is_subdir(src_path, path) {
            LocalStuff::move_dir(&path, src_path).ok();
        }

        let mut config = Config::load_config().unwrap_or_default();

        config.modify_and_save("Added", name, &src_path_str).ok();
        config.modify_and_save("Unlocked", name, "").ok();
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

        if !prof_path.exists() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("ERROR: Script not found: {}", prof_path.display()),
            ));
        }

        let status = if cfg!(windows) {
            Command::new("cmd")
            .args(&["/C", prof_path.to_str().unwrap(), prog_path.to_str().unwrap()])
            .current_dir(&src_path)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()?
        } else {
            Command::new("sh")
            .arg(prof_path.to_str().unwrap())
            .arg(prog_path.to_str().unwrap())
            .current_dir(&src_path)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()?
        };


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
