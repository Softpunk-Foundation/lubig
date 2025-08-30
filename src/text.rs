pub struct Text;
impl Text
{
    pub fn help()
    {
        println!("Welcome to the lubig command specification guide!:");
        println!("  conf: Use it to change the destination for src (source repositories path), prof (build profiles path) or prog (programs path).");
        println!("      | Examples:");
        println!("      | lubig conf src <path>");
        println!("      | lubig conf prof <path>");
        println!("      | lubig conf prog <path>");
        println!("  get: Use it to clone and add git repositories to lubig registries.");
        println!("      | Example:");
        println!("      | lubig get <https://url.com/wanted/repository.git> <custom_name>");
        println!("  add: Use it to add local git cloned repositories to lubig registries.");
        println!("      | Example:");
        println!("      | lubig add </local/repository/path> <custom_name>");
        println!("  lock/unlock: Use it to lock or unlock updates to a specific registered repository.");
        println!("      | Examples:");
        println!("      | lubig lock <registered_repository_name> (optional)<repository_branch_line> ('main' by default)");
        println!("      | lubig unlock <registered_repository_name>");
        println!("  update: Use it to upgrade every registered and unlocked repository.");
        println!("      | Example:");
        println!("      | lubig update");
        println!("  build: Use it to compile, build or rebuild a specific registered repository.");
        println!("      | Example:");
        println!("      | lubig build <registered_repository_name>");
        println!("  remove: Use it to delete a registered repository and its builds.");
        println!("      | Example:");
        println!("      | lubig remove <registered_repository_name>");
        println!("  list: Use it to list every registered repository name.");
        println!("      | Example:");
        println!("      | lubig list");
        println!("  status: Show a specific registered repository status.");
        println!("      | Example:");
        println!("      | lubig status <registered_repository_name>");
        println!("  For more information, refer to the lubig User Manual available on GitHub: https://github.com/GrayDay-git/lubig");
        println!("(END)");
    }
    pub fn general_error() {
        println!("ERROR: Invalid lubig argument. Use 'lubig help' to read the command specification guide.");
    }
    pub fn exceed_args() {
        println!("ERROR: Too many arguments. Use 'lubig help' to read the command specification guide.");
    }
    pub fn need_args() {
        println!("ERROR: Too few arguments. Use 'lubig help' to read the command specification guide.");
    }
    pub fn key_exists(name:&str){
        println!("ERROR: '{}' is already registered", name);
    }
    pub fn no_git_repo(path: &str){
        println!("ERROR: '{}' This path is not a git repository", path);
    }
    pub fn key_doesnt_exists(name:&str){
        println!("ERROR: '{}' is not registered", name);
    }
    pub fn error_dir(path:&str){
        println!("ERROR: '{}' this path doesn't exists", path)
    }
}