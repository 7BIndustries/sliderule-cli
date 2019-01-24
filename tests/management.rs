use std::process::Command;

#[cfg(test)]
mod management {
    use std::env;
    use std::fs;
    use std::path::{Path, PathBuf};
    use Command;

    #[test]
    /*
     * Makes sure the user will get the correct response if they run the CLI without any commands
     */
    fn test_calling_with_no_commands() {
        let orig_dir = env::current_dir().unwrap();
        let cmd_path = orig_dir
            .join("target")
            .join(cargo_mode())
            .join("sliderule-cli");

        let output = Command::new(cmd_path)
            .output()
            .expect("failed to execute process");

        assert_eq!(
            String::from_utf8_lossy(&output.stdout).trim(),
            "Please supply a command. Run with -h to see the options."
        );
    }

    #[test]
    /*
     * Makes sure that the correct files and directories are created for a new top level component, and that
     * the files and directories have the appropriate content in them.
     */
    fn test_create_top_level_component_structure() {
        let cmd_path = env::current_dir()
            .unwrap()
            .join("target")
            .join(cargo_mode())
            .join("sliderule-cli");

        let temp_dir = env::temp_dir();

        // Set up our temporary project directory for testing
        let test_dir = set_up(&temp_dir, "toplevel");

        // Verify that the directory was created
        let output = Command::new(cmd_path)
            .args(&[
                "create",
                "-s",
                "TestSourceLicense",
                "-d",
                "TestDocLicense",
                "test_top",
            ])
            .current_dir(&test_dir)
            .output()
            .expect("failed to execute process");

        assert_eq!(
            String::from_utf8_lossy(&output.stdout).trim(),
            "Component creation finished."
        );

        // Verify that the proper directories and files within the top level component were created
        assert!(is_valid_component(
            &test_dir.join("test_top"),
            "test_top",
            "TestSourceLicense",
            "TestDocLicense",
        ));
    }

    #[test]
    /*
     * Tests the ability to download (clone) a component from a repo.
     */
    fn test_download_component() {
        let cmd_path = env::current_dir()
            .unwrap()
            .join("target")
            .join(cargo_mode())
            .join("sliderule-cli");

        let temp_dir = env::temp_dir();

        // Set up our temporary project directory for testing
        let test_dir = set_up(&temp_dir, "toplevel");

        // Try to download the component
        let output = Command::new(cmd_path)
            .args(&["download", "https://github.com/jmwright/blink_firmware.git"])
            .current_dir(&test_dir)
            .output()
            .expect("failed to execute process");

        assert!(String::from_utf8_lossy(&output.stdout).contains("Component download finished."));

        assert!(is_valid_component(
            &test_dir.join("blink_firmware"),
            "blink_firmware",
            "Unlicense",
            "CC0-1.0",
        ));
    }

    #[test]
    /*
     * Tests the addition and removal of a remote component.
     */
    fn test_add_remove_component() {
        let cmd_path = env::current_dir()
            .unwrap()
            .join("target")
            .join(cargo_mode())
            .join("sliderule-cli");

        let temp_dir = env::temp_dir();

        // Set up our temporary project directory for testing
        let test_dir = set_up(&temp_dir, "toplevel");

        // The add command
        let add_output = Command::new(&cmd_path)
            .args(&["add", "https://github.com/jmwright/arduino-sr.git"])
            .current_dir(&test_dir.join("toplevel"))
            .output()
            .expect("failed to execute process");

        assert!(String::from_utf8_lossy(&add_output.stdout).contains("Component add finished."));

        // Make sure that the newly installed remote component is actually there
        assert!(
            &test_dir
                .join("toplevel")
                .join("node_modules")
                .join("arduino-sr")
                .exists(),
            "arduino-sr component directory does not exist."
        );

        // Make sure the component is valid
        assert!(is_valid_component(
            &test_dir
                .join("toplevel")
                .join("node_modules")
                .join("arduino-sr"),
            "arduino-sr",
            "Unlicense",
            "CC0-1.0",
        ));

        // The remove command
        let remove_output = Command::new(&cmd_path)
            .args(&["remove", "-y", "arduino-sr"])
            .current_dir(&test_dir.join("toplevel"))
            .output()
            .expect("failed to execute process");

        assert!(
            String::from_utf8_lossy(&remove_output.stdout).contains("Component remove finished.")
        );

        // Make sure that the newly removed remote component was actually removed
        assert!(
            !&test_dir
                .join("toplevel")
                .join("node_modules")
                .join("arduino-sr")
                .exists(),
            "arduino-sr component directory does not exist."
        );
    }

