mod conf_panel;
mod text_writer;
mod actions;
use std::path::Path;
use text_writer::Text;
use actions::{ConfCmd, Registry};
use conf_panel::{Config};

fn main() {
    if !conf_panel::conf_path().exists() {
        conf_panel::generate_config().ok();
    }

    let args: Vec<String> = std::env::args().collect();
    match args.get(1).map(|s| s.as_str()){
        Some("conf") => Validations::conf(args),
        Some("get") => Validations::get(args),
        Some("add") => Validations::add(args),
        Some("lock") => Validations::lock(args),
        Some("unlock") => Validations::unlock(args),
        Some("update") => Validations::update(args),
        Some("build") => Validations::build(args),
        Some("remove") => Validations::remove(args),
        Some("list") => Validations::list(args),
        Some("help") => Text::help(),
        None => Text::general_error(),
        _ => Text::general_error(),
    }
}

pub struct Validations;
impl Validations
{
    pub fn conf(cmd: Vec<String>){
        let mut exec: i8 = 0;
        let mut my_path: &String = &"empty".to_string();
        if cmd.len() < 3 {
            Text::general_error();
            return;
        }
        else if cmd.len() > 4 {
            Text::shut_up();
            return;
        }
        match cmd.get(2).map(|s| s.as_str()) {
            Some("src") => exec = 1,
            Some("prof") => exec = 2,
            Some("prog") => exec = 3,
            None => Text::general_error(),
            _ => Text::general_error(),
        }
        if exec > 0 && cmd.len() < 4 {
            print!("Error: To use this command, specify the destination path. Check 'redigit help' for more information.");
            return;
        }
        if cmd.len() == 4
        {
            my_path = &cmd[3];
        }
        match exec {
            1 => ConfCmd::src_cmd(my_path),
            2 => ConfCmd::prof_cmd(my_path),
            3 => ConfCmd::prog_cmd(my_path),
            _ => return,
        }
    }
    pub fn get(cmd: Vec<String>)
    {
        if cmd.len() < 4 {
            Text::general_error();
            return;
        }
        else if cmd.len() > 4 {
            Text::shut_up();
            return;
        }
        Registry::get_cmd(&cmd[2], &cmd[3]).ok();
    }

    pub fn add(cmd: Vec<String>) {
        let conf = Config::load_config().unwrap_or_default();
        if cmd.len() < 4 {
            Text::general_error();
            return;
        }
        else if cmd.len() > 4 {
            Text::shut_up();
            return;
        }

        if !conf.key_exists("added", &cmd[3])
        {
            Registry::add_cmd(&cmd[3], Path::new(&cmd[2]));
        } else {
            println!("Error: This name is already registered.");
        }
    }

    pub fn remove(cmd: Vec<String>){
        let conf = Config::load_config().unwrap_or_default();
        if cmd.len() > 3 {
            Text::shut_up();
            return;
        }
        if conf.key_exists("added", &cmd[2])
        {
            Registry::remove_cmd(&cmd[2]);
        } else {
            println!("Error: Unknown configuration.");
        }
    }

    pub fn lock(cmd: Vec<String>){
        let conf = Config::load_config().unwrap_or_default();
        if cmd.len() > 3 {
            Text::shut_up();
            return;
        }
        if conf.key_exists("Unlocked", &cmd[2])
        {
            Registry::lock_cmd(&cmd[2]);
        } else {
            println!("'{}' is already locked", cmd[2]);
        }
    }

    pub fn unlock(cmd: Vec<String>){
        let conf = Config::load_config().unwrap_or_default();
        if cmd.len() > 3 {
            Text::shut_up();
            return;
        }
        if !conf.key_exists("Unlocked", &cmd[2])
        {
            Registry::unlock_cmd(&cmd[2]);
        } else {
            println!("'{}' is already unlocked", cmd[2]);
        }
    }

    pub fn update(cmd: Vec<String>)
    {
        if cmd.len() > 2 {
            Text::shut_up();
            return;
        }

        Registry::update_cmd();
    }
    pub fn list(cmd: Vec<String>)
    {
        let conf = Config::load_config().unwrap_or_default();
        if cmd.len() > 3 {
            Text::shut_up();
            return;
        }

        match cmd.get(2).map(|s| s.as_str()) {
            Some("b") => { conf.list_keys("Build"); },
            Some("u") => { conf.list_keys("Unlocked"); },
            None => { conf.list_keys("Added"); },
            _ => { Text::general_error(); },
        }
    }
    pub fn build(cmd: Vec<String>){
        if cmd.len() > 3 {
            Text::shut_up();
            return;
        }
        Registry::build_cmd(&cmd[2]);
    }

}