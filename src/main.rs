extern crate argparse;
extern crate rpassword;
extern crate sliderule;

use argparse::{ArgumentParser, List, Store, StoreTrue};
use sliderule::SROutput;
use std::env;
use std::io;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

fn main() {
    let _version_num = "0.3.1";

    // What main command the user is wanting to use
    let mut command = String::new();
    let mut args: Vec<String> = Vec::new();
    let mut src_license = String::new();
    let mut docs_license = String::new();
    let mut message = String::new();
    let mut url = String::new();
    let mut yes_mode_active = false;
    let mut verbose = false;
    let mut version = false;

    // Some items for the command line help interface
    let app_description = "Tool to manage Sliderule projects.";
    let cmd_description = "Sliderule command to run: [create | download | upload | add | remove | refactor | licenses | login]";
    let args_description = "Arguments to Sliderule commands:
                            create [name],
                            download [all | dependencies | component_url],
                            add [remote_component_url],
                            remove [name],
                            refactor [name],
                            licenses [change | list],
                            changes [list]";

    // Parse the command line arguments
    {
        let mut ap = ArgumentParser::new();
        ap.set_description(app_description);
        ap.refer(&mut command)
            .add_argument("command", Store, cmd_description);
        ap.refer(&mut args)
            .add_argument("arguments", List, args_description);
        ap.refer(&mut src_license).add_option(
            &["-s"],
            Store,
            "Specify a source license on the command line.",
        );
        ap.refer(&mut docs_license).add_option(
            &["-d"],
            Store,
            "Specify a documentation license on the command line.",
        );
        ap.refer(&mut message).add_option(
            &["-m"],
            Store,
            "Specifies the message to attach to changes on an upload.",
        );
        ap.refer(&mut url).add_option(
            &["-u"],
            Store,
            "The URL to use when uploading, downloading or adding a component.",
        );
        ap.refer(&mut yes_mode_active).add_option(
            &["-y"],
            StoreTrue,
            "Answers yes to any questions for unattended operation.",
        );
        ap.refer(&mut verbose).add_option(
            &["-v"],
            StoreTrue,
            "Gives verbose output, helps with debugging why a command did not work.",
        );
        ap.refer(&mut version)
            .add_option(&["-V"], StoreTrue, "Outputs the version information.");
        ap.parse_args_or_exit();
    }

    // Check to see if the user wanted to see the version number
    if version {
        println!("sliderule-cli version: {}", _version_num);
        println!("sliderule-rs version: {}", sliderule::get_version());

        // We do not need to run any more code
        std::process::exit(1);
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

        let output =
            sliderule::create_component(&get_cwd(), name.to_string(), src_license, docs_license);

        // Show extra output only when the user requests it
        if verbose {
            print_stdout(&output);
        } else {
            println!("Component creation finished.");
        }

        // Show error information when it happens, whether the user has requested verbose output or not
        if !output.stderr.is_empty() {
            print_stderr(&output);
        }
    } else if command == "add" {
        // The user is expected to have provided a URL of a remote component that can be downloaded
        let url = &args[0];

        let output = sliderule::add_remote_component(&get_cwd(), &url, None);

        // Show extra output only when the user requests it
        if verbose {
            print_stdout(&output);
        } else {
            println!("Component add finished.");
        }

        // Show error information when it happens, whether the user has requested verbose output or not
        if !output.stderr.is_empty() {
            print_stderr(&output);
        }
    } else if command == "download" {
        let subcommand = &args[0];

        // Check to see if we have a URL
        if subcommand.contains("/") {
            // git clone here and warn the user that what they're downloading is possibly read only
            let output = sliderule::download_component(&get_cwd(), subcommand);

            // Show extra output only when the user requests it
            if verbose {
                print_stdout(&output);
            } else {
                println!("Component download finished.");
            }

            // Show error information when it happens, whether the user has requested verbose output or not
            if !output.stderr.is_empty() {
                print_stderr(&output);
            }

            println!("Unless you have write access to the downloaded repository, this copy will be read-only.")
        } else if subcommand == "all" {
            let output = sliderule::update_local_component(&get_cwd());

            // Show extra output only when the user requests it
            if verbose {
                print_stdout(&output);
            } else {
                println!("Component download finished.");
            }

            // Show error information when it happens, whether the user has requested verbose output or not
            if !output.stderr.is_empty() {
                print_stderr(&output);
            }

            // Just have npm update the entire project, not install a specific package
            let output = sliderule::update_dependencies(&get_cwd());

            // Show extra output only when the user requests it
            if verbose {
                print_stdout(&output);
            } else {
                println!("Component download of source and dependencies finished.");
            }

            // Show error information when it happens, whether the user has requested verbose output or not
            if !output.stderr.is_empty() {
                print_stderr(&output);
            }
        } else if subcommand == "dependencies" {
            // Just have npm update the entire project, not install a specific package
            let output = sliderule::update_dependencies(&get_cwd());

            // Show extra output only when the user requests it
            if verbose {
                print_stdout(&output);
            } else {
                println!("Component download of dependencies only finished.");
            }

            // Show error information when it happens, whether the user has requested verbose output or not
            if !output.stderr.is_empty() {
                print_stderr(&output);
            }
        } else {
            eprintln!("ERROR: Subcommand of download not recognized.");
            std::process::exit(3);
        }
    } else if command == "upload" {
        if message.is_empty() {
            // Get the upload message from the user to mark these changes with
            println!("Message to attach to these project changes:");

            io::stdin()
                .read_line(&mut message)
                .expect("ERROR: Failed to read upload message line from user");

            message = message.trim().to_string();
        }

        let mut userinfo = (String::new(), String::new());

        // Make sure this project has already been initialized as a repository
        if !Path::new(".git").exists() && url.is_empty() {
            println!("This project has not been initialized with a repository yet. Enter a URL of an existing repository to upload this component to:");

            io::stdin()
                .read_line(&mut url)
                .expect("ERROR: Failed to read name or URL from user.");

            url = url.trim().to_string();

            // Check to see if there needs to be a username and password set for this
            if url.contains("https") {
                userinfo = get_https_user_info();
            }
        }

        let mut user = None;
        let mut pass = None;
        if !userinfo.0.is_empty() && !userinfo.1.is_empty() {
            user = Some(userinfo.0.trim().to_string());
            pass = Some(userinfo.1.trim().to_string());
        }

        let output = sliderule::upload_component(&get_cwd(), message, url, user, pass);

        // Show extra output only when the user requests it
        if verbose {
            print_stdout(&output);
        } else {
            println!("Component upload finished.");
        }

        // Show error information when it happens, whether the user has requested verbose output or not
        if !output.stderr.is_empty() {
            print_stderr(&output);
        }
    } else if command == "remove" {
        let name = &args[0];

        if !yes_mode_active {
            let mut answer = String::new();

            println!(
                "Type Y/y and hit enter to continue removing this component: {}",
                name
            );

            io::stdin()
                .read_line(&mut answer)
                .expect("ERROR: Failed to read answer from user.");

            // Make sure that the answer was really yes on removal of the component
            if &answer.trim().to_uppercase() != "Y" {
                println!("Aborting component removal.");

                return;
            }
        }

        // Deletes a local component's directory, or npm uninstalls a remote component
        let output = sliderule::remove(&get_cwd(), name);

        // Show extra output only when the user requests it
        if verbose {
            print_stdout(&output);
        } else {
            println!("Component remove finished.");
        }

        // Show error information when it happens, whether the user has requested verbose output or not
        if !output.stderr.is_empty() {
            print_stderr(&output);
        }
    } else if command == "refactor" {
        let name = &args[0];

        let mut userinfo = (String::new(), String::new());

        if url.is_empty() {
            println!("Please enter the URL of an existing repository to upload the component to:");

            io::stdin()
                .read_line(&mut url)
                .expect("ERROR: Failed to read name or URL from user.");

            url = url.trim().to_string();
        }

        // Check to see if there needs to be a username and password set for this
        if url.contains("https") {
            userinfo = get_https_user_info();
        }

        let mut user = None;
        let mut pass = None;
        if !userinfo.0.is_empty() && !userinfo.1.is_empty() {
            user = Some(userinfo.0.trim().to_string());
            pass = Some(userinfo.1.trim().to_string());
        }

        // Convert the local component into a remote component
        let output = sliderule::refactor(&get_cwd(), name.to_string(), url, user, pass);

        // Show extra output only when the user requests it
        if verbose {
            print_stdout(&output);
        } else {
            println!("Component refactor finished.");
        }

        // Show error information when it happens, whether the user has requested verbose output or not
        if !output.stderr.is_empty() {
            print_stderr(&output);
        }
    } else if command == "licenses" {
        let subcommand = &args[0];
        let mut licenses = (String::new(), String::new());

        if subcommand == "change" {
            // Only ask for the licenses interactively if they weren't specified on the command line
            if src_license.is_empty() || docs_license.is_empty() {
                licenses = ask_for_licenses(false);
            }

            // Handle the occurrence of someone specifying licenses on the command line
            if src_license.is_empty() {
                src_license = licenses.0;
            }
            if docs_license.is_empty() {
                docs_license = licenses.1;
            }

            let output = sliderule::change_licenses(&get_cwd(), src_license, docs_license);

            // Show extra output only when the user requests it
            if verbose {
                print_stdout(&output);
            } else {
                println!("License change finished.");
            }

            // Show error information when it happens, whether the user has requested verbose output or not
            if !output.stderr.is_empty() {
                print_stderr(&output);
            }
        } else if subcommand == "list" {
            let license_list = sliderule::list_all_licenses(&get_cwd());

            println!("{}", license_list);
        } else {
            eprintln!("licenses subcommand not understood: {}", subcommand);
            std::process::exit(1);
        }
    } else if command == "login" {
        // Make sure this project has already been initialized as a repository
        if url.is_empty() {
            println!(
                "Enter a URL of an existing repository to that this component will be uploaded to:"
            );

            io::stdin()
                .read_line(&mut url)
                .expect("ERROR: Failed to read name or URL from user.");

            url = url.trim().to_string();
        }

        let mut userinfo = (String::new(), String::new());

        // Check to see if there needs to be a username and password set for this
        if url.contains("https") {
            userinfo = get_https_user_info();
        }

        let mut user = None;
        let mut pass = None;
        if !userinfo.0.is_empty() && !userinfo.1.is_empty() {
            user = Some(userinfo.0.trim().to_string());
            pass = Some(userinfo.1.trim().to_string());
        }

        // If a URL is not present, it will mess up the git config
        if url.is_empty() {
            println!("URL cannot be empty when logging into a repository.");
            std::process::exit(2);
        }

        // Change/add the login information for the user
        let output = sliderule::remote_login(&get_cwd(), Some(url), user, pass);

        // Show extra output only when the user requests it
        if verbose {
            print_stdout(&output);
        } else {
            println!("Finished setting username and password for remote repository.");
        }
    } else if command == "changes" {
        let subcommand = &args[0];

        // Allow the user to do multiple things with the changes, currently just 'list'
        if subcommand == "list" {
            let change_listing = sliderule::list_changes(&get_cwd());

            for line in change_listing.stdout {
                let mut out_line = line.replace(
                    "(use \"git add <file>...\" to include in what will be committed)",
                    "",
                );
                out_line = out_line.replace("nothing added to commit but untracked files present (use \"git add\" to track)", "");
                out_line = out_line.replace("Untracked files", "New files");

                println!("{}", out_line);
            }
            // If we have any errors, let the user know what they are
            if !change_listing.stderr.is_empty() {
                for line in change_listing.stderr {
                    println!("{}", line);
                }
            }
        }
    }

    // The user has to supply a command, and it needs to be recognized
    if command.is_empty() {
        println!("Please supply a command. Run with -h to see the options.");
        std::process::exit(1);
    }
}