    #[test]
    /*
     * Tests the removal of a local component.
     */
    fn test_remove_local() {
        let cmd_path = env::current_dir()
            .unwrap()
            .join("target")
            .join(cargo_mode())
            .join("sliderule-cli");

        let temp_dir = env::temp_dir();

        // Set up our temporary project directory for testing
        let test_dir = set_up(&temp_dir, "toplevel");

        // The remove command
        let remove_output = match Command::new(&cmd_path)
            .args(&["remove", "-y", "level1"])
            .current_dir(&test_dir.join("toplevel"))
            .output()
        {
            Ok(out) => out,
            Err(e) => panic!(
                "ERROR: On calling CLI with 'remove local_test' as arguments: {}",
                e
            ),
        };

        // stderr should be empty
        assert!(
            remove_output.stderr.is_empty(),
            "stderr was not empty on local component remove"
        );

        // Check to make sure the CLI thinks it was successful in removing the local component
        assert_eq!(
            String::from_utf8_lossy(&remove_output.stdout).trim(),
            "Component remove finished."
        );

        // Make sure that the local component was actually removed
        assert!(!&test_dir
            .join("toplevel")
            .join("components")
            .join("level1")
            .exists());
    }

    #[test]
    fn test_change_license() {
        let cmd_path = env::current_dir()
            .unwrap()
            .join("target")
            .join(cargo_mode())
            .join("sliderule-cli");

        let temp_dir = env::temp_dir();

        // Set up our temporary project directory for testing
        let test_dir = set_up(&temp_dir, "toplevel");

        let package_file = test_dir.join("toplevel").join("package.json");
        let dot_file = test_dir.join("toplevel").join(".sr");

        assert!(file_contains_content(
            &package_file,
            4,
            "\"license\": \"(Unlicense AND NotASourceLicense AND CC0-1.0 AND CC-BY-4.0 AND NotADocLicense)\",",
        ));

        assert!(file_contains_content(
            &dot_file,
            0,
            "source_license: Unlicense,"
        ));
        assert!(file_contains_content(
            &dot_file,
            1,
            "documentation_license: CC0-1.0"
        ));

        // Change the license and verify
        Command::new(cmd_path)
            .args(&[
                "licenses",
                "change",
                "-s",
                "TestSourceLicense",
                "-d",
                "TestDocLicense",
            ])
            .current_dir(test_dir.join("toplevel"))
            .output()
            .expect("failed to execute process");

        // Make sure the source license was changed
        assert!(file_contains_content(
            &package_file,
            9999,
            "TestSourceLicense",
        ));
        // Make sure the doc license was changed
        assert!(file_contains_content(&package_file, 9999, "TestDocLicense",));
        assert!(file_contains_content(
            &dot_file,
            0,
            "source_license: TestSourceLicense,"
        ));
        assert!(file_contains_content(
            &dot_file,
            1,
            "documentation_license: TestDocLicense"
        ));
    }

    #[test]
    fn test_list_licenses() {
        let cmd_path = env::current_dir()
            .unwrap()
            .join("target")
            .join(cargo_mode())
            .join("sliderule-cli");

        let temp_dir = env::temp_dir();

        // Set up our temporary project directory for testing
        let test_dir = set_up(&temp_dir, "toplevel");

        // Change the license and verify
        let output = Command::new(&cmd_path)
            .args(&["licenses", "list"])
            .current_dir(&test_dir.join("toplevel"))
            .output()
            .expect("failed to execute process");

        assert!(
            String::from_utf8_lossy(&output.stdout)
                .contains("Licenses Specified In This Component:"),
            "License listing not found."
        );
        assert!(String::from_utf8_lossy(&output.stdout).contains("Source License: NotASourceLicense, Documentation License: NotADocLicense"), "The correct licenses (source: NotASourceLicense, doc: NotADocLicense) were not listed.");
    }

