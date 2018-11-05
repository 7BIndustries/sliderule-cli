extern crate argparse;
extern crate sliderule_impl;

use argparse::{ArgumentParser, Store, List};
use sliderule_impl::sliderule;

fn main() {
    // What main command the user is wanting to use
    let mut command = String::new();
    let mut args: Vec<String> = Vec::new();
    let mut src_license = String::new();
    let mut docs_license = String::new();

    // Parse the command line arguments
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Tool to manage Sliderule projects.");
        ap.refer(&mut command)
            .add_argument("command", Store, "Sliderule command to run");
        ap.refer(&mut args)
            .add_argument("arguments", List, r#"Arguments for command"#);
        ap.refer(&mut src_license)
            .add_option(&["-s"], Store, "Specify a source license on the command line.");
        ap.refer(&mut docs_license)
            .add_option(&["-d"], Store, "Specify a documentation license on the command line.");
        ap.parse_args_or_exit();
    }

    // Handle the command line arguments
    if command == "create" {
        let name = &args[0];

        sliderule::create_component(&name, &src_license, &docs_license);
    }
    else if command == "add" {
        // The user is expected to have provided a URL of a remote component that can be downloaded
        let url = &args[0];

        sliderule::add_remote_component(&url);
    }
    else if command == "download" {
        let subcommand = &args[0];

        // Check to see if we have a URL
        if subcommand.contains("/") {
            // git clone here and warn the user that what they're downloading is possibly read only
            sliderule::download_component(subcommand);

            println!("Unless you have write access to the downloaded repository, this copy will be read-only.")
        }
        else if subcommand == "all" {
            sliderule::update_local_component();

            // Just have npm update the entire project, not install a specific package
            sliderule::update_dependencies();
        }
        else if subcommand == "dependencies" {
            // Just have npm update the entire project, not install a specific package
            sliderule::update_dependencies();
        }
        else {
            panic!("ERROR: Subcommand of download not recognized.");
        }
    }
    else if command == "upload" {
        sliderule::project_upload();
    }
    else if command == "remove" {
        let name = &args[0];

        println!("Remove called with {}.", name);

        // Deletes a local component's directory, or npm uninstalls a remote component
        sliderule::remove(name);
    }
    else if command == "refactor" {
        let name = &args[0];

        // Convert the local component into a remote component
        sliderule::refactor(name);
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
