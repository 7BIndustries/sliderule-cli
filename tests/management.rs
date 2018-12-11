use std::fs::File;
use std::io::Read;
use std::process::Command;

#[cfg(test)]
mod management {
    use Command;
    use File;
    use Read;
    use std::env;
    use std::fs;
    use std::path::Path;

    struct Noisy;
    struct Blink;
    struct TestBlank;
    struct TestLocalRemove;
    struct TestChangeLicense;
    struct TestListLicenses;
    struct TestUpload;
    struct TestRefactor;

    impl Drop for Noisy {
        fn drop(&mut self) {
            let temp_dir = env::temp_dir();

            // Clean up after ourselves
            if temp_dir.join("test_top").exists() {
                fs::remove_dir_all(temp_dir.join("test_top"))
                    .expect("ERROR: not able to delete top level component directory.");
            }
        }
    }

    impl Drop for Blink {
        fn drop(&mut self) {
            let temp_dir = env::temp_dir();

            // Clean up after ourselves
            if temp_dir.join("blink_firmware").exists() {
                fs::remove_dir_all(temp_dir.join("blink_firmware"))
                    .expect("ERROR: not able to delete top level component directory.");
            }
        }
    }

    impl Drop for TestBlank {
        fn drop(&mut self) {
            let temp_dir = env::temp_dir();

            // Clean up after ourselves
            if temp_dir.join("test_blank").exists() {
                fs::remove_dir_all(temp_dir.join("test_blank"))
                    .expect("ERROR: not able to delete top level component directory.");
            }
        }
    }

    impl Drop for TestLocalRemove {
        fn drop(&mut self) {
            let temp_dir = env::temp_dir();

            // Clean up after ourselves
            if temp_dir.join("test_local_remove").exists() {
                fs::remove_dir_all(temp_dir.join("test_local_remove"))
                    .expect("ERROR: not able to delete top level component directory.");
            }
        }
    }

    impl Drop for TestChangeLicense {
        fn drop(&mut self) {
            let temp_dir = env::temp_dir();

            // Clean up after ourselves
            if temp_dir.join("test_top_license").exists() {
                fs::remove_dir_all(temp_dir.join("test_top_license"))
                    .expect("ERROR: not able to delete top level component directory.");
            }
        }
    }

    impl Drop for TestListLicenses {
        fn drop(&mut self) {
            let temp_dir = env::temp_dir();

            // Clean up after ourselves
            if temp_dir.join("test_list_licenses").exists() {
                fs::remove_dir_all(temp_dir.join("test_list_licenses"))
                    .expect("ERROR: not able to delete top level component directory.");
            }
        }
    }

    impl Drop for TestUpload {
        fn drop(&mut self) {
            let temp_dir = env::temp_dir();

            let demo_dir = temp_dir.join("demo");
            let working_dir = temp_dir.join("topcomp");

            // Clean up after ourselves
            if demo_dir.exists() {
                fs::remove_dir_all(demo_dir)
                    .expect("ERROR not able to delete demo directory.");
            }
            if working_dir.exists() {
                fs::remove_dir_all(working_dir)
                    .expect("ERROR: not able to delete working directory.");
            }
        }
    }

    impl Drop for TestRefactor {
        fn drop(&mut self) {
            let temp_dir = env::temp_dir();

            let demo_dir = temp_dir.join("refactor");
            let remote_dir = temp_dir.join("refactor").join("remote");
            let working_dir = temp_dir.join("maincomp");

            // Clean up after ourselves
            if demo_dir.exists() {
                fs::remove_dir_all(demo_dir)
                    .expect("ERROR not able to delete demo directory.");
            }
            if remote_dir.exists() {
                fs::remove_dir_all(remote_dir)
                    .expect("ERROR: not able to delete remote directory.");
            }
            if working_dir.exists() {
                fs::remove_dir_all(working_dir)
                    .expect("ERROR: not able to delete remote directory.");
            }
        }
    }

    #[test]
    /*
     * Makes sure the user will get the correct response if they run the CLI without any commands
     */
    fn calling_with_no_commands() {
        let orig_dir = env::current_dir().unwrap();
        let cmd_path = orig_dir.join("target").join("debug").join("sliderule-cli");

        let output = Command::new(cmd_path)
            .output()
            .expect("failed to execute process");
    
        assert!(String::from_utf8_lossy(&output.stderr).contains("ERROR: Please supply an command to sliderule-cli. Run with -h to see the options."), "Running sliderule-cli without any commands or options did not throw an error.");
    }

