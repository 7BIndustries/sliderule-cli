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
    let mut description = String::new();
    let mut quantity = String::new();
    let mut quantity_units = String::new();
    let mut notes = String::new();
    let mut list = String::new();
    let mut item_name = String::new();
    let mut yes_mode_active = false;
    let mut verbose = false;
    let mut version = false;

    // Some items for the command line help interface
    let app_description = "Tool to manage Sliderule projects.";
    let cmd_description = "Sliderule command to run: [create | download | upload | add | remove | refactor | licenses | login]";
    let args_description = "Arguments to Sliderule commands:
                            create [name | list.item ] [description],
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
            &["-s", "--source-license"],
            Store,
            "Specify a source license on the command line.",
        );
        ap.refer(&mut docs_license).add_option(
            &["-d", "--doc-license"],
            Store,
            "Specify a documentation license on the command line.",
        );
        ap.refer(&mut message).add_option(
            &["-m", "--message"],
            Store,
            "Specifies the message to attach to changes on an upload.",
        );
        ap.refer(&mut url).add_option(
            &["-u", "--url"],
            Store,
            "The URL to use when uploading, downloading or adding a component.",
        );
        ap.refer(&mut yes_mode_active).add_option(
            &["-y", "--yes"],
            StoreTrue,
            "Answers yes to any questions for unattended operation.",
        );
        ap.refer(&mut verbose).add_option(
            &["-v", "--verbose"],
            StoreTrue,
            "Gives verbose output, helps with debugging why a command did not work.",
        );
        ap.refer(&mut version).add_option(
            &["-V", "--version"],
            StoreTrue,
            "Outputs the version information.",
        );
        ap.refer(&mut description).add_option(
            &["-d", "--description"],
            Store,
            "Specifies a description for a component.",
        );
        ap.refer(&mut quantity).add_option(
            &["-q", "--quantity"],
            Store,
            "Specify the quantity for a component.",
        );
        ap.refer(&mut notes).add_option(
            &["-n", "--notes"],
            Store,
            "Specify notes to be attached to a component.",
        );
        ap.refer(&mut list).add_option(
            &["-l", "--list"],
            Store,
            "Specifies the list (parts or tools) that a component belongs to.",
        );
        ap.refer(&mut item_name).add_option(
            &["-i", "--item-name"],
            Store,
            "Specifies the name of the item this component should be an option for.",
        );
        ap.refer(&mut quantity_units).add_option(
            &["-z", "--quantity-units"],
            Store,
            "Specifies which units the quantity of the component is in.",
        );
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
        let mut name = String::from("");

        // TODO: Check if not top level, ask for these other things
        if sliderule::get_level(&get_cwd()) == 0 {
            if description.is_empty() {
                // Ask for description and munge it into a name
                description = ask_for_description();
                name = sliderule::munge_component_description(&description);

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

                let output = sliderule::create_component(
                    &get_cwd(),
                    name.to_string(),
                    src_license,
                    docs_license,
                );

                // If we had an error, we need to display it and exit
                if output.status != 0 {
                    print_stdout(&output);
                    print_stderr(&output);

                    std::process::exit(1);
                }
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

                // TODO: Put the description into the readme
            }
        } else {
            if description.is_empty() {
                // If the list doesn't exist already, ask the user for a description (note: this is separate from the item description)
                description = ask_for_description();

                name = sliderule::munge_component_description(&description);
            }

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

            // If necessary, ask the user for the list this component belongs to
            if list.is_empty() {
                list = ask_for_list();
            }

            // If necessary, ask the user for the item name of the component being added to the project
            if item_name.is_empty() {
                item_name = ask_for_item_name();
            }

            // Ask the user for the quantity, and ask for it if it wasn't specified on the command line
            if quantity.is_empty() {
                quantity = ask_for_quantity();
            }

            // If needed, ask the user for the quantity units
            if quantity_units.is_empty() {
                quantity_units = ask_for_quantity_units();
            }

            // If necessary, ask the user for the item notes and ask for them if they weren't specified on the command line
            if notes.is_empty() {
                notes = ask_for_item_notes();
            }

            let output = sliderule::insert_item(
                &get_cwd(),
                list,
                item_name,
                description,
                quantity,
                quantity_units,
                notes,
                name.to_string(),
            );

            // If we had an error, we need to display it and exit
            if output.status != 0 {
                print_stdout(&output);
                print_stderr(&output);

                std::process::exit(1);
            }
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

            let output = sliderule::create_component(
                &get_cwd(),
                name.to_string(),
                src_license,
                docs_license,
            );

            // If we had an error, we need to display it and exit
            if output.status != 0 {
                print_stdout(&output);
                print_stderr(&output);

                std::process::exit(1);
            }
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
        }

    // If the name is a dotted string that starts with parts or tools, handle that accordingly in the parts.yaml and tools.yaml files
    // if name.contains(".") {
    //     let prefix_parts: Vec<&str> = name.split(".").collect();
    //     let prefix = prefix_parts[0];

    //     if prefix == "parts" || prefix == "tools" {
    //         // Make sure that we have a description to go along with these
    //         if args.len() < 2 {
    //             println!("You must supply a description when creating a part or a tool.");
    //             std::process::exit(1);
    //         }

    //         desc = sliderule::munge_component_description(&args[1]);
    //         // name = &desc;
    //     }
    // }
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
 * Prompt the user to enter a component description
 */
