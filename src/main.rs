extern crate argparse;

use std::io;
use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;
use argparse::{ArgumentParser, Store, List};
extern crate os_info;

fn main() {
    // What main command the user is wanting to use
    let mut command = String::new();
    let mut args: Vec<String> = Vec::new();

    // Parse the command line arguments
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Tool to manage Sliderule projects.");
        ap.refer(&mut command)
            .add_argument("command", Store, "Sliderule command to run");
        ap.refer(&mut args)
            .add_argument("arguments", List, r#"Arguments for command"#);
        ap.parse_args_or_exit();
    }

    // Handle the command line arguments
    if command == "create" {
        // The user is supposed to pass a name for the component in as an argument
        let name = &args[0];

        create_component(&name);
    }
    else if command == "add" {
        // The user is expected to have provided a URL of a remote component that can be downloaded
        let url = &args[0];

        npm_install(&url);
    }
    else if command == "download" {
        // TODO: Handle the different command line arguments (all, dependencies, component_url) here

        // Just have npm update the entire project, not install a specific package
        npm_install("");
    }
    else if command == "upload" {
        project_upload();
    }
    else {
        println!("Command not recognized: {}", command);
    }
}

/*
 * Create a new Sliderule component or convert an existing project to being a Sliderule project.
 */
fn create_component(name: &String) {
    // Check to see if the current directory is a component
    let is_component = Path::new("components").exists() && Path::new("bom_data.yaml").exists();

    // The path can either lead to a top level component (project), or a component nested within a project
    let mut component_dir = Path::new("components").join(name);

    // This is a top level component (project)
    if !is_component {
        component_dir = Path::new(name).to_path_buf();
    }

    // Create a directory for our component inside the components directory
    match fs::create_dir(&component_dir) {
        Ok(dir) => dir,
        Err(error) => {
            println!("ERROR: Could not create dist directory: {:?}", error);
        }
    };

    // Make a new directory in componenets, cd into it, and then run the rest of this code
    match env::set_current_dir(&component_dir) {
        Ok(dir) => dir,
        Err(e) => {
            println!("Could not change into components directory: {}", e);
        }
    };

    // Create the components directory, if needed
    if !Path::new("components").exists() {
        match fs::create_dir("components") {
            Ok(dir) => dir,
            Err(error) => {
                println!("ERROR: Could not create components directory: {:?}", error);
            }
        };
    }
    else {
        println!("components directory already exists, using existing directory.")
    }

    // Create the dist directory, if needed
    if !Path::new("dist").exists() {
        match fs::create_dir("dist") {
            Ok(dir) => dir,
            Err(error) => {
                println!("ERROR: Could not create dist directory: {:?}", error);
            }
        };
    }
    else {
        println!("dist directory already exists, using existing directory.")
    }

    // Create the docs directory, if needed
    if !Path::new("docs").exists() {
        match fs::create_dir("docs") {
            Ok(dir) => dir,
            Err(error) => {
                println!("ERROR: Could not create docs directory: {:?}", error);
            }
        };
    }
    else {
        println!("docs directory already exists, using existing directory.")
    }

    //Create the source directory, if needed
    if !Path::new("source").exists() {
        match fs::create_dir("source") {
            Ok(dir) => dir,
            Err(error) => {
                println!("ERROR: Could not create source directory: {:?}", error);
            }
        };
    }
    else {
        println!("source directory already exists, using existing directory.")
    }

    // Generate the template readme file
    generate_readme(&name);

    // Generate bom_data.yaml
    generate_bom(&name);

    // Track whether this is a subcomponent or a project
    if is_component {
        generate_dot_file();
    }

    println!("Finished setting up component.");
}

/*
 * Uses the installed git command to initialize a project repo.
 */
fn git_init(url: &str) {
    println!("Working...");

    // Initialize the current directory as a git repo
    match Command::new("git").args(&["init"]).output() {
        Ok(_) => println!("git repository initialized for project."),
        Err(e) => {
            if let std::io::ErrorKind::NotFound = e.kind() {
                println!("`git` was not found, please install");
            } else {
                println!("Could not initialize git repository.");
            }
        }
    }

    // Add the remote URL
    match Command::new("git").args(&["remote", "add", "origin", url]).output() {
        Ok(_) => println!("Set git remote for project."),
        Err(_) => println!("Unable to set remote URL for project.")
    }

    println!("Done initializing git repository for project.");
}

/*
 * Adds, commits and pushes any changes to the remote git repo.
 */
fn git_add_and_commit() {
    let mut message = String::new();

    // Get the commit message from the user to mark these changes with
    println!("Message to attach to these project changes:");

    io::stdin().read_line(&mut message)
        .expect("Failed to read change message line from user");

    match Command::new("git").args(&["add", "."]).output() {
        Ok(_) => println!("Staged changes for git."),
        Err(_) => println!("Unable to stage changes using git.")
    }

    match Command::new("git").args(&["commit", "-m", &message]).output() {
        Ok(_) => println!("Created commit."),
        Err(_) => println!("Unable to create commit using git.")
    }

    match Command::new("git").args(&["push", "origin", "master"]).output() {
        Ok(_) => println!("Pushed changes to remote git repository."),
        Err(_) => println!("Unable to push changes to remote git repository.")
    }
}