    #[test]
    /*
     * Makes sure that the correct files and directories are created for a new top level component, and that
     * the files and directories have the appropriate content in them.
     */
    fn test_create_top_level_component_structure() {
        let _my_setup = Noisy;
        let orig_dir = env::current_dir().unwrap();
        let cmd_path = orig_dir.join("target").join("debug").join("sliderule-cli");

        let temp_dir = env::temp_dir();

        // Check to see if the last test left things dirty
        if temp_dir.join("test_top").exists() {
            panic!("ERROR: Please delete the temporary test_top directory before running these tests.");
        }

        // We can put the test directories in tmp without breaking anything or running into permission issues
        env::set_current_dir(&temp_dir)
            .expect("ERROR: Could not change into temporary directory.");

        // Verify that the directory was created
        let output = Command::new(cmd_path)
            .args(&["create", "-s", "NotASourceLicense", "-d", "NotADocLicense", "test_top"])
            .output()
            .expect("failed to execute process");

        assert!(String::from_utf8_lossy(&output.stdout).contains("Finished setting up component."), "The test_top component was not created correctly.");

        // Verify that the proper directories and files within the top level component were created
        is_valid_component(&temp_dir.join("test_top"), "test_top", "NotASourceLicense", "NotADocLicense");

        // Set things back the way they were
        env::set_current_dir(orig_dir)
            .expect("ERROR: Could not change into original directory.");
    }

    #[test]
    /*
     * Tests the ability to download (clone) a component from a repo.
     */
    fn test_download_component() {
        let _my_setup = Blink;
        let orig_dir = env::current_dir().unwrap();
        let cmd_path = orig_dir.join("target").join("debug").join("sliderule-cli");

        let temp_dir = env::temp_dir();

        // Check to see if the last test left things dirty
        if temp_dir.join("blink_firmware").exists() {
            panic!("ERROR: Please delete the temporary blink_firmware directory before running these tests.");
        }

        // We can put the test directories in tmp without breaking anything or running into permission issues
        env::set_current_dir(&temp_dir)
            .expect("ERROR: Could not change into temporary directory.");

        // Try to download the component
        let output = Command::new(cmd_path)
            .args(&["download", "https://github.com/jmwright/blink_firmware.git"])
            .output()
            .expect("failed to execute process");

        assert_eq!(String::from_utf8_lossy(&output.stdout), "Successfully cloned component repository.\n");

        is_valid_component(&temp_dir.join("blink_firmware"), "blink_firmware", "Unlicense", "CC0-1.0");

        // Set things back the way they were
        env::set_current_dir(orig_dir)
            .expect("ERROR: Could not change into original directory.");
    }

    #[test]
    /*
     * Tests the addition and removal of a remove component.
     */
    fn test_add_remove_component() {
        let _my_setup = TestBlank;
        let orig_dir = env::current_dir().unwrap();
        let cmd_path = orig_dir.join("target").join("debug").join("sliderule-cli");

        let temp_dir = env::temp_dir();

        // Check to see if the last test left things dirty
        if temp_dir.join("test_blank").exists() {
            panic!("ERROR: Please delete the temporary test_top directory before running these tests.");
        }

        // We can put the test directories in tmp without breaking anything or running into permission issues
        env::set_current_dir(&temp_dir)
            .expect("Could not change into temporary directory.");

        // Try to download the component
        Command::new(&cmd_path)
            .args(&["create", "test_blank"])
            .output()
            .expect("failed to execute process");

        env::set_current_dir(temp_dir.join("test_blank"))
            .expect("Could not change into the temporary test_blank directory.");

        // The add command
        let add_output = Command::new(&cmd_path)
            .args(&["add", "https://github.com/m30-jrs/blink_firmware.git"])
            .output()
            .expect("failed to execute process");

        assert!(String::from_utf8_lossy(&add_output.stdout).contains("Component installed from remote repository."), "Component blink_firmware was not installed from remote repository.");
        assert!(temp_dir.join("test_blank").join("node_modules").join("blink_firmware").exists(), "blink_firmware directory does not exist.");

        // The remove command
        let remove_output = Command::new(&cmd_path)
            .args(&["remove", "-y", "blink_firmware"])
            .output()
            .expect("failed to execute process");

        assert!(String::from_utf8_lossy(&remove_output.stdout).contains("Component uninstalled using npm."), "blink_firmware was not successfully uninstalled using npm.");

        // Set things back the way they were
        env::set_current_dir(orig_dir)
            .expect("ERROR: Could not change into original directory.");

        // TODO: We can't control when the OS will actually remove the file/directory. Figure this out
        // assert_eq!(Path::new("/tmp").join("test_blank").join("node_modules").join("blink_firmware").exists(), false);
    }

