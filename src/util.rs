use std::fs;
use std::path::{Path, PathBuf};
use git2::{Repository, FetchOptions};

use crate::text::Text;

/// Utility functions for local filesystem operations.
pub struct LocalStuff;

/// Utility functions for remote Git operations.
pub struct RemoteStuff;

impl LocalStuff {
    /// Validates whether a given path string is usable for LUBIG operations.
    /// - Rejects empty strings or paths containing invalid characters.
    /// - Accepts non-existent paths (will be created later).
    /// - Rejects existing non-empty directories.
    pub fn usable_path(path: &String) -> bool {
        let valid = !path.trim().is_empty()
            && !path.contains(['<', '>', ':', '"', '|', '?', '*']);

        if !valid {
            println!("{} is not a valid path. Please avoid the use of: '<', '>', ':', '|', '?', '*'.", path);
            return false;
        }
        let p = Path::new(path);

        if !p.exists() {
            println!("{} is an avalaible path...", path);
            return true;
        }

        if Self::not_empty(path) {
            println!("{} This directory is not empty. Use an empty directory.", path);
            return false;
        } else {
            println!("{} is an avalaible path...", path);
            return true;
        }
    }

    /// Checks if a directory is not empty.
    pub fn not_empty<P: AsRef<Path>>(path: P) -> bool {
        match fs::read_dir(path) {
            Ok(mut entries) => entries.next().is_some(),
            Err(_) => false,
        }
    }

    /// Determines if a given path is a valid Git repository.
    pub fn is_git_repo<P: AsRef<Path>>(path: P) -> bool {
        Repository::open(path).is_ok()
    }

    /// Validates the number of arguments for a command.
    /// Prints an error if too many or too few arguments are provided.
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

    /// Creates a directory path recursively if it does not exist.
    /// On Unix systems, sets permissions to `755`.
    pub fn generate_path<P: AsRef<Path>>(path: P) -> std::io::Result<()> {
        let p = path.as_ref();

        if !p.exists() {
            fs::create_dir_all(p)?;
        }

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(p, fs::Permissions::from_mode(0o755))?;
        }

        Ok(())
    }

    /// Deletes a directory and all its contents.
    pub fn delete_dir<P: AsRef<Path>>(path: P) -> std::io::Result<()> {
        let path_ref = path.as_ref();

        if path_ref.exists() && path_ref.is_dir() {
            fs::remove_dir_all(path_ref)?;
        }

        Ok(())
    }

    /// Moves a directory from origin to destination.
    /// Fails if origin does not exist or destination already exists.
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

    /// Checks if `child` is a subdirectory of `parent`.
    pub fn is_subdir<P: AsRef<Path>, Q: AsRef<Path>>(parent: P, child: Q) -> bool {
        let parent_abs = parent.as_ref().canonicalize().unwrap();
        let child_abs = child.as_ref().canonicalize().unwrap();

        child_abs.starts_with(&parent_abs)
    }

    /// Removes a build script file for a given repository.
    /// The extension is `.bat` on Windows and `.sh` on Unix.
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
    /// Performs a fast-forward pull from the remote `origin` for a given branch.
    /// - Opens the repository at `path`.
    /// - Fetches the latest commits for the branch.
    /// - Checks out the fetched commit and updates HEAD.
    pub fn pull_fast_forward(path: &str, branch: &str) -> Result<(), git2::Error> {
        let repo = Repository::open(path)?;

        let mut remote = repo.find_remote("origin")?;
        remote.fetch(&[branch], Some(&mut FetchOptions::new()), None)?;

        let fetch_head = repo.find_reference("FETCH_HEAD")?;
        let fetch_commit = repo.reference_to_annotated_commit(&fetch_head)?;
        let commit = repo.find_commit(fetch_commit.id())?;

        repo.checkout_tree(commit.as_object(), None)?;
        repo.set_head(&format!("refs/heads/{}", branch))?;

        Ok(())
    }
}