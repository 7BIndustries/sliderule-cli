extern crate os_info;

use std::fs::File;
use std::io::Read;
use std::process::Command;

#[cfg(test)]
mod management {
    use Command;
    use File;
    use Read;
    use os_info;
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
            // Clean up after ourselves
            if Path::new("/tmp").join("test_top").exists() {
                fs::remove_dir_all(Path::new("/tmp").join("test_top"))
                    .expect("ERROR: not able to delete top level component directory.");
            }
        }
    }

    impl Drop for Blink {
        fn drop(&mut self) {
            // Clean up after ourselves
            if Path::new("/tmp").join("blink").exists() {
                fs::remove_dir_all(Path::new("/tmp").join("blink"))
                    .expect("ERROR: not able to delete top level component directory.");
            }
        }
    }

    impl Drop for TestBlank {
        fn drop(&mut self) {
            // Clean up after ourselves
            if Path::new("/tmp").join("test_blank").exists() {
                fs::remove_dir_all(Path::new("/tmp").join("test_blank"))
                    .expect("ERROR: not able to delete top level component directory.");
            }
        }
    }

    impl Drop for TestLocalRemove {
        fn drop(&mut self) {
            // Clean up after ourselves
            if Path::new("/tmp").join("test_local_remove").exists() {
                fs::remove_dir_all(Path::new("/tmp").join("test_local_remove"))
                    .expect("ERROR: not able to delete top level component directory.");
            }
        }
    }

    impl Drop for TestChangeLicense {
        fn drop(&mut self) {
            // Clean up after ourselves
            if Path::new("/tmp").join("test_top_license").exists() {
                fs::remove_dir_all(Path::new("/tmp").join("test_top_license"))
                    .expect("ERROR: not able to delete top level component directory.");
            }
        }
    }

    impl Drop for TestListLicenses {
        fn drop(&mut self) {
            // Clean up after ourselves
            if Path::new("/tmp").join("test_list_licenses").exists() {
                fs::remove_dir_all(Path::new("/tmp").join("test_list_licenses"))
                    .expect("ERROR: not able to delete top level component directory.");
            }
        }
    }

    impl Drop for TestUpload {
        fn drop(&mut self) {
            let demo_dir = Path::new("/tmp").join("demo");
            let working_dir = Path::new("/tmp").join("topcomp");

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
            let demo_dir = Path::new("/tmp").join("refactor");
            let remote_dir = Path::new("/tmp").join("refactor").join("remote");
            let working_dir = Path::new("/tmp").join("maincomp");

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
    
        assert!(String::from_utf8_lossy(&output.stderr).contains("ERROR: Please supply an command to sliderule-cli. Run with -h to see the options."));
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

        // The test framework doesn't support Windows at this time
        let info = os_info::get();
        if info.os_type() == os_info::Type::Windows {
            panic!("ERROR: This testing framework only supports Linux and MacOS at this time.");
        }

        // Check to see if the last test left things dirty
        if Path::new("/tmp").join("test_top").exists() {
            panic!("ERROR: Please delete /tmp/test_top before running these tests.");
        }

        // We can put the test directories in tmp without breaking anything or running into permission issues
        env::set_current_dir("/tmp")
            .expect("ERROR: Could not change into tmp directory.");

        // Verify that the directory was created
        let output = Command::new(cmd_path)
            .args(&["create", "-s", "NotASourceLicense", "-d", "NotADocLicense", "test_top"])
            .output()
            .expect("failed to execute process");

        assert!(String::from_utf8_lossy(&output.stdout).contains("Finished setting up component."));

        // Verify that the proper directories and files within the top level component were created
        assert!(Path::new("/tmp").join("test_top").join("bom_data.yaml").exists());
        assert!(Path::new("/tmp").join("test_top").join("components").exists());
        assert!(Path::new("/tmp").join("test_top").join("dist").exists());
        assert!(Path::new("/tmp").join("test_top").join("docs").exists());
        assert!(Path::new("/tmp").join("test_top").join("package.json").exists());
        assert!(Path::new("/tmp").join("test_top").join("README.md").exists());
        assert!(Path::new("/tmp").join("test_top").join("source").exists());

        let bom_file = Path::new("/tmp").join("test_top").join("bom_data.yaml");
        let package_file = Path::new("/tmp").join("test_top").join("package.json");
        let readme_file = Path::new("/tmp").join("test_top").join("README.md");
        let dot_file = Path::new("/tmp").join("test_top").join(".sr");

        // Check the content of the files and directories as appropriate here
        file_contains_content(&bom_file, 0, "# Bill of Materials Data for test_top");
        file_contains_content(&bom_file, 12, "-component_1");
        file_contains_content(&package_file, 1, "\"name\": \"test_top\",");
        file_contains_content(&package_file, 4, "\"license\": \"(NotASourceLicense AND NotADocLicense)\",");
        file_contains_content(&readme_file, 0, "# test_top");
        file_contains_content(&readme_file, 1, "New Sliderule component.");
        file_contains_content(&dot_file, 0, "source_license: NotASourceLicense,");
        file_contains_content(&dot_file, 1, "documentation_license: NotADocLicense");

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

        // The test framework doesn't support Windows at this time
        let info = os_info::get();
        if info.os_type() == os_info::Type::Windows {
            panic!("ERROR: This testing framework only supports Linux and MacOS at this time.");
        }

        // Check to see if the last test left things dirty
        if Path::new("/tmp").join("blink").exists() {
            panic!("ERROR: Please delete /tmp/blink before running these tests.");
        }

        // We can put the test directories in tmp without breaking anything or running into permission issues
        env::set_current_dir("/tmp")
            .expect("ERROR: Could not change into tmp directory.");

        // Try to download the component
        let output = Command::new(cmd_path)
            .args(&["download", "https://github.com/m30-jrs/blink.git"])
            .output()
            .expect("failed to execute process");

        assert_eq!(String::from_utf8_lossy(&output.stdout), "Successfully cloned component repository.\n");

        // Verify that the proper directories and files within the top level compoent were created
        assert!(Path::new("/tmp").join("blink").join("bom_data.yaml").exists());
        assert!(Path::new("/tmp").join("blink").join("components").exists());
        assert!(Path::new("/tmp").join("blink").join("dist").exists());
        assert!(Path::new("/tmp").join("blink").join("docs").exists());
        assert!(Path::new("/tmp").join("blink").join("package.json").exists());
        assert!(Path::new("/tmp").join("blink").join("README.md").exists());
        assert!(Path::new("/tmp").join("blink").join("source").exists());

        let bom_file = Path::new("/tmp").join("blink").join("bom_data.yaml");
        let package_file = Path::new("/tmp").join("blink").join("package.json");
        let readme_file = Path::new("/tmp").join("blink").join("README.md");

        // Check the content of the files and directories as appropriate here
        file_contains_content(&bom_file, 0, "# Bill of Materials Data for blink");
        file_contains_content(&bom_file, 12, "options:");
        file_contains_content(&package_file, 1, "\"name\": \"blink\",");
        file_contains_content(&package_file, 4, "\"dependencies\": {");
        file_contains_content(&readme_file, 0, "# blink_firmware");
        file_contains_content(&readme_file, 1, "The Arduino Blink demo as a DOF component");

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

        // The test framework doesn't support Windows at this time
        let info = os_info::get();
        if info.os_type() == os_info::Type::Windows {
            panic!("ERROR: This testing framework only supports Linux and MacOS at this time.");
        }

        // Check to see if the last test left things dirty
        if Path::new("/tmp").join("test_blank").exists() {
            panic!("ERROR: Please delete /tmp/test_top before running these tests.");
        }

        // We can put the test directories in tmp without breaking anything or running into permission issues
        env::set_current_dir("/tmp")
            .expect("Could not change into tmp directory.");

        // Try to download the component
        Command::new(&cmd_path)
            .args(&["create", "test_blank"])
            .output()
            .expect("failed to execute process");

        env::set_current_dir(Path::new("/tmp").join("test_blank"))
            .expect("Could not change into /tmp/test_blank directory.");

        // The add command
        let add_output = Command::new(&cmd_path)
            .args(&["add", "https://github.com/m30-jrs/blink_firmware.git"])
            .output()
            .expect("failed to execute process");

        assert!(String::from_utf8_lossy(&add_output.stdout).contains("Component installed from remote repository."));
        assert!(Path::new("/tmp").join("test_blank").join("node_modules").join("blink_firmware").exists());

        // The remove command
        let remove_output = Command::new(&cmd_path)
            .args(&["remove", "-y", "blink_firmware"])
            .output()
            .expect("failed to execute process");

        assert!(String::from_utf8_lossy(&remove_output.stdout).contains("Component uninstalled using npm."));

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

        // The test framework doesn't support Windows at this time
        let info = os_info::get();
        if info.os_type() == os_info::Type::Windows {
            panic!("ERROR: This testing framework only supports Linux and MacOS at this time.");
        }

        // Check to see if the last test left things dirty
        if Path::new("/tmp").join("test_local_remove").exists() {
            panic!("ERROR: Please delete /tmp/test_local_remove before running these tests.");
        }

        // We can put the test directories in tmp without breaking anything or running into permission issues
        env::set_current_dir("/tmp")
            .expect("ERROR: Could not change into tmp directory.");

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

        env::set_current_dir(Path::new("/tmp").join("test_local_remove"))
            .expect("ERROR: Could not change into /tmp/test_local_remove directory.");

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

        assert!(String::from_utf8_lossy(&add_output.stdout).contains("Finished setting up component."));
        assert!(Path::new("/tmp").join("test_local_remove").join("components").join("local_test").exists());

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

        assert!(String::from_utf8_lossy(&remove_output.stdout).contains("component removed"));

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

        // The test framework doesn't support Windows at this time
        let info = os_info::get();
        if info.os_type() == os_info::Type::Windows {
            panic!("ERROR: This testing framework only supports Linux and MacOS at this time.");
        }

        // Check to see if the last test left things dirty
        if Path::new("/tmp").join("test_top_license").exists() {
            panic!("ERROR: Please delete /tmp/test_top_license before running these tests.");
        }

        // We can put the test directories in tmp without breaking anything or running into permission issues
        env::set_current_dir("/tmp")
            .expect("ERROR: Could not change into tmp directory.");

        // Verify that the directory was created
        let output = Command::new(&cmd_path)
            .args(&["create", "-s", "NotASourceLicense", "-d", "NotADocLicense", "test_top_license"])
            .output()
            .expect("failed to execute process");

        let package_file = Path::new("/tmp").join("test_top_license").join("package.json");
        let dot_file = Path::new("/tmp").join("test_top_license").join(".sr");

        assert!(String::from_utf8_lossy(&output.stdout).contains("Finished setting up component."));

        file_contains_content(&package_file, 4, "\"license\": \"(NotASourceLicense AND NotADocLicense)\",");
        file_contains_content(&dot_file, 0, "source_license: NotASourceLicense,");
        file_contains_content(&dot_file, 1, "documentation_license: NotADocLicense");

        env::set_current_dir(Path::new("/tmp").join("test_top_license"))
            .expect("ERROR: Could not change into /tmp/test_top_license directory.");

        // Change the license and verify
        Command::new(cmd_path)
            .args(&["licenses", "change", "-s", "Unlicense", "-d", "CC0-1.0"])
            .output()
            .expect("failed to execute process");

        let package_file = Path::new("/tmp").join("test_top_license").join("package.json");
        let dot_file = Path::new("/tmp").join("test_top_license").join(".sr");

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

        // The test framework doesn't support Windows at this time
        let info = os_info::get();
        if info.os_type() == os_info::Type::Windows {
            panic!("ERROR: This testing framework only supports Linux and MacOS at this time.");
        }

        // Check to see if the last test left things dirty
        if Path::new("/tmp").join("test_list_licenses").exists() {
            panic!("ERROR: Please delete /tmp/test_list_licenses before running these tests.");
        }

        // We can put the test directories in tmp without breaking anything or running into permission issues
        env::set_current_dir("/tmp")
            .expect("ERROR: Could not change into tmp directory.");

        // Verify that the directory was created
        let output = Command::new(&cmd_path)
            .args(&["create", "-s", "NotASourceLicense", "-d", "NotADocLicense", "test_list_licenses"])
            .output()
            .expect("failed to execute process");

        assert!(String::from_utf8_lossy(&output.stdout).contains("Finished setting up component."));

        env::set_current_dir(Path::new("/tmp").join("test_list_licenses"))
            .expect("ERROR: Could not change into /tmp/test_list_licenses directory.");

        // Change the license and verify
        let output = Command::new(&cmd_path)
            .args(&["licenses", "list"])
            .output()
            .expect("failed to execute process");

        // Set things back the way they were
        env::set_current_dir(orig_dir)
            .expect("ERROR: Could not change into original directory.");

        assert_eq!(String::from_utf8_lossy(&output.stdout), "Licenses Specified In This Component:\nPath: /tmp/test_list_licenses, Source License: NotASourceLicense, Documentation License: NotADocLicense\n");
    }

    #[test]
    /*
     * Tests pushing component changes to a remote repository.
     */
    fn test_upload() {
        let _my_setup = TestUpload;
        let orig_dir = env::current_dir().unwrap();
        let cmd_path = orig_dir.join("target").join("debug").join("sliderule-cli");
        let demo_dir = Path::new("/tmp").join("demo");
        let working_dir = Path::new("/tmp").join("topcomp");

        // The test framework doesn't support Windows at this time
        let info = os_info::get();
        if info.os_type() == os_info::Type::Windows {
            panic!("ERROR: This testing framework only supports Linux and MacOS at this time.");
        }

        // Check to make sure any previous runs got cleaned up
        if demo_dir.exists() {
            panic!("ERROR: Please delete {} and rerun tests.", demo_dir.display());
        }
        if working_dir.exists() {
            panic!("ERROR: please delete {} and rerun tests.", working_dir.display());
        }

        // We can put the test directories in tmp without breaking anything or running into permission issues
        env::set_current_dir("/tmp")
            .expect("ERROR: Could not change into tmp directory.");

        // Create the demo directory
        fs::create_dir("demo")
            .expect("Failed to create demo directory.");

        // Change into the demo directory and create a bare git repo
        env::set_current_dir(Path::new("/tmp").join("demo"))
            .expect("ERROR: Could not change into /tmp/demo directory.");

        Command::new("git")
            .args(&["init", "--bare"])
            .output()
            .expect("failed to initialize bare git repository in demo directory");

        // Create the remote directory for the topcomp project
        fs::create_dir("topcomp")
            .expect("Failed to create top component directory.");

        // Change into the topcomp directory and create a bare git repo
        env::set_current_dir(Path::new("/tmp").join("demo").join("topcomp"))
            .expect("ERROR: Could not change into /tmp/demo/topcomp directory");

        Command::new("git")
            .args(&["init", "--bare"])
            .output()
            .expect("failed to initialize bare git repository in demo directory");

        // Go back to the demo directory
        env::set_current_dir(Path::new("/tmp").join("demo"))
            .expect("ERROR: Could not change into /tmp/demo directory.");

        // Start a new git deamon server in the current remote repository
        let mut git_cmd = Command::new("git")
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .args(&["daemon", "--reuseaddr", "--export-all", "--base-path=.", "--verbose", "--enable=receive-pack", "."])
            .spawn()
            .expect("ERROR: Could not launch git daemon.");

        // We can put the test directories in tmp without breaking anything or running into permission issues
        env::set_current_dir("/tmp")
            .expect("ERROR: Could not change into tmp directory.");

        // Verify that the directory was created
        let output = Command::new(&cmd_path)
            .args(&["create", "-s", "NotASourceLicense", "-d", "NotADocLicense", "topcomp"])
            .output()
            .expect("failed to execute process");

        assert!(String::from_utf8_lossy(&output.stdout).contains("Finished setting up component."));

        // We can put the test directories in tmp without breaking anything or running into permission issues
        env::set_current_dir(Path::new("/tmp").join("topcomp"))
            .expect("ERROR: Could not change into /tmp/topcomp directory.");

        // Upload the component to our local server
        let output = Command::new(&cmd_path)
            .args(&["upload", "-m", "Initial commit", "-u", "git://127.0.0.1/topcomp"])
            .output()
            .expect("failed to upload component using sliderule-cli");

        git_cmd.kill().expect("ERROR: git daemon wasn't running");

        // Set things back the way they were
        env::set_current_dir(orig_dir)
            .expect("ERROR: Could not change into original directory.");

        assert!(&output.stderr.is_empty());
        assert!(String::from_utf8_lossy(&output.stdout).contains("Done uploading component."));
        assert!(!String::from_utf8_lossy(&output.stdout).contains("fatal: unable to connect to 127.0.0.1"));
    }

    #[test]
    fn test_refactor() {
        let _my_setup = TestRefactor;
        let orig_dir = env::current_dir().unwrap();
        let cmd_path = orig_dir.join("target").join("debug").join("sliderule-cli");

        let refactor_dir = Path::new("/tmp").join("refactor");
        let remote_dir = Path::new("/tmp").join("refactor").join("remote");

        // The test framework doesn't support Windows at this time
        let info = os_info::get();
        if info.os_type() == os_info::Type::Windows {
            panic!("ERROR: This testing framework only supports Linux and MacOS at this time.");
        }

        // Check to make sure any previous runs got cleaned up
        if refactor_dir.exists() {
            panic!("ERROR: Please delete {} and rerun tests.", refactor_dir.display());
        }
        if remote_dir.exists() {
            panic!("ERROR: please delete {} and rerun tests.", remote_dir.display());
        }

        // We can put the test directories in tmp without breaking anything or running into permission issues
        env::set_current_dir("/tmp")
            .expect("ERROR: Could not change into tmp directory.");

        // Create the refactor directory
        fs::create_dir("refactor")
            .expect("Failed to create refactor directory.");

        // Change into the refactor directory and create a bare git repo
        env::set_current_dir(Path::new("/tmp").join("refactor"))
            .expect("ERROR: Could not change into /tmp/refactor directory.");

        Command::new("git")
            .args(&["init", "--bare"])
            .output()
            .expect("failed to initialize bare git repository in refactor directory");

        // Create the remote component directory
        fs::create_dir("remote")
            .expect("Failed to create remote component directory.");

        // Change into the remote directory and create a bare git repo
        env::set_current_dir(Path::new("/tmp").join("refactor").join("remote"))
            .expect("ERROR: Could not change into /tmp/refactor/remote directory.");

        Command::new("git")
            .args(&["init", "--bare"])
            .output()
            .expect("failed to initialize bare git repository in refactor directory");

        // Go back to the refactor directory
        env::set_current_dir(Path::new("/tmp").join("refactor"))
            .expect("ERROR: Could not change into /tmp/refactor directory.");

        // Start a new git deamon server in the current remote repository
        let mut git_cmd = Command::new("git")
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .args(&["daemon", "--reuseaddr", "--export-all", "--base-path=.", "--verbose", "--enable=receive-pack", "."])
            .spawn()
            .expect("ERROR: Could not launch git daemon.");

        // We can put the test directories in tmp without breaking anything or running into permission issues
        env::set_current_dir("/tmp")
            .expect("ERROR: Could not change into tmp directory: {}");

        // Verify that the directory was created
        let output = Command::new(&cmd_path)
            .args(&["create", "-s", "NotASourceLicense", "-d", "NotADocLicense", "maincomp"])
            .output()
            .expect("failed to execute process");

        assert!(String::from_utf8_lossy(&output.stdout).contains("Finished setting up component."));

        // We can put the test directories in tmp without breaking anything or running into permission issues
        env::set_current_dir(Path::new("/tmp").join("maincomp"))
            .expect("ERROR: Could not change into /tmp/refactor/maincomp directory.");

        // Create a local component
        let output = Command::new(&cmd_path)
            .args(&["create", "-s", "NotASourceLicense", "-d", "NotADocLicense", "local"])
            .output()
            .expect("failed to execute process");

        assert!(String::from_utf8_lossy(&output.stdout).contains("Finished setting up component."));

        // Attempt to refactor the component to the remote
        Command::new(&cmd_path)
            .args(&["refactor", "-u", "git://127.0.0.1/remote", "local"])
            .output()
            .expect("failed to execute process");

        // Set things back the way they were
        env::set_current_dir(orig_dir)
            .expect("ERROR: Could not change into original directory.");

        assert!(Path::new("/tmp").join("maincomp").exists());
        assert!(Path::new("/tmp").join("refactor").join("remote").exists());

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
}