    #[test]
    /*
     * Tests the removal of a local component.
     */
    fn test_remove_local() {
        let _my_setup = TestLocalRemove;
        let orig_dir = env::current_dir().unwrap();
        let cmd_path = orig_dir.join("target").join("debug").join("sliderule-cli");

        let temp_dir = env::temp_dir();

        // Check to see if the last test left things dirty
        if temp_dir.join("test_local_remove").exists() {
            panic!("ERROR: Please delete the temporary test_local_remove directory before running these tests.");
        }

        // We can put the test directories in tmp without breaking anything or running into permission issues
        env::set_current_dir(&temp_dir)
            .expect("ERROR: Could not change into the temporary directory.");

        // Create the parent component so that we can test sub-component removal
        let create_output = match Command::new(&cmd_path)
            .args(&["create", "test_local_remove"])
            .output() {
                Ok(out) => out,
                Err(e) => panic!("ERROR: On calling CLI with 'create test_local_remove' as arguments: {}", e)
            };

        // Let the user know if something went wrong
        if !create_output.stderr.is_empty() {
            panic!("ERROR: {}", String::from_utf8_lossy(&create_output.stderr));
        }

        env::set_current_dir(temp_dir.join("test_local_remove"))
            .expect("ERROR: Could not change into the temporary test_local_remove directory.");

        // Create the local_test component
        let add_output = match Command::new(&cmd_path)
            .args(&["create", "local_test"])
            .output() {
                Ok(out) => out,
                Err(e) => panic!("ERROR: On calling CLI with 'create local_test' as arguments: {}", e)
            };

        // Let the user know if something went wrong
        if !add_output.stderr.is_empty() {
            panic!("ERROR: {:?}", String::from_utf8_lossy(&add_output.stderr));
        }

        assert!(String::from_utf8_lossy(&add_output.stdout).contains("Finished setting up component."), "local_test component not set up successfully.");
        assert!(temp_dir.join("test_local_remove").join("components").join("local_test").exists(), "local_test component directory does not exist.");

         // The remove command
        let remove_output = match Command::new(&cmd_path)
            .args(&["remove", "-y", "local_test"])
            .output() {
                Ok(out) => out,
                Err(e) => panic!("ERROR: On calling CLI with 'remove local_test' as arguments: {}", e)
            };

        // Let the user know if something went wrong
        if !remove_output.stderr.is_empty() {
            panic!("ERROR: {}", String::from_utf8_lossy(&remove_output.stderr));
        }

        assert!(String::from_utf8_lossy(&remove_output.stdout).contains("component removed"), "local_test component not removed successfully.");

        // Set things back the way they were
        env::set_current_dir(orig_dir)
            .expect("ERROR: Could not change into original directory.");

        // TODO: We can't control when the OS will actually remove the file/directory. Figure this out
        // assert_eq!(Path::new("/tmp").join("test_local_remove").join("components").join("local_test").exists(), false);
    }

