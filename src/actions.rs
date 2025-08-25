use std::path::{Path, PathBuf};
use std::fs;
use std::process::Command;
use crate::conf_panel::Config;
use crate::text_writer::Text;
use git2::{Repository, RemoteCallbacks, FetchOptions, build::CheckoutBuilder};

pub struct ConfCmd;
impl ConfCmd {
    pub fn src_cmd(path: &String) {
        Text::path_state(path);
        if valid_path(path) == PathStatus::Available {
            let mut cmd = Config::load_config().unwrap_or_default();
            let _ = cmd.modify_and_save("Directories", "sources", path);
        }
    }

    pub fn prof_cmd(path: &String) {
        Text::path_state(path);
        if valid_path(path) == PathStatus::Available {
            let mut cmd = Config::load_config().unwrap_or_default();
            let _ = cmd.modify_and_save("Directories", "profiles", path);
        }
    }

    pub fn prog_cmd(path: &String) {
        Text::path_state(path);
        if valid_path(path) == PathStatus::Available {
            let mut cmd = Config::load_config().unwrap_or_default();

            let _ = cmd.modify_and_save("Directories", "programs", path);
        }
    }
}

pub struct Registry;
impl Registry {
    pub fn get_cmd(url: &String, name: &String) -> Result<(), git2::Error> {
        let cmd = Config::load_config().unwrap_or_default();

        if let Some(path_str) = cmd.get_value("Directories", "sources") {
            let path = PathBuf::from(format!("{}/{}", path_str, name));

            let repo = Repository::clone(url, &path)?;

            Registry::add_cmd(name, &path);

            println!("Cloned repository in {}, adding {}...", repo.path().display(), name);
        } else {
            eprintln!("Error: Unknown Configuration");
        }

        Ok(())
    }

    pub fn add_cmd(name: &String, path: &Path) {
        let mut cmd = Config::load_config().unwrap_or_default();

        if let Some(sources_str) = cmd.get_value("Directories", "sources") {
            let sources_path = PathBuf::from(sources_str);

            match Repository::open(path) {
                Ok(_) => {
                }
                Err(_) => {
                    eprintln!("Error: '{}' This is not a git repository", path.display());
                    return;
                }
            }

            let folder_name = path
                .file_name()
                .map(|n| n.to_owned())
                .unwrap_or_else(|| name.clone().into());

            let target_path = sources_path.join(folder_name);

            let abs_sources = fs::canonicalize(&sources_path).unwrap_or(sources_path.clone());
            let abs_path = fs::canonicalize(path).unwrap_or(path.to_path_buf());

            if !abs_path.starts_with(&abs_sources) {
                println!("Moving {} to {}...", abs_path.display(), target_path.display());

                if let Some(parent) = target_path.parent() {
                    let _ = fs::create_dir_all(parent);
                }

                if let Err(e) = fs::rename(&abs_path, &target_path) {
                    eprintln!("Error moving to directory: {}", e);
                    return;
                }

                cmd.modify_and_save("Added", name, &target_path.to_string_lossy()).ok();
            } else {
                cmd.modify_and_save("Added", name, &abs_path.to_string_lossy()).ok();
            }

            cmd.modify_and_save("Unlocked", name, "").ok();
        } else {
            eprintln!("Error: Unknown Configuration");
        }
    }