    /*
     * Tests pushing component changes to a remote repository.
     */
    // #[test]
    fn test_upload() {
        let cmd_path = env::current_dir()
            .unwrap()
            .join("target")
            .join(cargo_mode())
            .join("sliderule-cli");

        let temp_dir = env::temp_dir();

        // Set up our temporary project directory for testing
        let test_dir = set_up(&temp_dir, "toplevel");

        let demo_dir = test_dir.join("demo");
        let working_dir = demo_dir.join("topcomp");

        // Check to make sure any previous runs got cleaned up
        if demo_dir.exists() {
            panic!(
                "ERROR: Please delete {} and rerun tests.",
                demo_dir.display()
            );
        }
        if working_dir.exists() {
            panic!(
                "ERROR: please delete {} and rerun tests.",
                working_dir.display()
            );
        }

        // Create the demo directory
        fs::create_dir(&demo_dir).expect("Failed to create demo directory.");

        Command::new("git")
            .args(&["init", "--bare"])
            .current_dir(&demo_dir)
            .output()
            .expect("failed to initialize bare git repository in demo directory");

        // Create the remote directory for the topcomp project
        fs::create_dir(&working_dir).expect("Failed to create top component directory.");

        Command::new("git")
            .args(&["init", "--bare"])
            .current_dir(&working_dir)
            .output()
            .expect("failed to initialize bare git repository in demo directory");

        // Start a new git deamon server in the current remote repository
        let mut git_cmd = Command::new("git")
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .args(&[
                "daemon",
                "--reuseaddr",
                "--export-all",
                "--base-path=.",
                "--verbose",
                "--enable=receive-pack",
                ".",
            ])
            .current_dir(demo_dir)
            .spawn()
            .expect("ERROR: Could not launch git daemon.");

        // Verify that the directory was created
        let output = Command::new(&cmd_path)
            .args(&[
                "create",
                "-s",
                "NotASourceLicense",
                "-d",
                "NotADocLicense",
                "topcomp",
            ])
            .current_dir(test_dir)
            .output()
            .expect("failed to execute process");

        assert_eq!(
            String::from_utf8_lossy(&output.stdout).trim(),
            "Component creation finished."
        );

        // Upload the component to our local server
        let output = Command::new(&cmd_path)
            .args(&[
                "upload",
                "-m",
                "Initial commit",
                "-u",
                "git://127.0.0.1/topcomp",
            ])
            .current_dir(working_dir)
            .output()
            .expect("failed to upload component using sliderule-cli");

        git_cmd.kill().expect("ERROR: git daemon wasn't running");

        assert!(
            &output.stderr.is_empty(),
            "upload command stderr is not empty."
        );
        assert!(String::from_utf8_lossy(&output.stdout)
            .contains("git repository initialized for project."));
        assert!(String::from_utf8_lossy(&output.stdout).contains("Component upload finished."));
        assert!(
            !String::from_utf8_lossy(&output.stdout)
                .contains("fatal: unable to connect to 127.0.0.1"),
            "sliderule-cli not able to connect to local instance of git daemon."
        );
    }

