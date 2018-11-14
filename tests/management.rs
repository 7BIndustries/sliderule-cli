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
                    .expect("ERROR: not able to delete top levelcomponent directory.");
            }
        }
    }

    impl Drop for TestBlank {
        fn drop(&mut self) {
            // Clean up after ourselves
            if Path::new("/tmp").join("test_blank").exists() {
                fs::remove_dir_all(Path::new("/tmp").join("test_blank"))
                    .expect("ERROR: not able to delete top levelcomponent directory.");
            }
        }
    }

    impl Drop for TestLocalRemove {
        fn drop(&mut self) {
            // Clean up after ourselves
            if Path::new("/tmp").join("test_local_remove").exists() {
                fs::remove_dir_all(Path::new("/tmp").join("test_local_remove"))
                    .expect("ERROR: not able to delete top levelcomponent directory.");
            }
        }
    }

    #[test]
    /*
     * Makes sure the user will get the correct response if they run the CLI without any commands
     */
    fn calling_with_no_commands() {
        let output = Command::new("./target/debug/sliderule-cli")
            .output()
            .expect("failed to execute process");
    
        assert_eq!(String::from_utf8_lossy(&output.stderr).contains("ERROR: Please supply an command to sliderule-cli. Run with -h to see the options."), true);
    }

    #[test]
    /*
     * Makes sure that the correct files and directories are created for a new top level component, and that
     * the files and directories have the appropriate content in them.
     */
    fn test_create_top_level_component_structure() {
        let _my_setup = Noisy;
        let orig_path = env::current_dir().unwrap().join("target").join("debug").join("sliderule-cli");

        // The test framework doesn't support Windows at this time
        let info = os_info::get();
        if info.os_type() == os_info::Type::Windows {
            eprintln!("ERROR: This testing framework only supports Linux and MacOS at this time.");
            return;
        }

        // Check to see if the last test left things dirty
        if Path::new("/tmp").join("test_top").exists() {
            eprintln!("ERROR: Please delete /tmp/test_top before running these tests.");

            return;
        }

        // We can put the test directories in tmp without breaking anything or running into permission issues
        match env::set_current_dir("/tmp") {
            Ok(dir) => dir,
            Err(e) => {
                eprintln!("ERROR: Could not change into tmp directory: {}", e);
                return;
            }
        };

        // Verify that the directory was created
        let output = Command::new(orig_path)
            .args(&["create", "-s", "NotASourceLicense", "-d", "NotADocLicense", "test_top"])
            .output()
            .expect("failed to execute process");

        assert_eq!(String::from_utf8_lossy(&output.stdout).contains("Finished setting up component."), true);

        // Verify that the proper directories and files within the top level compoent were created
        assert_eq!(Path::new("/tmp").join("test_top").join("bom_data.yaml").exists(), true);
        assert_eq!(Path::new("/tmp").join("test_top").join("components").exists(), true);
        assert_eq!(Path::new("/tmp").join("test_top").join("dist").exists(), true);
        assert_eq!(Path::new("/tmp").join("test_top").join("docs").exists(), true);
        assert_eq!(Path::new("/tmp").join("test_top").join("package.json").exists(), true);
        assert_eq!(Path::new("/tmp").join("test_top").join("README.md").exists(), true);
        assert_eq!(Path::new("/tmp").join("test_top").join("source").exists(), true);

        let bom_file = Path::new("/tmp").join("test_top").join("bom_data.yaml");
        let package_file = Path::new("/tmp").join("test_top").join("package.json");
        let readme_file = Path::new("/tmp").join("test_top").join("README.md");

        // Check the content of the files and directories as appropriate here
        file_contains_content(&bom_file, 0, "# Bill of Materials Data for test_top");
        file_contains_content(&bom_file, 12, "-component_1");
        file_contains_content(&package_file, 1, "\"name\": \"test_top\",");
        file_contains_content(&package_file, 4, "\"license\": \"Unlicense AND CC-BY-4.0\",");
        file_contains_content(&readme_file, 0, "# test_top");
        file_contains_content(&readme_file, 1, "New Sliderule component.");
    }

    #[test]
    /*
     * Tests the ability to download (clone) a component from a repo.
     */
    fn test_download_component() {
        let _my_setup = Blink;
        let orig_path = env::current_dir().unwrap().join("target").join("debug").join("sliderule-cli");

        // The test framework doesn't support Windows at this time
        let info = os_info::get();
        if info.os_type() == os_info::Type::Windows {
            eprintln!("ERROR: This testing framework only supports Linux and MacOS at this time.");
            return;
        }

        // Check to see if the last test left things dirty
        if Path::new("/tmp").join("blink").exists() {
            eprintln!("ERROR: Please delete /tmp/blink before running these tests.");

            return;
        }

        // We can put the test directories in tmp without breaking anything or running into permission issues
        match env::set_current_dir("/tmp") {
            Ok(dir) => dir,
            Err(e) => {
                eprintln!("ERROR: Could not change into tmp directory: {}", e);
                return;
            }
        };

        // Try to download the component
        let output = Command::new(orig_path)
            .args(&["download", "https://github.com/m30-jrs/blink.git"])
            .output()
            .expect("failed to execute process");

        assert_eq!(String::from_utf8_lossy(&output.stdout), "Successfully cloned component repository.\n");

        // Verify that the proper directories and files within the top level compoent were created
        assert_eq!(Path::new("/tmp").join("blink").join("bom_data.yaml").exists(), true);
        assert_eq!(Path::new("/tmp").join("blink").join("components").exists(), true);
        assert_eq!(Path::new("/tmp").join("blink").join("dist").exists(), true);
        assert_eq!(Path::new("/tmp").join("blink").join("docs").exists(), true);
        assert_eq!(Path::new("/tmp").join("blink").join("package.json").exists(), true);
        assert_eq!(Path::new("/tmp").join("blink").join("README.md").exists(), true);
        assert_eq!(Path::new("/tmp").join("blink").join("source").exists(), true);

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
    }

    #[test]
    /*
     * Tests the addition and removal of a remove component.
     */
    fn test_add_remove_component() {
        let _my_setup = TestBlank;
        let orig_path = env::current_dir().unwrap().join("target").join("debug").join("sliderule-cli");

        // The test framework doesn't support Windows at this time
        let info = os_info::get();
        if info.os_type() == os_info::Type::Windows {
            eprintln!("ERROR: This testing framework only supports Linux and MacOS at this time.");
            return;
        }

        // Check to see if the last test left things dirty
        if Path::new("/tmp").join("test_blank").exists() {
            eprintln!("ERROR: Please delete /tmp/test_top before running these tests.");

            return;
        }

        // We can put the test directories in tmp without breaking anything or running into permission issues
        match env::set_current_dir("/tmp") {
            Ok(dir) => dir,
            Err(e) => {
                eprintln!("ERROR: Could not change into tmp directory: {}", e);
                return;
            }
        };

        // Try to download the component
        Command::new(&orig_path)
            .args(&["create", "test_blank"])
            .output()
            .expect("failed to execute process");

        match env::set_current_dir(Path::new("/tmp").join("test_blank")) {
            Ok(dir) => dir,
            Err(e) => {
                eprintln!("ERROR: Could not change into tmp directory: {}", e);
                return;
            }
        };

        // The add command
        let add_output = Command::new(&orig_path)
            .args(&["add", "https://github.com/m30-jrs/blink_firmware.git"])
            .output()
            .expect("failed to execute process");

        assert_eq!(String::from_utf8_lossy(&add_output.stdout).split("\n").collect::<Vec<&str>>()[1] , "Component installed from remote repository.");
        assert_eq!(Path::new("/tmp").join("test_blank").join("node_modules").join("blink_firmware").exists(), true);

        // The remove command
        let remove_output = Command::new(&orig_path)
            .args(&["remove", "-y", "blink_firmware"])
            .output()
            .expect("failed to execute process");

        assert_eq!(String::from_utf8_lossy(&remove_output.stdout).split("\n").collect::<Vec<&str>>()[2] , "Component uninstalled using npm.");

        // TODO: We can't control when the OS will actually remove the file/directory. Figure this out
        // assert_eq!(Path::new("/tmp").join("test_blank").join("node_modules").join("blink_firmware").exists(), false);
    }

    #[test]
    /*
     * Tests the removal of a local component.
     */
    fn test_remove_local() {
        let _my_setup = TestLocalRemove;
        let orig_path = env::current_dir().unwrap().join("target").join("debug").join("sliderule-cli");

        // The test framework doesn't support Windows at this time
        let info = os_info::get();
        if info.os_type() == os_info::Type::Windows {
            panic!("ERROR: This testing framework only supports Linux and MacOS at this time.");
        }

        // Check to see if the last test left things dirty
        if Path::new("/tmp").join("test_local_remove").exists() {
            panic!("ERROR: Please delete /tmp/test_top before running these tests.");
        }

        // We can put the test directories in tmp without breaking anything or running into permission issues
        match env::set_current_dir("/tmp") {
            Ok(dir) => dir,
            Err(e) => {
                panic!("ERROR: Could not change into tmp directory: {}", e);
            }
        };

        // Create the parent component so that we can test subcomponent removal
        let create_output = match Command::new(&orig_path)
            .args(&["create", "test_local_remove"])
            .output() {
                Ok(out) => out,
                Err(e) => panic!("ERROR: On calling CLI with 'create test_local_remove' as arguments: {}", e)
            };

        // Let the user know if something went wrong
        if !create_output.stderr.is_empty() {
            panic!("ERROR: {}", String::from_utf8_lossy(&create_output.stderr));
        }

        match env::set_current_dir(Path::new("/tmp").join("test_local_remove")) {
            Ok(_) => (),
            Err(e) => {
                panic!("ERROR: Could not change into tmp directory: {}", e);
            }
        };

        // The add command
        let add_output = match Command::new(&orig_path)
            .args(&["create", "local_test"])
            .output() {
                Ok(out) => out,
                Err(e) => panic!("ERROR: On calling CLI with 'create local_test' as arguments: {}", e)
            };

        // Let the user know if something went wrong
        if !add_output.stderr.is_empty() {
            panic!("ERROR: {:?}", String::from_utf8_lossy(&add_output.stderr));
        }

        assert_eq!(String::from_utf8_lossy(&add_output.stdout).contains("Finished setting up component."), true);
        assert_eq!(Path::new("/tmp").join("test_local_remove").join("components").join("local_test").exists(), true);

         // The remove command
        let remove_output = match Command::new(&orig_path)
            .args(&["remove", "-y", "local_test"])
            .output() {
                Ok(out) => out,
                Err(e) => panic!("ERROR: On calling CLI with 'remove local_test' as arguments: {}", e)
            };

        // Let the user know if something went wrong
        if !remove_output.stderr.is_empty() {
            panic!("ERROR: {}", String::from_utf8_lossy(&remove_output.stderr));
        }

        assert_eq!(String::from_utf8_lossy(&remove_output.stdout).contains("component removed") , true);

        // TODO: We can't control when the OS will actually remove the file/directory. Figure this out
        // assert_eq!(Path::new("/tmp").join("test_local_remove").join("components").join("local_test").exists(), false);
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
