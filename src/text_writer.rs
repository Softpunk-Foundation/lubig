use crate::actions::{valid_path, PathStatus};

pub struct Text;
impl Text
{
    pub fn help()
    {
        println!("Welcome to the redigit command specification guide!:");
        println!("  conf: Use it to change the destination for src (source repositories path), prof (build profiles path) or prog (programs path).");
        println!("      | Examples:");
        println!("      | redigit conf src <path>");
        println!("      | redigit conf prof <path>");
        println!("      | redigit conf prog <path>");
        println!("  get: Use it to clone and add git repositories to redigit registries.");
        println!("      | Example:");
        println!("      | redigit get <https://url.com/wanted/repository.git> <custom_name>");
        println!("  add: Use it to add local git cloned repositories to redigit registries.");
        println!("      | Example:");
        println!("      | redigit add </local/repository/path> <custom_name>");
        println!("  lock/unlock: Use it to lock or unlock updates to a specific registered repository.");
        println!("      | Examples:");
        println!("      | redigit lock <registered_repository_name>");
        println!("      | redigit unlock <registered_repository_name>");
        println!("  update: Use it to upgrade every registered and unlocked repository.");
        println!("      | Example:");
        println!("      | redigit update");
        println!("  build: Use it to compile, build or rebuild a specific registered repository.");
        println!("      | Example:");
        println!("      | redigit build <registered_repository_name>");
        println!("  remove: Use it to delete a registered repository and its builds.");
        println!("      | Example:");
        println!("      | redigit remove <registered_repository_name>");
        println!("  list: Use it to list every registered, build (b) or unlocked (u) repository name.");
        println!("      | Examples:");
        println!("      | redigit list");
        println!("      | redigit list b");
        println!("      | redigit list u");
        println!("  For more information, refer to the Redigit User Manual available on GitHub: https://github.com/GrayDay-git/redigit");
        println!("(END)");
    }
    pub fn general_error() {
        println!("ERROR: Invalid redigit argument. Use 'redigit help' to read the command specification guide.");
    }
    pub fn shut_up() {
        println!("ERROR: To many arguments. Use 'redigit help' to read the command specification guide.");
    }
    pub fn path_state(path: &String) {
        match valid_path(path) {
            PathStatus::Available => println!("SUCCESS: Path accepted. Creating..."),
            PathStatus::Exist => println!("ERROR: This path exist, you need to use a non-existent path."),
            PathStatus::Invalid => println!("ERROR: This is an invalid path. You need to use a valid and non-existent path"),
        }
    }
}