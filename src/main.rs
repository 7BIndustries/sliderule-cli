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

    // Handle the command line arguments
    if command == "create" {
        // The argument after the new command should be the name of the new component
        // let name = &args[0];

        create_component();
        // create_new_component(&name);
    }
    else {
        println!("Command not recognized: {}", command);
    }
}

/*
 * Create a new Sliderule component or convert an existing project to being a Sliderule project.
 */
// fn create_new_component(name: &str) {
fn create_component() {
    println!("Enter the URL of the repository that you previously created for this project: ");

    let mut url = String::new();

    io::stdin().read_line(&mut url)
        .expect("Failed to read line from user");

    // Figure out what type of repository we're working with
    if url.contains("git") {
        println!("Setting up new componenet using git.");

        git_init(&url);
    }
    else {
        println!("ERROR: URL not recognized as a valid repository.");
    }

    // Create the components directory, if needed
    if !Path::new("components").exists() {
        match fs::create_dir("components") {
            Ok(dir) => dir,
            Err(_) => return
        };
    }
    else {
        println!("components directory already exists, using existing directory.")
    }
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