    #[test]
    fn test_change_license() {
        let _my_setup = TestChangeLicense;
        let orig_dir = env::current_dir().unwrap();
        let cmd_path = orig_dir.join("target").join("debug").join("sliderule-cli");

        let temp_dir = env::temp_dir();

        // Check to see if the last test left things dirty
        if temp_dir.join("test_top_license").exists() {
            panic!("ERROR: Please delete the temporary test_top_license directory before running these tests.");
        }

        // We can put the test directories in tmp without breaking anything or running into permission issues
        env::set_current_dir(&temp_dir)
            .expect("ERROR: Could not change into the temporary directory.");

        // Verify that the directory was created
        let output = Command::new(&cmd_path)
            .args(&["create", "-s", "NotASourceLicense", "-d", "NotADocLicense", "test_top_license"])
            .output()
            .expect("failed to execute process");

        let package_file = temp_dir.join("test_top_license").join("package.json");
        let dot_file = temp_dir.join("test_top_license").join(".sr");

        assert!(String::from_utf8_lossy(&output.stdout).contains("Finished setting up component."), "test_top_license not created successfully.");

        file_contains_content(&package_file, 4, "\"license\": \"(NotASourceLicense AND NotADocLicense)\",");
        file_contains_content(&dot_file, 0, "source_license: NotASourceLicense,");
        file_contains_content(&dot_file, 1, "documentation_license: NotADocLicense");

        env::set_current_dir(temp_dir.join("test_top_license"))
            .expect("ERROR: Could not change into the temporary test_top_license directory.");

        // Change the license and verify
        Command::new(cmd_path)
            .args(&["licenses", "change", "-s", "Unlicense", "-d", "CC0-1.0"])
            .output()
            .expect("failed to execute process");

        let package_file = temp_dir.join("test_top_license").join("package.json");
        let dot_file = temp_dir.join("test_top_license").join(".sr");

        file_contains_content(&package_file, 4, "\"license\": \"(Unlicense AND CC0-1.0)\",");
        file_contains_content(&dot_file, 0, "source_license: Unlicense,");
        file_contains_content(&dot_file, 1, "documentation_license: CC0-1.0");

        // Set things back the way they were
        env::set_current_dir(orig_dir)
            .expect("ERROR: Could not change into original directory.");
    }

    #[test]
    fn test_list_licenses() {
        let _my_setup = TestListLicenses;
        let orig_dir = env::current_dir().unwrap();
        let cmd_path = orig_dir.join("target").join("debug").join("sliderule-cli");

        let temp_dir = env::temp_dir();

        // Check to see if the last test left things dirty
        if temp_dir.join("test_list_licenses").exists() {
            panic!("ERROR: Please delete the temporary test_list_licenses directory before running these tests.");
        }

        // We can put the test directories in tmp without breaking anything or running into permission issues
        env::set_current_dir(&temp_dir)
            .expect("ERROR: Could not change into the temporary directory.");

        // Verify that the directory was created
        let output = Command::new(&cmd_path)
            .args(&["create", "-s", "NotASourceLicense", "-d", "NotADocLicense", "test_list_licenses"])
            .output()
            .expect("failed to execute process");

        assert!(String::from_utf8_lossy(&output.stdout).contains("Finished setting up component."), "test_list_licenses component not successfully set up.");

        env::set_current_dir(temp_dir.join("test_list_licenses"))
            .expect("ERROR: Could not change into the temporary test_list_licenses directory.");

        // Change the license and verify
        let output = Command::new(&cmd_path)
            .args(&["licenses", "list"])
            .output()
            .expect("failed to execute process");

        // Set things back the way they were
        env::set_current_dir(orig_dir)
            .expect("ERROR: Could not change into original directory.");

        assert!(String::from_utf8_lossy(&output.stdout).contains("Licenses Specified In This Component:"), "License listing not found.");
        assert!(String::from_utf8_lossy(&output.stdout).contains("Source License: NotASourceLicense, Documentation License: NotADocLicense"), "The correct licenses (source: NotASourceLicense, doc: NotADocLicense) were not listed.");
    }