/*
 * Uploads any changes to the project to the remote repository.
 */
fn project_upload() {
    // TODO If the project hasn't been git inited yet, prompt the user for a URL and take care of it.

    // println!("Enter a name of a new local component, or a URL for a remote component to download: ");

    // io::stdin().read_line(&mut component_info)
    //     .expect("Failed to read name or URL from user.");

    // if init_remote == true {
    //     let last_path_part = url.split("/").last().unwrap().trim();
    //     component_name = str::replace(last_path_part, ".git", "");
    // }

    // If we're not setting this component up to be a remote component, we don't want a package.json
    // if init_remote == true {
    //     // Generate package.json
    //     generate_package_json(&component_name);
    // }

    // Make sure this project has already been initialized as a repo
    if !Path::new(".git").exists() {
        println!("This project has not been initialized with a repository yet. Try running 'sliderule-cli create' and then try again.");
    }
    else {
        git_add_and_commit();
    }
}

/*
 * Generates a template README.md file to help the user get started.
 */
fn generate_readme(name: &str) {
    if !Path::new("README.md").exists() {
        let contents = format!("# {}\r\nNew Sliderule DOF component.\r\n", name);

        // Write the temmplate text into the readme file
        match fs::write("README.md", contents) {
            Ok(res) => res,
            Err(error) => {
                println!("Could not write to README.md file: {:?}", error);
            } 
        };
    }
    else {
        println!("README.md already exists, using existing file and refusting to overwrite.");
    }
}

fn generate_bom(name: &str) {
    if !Path::new("bom_data.yaml").exists() {
        let contents = format!("# Bill of Materials for {}\r\nparts:\r\n  component_1:\r\n    options:\r\n    - specific_component_variation\r\n    default_option: 0\r\n    quantity: 1\r\n    quantity_units: part\r\n    name: Sample Component\r\n    notes: ''\r\n\r\norder:\r\n  -component_1\r\n", name);

        // Write the temmplate text into the readme file
        match fs::write("bom_data.yaml", contents) {
            Ok(res) => res,
            Err(error) => {
                println!("Cound not write to bom_data.yaml: {:?}", error);
            } 
        };
    }
    else {
        println!("bom_data.yaml already exists, using existing file and refusing to overwrite.");
    }
}

fn generate_package_json(name: &str) {
    if !Path::new("package.json").exists() {
        let mut contents: String = "{\r\n  \"name\": \"".to_owned();
        contents.push_str(&name);
        let append: &str = "\",\r\n  \"version\": \"1.0.0\",\r\n  \"description\": \"Sliderule DOF component.\",\r\n  \"dependencies\": {\r\n  }\r\n}\r\n";
        contents.push_str(append);

        // Write the contents into the file
        match fs::write("package.json", contents) {
            Ok(res) => res,
            Err(error) => {
                println!("Could not write to package.json: {:?}", error);
            }
        };
    }
    else {
        println!("package.json already exists, using existing file and refusting to overwrite.");
    }
}

/*
 * Generates the .gitignore file used by the git command to ignore files and directories.
 */
fn generate_gitignore() {
    if !Path::new(".gitignore").exists() {
        let contents: String = "# Dependency directories\r\nnode_modules/\r\n\r\n# Distribution directory\r\ndist/\r\n".to_string();

        // Write the contents to the file
        match fs::write(".gitignore", contents) {
            Ok(res) => res,
            Err(error) => {
                println!("Cound not write to .gitignore: {:?}", error);
            }
        };
    }
    else {
        println!(".gitignore already exists, using existing file and refusing to overwrite.");
    }
}

/*
 * Generates the dot file that tracks whether this is a top level component/project or a subcomponent
 */
fn generate_dot_file() {
    if !Path::new(".subcomponent").exists() {
        let contents: String = "".to_string();

        // Write the contents to the file
        match fs::write(".subcomponent", contents) {
            Ok(res) => res,
            Err(error) => {
                println!("Cound not write to .subcomponent: {:?}", error);
            }
        };
    }
    else {
        println!(".subcomponent already exists, using existing file and refusing to overwrite.");
    }
}

/*
 * Attemps to use npm, if installed, otherwise tries to mimic what npm would do.
 */
fn npm_install(url: &str) {
    let mut vec = Vec::new();
    vec.push("install");
    
    let info = os_info::get();
    let mut cmd_name = "npm";

    // Set the command name properly based on which OS the user is running
    if info.os_type() == os_info::Type::Windows {
        cmd_name = r"C:\Program Files\nodejs\npm.cmd";
    }

    // If no URL was specified, just npm update the whole project
    if !url.is_empty() {
        vec.push("--save");
        vec.push(url);
    }

    println!("Working...");

    match Command::new(&cmd_name).args(&vec).output() {
        Ok(_) => {
            if !url.is_empty() {
                println!("Component installed from remote repository.");
            }
            else {
                println!("Sliderule project updated.");
            }
        },
        Err(e) => {
            if let std::io::ErrorKind::NotFound = e.kind() {
                println!("`npm` was not found.");

                // TODO: Call internal npm implementation here
            } else {
                println!("Could not install component from remote repository: {}", e);
            }
        }
    }
}