    pub fn remove_cmd(name: &String) {
        let mut cmd = Config::load_config().unwrap_or_default();

        if let Some(src_path) = cmd.get_value("Added", name) {
            let path = PathBuf::from(src_path);
            if path.exists() {
                if let Err(e) = fs::remove_dir_all(&path) {
                    eprintln!("Error eliminating source directory {}: {}", path.display(), e);
                } else {
                    println!("SUCCESS, Source directory has been eliminated: {}", path.display());
                }
            }
        }

        if let Some(profiles_dir) = cmd.get_value("Directories", "profiles") {
            let ext = match std::env::consts::OS {
                "windows" => "bat",
                _ => "sh",
            };
            let filename = format!("{}.{}", name, ext);

            let script_path = PathBuf::from(profiles_dir).join(filename);
            if script_path.exists() {
                if let Err(e) = fs::remove_file(&script_path) {
                    eprintln!("Error eliminating profile in {}: {}", script_path.display(), e);
                } else {
                    println!("SUCCESS, Build profile eliminated: {}", script_path.display());
                }
            }
        }

        if let Some(build_path) = cmd.get_value("Build", name) {
            let path = PathBuf::from(build_path);
            if path.exists() {
                if let Err(e) = fs::remove_dir_all(&path) {
                    eprintln!("Error eliminating build {}: {}", path.display(), e);
                } else {
                    println!("Success, build eliminated: {}", path.display());
                }
            }
        }

        cmd.remove_and_save("Added", name).ok();
        cmd.remove_and_save("Build", name).ok();
        cmd.remove_and_save("Unlocked", name).ok();
    }

    pub fn unlock_cmd(name: &String){
        let mut cmd = Config::load_config().unwrap_or_default();
        cmd.modify_and_save("Unlocked", name, "").ok();
    }

    pub fn lock_cmd(name: &String){
        let mut cmd = Config::load_config().unwrap_or_default();
        cmd.remove_and_save("Unlocked", name).ok();
    }
    pub fn update_cmd() {
        let cmd = Config::load_config().unwrap_or_default();

        for (name, path_str) in cmd.added.iter() {
            if !cmd.unlocked.contains_key(name) {
                println!("Skipped '{}': repository is locked.", name);
                continue;
            }

            let path = Path::new(path_str);
            println!("Updating '{}'", name);

            let repo = match Repository::open(path) {
                Ok(r) => r,
                Err(e) => {
                    eprintln!("Error: Cannot open repository '{}': {}", name, e);
                    continue;
                }
            };

            let mut remote = match repo.find_remote("origin") {
                Ok(r) => r,
                Err(e) => {
                    eprintln!("Error: Remote 'origin' not found in '{}': {}", name, e);
                    continue;
                }
            };

            let mut callbacks = RemoteCallbacks::new();
            callbacks.transfer_progress(|stats| {
                println!("Received {} objects", stats.received_objects());
                true
            });

            let mut fetch_opts = FetchOptions::new();
            fetch_opts.remote_callbacks(callbacks);

            if let Err(e) = remote.fetch(&["refs/heads/*:refs/remotes/origin/*"], Some(&mut fetch_opts), None) {
                eprintln!("Error: Fetch failed for '{}': {}", name, e);
                continue;
            }

            let head = match repo.head() {
                Ok(h) => h,
                Err(e) => {
                    eprintln!("Error: Cannot access HEAD in '{}': {}", name, e);
                    continue;
                }
            };

            let branch = head.shorthand().unwrap_or("main");
            let local_ref = format!("refs/heads/{}", branch);
            let remote_ref = format!("refs/remotes/origin/{}", branch);

            let annotated = match repo.find_reference(&remote_ref)
                .and_then(|r| repo.reference_to_annotated_commit(&r)) {
                    Ok(a) => a,
                    Err(e) => {
                        eprintln!("Error: Cannot retrieve remote commit in '{}': {}", name, e);
                        continue;
                    }
                };

            let analysis = match repo.merge_analysis(&[&annotated]) {
                Ok((a, _)) => a,
                Err(e) => {
                    eprintln!("Error: Merge analysis failed in '{}': {}", name, e);
                    continue;
                }
            };

            if analysis.is_fast_forward() {
                let mut ref_head = match repo.find_reference(&local_ref) {
                    Ok(r) => r,
                    Err(e) => {
                        eprintln!("Error: Local reference not found in '{}': {}", name, e);
                        continue;
                    }
                };

                if let Err(e) = ref_head.set_target(annotated.id(), "Fast-forward") {
                    eprintln!("Error: Failed to apply fast-forward in '{}': {}", name, e);
                    continue;
                }

                if let Err(e) = repo.set_head(&local_ref) {
                    eprintln!("Error: Failed to set HEAD in '{}': {}", name, e);
                    continue;
                }

                let mut checkout = CheckoutBuilder::new();
                checkout.force();

                if let Err(e) = repo.checkout_head(Some(&mut checkout)) {
                    eprintln!("Error: Checkout failed in '{}': {}", name, e);
                    continue;
                }

                println!("Success: '{}' has been updated.", name);
            } else {
                println!("Notice: '{}' requires manual merge or is already up to date.", name);
            }
        }
    }

