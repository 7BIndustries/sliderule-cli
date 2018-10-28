extern crate argparse;
extern crate os_info;
extern crate walkdir;
extern crate liquid;

use std::io;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::io::prelude::*;
use argparse::{ArgumentParser, Store, List};
use walkdir::WalkDir;

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
        let subcommand = &args[0];

        // Check to see if we have a URL
        if subcommand.contains("/") {
            // git clone here and warn the user that what they're downloading is possibly read only
            git_clone(subcommand);

            println!("Unless you have write access to the downloaded repository, this copy will be read-only.")
        }
        else if subcommand == "all" {
            if Path::new(".git").exists() {
                git_pull();
            }

            // Just have npm update the entire project, not install a specific package
            npm_install("");
        }
        else if subcommand == "dependencies" {
            // Just have npm update the entire project, not install a specific package
            npm_install("");
        }
        else {
            panic!("ERROR: Subcommand of download not recognized.");
        }
    }
    else if command == "upload" {
        project_upload();
    }
    else if command == "remove" {
        let name = &args[0];

        println!("Remove called with {}.", name);

        // Deletes a local component's directory, or npm uninstalls a remote component
        remove(name);
    }
    else if command == "refactor" {
        let name = &args[0];

        // Convert the local component into a remote component
        refactor(name);
    }
    else {
        // The user has to supply a command, and it needs to be recognized
        if args.is_empty() {
            panic!("ERROR: Please supply an command to sliderule-cli. Run with -h to see the options.");
        }
        else {
            panic!("ERROR: Command not recognized: {}", command);
        }
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
            panic!("ERROR: Could not create dist directory: {:?}", error);
        }
    };

    // Make a new directory in componenets, cd into it, and then run the rest of this code
    match env::set_current_dir(&component_dir) {
        Ok(dir) => dir,
        Err(e) => {
            panic!("ERROR: Could not change into components directory: {}", e);
        }
    };

    // Create the components directory, if needed
    if !Path::new("components").exists() {
        match fs::create_dir("components") {
            Ok(dir) => dir,
            Err(error) => {
                panic!("ERROR: Could not create components directory: {:?}", error);
            }
        };
    }
    else {
        println!("components directory already exists, using existing directory.");
    }

    // Create the dist directory, if needed
    if !Path::new("dist").exists() {
        match fs::create_dir("dist") {
            Ok(dir) => dir,
            Err(error) => {
                panic!("ERROR: Could not create dist directory: {:?}", error);
            }
        };
    }
    else {
        println!("dist directory already exists, using existing directory.");
    }

    // Create the docs directory, if needed
    if !Path::new("docs").exists() {
        match fs::create_dir("docs") {
            Ok(dir) => dir,
            Err(error) => {
                panic!("ERROR: Could not create docs directory: {:?}", error);
            }
        };
    }
    else {
        println!("docs directory already exists, using existing directory.");
    }

    //Create the source directory, if needed
    if !Path::new("source").exists() {
        match fs::create_dir("source") {
            Ok(dir) => dir,
            Err(error) => {
                panic!("ERROR: Could not create source directory: {:?}", error);
            }
        };
    }
    else {
        println!("source directory already exists, using existing directory.");
    }

    // Generate the template readme file
    generate_readme(&name);

    // Generate bom_data.yaml
    generate_bom(&name);

    // Generate package.json, if needed
    generate_package_json(&name);

    // Generate the .top file that marks this as a top-level component
    if !is_component {
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
                panic!("ERROR: `git` was not found, please install");
            } else {
                panic!("ERROR: Could not initialize git repository: {}", e);
            }
        }
    }

    // Add the remote URL
    let output = Command::new("git").args(&["remote", "add", "origin", url]).output()
        .expect("ERROR: Unable to set remote URL for project.");

    // Let the user know if something went wrong
    if !output.stderr.is_empty() {
        panic!("ERROR: {:?}", output.stderr);
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
        .expect("ERROR: Failed to read change message line from user");

    let output = Command::new("git").args(&["add", "."]).output()
        .expect("ERROR: Unable to stage changes using git.");

    println!("status: {}", output.status);
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
    // {
    //     Ok(_) => println!("Staged changes for git."),
    //     Err(_) => println!("ERROR: Unable to stage changes using git.")
    // }

    let output = Command::new("git").args(&["commit", "-m", &message]).output()
        .expect("ERROR: Unable to push changes to remote git repository.");

    println!("status: {}", output.status);
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));

    // {
    //     Ok(_) => println!("Created commit."),
    //     Err(_) => println!("ERROR: Unable to create commit using git.")
    // }

    let output = Command::new("git").args(&["push", "origin", "master"]).output()
        .expect("ERROR: Unable to push changes to remote git repository.");

    println!("status: {}", output.status);
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));

    // {
    //     Ok(_) => println!("Pushed changes to remote git repository."),
    //     Err(_) => println!("ERROR: Unable to push changes to remote git repository.")
    // }
}

