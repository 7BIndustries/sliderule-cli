extern crate argparse;
extern crate sliderule;

use std::env;
use std::io;
use argparse::{ArgumentParser, Store, StoreTrue, List};
use std::path::{Path, PathBuf};

fn main() {
    // What main command the user is wanting to use
    let mut command = String::new();
    let mut args: Vec<String> = Vec::new();
    let mut src_license = String::new();
    let mut docs_license = String::new();
    let mut message = String::new();
    let mut url = String::new();
    let mut yes_mode_active = false;

    // Some items for the command line help interface
    let app_description = "Tool to manage Sliderule projects.";
    let cmd_description = "Sliderule command to run: [create | download | upload | add | remove | refactor | licenses ]";
    let args_description = "Arguments to Sliderule commands:
                            create [name],
                            download [all | dependencies | component_url],
                            add [remote_component_url],
                            remove [name],
                            refactor [name],
                            licenses [change | list]";

    // Parse the command line arguments
    {
        let mut ap = ArgumentParser::new();
        ap.set_description(app_description);
        ap.refer(&mut command)
            .add_argument("command", Store, cmd_description);
        ap.refer(&mut args)
            .add_argument("arguments", List, args_description);
        ap.refer(&mut src_license)
            .add_option(&["-s"], Store, "Specify a source license on the command line.");
        ap.refer(&mut docs_license)
            .add_option(&["-d"], Store, "Specify a documentation license on the command line.");
        ap.refer(&mut message)
            .add_option(&["-m"], Store, "Specifies the message to attach to changes on an upload.");
        ap.refer(&mut url)
            .add_option(&["-u"], Store, "The URL to use when uploading, downloading or adding a component.");
        ap.refer(&mut yes_mode_active)
            .add_option(&["-y"], StoreTrue, "Answers yes to any questions for unattended operation.");
        ap.parse_args_or_exit();
    }

    // Handle the command line arguments
    if command == "create" {
        let name = &args[0];

        // Only ask for licenses if they are not specified on the command line
        if src_license.is_empty() || docs_license.is_empty() {
            // Find out what licenses the user wants to use
            let licenses = ask_for_licenses(false);

            // Handle the occurrence of someone specifying licenses on the command line
            if src_license.is_empty() {
                src_license = licenses.0;
            }
            if docs_license.is_empty() {
                docs_license = licenses.1;
            }
        }

        sliderule::create_component(&get_cwd(), &name, &src_license, &docs_license);
    }
    else if command == "add" {
        // The user is expected to have provided a URL of a remote component that can be downloaded
        let url = &args[0];

        sliderule::add_remote_component(&get_cwd(), &url);
    }
    else if command == "download" {
        let subcommand = &args[0];

        // Check to see if we have a URL
        if subcommand.contains("/") {
            // git clone here and warn the user that what they're downloading is possibly read only
            sliderule::download_component(&get_cwd(), subcommand);

            println!("Unless you have write access to the downloaded repository, this copy will be read-only.")
        }
        else if subcommand == "all" {
            sliderule::update_local_component(&get_cwd());

            // Just have npm update the entire project, not install a specific package
            sliderule::update_dependencies(&get_cwd());
        }
        else if subcommand == "dependencies" {
            // Just have npm update the entire project, not install a specific package
            sliderule::update_dependencies(&get_cwd());
        }
        else {
            panic!("ERROR: Subcommand of download not recognized.");
        }
    }
    else if command == "upload" {
        if message.is_empty() {
            // Get the upload message from the user to mark these changes with
            println!("Message to attach to these project changes:");

            io::stdin().read_line(&mut message)
                .expect("ERROR: Failed to read upload message line from user");

            message = message.trim().to_string();
        }

        // Make sure this project has already been initialized as a repository
        if !Path::new(".git").exists() && url.is_empty() {
            println!("This project has not been initialized with a repository yet. Enter a URL of an existing repository to upload this component to:");

            io::stdin().read_line(&mut url)
                .expect("ERROR: Failed to read name or URL from user.");

            url = url.trim().to_string();
        }

        sliderule::upload_component(&get_cwd(), message, &url);
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
        sliderule::remove(&get_cwd(), name);
    }
    else if command == "refactor" {
        let name = &args[0];

        if url.is_empty() {
            println!("Please enter the URL of an existing repository to upload the component to:");

            io::stdin().read_line(&mut url)
                .expect("ERROR: Failed to read name or URL from user.");

            url = url.trim().to_string();
        }

        // Convert the local component into a remote component
        sliderule::refactor(&get_cwd(), name.to_string(), url);
    }
    else if command == "licenses" {
        let subcommand = &args[0];

        if subcommand == "change" {
            let licenses = ask_for_licenses(true);

            // Handle the occurrence of someone specifying licenses on the command line
            if src_license.is_empty() {
                src_license = licenses.0;
            }
            if docs_license.is_empty() {
                docs_license = licenses.1;
            }

            sliderule::change_licenses(&get_cwd(), &src_license, &docs_license);
        }
        else if subcommand == "list" {
            let license_list = sliderule::list_all_licenses(&get_cwd());

            println!("{}", license_list);
        }
        else {
            println!("licenses subcommand not understood: {}", subcommand);
            std::process::exit(1);
        }
    }

    // The user has to supply a command, and it needs to be recognized
    if args.is_empty() {
        println!("Please supply an command. Run with -h to see the options.");
        std::process::exit(1);
    }
}

/*
 * Prompt the user to ask for licenses.
 */
fn ask_for_licenses(display_anyway: bool) -> (String, String) {
    let licenses = sliderule::get_licenses(&get_cwd());
    let default_src_license = licenses.0;
    let default_docs_lic = licenses.1;

    let mut source_license = String::new();
    let mut doc_license = String::new();

    // Ask the user for their license choice for the source of this component if they haven't specified it on the command line
    if sliderule::get_level(&get_cwd()) == 0 || display_anyway {
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
    if sliderule::get_level(&get_cwd()) == 0 || display_anyway {
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

/*
* Gets the current working directory for us, and handles any errors.
*/
fn get_cwd() -> PathBuf {
    let path = env::current_dir();

    let cwd = path
        .expect("Could not get current working directory.");

    cwd
}
