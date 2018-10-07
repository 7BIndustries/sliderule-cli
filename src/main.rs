extern crate argparse;
extern crate git2;

use std::io;
use std::env;
use std::fs;
use std::path::Path;
use argparse::{ArgumentParser, Store, List};
use git2::{Repository};

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

    let mut component_info = String::new();

    // Handle the command line arguments
    if command == "create" {
        component_info = String::from("sliderule-cli_new_project");

        create_component(&component_info, true);
    }
    else if command == "add_component" {
        println!("Enter a name of a new local component, or a URL for a remote component to download: ");

        io::stdin().read_line(&mut component_info)
            .expect("Failed to read name or URL from user.");

        if component_info.contains("/") {
            // TODO: We have a remote URL, install using npm
            // npm_install(component_info);
        }
        else {
            // The user wants to create a local component
            create_component(&component_info.trim().to_string(), false);
        }
    }
    else {
        println!("Command not recognized: {}", command);
    }
}

/*
 * Create a new Sliderule component or convert an existing project to being a Sliderule project.
 */
fn create_component(component_info: &String, init_remote: bool) {
    let mut url = String::new();
    let mut component_name = component_info.to_string();

    // TODO: Make new directory in components and cd into it unless this is a project level repo
    if component_info != "sliderule-cli_new_project" {
        // let mut component_dir_str: String = "components/".to_owned();
        // component_dir_str.push_str(&component_info);
        let component_dir_str = format!("components/{}", component_info);

        // Create a directory for our component inside the components directory
        match fs::create_dir(component_dir_str) {
            Ok(dir) => dir,
            Err(error) => {
                println!("ERROR: Could not create dist directory: {:?}", error);
            }
        };

        // Make a new directory in componenets, cd into it, and then run the rest of this code
        let components_dir = Path::new("components").join(component_info);
        match env::set_current_dir(&components_dir) {
            Ok(dir) => dir,
            Err(_) => {
                println!("Could not change into components directory. Has this project been initialized as a Sliderule project?");
            }
        };
    }

    // See if we need to set up a repository for this component
    if init_remote == true {
        println!("Enter the URL of the repository that you previously created for this project: ");

        io::stdin().read_line(&mut url)
            .expect("Failed to read line from user");

        // Figure out what type of repository we're working with
        if url.contains("git") {
            println!("Setting up new componenet using git.");

            // Set the current directory up as a git repo
            git_init(&url);

            // Make sure we have an appropriate gitignore file
            generate_gitignore();
        }
        else {
            println!("ERROR: URL not recognized as a valid repository.");
        }
    }

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

    if init_remote == true {
        let last_path_part = url.split("/").last().unwrap().trim();
        component_name = str::replace(last_path_part, ".git", "");
    }

    // Generate the template readme file
    generate_readme(&component_name);

    // Generate bom_data.yaml
    generate_bom(&component_name);

    // If we're not setting this component up to be a remote component, we don't want a package.json
    if init_remote == true {
        // Generate package.json
        generate_package_json(&component_name);
    }

    println!("Finished setting up component.");
}

/*
 * Same as calling git init
 */
fn git_init(url: &str) {
    let path = env::current_dir().unwrap();

    // We don't want to re-initialize an existing git structure
    if !Path::new(".git").exists() {
        let repo = match Repository::init(&path) {
            Ok(path) => path,
            Err(_) => return
        };

        match repo.remote("origin", url) {
            Ok(path) => path,
            Err(_) => return
        };

        println!("Initialized git repository in {}", path.display());
    }
    else {
        println!("WARNING: Directory {} already initialized as a git repository.", path.display());
    }
}

/*
 * Generates a template README.md file to help the user get started.
 */
fn generate_readme(name: &str) {
    if !Path::new("README.md").exists() {
        let mut contents: String = "# ".to_owned();
        contents.push_str(&name);
        let append: &str = "\r\nNew Sliderule DOF component.\r\n";
        contents.push_str(append);

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
        let mut contents: String = "# Bill of Materials for ".to_owned();
        contents.push_str(&name);
        let append: &str = "\r\nparts:\r\n  component_1:\r\n    options:\r\n    - specific_component_variation\r\n    default_option: 0\r\n    quantity: 1\r\n    quantity_units: part\r\n    name: Sample Component\r\n    notes: ''\r\n\r\norder:\r\n  -component_1\r\n";
        contents.push_str(append);

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

fn generate_gitignore() {
    if !Path::new(".gitignore").exists() {
        let contents: String = "# Dependency directories\r\nnode_modules/\r\n\r\n# Distribution directory\r\ndist/\r\n".to_owned();

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