/*
 * Pulls latest updates from a component's git repo.
 */
fn git_pull() {
    match Command::new("git").args(&["pull", "origin", "master"]).output() {
        Ok(_) => println!("Pulled changes from component repository."),
        Err(_) => eprintln!("ERROR: Unable to pull changes from component repository.")
    }
}

//TODO: Check stderr on any commands that have output

fn git_clone(url: &str) {
    match Command::new("git").args(&["clone", "--recursive", url]).output() {
        Ok(_) => println!("Sucessfully cloned component repository."),
        Err(_) => eprintln!("ERROR: Unable to clone component repository.")
    }
}

/*
 * Uploads any changes to the project to the remote repository.
 */
fn project_upload() {
    let mut url = String::new();

    // Make sure this project has already been initialized as a repo
    if !Path::new(".git").exists() {
        println!("This project has not been initialized with a repository yet. Enter a URL of an existing repository to upload this component to:");

        io::stdin().read_line(&mut url)
            .expect("ERROR: Failed to read name or URL from user.");

        // Initialize the git repo and set the remote URL to push to
        git_init(url.trim());

        // Genreate gitignore file so that we don't commit and push things we shouldn't be
        generate_gitignore();
    }
    
    // Add all changes, commit and push
    git_add_and_commit();
}

/*
 * Converts a local component into a remote component, asking for a remote repo to push it to.
 */
fn refactor(name: &str) {
    let mut url = String::new();
    
    println!("Please enter the URL of an existing repository to upload the component to:");

    io::stdin().read_line(&mut url)
            .expect("ERROR: Failed to read name or URL from user.");

    let orig_dir = get_cwd();
    let component_dir = Path::new("components").join(name);

    if component_dir.exists() {
        // We need to be in the component's directory before running the next commands
        match env::set_current_dir(&component_dir) {
            Ok(dir) => dir,
            Err(e) => {
                panic!("ERROR: Could not change into components directory: {}", e);
            }
        };

        // Set the directory up as a git repo and then push the changes to the remote
        git_init(&url.trim());
        git_add_and_commit();

        // Change back up to the original, top level directory
        match env::set_current_dir(&orig_dir) {
            Ok(dir) => dir,
            Err(e) => {
                panic!("ERROR: Could not change into original parent directory: {}", e);
            }
        };

        // Remove the local component and then install it from the remote using npm
        remove(&name);
        npm_install(&url.trim());
    }
    else {
        panic!("ERROR: The component does not exist in the components directory.");
    }
}

/*
 * Removes a component from the project structure.
 */
fn remove(name: &str) {
    // let mut answer = String::new();

    // TODO: The user has to spell the component name out, so maybe that's enough of a safety check?
    // println!("Type Y/y and hit enter to continue removing this component: {}", name);

    // io::stdin().read_line(&mut answer)
    //     .expect("ERROR: Failed to read answer from user.");

    // Make sure that the answer was really yes on removal of the component
    // if &answer.trim().to_uppercase() != "Y" {
    //     println!("Aborting component removal.");

    //     return;
    // }

    let component_dir = Path::new("components").join(name);

    // If the component exists as a subdirectory of components delete the directory directly otherwise use npm to remove it.
    if component_dir.exists() {
        println!("Deleting component directory.");

        // Step through every file and directory in the path to be deleted and make sure that none are read-only
        for entry in WalkDir::new(&component_dir) {
            let entry = match entry {
                Ok(ent) => ent,
                Err(e) => panic!("ERROR: Could not handle entry while walking components directory tree: {}", e)
            };

            // Remove read-only permissions on every entry
            let md = &entry.path().metadata().
                expect("ERROR: Could not get metadata.");
            let mut perms = md.permissions();
            perms.set_readonly(false);
            fs::set_permissions(&entry.path(), perms)
                .expect("Error: Failed to set permissions on .git directory");
        }

        fs::remove_dir_all(component_dir)
            .expect("ERROR: not able to delete component directory.");
    }
    else {
        // Use npm to remove the remote component
        npm_uninstall(name);
    }

    println!("{} component removed.", name);
}

/*
 * Generates a template README.md file to help the user get started.
 */
fn generate_readme(name: &str) {
    if !Path::new("README.md").exists() {
        // Add the things that need to be put substituted into the README file
        let mut globals = liquid::value::Object::new();
        globals.insert("name".into(), liquid::value::Value::scalar(name.to_owned()));

        let contents = render_template("README.md.liquid", &mut globals);

        // Write the temmplate text into the readme file
        match fs::write("README.md", contents) {
            Ok(res) => res,
            Err(error) => {
                panic!("ERROR: Could not write to README.md file: {:?}", error);
            } 
        };
    }
    else {
        println!("README.md already exists, using existing file and refusting to overwrite.");
    }
}