    #[test]
    /*
     * Tests pushing component changes to a remote repository.
     */
    fn test_upload() {
        let _my_setup = TestUpload;
        let orig_dir = env::current_dir().unwrap();
        let cmd_path = orig_dir.join("target").join("debug").join("sliderule-cli");

        let temp_dir = env::temp_dir();

        let demo_dir = temp_dir.join("demo");
        let working_dir = temp_dir.join("topcomp");

        // Check to make sure any previous runs got cleaned up
        if demo_dir.exists() {
            panic!("ERROR: Please delete {} and rerun tests.", demo_dir.display());
        }
        if working_dir.exists() {
            panic!("ERROR: please delete {} and rerun tests.", working_dir.display());
        }

        // We can put the test directories in tmp without breaking anything or running into permission issues
        env::set_current_dir(&temp_dir)
            .expect("ERROR: Could not change into the temporary directory.");

        // Create the demo directory
        fs::create_dir("demo")
            .expect("Failed to create demo directory.");

        // Change into the demo directory and create a bare git repo
        env::set_current_dir(temp_dir.join("demo"))
            .expect("ERROR: Could not change into the temporary demo directory.");

        Command::new("git")
            .args(&["init", "--bare"])
            .output()
            .expect("failed to initialize bare git repository in demo directory");

        // Create the remote directory for the topcomp project
        fs::create_dir("topcomp")
            .expect("Failed to create top component directory.");

        // Change into the topcomp directory and create a bare git repo
        env::set_current_dir(temp_dir.join("demo").join("topcomp"))
            .expect("ERROR: Could not change into the temporary demo/topcomp directory");

        Command::new("git")
            .args(&["init", "--bare"])
            .output()
            .expect("failed to initialize bare git repository in demo directory");

        // Go back to the demo directory
        env::set_current_dir(temp_dir.join("demo"))
            .expect("ERROR: Could not change into the temporary demo directory.");

        // Start a new git deamon server in the current remote repository
        let mut git_cmd = Command::new("git")
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .args(&["daemon", "--reuseaddr", "--export-all", "--base-path=.", "--verbose", "--enable=receive-pack", "."])
            .spawn()
            .expect("ERROR: Could not launch git daemon.");

        // We can put the test directories in tmp without breaking anything or running into permission issues
        env::set_current_dir(&temp_dir)
            .expect("ERROR: Could not change into the temporary directory.");

        // Verify that the directory was created
        let output = Command::new(&cmd_path)
            .args(&["create", "-s", "NotASourceLicense", "-d", "NotADocLicense", "topcomp"])
            .output()
            .expect("failed to execute process");

        assert!(String::from_utf8_lossy(&output.stdout).contains("Finished setting up component."), "topcomp component not set up successfully.");

        // We can put the test directories in tmp without breaking anything or running into permission issues
        env::set_current_dir(temp_dir.join("topcomp"))
            .expect("ERROR: Could not change into the temporary topcomp directory.");

        // Upload the component to our local server
        let output = Command::new(&cmd_path)
            .args(&["upload", "-m", "Initial commit", "-u", "git://127.0.0.1/topcomp"])
            .output()
            .expect("failed to upload component using sliderule-cli");

        git_cmd.kill().expect("ERROR: git daemon wasn't running");

        // Set things back the way they were
        env::set_current_dir(orig_dir)
            .expect("ERROR: Could not change into original directory.");

        assert!(&output.stderr.is_empty(), "upload command stderr is not empty.");
        assert!(String::from_utf8_lossy(&output.stdout).contains("Done uploading component."), "topcomp component not uploaded successfully.");
        assert!(!String::from_utf8_lossy(&output.stdout).contains("fatal: unable to connect to 127.0.0.1"), "sliderule-cli not able to connect to local instance of git daemon.");
    }