    // #[test]
    fn test_refactor() {
        let cmd_path = env::current_dir()
            .unwrap()
            .join("target")
            .join(cargo_mode())
            .join("sliderule-cli");

        let temp_dir = env::temp_dir();

        // Set up our temporary project directory for testing
        let test_dir = set_up(&temp_dir, "toplevel");

        let refactor_dir = test_dir.join("refactor");
        let remote_dir = test_dir.join("refactor").join("remote");

        // Check to make sure any previous runs got cleaned up
        if refactor_dir.exists() {
            panic!(
                "ERROR: Please delete {} and rerun tests.",
                refactor_dir.display()
            );
        }
        if remote_dir.exists() {
            panic!(
                "ERROR: please delete {} and rerun tests.",
                remote_dir.display()
            );
        }

        // Create the refactor directory
        fs::create_dir(test_dir.join("refactor")).expect("Failed to create refactor directory.");

        Command::new("git")
            .args(&["init", "--bare"])
            .current_dir(test_dir.join("refactor"))
            .output()
            .expect("failed to initialize bare git repository in refactor directory");

        // Create the remote component directory
        fs::create_dir(test_dir.join("refactor").join("remote"))
            .expect("Failed to create remote component directory.");

        Command::new("git")
            .args(&["init", "--bare"])
            .current_dir(test_dir.join("refactor").join("remote"))
            .output()
            .expect("failed to initialize bare git repository in refactor directory");

        // Start a new git deamon server in the current remote repository
        let mut git_cmd = Command::new("git")
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .args(&[
                "daemon",
                "--reuseaddr",
                "--export-all",
                "--base-path=.",
                "--verbose",
                "--enable=receive-pack",
                ".",
            ])
            .current_dir(test_dir.join("refactor"))
            .spawn()
            .expect("ERROR: Could not launch git daemon.");

        // Verify that the directory was created
        let output = Command::new(&cmd_path)
            .args(&[
                "create",
                "-s",
                "NotASourceLicense",
                "-d",
                "NotADocLicense",
                "-v",
                "maincomp",
            ])
            .current_dir(&test_dir)
            .output()
            .expect("failed to execute process");

        println!("{}", String::from_utf8_lossy(&output.stdout));
        assert!(
            String::from_utf8_lossy(&output.stdout).contains("Finished setting up component."),
            "maincomp component not successfully created."
        );

        // Create a local component
        let output = Command::new(&cmd_path)
            .args(&[
                "create",
                "-s",
                "NotASourceLicense",
                "-d",
                "NotADocLicense",
                "-v",
                "local",
            ])
            .current_dir(test_dir.join("maincomp"))
            .output()
            .expect("failed to execute process");

        assert!(
            String::from_utf8_lossy(&output.stdout).contains("Finished setting up component."),
            "local component not successfully created."
        );

        // Attempt to refactor the component to the remote
        Command::new(&cmd_path)
            .args(&["refactor", "-u", "git://127.0.0.1/remote", "local"])
            .current_dir(test_dir.join("maincomp"))
            .output()
            .expect("failed to execute process");

        assert!(
            test_dir.join("maincomp").exists(),
            "the temporary maincomp directory does not exist."
        );
        assert!(
            test_dir.join("refactor").join("remote").exists(),
            "the temporary refactor/remote directory does not exist."
        );

        git_cmd.kill().expect("ERROR: git daemon wasn't running");
    }

    /*
     * Sets up a test directory for our use.
     */
    fn set_up(temp_dir: &PathBuf, dir_name: &str) -> PathBuf {
        // let url = format!("git://127.0.0.1/{}", dir_name);
        let url = "https://github.com/jmwright/toplevel.git";

        let uuid_dir = uuid::Uuid::new_v4();
        let test_dir_name = format!("temp_{}", uuid_dir);

        // Create the temporary test directory
        fs::create_dir(temp_dir.join(&test_dir_name))
            .expect("Unable to create temporary directory.");

        match git2::Repository::clone(&url, temp_dir.join(&test_dir_name).join(dir_name)) {
            Ok(repo) => repo,
            Err(e) => panic!("failed to clone: {}", e),
        };

        temp_dir.join(test_dir_name)
    }

    /*
     * Helper function that checks to make sure that given text is present in the files.
     */
    fn file_contains_content(file_path: &Path, line: usize, text: &str) -> bool {
        let contains_content: bool;

        // Read the contents of the file
        let contents =
            fs::read_to_string(file_path).expect("ERROR: Cannot read the contents of the file.");

        // See if the user just wants to make sure the content is somewhere in the file
        if line == 9999 {
            contains_content = contents.contains(text);
        } else {
            // Break the file down into something we can index
            let contents: Vec<&str> = contents.lines().collect();

            // See if the line we are interested in is exactly the content specified
            contains_content = contents[line].trim() == text;
        }

        contains_content
    }