/*
 * Prompts the user for an https username and a password, with a warning that doing so is a security concern.
 */
fn get_https_user_info() -> (String, String) {
    let mut username = String::new();
    let password: String;

    println!(
        "You need to enter a username and password for using an https URL. Please do that now."
    );
    println!("WARNING: Using https with sliderule-cli can lead to passwords being stored in plain text. It is better to use ssh.");
    print!("User: ");
    io::stdout().flush().ok().expect("Could not flush stdout");
    io::stdin()
        .read_line(&mut username)
        .expect("ERROR: Failed to read username from user");

    password = rpassword::prompt_password_stdout("Password: ").unwrap();

    (username, password)
}

/*
 * Prints all the lines of standard output to standard output.
 */
fn print_stdout(output: &SROutput) {
    for line in &output.stdout {
        if !line.is_empty() {
            println!("{}", line);
        }
    }
}

/*
 * Prints all the lines of standard output to standard output.
 */
fn print_stderr(output: &SROutput) {
    for line in &output.stderr {
        if !line.is_empty() {
            println!("{}", line);
        }
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
        io::stdin()
            .read_line(&mut source_license)
            .expect("ERROR: Failed to read name or license from user.");

        // If the user didn't choose a license, default to The Unlicense
        source_license = source_license.trim().to_string();
    }

    // Ask the user for their license choice for the documentation of this component
    if sliderule::get_level(&get_cwd()) == 0 || display_anyway {
        println!("Please choose a documentation license for this component.");
        println!("For a list of available licenses see https://spdx.org/licenses/");
        println!("Choice [{}]:", default_docs_lic);
        io::stdin()
            .read_line(&mut doc_license)
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

    let cwd = path.expect("Could not get current working directory.");

    cwd
}