    #[test]
    fn test_refactor() {
        let _my_setup = TestRefactor;
        let orig_dir = env::current_dir().unwrap();
        let cmd_path = orig_dir.join("target").join("debug").join("sliderule-cli");

        let temp_dir = env::temp_dir();

        let refactor_dir = temp_dir.join("refactor");
        let remote_dir = temp_dir.join("refactor").join("remote");

        // Check to make sure any previous runs got cleaned up
        if refactor_dir.exists() {
            panic!("ERROR: Please delete {} and rerun tests.", refactor_dir.display());
        }
        if remote_dir.exists() {
            panic!("ERROR: please delete {} and rerun tests.", remote_dir.display());
        }

        // We can put the test directories in tmp without breaking anything or running into permission issues
        env::set_current_dir(&temp_dir)
            .expect("ERROR: Could not change into the temporary directory.");

        // Create the refactor directory
        fs::create_dir("refactor")
            .expect("Failed to create refactor directory.");

        // Change into the refactor directory and create a bare git repo
        env::set_current_dir(temp_dir.join("refactor"))
            .expect("ERROR: Could not change into the temporary refactor directory.");

        Command::new("git")
            .args(&["init", "--bare"])
            .output()
            .expect("failed to initialize bare git repository in refactor directory");

        // Create the remote component directory
        fs::create_dir("remote")
            .expect("Failed to create remote component directory.");

        // Change into the remote directory and create a bare git repo
        env::set_current_dir(temp_dir.join("refactor").join("remote"))
            .expect("ERROR: Could not change into the temporary refactor/remote directory.");

        Command::new("git")
            .args(&["init", "--bare"])
            .output()
            .expect("failed to initialize bare git repository in refactor directory");

        // Go back to the refactor directory
        env::set_current_dir(temp_dir.join("refactor"))
            .expect("ERROR: Could not change into the temporary refactor directory.");

        // Start a new git deamon server in the current remote repository
        let mut git_cmd = Command::new("git")
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .args(&["daemon", "--reuseaddr", "--export-all", "--base-path=.", "--verbose", "--enable=receive-pack", "."])
            .spawn()
            .expect("ERROR: Could not launch git daemon.");

        // We can put the test directories in tmp without breaking anything or running into permission issues
        env::set_current_dir(&temp_dir)
            .expect("ERROR: Could not change into the temporary directory: {}");

        // Verify that the directory was created
        let output = Command::new(&cmd_path)
            .args(&["create", "-s", "NotASourceLicense", "-d", "NotADocLicense", "maincomp"])
            .output()
            .expect("failed to execute process");

        assert!(String::from_utf8_lossy(&output.stdout).contains("Finished setting up component."), "maincomp component not successfully created.");

        // We can put the test directories in tmp without breaking anything or running into permission issues
        env::set_current_dir(temp_dir.join("maincomp"))
            .expect("ERROR: Could not change into the temporary refactor/maincomp directory.");

        // Create a local component
        let output = Command::new(&cmd_path)
            .args(&["create", "-s", "NotASourceLicense", "-d", "NotADocLicense", "local"])
            .output()
            .expect("failed to execute process");

        assert!(String::from_utf8_lossy(&output.stdout).contains("Finished setting up component."), "local component not successfully created.");

        // Attempt to refactor the component to the remote
        Command::new(&cmd_path)
            .args(&["refactor", "-u", "git://127.0.0.1/remote", "local"])
            .output()
            .expect("failed to execute process");

        // Set things back the way they were
        env::set_current_dir(orig_dir)
            .expect("ERROR: Could not change into original directory.");

        assert!(temp_dir.join("maincomp").exists(), "the temporary maincomp directory does not exist.");
        assert!(temp_dir.join("refactor").join("remote").exists(), "the temporary refactor/remote directory does not exist.");

        git_cmd.kill().expect("ERROR: git daemon wasn't running");
    }

    /*
     * Helper function that checks to make sure that given text is present in the files.
     */
    fn file_contains_content(file_path: &Path, line: usize, text: &str) {
        let mut file = File::open(file_path)
            .expect("ERROR: Cannot open file to check its contents.");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Unable to read the file");
        let contents: Vec<&str> = contents.split("\n").collect();

        assert_eq!(contents[line].trim(), text);
    }

    /*
     * Tests if a directory has the correct contents to be a component.
     */
    fn is_valid_component(component_path: &Path, component_name: &str, source_license: &str, doc_license: &str) {
        assert!(component_path.join("bom_data.yaml").exists(), "The file {}/bom_data.yaml does not exist.", component_path.display());
        assert!(component_path.join("components").exists(), "The directory {}/components does not exist.", component_path.display());
        // assert!(component_path.join("dist").exists(), "The directory {}/dist does not exist.",  component_path.display());
        assert!(component_path.join("docs").exists(), "The directory {}/docs does not exist.",  component_path.display());
        assert!(component_path.join("package.json").exists(), "The file {}/package.json does not exist.",  component_path.display());
        assert!(component_path.join("README.md").exists(), "The file {}/README.md does not exist.",  component_path.display());
        assert!(component_path.join("source").exists(), "The directory {}/source does not exist.",  component_path.display());

        let bom_file = component_path.join("bom_data.yaml");
        let package_file = component_path.join("package.json");
        let readme_file = component_path.join("README.md");
        let dot_file = component_path.join(".sr");

        // Check the content of the files and directories as appropriate here
        file_contains_content(&bom_file, 0, &format!("# Bill of Materials Data for {}", component_name));
        file_contains_content(&bom_file, 12, "-component_1");
        file_contains_content(&package_file, 1, &format!("\"name\": \"{}\",", component_name));
        file_contains_content(&package_file, 4, &format!("\"license\": \"({} AND {})\",", source_license, doc_license));
        file_contains_content(&readme_file, 0, &format!("# {}", component_name));
        file_contains_content(&readme_file, 1, "New Sliderule component.");
        file_contains_content(&dot_file, 0, &format!("source_license: {},", source_license));
        file_contains_content(&dot_file, 1, &format!("documentation_license: {}", doc_license));
    }
}
