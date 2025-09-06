/// `Text` is a utility struct containing only static methods.
/// It centralizes all user-facing output messages, ensuring consistency
/// and making it easier to maintain or localize in the future.
pub struct Text;

impl Text
{
    /// Displays the help guide for all LUBIG commands.
    /// This is the main reference for end users to understand usage patterns.
    pub fn help()
    {
        println!("Welcome to the lubig command specification guide!:");
        
        // Configuration command
        println!("  conf: Use it to change the destination for src (source repositories path), prof (build profiles path) or prog (programs path).");
        println!("      | Examples:");
        println!("      | lubig conf src <path>");
        println!("      | lubig conf prof <path>");
        println!("      | lubig conf prog <path>");
        
        // Clone and register remote repository
        println!("  get: Use it to clone and add git repositories to lubig registries.");
        println!("      | Example:");
        println!("      | lubig get <https://url.com/wanted/repository.git> <custom_name>");
        
        // Register local repository
        println!("  add: Use it to add local git cloned repositories to lubig registries.");
        println!("      | Example:");
        println!("      | lubig add </local/repository/path> <custom_name>");
        
        // Lock/unlock repository updates
        println!("  lock/unlock: Use it to lock or unlock updates to a specific registered repository.");
        println!("      | Examples:");
        println!("      | lubig lock <registered_repository_name>");
        println!("      | lubig unlock <registered_repository_name> (optional)<repository_branch_line> ('main' by default)");
        
        // Upgrade all unlocked repositories
        println!("  upgrade: Use it to upgrade every registered and unlocked repository.");
        println!("      | Example:");
        println!("      | lubig upgrade");
        
        // Build a registered repository
        println!("  build: Use it to compile, build or rebuild a specific registered repository.");
        println!("      | Example:");
        println!("      | lubig build <registered_repository_name>");
        
        // Remove a registered repository
        println!("  remove: Use it to delete a registered repository and its builds.");
        println!("      | Example:");
        println!("      | lubig remove <registered_repository_name>");
        
        // List all registered repositories
        println!("  list: Use it to list every registered repository name.");
        println!("      | Example:");
        println!("      | lubig list");
        
        // Show repository status
        println!("  status: Show a specific registered repository status.");
        println!("      | Example:");
        println!("      | lubig status <registered_repository_name>");
        
        // External reference to user manual
        println!("  For more information, refer to the lubig User Manual available on GitHub: https://github.com/GrayDay-git/lubig");
        println!("(END)");
    }

    /// Generic error for invalid commands.
    pub fn general_error() {
        println!("ERROR: Invalid lubig argument. Use 'lubig help' to read the command specification guide.");
    }

    /// Error for too many arguments.
    pub fn exceed_args() {
        println!("ERROR: Too many arguments. Use 'lubig help' to read the command specification guide.");
    }

    /// Error for too few arguments.
    pub fn need_args() {
        println!("ERROR: Too few arguments. Use 'lubig help' to read the command specification guide.");
    }

    /// Error when a repository name is already registered.
    pub fn key_exists(name:&str){
        println!("ERROR: '{}' is already registered", name);
    }

    /// Error when a path is not a Git repository.
    pub fn no_git_repo(path: &str){
        println!("ERROR: '{}' This path is not a git repository", path);
    }

    /// Error when a repository name is not registered.
    pub fn key_doesnt_exists(name:&str){
        println!("ERROR: '{}' is not registered", name);
    }

    /// Error when a directory path does not exist.
    pub fn error_dir(path:&str){
        println!("ERROR: '{}' this path doesn't exists", path)
    }
}