    pub fn build_cmd(name: &String) {
        let mut cmd = Config::load_config().unwrap_or_default();

        let source_path_str = match cmd.get_value("Added", name) {
            Some(p) => p,
            None => {
                eprintln!("Error: '{}' is not registered in [Added].", name);
                return;
            }
        };
        let source_path = PathBuf::from(&source_path_str);

        let profiles_dir_str = match cmd.get_value("Directories", "profiles") {
            Some(p) => p,
            None => {
                eprintln!("Error: 'profiles' directory is missing in [Directories].");
                return;
            }
        };
        let profiles_dir = PathBuf::from(&profiles_dir_str);

        let programs_dir_str = match cmd.get_value("Directories", "programs") {
            Some(p) => p,
            None => {
                eprintln!("Error: 'programs' directory is missing in [Directories].");
                return;
            }
        };
        let programs_dir = PathBuf::from(&programs_dir_str);

        let ext = if cfg!(target_os = "windows") { "bat" } else { "sh" };
        let script_path = profiles_dir.join(format!("{}.{}", name, ext));

        if !script_path.exists() {
            eprintln!(
                "Error: Build profile '{}' not found at {}",
                name,
                script_path.display()
            );
            return;
        }

        println!("Building '{}' using profile {}", name, script_path.display());
        let status = if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(&["/C", script_path.to_string_lossy().as_ref()])
                .current_dir(&source_path)
                .status()
        } else {
            Command::new("sh")
                .arg(script_path.to_string_lossy().as_ref())
                .current_dir(&source_path)
                .status()
        };

        match status {
            Ok(s) if s.success() => {
                println!("Build completed successfully for '{}'.", name);
            }
            Ok(_) => {
                eprintln!("Error: Build script failed for '{}'.", name);
                return;
            }
            Err(e) => {
                eprintln!("Error: Failed to execute build script for '{}': {}", name, e);
                return;
            }
        }

        let release_path = source_path.join("release").join(name);
        if !release_path.exists() {
            eprintln!("Error: Release folder '{}' not found.", release_path.display());
            return;
        }

        let target_path = programs_dir.join(name);
        if let Some(parent) = target_path.parent() {
            if let Err(e) = fs::create_dir_all(parent) {
                eprintln!(
                    "Error: Could not create target directory '{}': {}",
                    parent.display(),
                    e
                );
                return;
            }
        }

        if let Err(e) = fs::rename(&release_path, &target_path) {
            eprintln!(
                "Error: Failed to move release to '{}': {}",
                target_path.display(),
                e
            );
            return;
        }

        println!("Release for '{}' moved to '{}'.", name, target_path.display());

        cmd.modify_and_save("Build", name, &target_path.to_string_lossy()).ok();
    }
}

#[derive(Debug, PartialEq)]
pub enum PathStatus {
    Available,
    Invalid, 
    Exist,
}

pub fn valid_path(received: &String) -> PathStatus {
    let valid = !received.trim().is_empty()
        && !received.contains(['<', '>', ':', '"', '|', '?', '*']);

    if !valid {
        return PathStatus::Invalid;
    }
    let path = Path::new(received);

    if path.exists() {
        PathStatus::Exist
    } else {
        PathStatus::Available
    }
}
