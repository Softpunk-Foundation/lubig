use std::fs;
use std::path::{Path, PathBuf};
use git2::{Repository, FetchOptions};

use crate::text::Text;

pub struct LocalStuff;
pub struct RemoteStuff;



impl LocalStuff{
    pub fn usable_path(path: &String) -> bool{
        let valid = !path.trim().is_empty()
            && !path.contains(['<', '>', ':', '"', '|', '?', '*']);

        if !valid {
            println! ("{} is not a valid path. Please avoid the use of: '<', '>', ':', '|', '?', '*'.", path);
            return false;
        }
        let p = Path::new(path);

        if !p.exists() {
            println! ("{} is an avalaible path...", path);
            return true;
        }

        if Self::not_empty(path) {
            println! ("{} This directory is not empty. Use an empty directory.", path);
            return false;
        } else {
            println! ("{} is an avalaible path...", path);
            return true;
        }
    }

    pub fn not_empty<P: AsRef<Path>>(path: P) -> bool {
        match fs::read_dir(path) {
            Ok(mut entries) => entries.next().is_some(),
            Err(_) => false,
        }
    }

    pub fn is_git_repo<P: AsRef<Path>>(path: P) -> bool {
        Repository::open(path).is_ok()
    }

    pub fn cmd_len(cmd: &Vec<String>, expected: usize) -> bool {
        if cmd.len() > expected {
            Text::exceed_args();
            false
        } else if cmd.len() < expected {
            Text::need_args();
            false
        } else {
            true
        }
    }

    pub fn generate_path<P: AsRef<Path>>(path: P) -> std::io::Result<()> {
        if !path.as_ref().exists() {
            fs::create_dir_all(&path)?;
        }
        Ok(())
    }

    pub fn delete_dir<P: AsRef<Path>>(path: P) -> std::io::Result<()> {
        let path_ref = path.as_ref();

        // Sólo actuamos si existe y es un directorio
        if path_ref.exists() && path_ref.is_dir() {
            fs::remove_dir_all(path_ref)?;
        }

        Ok(())
    }

    pub fn move_dir<P: AsRef<Path>, D: AsRef<Path>>(origin: P, destiny: D) -> std::io::Result<()> {
        let from_path = origin.as_ref();
        let to_path = destiny.as_ref();

        if !from_path.exists() || !from_path.is_dir() {
            return Err(std::io::Error::new(std::io::ErrorKind::NotFound, "Invalid origin."));
        }

        if to_path.exists() {
            return Err(std::io::Error::new(std::io::ErrorKind::AlreadyExists, "Destiny exist."));
        }

        fs::rename(from_path, to_path)?;
        Ok(())
    }

    pub fn is_subdir<P: AsRef<Path>, Q: AsRef<Path>>(parent: P, child: Q) -> bool {
        let parent_abs = parent.as_ref().canonicalize().unwrap();
        let child_abs = child.as_ref().canonicalize().unwrap();

        child_abs.starts_with(&parent_abs)
    }

    pub fn remove_script(path: &str, name: &str) -> std::io::Result<()> {
        let ext = if cfg!(windows) { ".bat" } else { ".sh" };
        let mut full_path = PathBuf::from(path);
        full_path.push(format!("{}{}", name, ext));

        if full_path.exists() {
            fs::remove_file(full_path)?;
        }

        Ok(())
    }
}
impl RemoteStuff {
    pub fn is_valid_git_url(url: &str) -> bool {
        let Ok(repo) = Repository::init_bare("/tmp/placeholder_repo") else {
            println!("Error: Unknown.");
            return false;
        };
        let Ok(mut remote) = repo.remote_anonymous(url) else {
            println!("Error: Invalid git remote repository url.");
            return false;
        };
        remote.connect(git2::Direction::Fetch).is_ok()
    }

    pub fn pull_fast_forward(path: &str, branch: &str) -> Result<(), git2::Error> {
        let repo = Repository::open(path)?;

        // Fetch desde el remoto
        let mut remote = repo.find_remote("origin")?;
        remote.fetch(&[branch], Some(&mut FetchOptions::new()), None)?;

        // Obtener el commit remoto
        let fetch_head = repo.find_reference("FETCH_HEAD")?;
        let fetch_commit = repo.reference_to_annotated_commit(&fetch_head)?;
        let commit = repo.find_commit(fetch_commit.id())?;

        // Checkout directo al commit remoto
        repo.checkout_tree(commit.as_object(), None)?;
        repo.set_head(&format!("refs/heads/{}", branch))?;

        Ok(())
    }
}