fn ask_for_description() -> String {
    let mut description = String::new();

    print!("Please enter a description for this component: ");
    io::stdout().flush().ok().expect("Could not flush stdout");
    io::stdin()
        .read_line(&mut description)
        .expect("ERROR: Failed to read the component description from the user.");

    description = description.trim().to_string();

    return description;
}

/*
 * Prompt the user to enter the list that this component belongs to
 */
fn ask_for_list() -> String {
    let mut list = String::new();

    print!("Please enter which list this component belongs to (parts or tools) [parts]: ");
    io::stdout().flush().ok().expect("Could not flush stdout");
    io::stdin()
        .read_line(&mut list)
        .expect("ERROR: Failed to read the component item list from the user.");

    list = list.trim().to_string();

    // Default to 'parts'
    if list.is_empty() {
        list = String::from("parts");
    }

    return list;
}

/*
 * Prompt the user to enter the list item that this component should be attached to as an option
 */
fn ask_for_item_name() -> String {
    let mut item_name = String::new();

    print!("Please enter which list item this component should be attached to as an option: ");
    io::stdout().flush().ok().expect("Could not flush stdout");
    io::stdin()
        .read_line(&mut item_name)
        .expect("ERROR: Failed to read the component item list from the user.");

    item_name = item_name.trim().to_string();

    return item_name;
}

/*
 * Prompt the user for the quantity of the component that are required
 */
fn ask_for_quantity() -> String {
    let mut quantity = String::new();

    print!("Please enter a quantity for this component [1]: ");
    io::stdout().flush().ok().expect("Could not flush stdout");
    io::stdin()
        .read_line(&mut quantity)
        .expect("ERROR: Failed to read the component quantity from the user.");

    // Default to a quantity of 1
    if quantity.is_empty() {
        quantity = String::from("1");
    }

    return quantity;
}

/*
 * Prompts the user for the units the component quantity is in
 */
fn ask_for_quantity_units() -> String {
    let mut quantity_units = String::from("parts");

    print!("Please enter the units for the quantity of this component [part]: ");
    io::stdout().flush().ok().expect("Could not flush stdout");
    io::stdin()
        .read_line(&mut quantity_units)
        .expect("ERROR: Failed to read the component quantity from the user.");

    // Default to a quantity of 1
    if quantity_units.is_empty() {
        quantity_units = String::from("part");
    }

    return quantity_units;
}

/*
 * Prompts the user for notes to attach to the list item
 */
fn ask_for_item_notes() -> String {
    let mut notes = String::new();

    print!("Please enter any notes you would like attached to the list item [blank]: ");
    io::stdout().flush().ok().expect("Could not flush stdout");
    io::stdin()
        .read_line(&mut notes)
        .expect("ERROR: Failed to read the component quantity from the user.");

    if notes.is_empty() {
        notes = String::from("''");
    }

    return notes;
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
