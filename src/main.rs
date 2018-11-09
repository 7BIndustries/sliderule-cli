extern crate argparse;
extern crate sliderule_impl;

use std::io;
use argparse::{ArgumentParser, Store, StoreTrue, List};
use sliderule_impl::sliderule;

fn main() {
    // What main command the user is wanting to use
    let mut command = String::new();
    let mut args: Vec<String> = Vec::new();
    let mut src_license = String::new();
    let mut docs_license = String::new();
    let mut yes_mode_active = false;

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
        ap.refer(&mut yes_mode_active)
            .add_option(&["-y"], StoreTrue, "Answers yes to any questions for unattended operation.");
        ap.parse_args_or_exit();
    }

    // Handle the command line arguments
    if command == "create" {
        let name = &args[0];

        // Find out what licenses the user wants to use
        let licenses = ask_for_licenses(false);
        src_license = licenses.0;
        docs_license = licenses.1;

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

        if !yes_mode_active {
            let mut answer = String::new();

            println!("Type Y/y and hit enter to continue removing this component: {}", name);

            io::stdin().read_line(&mut answer)
                .expect("ERROR: Failed to read answer from user.");

            // Make sure that the answer was really yes on removal of the component
            if &answer.trim().to_uppercase() != "Y" {
                println!("Aborting component removal.");

                return;
            }
        }

        println!("Remove called with {}.", name);

        // Deletes a local component's directory, or npm uninstalls a remote component
        sliderule::remove(name);
    }
    else if command == "refactor" {
        let name = &args[0];

        // Convert the local component into a remote component
        sliderule::refactor(name);
    }
    else if command == "change_license" {
        let licenses = ask_for_licenses(true);
        src_license = licenses.0;
        docs_license = licenses.1;

        sliderule::change_licenses(&src_license, &docs_license);
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
 * Prompt the user to ask for licenses.
 */
fn ask_for_licenses(display_anyway: bool) -> (String, String) {
    let licenses = sliderule::get_licenses();
    let default_src_license = licenses.0;
    let default_docs_lic = licenses.1;

    let mut source_license = String::new();
    let mut doc_license = String::new();

    // Ask the user for their license choice for the source of this component if they haven't specified it on the command line
    if sliderule::get_level() == 0 || display_anyway {
        // Ask the user to choose a source license
        println!("Please choose a source license for this component.");
        println!("For a list of available licenses see https://spdx.org/licenses/");
        println!("Choice [{}]:", default_src_license);
        io::stdin().read_line(&mut source_license)
            .expect("ERROR: Failed to read name or license from user.");

        // If the user didn't choose a license, default to The Unlicense
        source_license = source_license.trim().to_string();
    }

    // Ask the user for their license choice for the documentation of this component
    if sliderule::get_level() == 0 || display_anyway {
        println!("Please choose a documentation license for this component.");
        println!("For a list of available licenses see https://spdx.org/licenses/");
        println!("Choice [{}]:", default_docs_lic);
        io::stdin().read_line(&mut doc_license)
            .expect("ERROR: Failed to read name or license from user.");

        // If the user didn't choose a license, default to The Unlicense
        doc_license = doc_license.trim().to_string();
    }

    // If we didn't get anything, we need to stick with the default
    if source_license.is_empty() {
        source_license = default_src_license;
    }
    if doc_license.is_empty() {
        doc_license = default_docs_lic;
    }

    (source_license, doc_license)
}
