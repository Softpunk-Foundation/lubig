use git2::Repository;
use std::{process::Command, path::{Path, PathBuf}};

use crate::conf::Config;
use crate::util::{LocalStuff, RemoteStuff};

pub struct Execute;
impl Execute {
    pub fn get(url: &str, name: &str) {
        let config = Config::load_config().unwrap_or_default();
        let value_to_str = config.get_value("Directories", "sources").unwrap() + "/" + name;
        let path= Path::new(&value_to_str);
        Repository::clone(url, path).ok();
        Self::add(&path, name);
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

    pub fn build (name: &str) -> std::io::Result<()> {
        let mut config = Config::load_config().unwrap_or_default();
        let ext = if cfg!(windows) { ".bat" } else { ".sh" };
        let src_path = config.get_value("Directories", "sources").unwrap();
        let mut prog_path = PathBuf::from(config.get_value("Directories", "programs").unwrap());
        let mut script_path = PathBuf::from(src_path);
        script_path.push(format!("{}{}", name, ext));

        if !script_path.exists() {
            return Err(std::io::Error::new(std::io::ErrorKind::NotFound, "Error: Script not found."));
        }

        let status = if cfg!(windows) {
            Command::new("cmd")
                .args(&["/C", script_path.to_str().unwrap(), prog_path.to_str().unwrap()])
                .status()?
        } else {
            Command::new("sh")
                .arg(script_path.to_str().unwrap())
                .arg(prog_path.to_str().unwrap())
                .status()?
        };

        if !status.success() {
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "Error: The script has failed."));
        } else {
            prog_path.push(format!("{}{}", name, ext));
            config.modify_and_save("Build", name, prog_path.to_str().unwrap()).ok();
        }

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