fn generate_bom(name: &str) {
    if !Path::new("bom_data.yaml").exists() {
        // Add the things that need to be put substituted into the BoM file
        let mut globals = liquid::value::Object::new();
        globals.insert("name".into(), liquid::value::Value::scalar(name.to_owned()));

        let contents = render_template("bom_data.yaml.liquid", &mut globals);

        // Write the temmplate text into the readme file
        match fs::write("bom_data.yaml", contents) {
            Ok(res) => res,
            Err(error) => {
                panic!("ERROR: Cound not write to bom_data.yaml: {:?}", error);
            } 
        };
    }
    else {
        println!("bom_data.yaml already exists, using existing file and refusing to overwrite.");
    }
}

fn generate_package_json(name: &str) {
    if !Path::new("package.json").exists() {
        // Add the things that need to be put substituted into the package file
        let mut globals = liquid::value::Object::new();
        globals.insert("name".into(), liquid::value::Value::scalar(name.to_owned()));

        let contents = render_template("package.json.liquid", &mut globals);

        // Write the contents into the file
        match fs::write("package.json", contents) {
            Ok(res) => res,
            Err(error) => {
                panic!("ERROR: Could not write to package.json: {:?}", error);
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
        // Add the things that need to be put substituted into the gitignore file (none at this time)
        let mut globals = liquid::value::Object::new();

        let contents = render_template(".gitignore.liquid", &mut globals);

        // Write the contents to the file
        match fs::write(".gitignore", contents) {
            Ok(res) => res,
            Err(error) => {
                panic!("ERROR: Cound not write to .gitignore: {:?}", error);
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
    if !Path::new(".top").exists() {
        // Add the things that need to be put substituted into the .top file (none at this time)
        let mut globals = liquid::value::Object::new();

        let contents = render_template(".top.liquid", &mut globals);

        // Write the contents to the file
        match fs::write(".top", contents) {
            Ok(res) => res,
            Err(error) => {
                panic!("ERROR: Could not write to .top: {:?}", error);
            }
        };
    }
    else {
        println!(".top already exists, using existing file and refusing to overwrite.");
    }
}

/*
 * Reads a template to a string so that it can be written to a new components directory structure.
 */
fn render_template(template_name: &str, globals: &mut liquid::value::Object) -> String {
    // Figure out where the templates are stored
    let template_file = match env::current_exe() {
        Ok(path) => path,
        Err(e) => panic!("ERROR: Could not get sliderule-cli executable directory: {}", e)
    };
    let template_file = match template_file.parent() {
        Some(path) => path,
        None => panic!("ERROR: Could not get parent of sliderule-cli executable directory.")
    };
    let template_file = template_file.join("templates").join(template_name);

    // Read the template file into a string so that it can be rendered using Liquid
    let mut file = fs::File::open(&template_file).expect("Unable to open the file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read the file");

    // Render the output of the template using Liquid
    let template = liquid::ParserBuilder::with_liquid()
        .build()
        .parse(&contents).unwrap();

    let output = template.render(globals).unwrap();

    output
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
                panic!("ERROR: `npm` was not found, please install it.");
            } else {
                panic!("ERROR: Could not install component from remote repository: {}", e);
            }
        }
    }
}

/*
 * Uses the npm command to remove a remote component.
 */
fn npm_uninstall(name: &str) {
    let mut vec = Vec::new();
    vec.push("uninstall");
    
    let info = os_info::get();
    let mut cmd_name = "npm";

    // Set the command name properly based on which OS the user is running
    if info.os_type() == os_info::Type::Windows {
        cmd_name = r"C:\Program Files\nodejs\npm.cmd";
    }

    // If no URL was specified, just npm update the whole project
    if !name.is_empty() {
        vec.push("--save");
        vec.push(name);
    }

    println!("Working...");

    match Command::new(&cmd_name).args(&vec).output() {
        Ok(_) => {
            println!("Component uninstalled using npm.");
        },
        Err(e) => {
            if let std::io::ErrorKind::NotFound = e.kind() {
                panic!("ERROR: `npm` was not found, please install it.");
            } else {
                panic!("ERROR: Could not install component from remote repository: {}", e);
            }
        }
    }
}

/*
 * Gets the current working directory for us, and handles any errors.
 */
fn get_cwd() -> PathBuf {
    let path = env::current_dir();

    let cwd = match path {
        Ok(dir) => dir,
        Err(e) => {
            panic!("ERROR: Could not get current working directory: {}", e);
       }
    };

    cwd
}
