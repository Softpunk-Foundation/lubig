mod text;
mod util;
mod conf;
mod func;

use std::{env, path::Path};

use text::Text;
use util::{LocalStuff, RemoteStuff};
use conf::Config;
use func::Execute;

fn main() {
    let args: Vec<String> = env::args().collect();

    if !conf::conf_path().exists() {
        conf::generate_config().ok();
    }

    match args.get(1).map(|s| s.as_str()) {
        Some("conf") => Validate::conf(args),
        Some("get") => Validate::get(args),
        Some("add") => Validate::add(args),
        Some ("lock") => Validate::lock(args),
        Some ("unlock") => Validate::unlock(args),
        Some("upgrade") => Validate::upgrade(args),
        Some("build") => Validate::build(args),
        Some("list") => Validate::list(args),
        Some("status") => Validate::status(args),
        Some("remove") => Validate::remove(args),
        Some("help") => Validate::help(args),
        Some(_) => Text::general_error(),
        None => Text::need_args(),
    }
}

pub struct Validate;
impl Validate {
    pub fn conf(cmd: Vec<String>){
        if !LocalStuff::cmd_len(&cmd, 4) { return; }

        let next = match cmd.get(2).map(|s| s.as_str()) {
            Some("src") => "sources",
            Some("prof") => "profiles",
            Some("prog") => "programs",
            Some(_) => {Text::general_error(); return;},
            None => {Text::need_args(); return;},
        };

        if !LocalStuff::usable_path(&cmd[3]) {
            return;
        }

        let mut config = Config::load_config().unwrap_or_default();

        config.modify_and_save("Directories", next, &cmd[3]).ok();
    }

    pub fn get(cmd: Vec<String>){
        if !LocalStuff::cmd_len(&cmd, 4) { return; }

        if !RemoteStuff::is_valid_git_url(&cmd[2]) {
            return;
        }

        let config = Config::load_config().unwrap_or_default();

        if config.key_exists("Added", &cmd[3]) {
            Text::key_exists(&cmd[3]);
            return;
        }

        Execute::get(&cmd[2], &cmd[3]);
    }

    pub fn add(cmd: Vec<String>){
        if !LocalStuff::cmd_len(&cmd, 4) { return; }

        if !Path::new(&cmd[2]).exists() {
            Text::error_dir(&cmd[2]);
            return;
        }

        if !LocalStuff::is_git_repo(&cmd[2]) {
            Text::no_git_repo(&cmd[2]);
            return;
        }

        let config = Config::load_config().unwrap_or_default();

        if config.key_exists("Added", &cmd[3]) {
            Text::key_exists(&cmd[3]);
            return;
        }

        Execute::add(&cmd[2], &cmd[3]);
    }

    pub fn lock(cmd: Vec<String>) {
        if !LocalStuff::cmd_len(&cmd, 3) { return; }

        let mut config = Config::load_config().unwrap_or_default();

        if !config.key_exists("Added", &cmd[2]) {
            Text::key_doesnt_exists(&cmd[2]);
            return;
        }

        if config.key_exists("Unlocked", &cmd[2])
        {
            config.remove_and_save("Unlocked", &cmd[2]).ok();
            println!("SUCCESS: '{}' was lock for updates.", &cmd[2]);
        } else {
            println!("Error: '{}' is already locked for updates.", &cmd[2]);
        }
    }

    pub fn unlock(cmd: Vec<String>) {
        if cmd.len() < 3 {
            Text::need_args(); 
            return; 
        } else if cmd.len() > 4 {
            Text::exceed_args();
            return;
        }

        let branch = match cmd.get(3).map(|s| s.as_str()) {
            Some(_) => &cmd[3],
            None => "main",
        };

        let mut config = Config::load_config().unwrap_or_default();

        if !config.key_exists("Added", &cmd[2]) {
            Text::key_doesnt_exists(&cmd[2]);
            return;
        }

        config.modify_and_save("Unlocked", &cmd[2], branch).ok();
        println!("SUCCESS: '{}' was unlock for updates. From branch: '{}'", &cmd[2], branch);
    }

    pub fn upgrade(cmd: Vec<String>){
        if !LocalStuff::cmd_len(&cmd, 2) { return; }
        Execute::upgrade();
    }
    pub fn build(cmd: Vec<String>){
        if !LocalStuff::cmd_len(&cmd, 3) { return; }

        let config = Config::load_config().unwrap_or_default();

        if !config.key_exists("Added", &cmd[2]) {
            Text::key_doesnt_exists(&cmd[2]);
            return;
        }

        Execute::build(&cmd[3]).ok();
    }
    pub fn list(cmd: Vec<String>){
        if !LocalStuff::cmd_len(&cmd, 2) { return; }
        
        let config = Config::load_config().unwrap_or_default();
        
        config.list_keys("Added");
    }
    pub fn status(cmd: Vec<String>){
        if !LocalStuff::cmd_len(&cmd, 3) { return; }
        
        let config = Config::load_config().unwrap_or_default();
        
        if !config.key_exists("Added", &cmd[2]) {
            Text::key_doesnt_exists(&cmd[2]);
            return;
        }

        let locked;

        if config.key_exists("Unlocked", &cmd[2]) {
            locked = "false";
        } else {
            locked = "true";
        }

        let build;

        if config.key_exists("Build", &cmd[2]) {
            build = "true";
        } else {
            build = "false";
        }

        println!("'{}' state is: lock = {}, build = {}", &cmd[2], locked, build);
    }
    pub fn remove(cmd: Vec<String>){
        let config = Config::load_config().unwrap_or_default();

        if !LocalStuff::cmd_len(&cmd, 3) { return; }

        if !config.key_exists("Added", &cmd[2]) {
            Text::key_doesnt_exists(&cmd[2]);
            return;
        }

        Execute::remove(&cmd[2]);
    }
    pub fn help(cmd: Vec<String>){
        if !LocalStuff::cmd_len(&cmd, 2) { return; }
        Text::help();
    }
}