    /*
     * Tests if a directory has the correct contents to be a component.
     */
    fn is_valid_component(
        component_path: &Path,
        component_name: &str,
        source_license: &str,
        doc_license: &str,
    ) -> bool {
        let mut is_valid = true;

        // Make sure the BoM data file exists
        if !component_path.join("bom_data.yaml").exists() {
            is_valid = false;
            println!(
                "The file {:?}/bom_data.yaml does not exist.",
                component_path
            );
        }

        // Make sure the component directory exists
        if !component_path.join("components").exists() {
            is_valid = false;
            println!(
                "The directory {:?}/components does not exist.",
                component_path
            );
        }

        // Make sure the docs directory exists
        if !component_path.join("docs").exists() {
            is_valid = false;
            println!("The directory {:?}/docs does not exist.", component_path);
        }

        // Make sure the package.json file exists
        if !component_path.join("package.json").exists() {
            is_valid = false;
            println!("The file {:?}/package.json does not exist.", component_path);
        }

        // Make sure the README.md file exists
        if !component_path.join("README.md").exists() {
            is_valid = false;
            println!("The file {:?}/README.md does not exist.", component_path);
        }

        // Make sure the source directory exists
        if !component_path.join("source").exists() {
            is_valid = false;
            println!("The directory {:?}/source does not exist.", component_path);
        }

        let bom_file = component_path.join("bom_data.yaml");
        let package_file = component_path.join("package.json");
        let readme_file = component_path.join("README.md");
        let dot_file = component_path.join(".sr");

        // Check the content of the files and directories as appropriate here
        if !file_contains_content(
            &bom_file,
            0,
            &format!("# Bill of Materials Data for {}", component_name),
        ) {
            is_valid = false;
            println!(
                "The bill to materials file in {:?} does not contain the correct header.",
                component_path
            );
        }
        if !file_contains_content(&bom_file, 12, "-component_1") {
            is_valid = false;
            println!("The bill to materials file in {:?} does not contain the '-component_1' entry in the right place.", component_path);
        }
        if !file_contains_content(
            &package_file,
            9999,
            &format!("\"name\": \"{}\",", component_name),
        ) {
            is_valid = false;
            println!("The package.json file in {:?} does not contain the component name entry in the right place.", component_path);
        }
        if !file_contains_content(
            &package_file,
            9999,
            &format!("\"license\": \"({} AND {})\",", source_license, doc_license),
        ) {
            is_valid = false;
            println!("The package.json file in {:?} does not contain the the correct license entry in the right place.", component_path);
        }
        if !file_contains_content(&readme_file, 0, &format!("# {}", component_name)) {
            is_valid = false;
            println!("The README.md file in {:?} does not contain the the correct header entry in the right place.", component_path);
        }
        if !file_contains_content(&readme_file, 1, "New Sliderule component.") {
            is_valid = false;
            println!("The README.md file in {:?} does not contain the the correct Sliderule mention in the right place.", component_path);
        }
        if !file_contains_content(
            &dot_file,
            0,
            &format!("source_license: {},", source_license),
        ) {
            is_valid = false;
            println!(
                "The .sr file in {:?} does not contain the the correct source license in the right place.",
                component_path
            );
        }
        if !file_contains_content(
            &dot_file,
            1,
            &format!("documentation_license: {}", doc_license),
        ) {
            is_valid = false;
            println!("The .sr file in {:?} does not contain the the correct documentation license in the right place.", component_path);
        }

        is_valid
    }

    // Tells us whether cargo was run in debug or release mode
    fn cargo_mode() -> String {
        let mode: String;

        if cfg!(debug_assertions) {
            mode = String::from("debug");
        } else {
            mode = String::from("release");
        }

        mode
    }
}
