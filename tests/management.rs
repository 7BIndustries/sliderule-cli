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
        let orig_dir = env::current_dir().unwrap();
        let orig_path = orig_dir.join("target").join("debug").join("sliderule-cli");

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

        // Set things back the way they were
        match env::set_current_dir(orig_dir) {
            Ok(dir) => dir,
            Err(e) => {
                eprintln!("ERROR: Could not change into tmp directory: {}", e);
                return;
            }
        };

        assert_eq!(String::from_utf8_lossy(&output.stdout).contains("Finished setting up component."), true);

        // Verify that the proper directories and files within the top level component were created
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
    }

    #[test]
    /*
     * Tests the ability to download (clone) a component from a repo.
     */
    fn test_download_component() {
        let _my_setup = Blink;
        let orig_dir = env::current_dir().unwrap();
        let orig_path = orig_dir.join("target").join("debug").join("sliderule-cli");

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

        // Set things back the way they were
        match env::set_current_dir(orig_dir) {
            Ok(dir) => dir,
            Err(e) => {
                eprintln!("ERROR: Could not change into tmp directory: {}", e);
                return;
            }
        };

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
        let orig_dir = env::current_dir().unwrap();
        let orig_path = orig_dir.join("target").join("debug").join("sliderule-cli");

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

        // Set things back the way they were
        match env::set_current_dir(orig_dir) {
            Ok(dir) => dir,
            Err(e) => {
                eprintln!("ERROR: Could not change into tmp directory: {}", e);
                return;
            }
        };

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
        let orig_dir = env::current_dir().unwrap();
        let orig_path = orig_dir.join("target").join("debug").join("sliderule-cli");

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
        match env::set_current_dir("/tmp") {
            Ok(dir) => dir,
            Err(e) => {
                panic!("ERROR: Could not change into tmp directory: {}", e);
            }
        };

        // Create the parent component so that we can test sub-component removal
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

        // Set things back the way they were
        match env::set_current_dir(orig_dir) {
            Ok(dir) => dir,
            Err(e) => {
                eprintln!("ERROR: Could not change into tmp directory: {}", e);
                return;
            }
        };

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

    #[test]
    fn test_change_license() {
        let _my_setup = TestChangeLicense;
        let orig_dir = env::current_dir().unwrap();
        let orig_path = orig_dir.join("target").join("debug").join("sliderule-cli");

        // The test framework doesn't support Windows at this time
        let info = os_info::get();
        if info.os_type() == os_info::Type::Windows {
            eprintln!("ERROR: This testing framework only supports Linux and MacOS at this time.");
            return;
        }

        // Check to see if the last test left things dirty
        if Path::new("/tmp").join("test_top_license").exists() {
            eprintln!("ERROR: Please delete /tmp/test_top_license before running these tests.");

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
        let output = Command::new(&orig_path)
            .args(&["create", "-s", "NotASourceLicense", "-d", "NotADocLicense", "test_top_license"])
            .output()
            .expect("failed to execute process");

        let package_file = Path::new("/tmp").join("test_top_license").join("package.json");
        let dot_file = Path::new("/tmp").join("test_top_license").join(".sr");

        assert_eq!(String::from_utf8_lossy(&output.stdout).contains("Finished setting up component."), true);

        file_contains_content(&package_file, 4, "\"license\": \"(NotASourceLicense AND NotADocLicense)\",");
        file_contains_content(&dot_file, 0, "source_license: NotASourceLicense,");
        file_contains_content(&dot_file, 1, "documentation_license: NotADocLicense");

        match env::set_current_dir("/tmp/test_top_license") {
            Ok(dir) => dir,
            Err(e) => {
                eprintln!("ERROR: Could not change into tmp directory: {}", e);
                return;
            }
        };

        // Change the license and verify
        Command::new(orig_path)
            .args(&["licenses", "change", "-s", "Unlicense", "-d", "CC0-1.0"])
            .output()
            .expect("failed to execute process");

        let package_file = Path::new("/tmp").join("test_top_license").join("package.json");
        let dot_file = Path::new("/tmp").join("test_top_license").join(".sr");

        file_contains_content(&package_file, 4, "\"license\": \"(Unlicense AND CC0-1.0)\",");
        file_contains_content(&dot_file, 0, "source_license: Unlicense,");
        file_contains_content(&dot_file, 1, "documentation_license: CC0-1.0");

        // Set things back the way they were
        match env::set_current_dir(orig_dir) {
            Ok(dir) => dir,
            Err(e) => {
                eprintln!("ERROR: Could not change into original directory: {}", e);
                return;
            }
        };
    }

    #[test]
    fn test_list_licenses() {
        let _my_setup = TestListLicenses;
        let orig_dir = env::current_dir().unwrap();
        let orig_path = orig_dir.join("target").join("debug").join("sliderule-cli");

        // The test framework doesn't support Windows at this time
        let info = os_info::get();
        if info.os_type() == os_info::Type::Windows {
            eprintln!("ERROR: This testing framework only supports Linux and MacOS at this time.");
            return;
        }

        // Check to see if the last test left things dirty
        if Path::new("/tmp").join("test_list_licenses").exists() {
            eprintln!("ERROR: Please delete /tmp/test_list_licenses before running these tests.");

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
        let output = Command::new(&orig_path)
            .args(&["create", "-s", "NotASourceLicense", "-d", "NotADocLicense", "test_list_licenses"])
            .output()
            .expect("failed to execute process");

        assert_eq!(String::from_utf8_lossy(&output.stdout).contains("Finished setting up component."), true);

        match env::set_current_dir(Path::new("/tmp").join("test_list_licenses")) {
            Ok(dir) => dir,
            Err(e) => {
                eprintln!("ERROR: Could not change into tmp directory: {}", e);
                return;
            }
        };
        println!("{:?}", &orig_path);
        // Change the license and verify
        let output = Command::new(&orig_path)
            .args(&["licenses", "list"])
            .output()
            .expect("failed to execute process");

        // println!("{:?}", output.stderr);
        assert_eq!(String::from_utf8_lossy(&output.stdout), "Licenses Specified In This Component:\nPath: /tmp/test_list_licenses, Source License: NotASourceLicense, Documentation License: NotADocLicense\n");

        // Set things back the way they were
        match env::set_current_dir(orig_dir) {
            Ok(dir) => dir,
            Err(e) => {
                eprintln!("ERROR: Could not change into original directory: {}", e);
                return;
            }
        };
    }

    #[test]
    fn test_upload() {
        let _my_setup = TestUpload;
        let orig_dir = env::current_dir().unwrap();
        let orig_path = orig_dir.join("target").join("debug").join("sliderule-cli");
        let demo_dir = Path::new("/tmp").join("demo");
        let working_dir = Path::new("/tmp").join("topcomp");

        // The test framework doesn't support Windows at this time
        let info = os_info::get();
        if info.os_type() == os_info::Type::Windows {
            eprintln!("ERROR: This testing framework only supports Linux and MacOS at this time.");
            return;
        }

        // Check to make sure any previous runs got cleaned up
        if demo_dir.exists() {
            eprintln!("ERROR: Please delete {} and rerun tests.", demo_dir.display());
        }
        if working_dir.exists() {
            eprintln!("ERROR: please delete {} and rerun tests.", working_dir.display());
        }

        // We can put the test directories in tmp without breaking anything or running into permission issues
        match env::set_current_dir("/tmp") {
            Ok(dir) => dir,
            Err(e) => {
                eprintln!("ERROR: Could not change into tmp directory: {}", e);
                return;
            }
        };

        // Create the demo directory
        Command::new("mkdir")
            .args(&["demo"])
            .output()
            .expect("Failed to create demo directory.");

        // Change into the demo directory and create a bare git repo
        match env::set_current_dir(Path::new("/tmp").join("demo")) {
            Ok(dir) => dir,
            Err(e) => {
                eprintln!("ERROR: Could not change into demo directory: {}", e);
                return;
            }
        };
        Command::new("git")
            .args(&["init", "--bare"])
            .output()
            .expect("failed to initialize bare git repository in demo directory");

        // Create the remote directory for the topcomp project
        Command::new("mkdir")
            .args(&["topcomp"])
            .output()
            .expect("Failed to create demo directory.");

        // Change into the topcomp directory and create a bare git repo
        match env::set_current_dir(Path::new("/tmp").join("demo").join("topcomp")) {
            Ok(dir) => dir,
            Err(e) => {
                eprintln!("ERROR: Could not change into demo directory: {}", e);
                return;
            }
        };
        Command::new("git")
            .args(&["init", "--bare"])
            .output()
            .expect("failed to initialize bare git repository in demo directory");

        // Go back to the demo directory
        match env::set_current_dir(Path::new("/tmp").join("demo")) {
            Ok(dir) => dir,
            Err(e) => {
                eprintln!("ERROR: Could not change into demo directory: {}", e);
                return;
            }
        };

        // Start a new git deamon server in the current remote repository
        let mut git_cmd = Command::new("git")
            .args(&["daemon", "--reuseaddr", "--export-all", "--base-path=.", "--verbose", "--enable=receive-pack", "."])
            .spawn()
            .expect("failed to start git daemon");

        // We can put the test directories in tmp without breaking anything or running into permission issues
        match env::set_current_dir("/tmp") {
            Ok(dir) => dir,
            Err(e) => {
                eprintln!("ERROR: Could not change into tmp directory: {}", e);
                return;
            }
        };

        // Verify that the directory was created
        let output = Command::new(&orig_path)
            .args(&["create", "-s", "NotASourceLicense", "-d", "NotADocLicense", "topcomp"])
            .output()
            .expect("failed to execute process");

        assert_eq!(String::from_utf8_lossy(&output.stdout).contains("Finished setting up component."), true);

        // We can put the test directories in tmp without breaking anything or running into permission issues
        match env::set_current_dir(Path::new("/tmp").join("topcomp")) {
            Ok(dir) => dir,
            Err(e) => {
                eprintln!("ERROR: Could not change into tmp directory: {}", e);
                return;
            }
        };

        // Upload the component to our local server
        let output = Command::new(&orig_path)
            .args(&["upload", "-m", "Initial commit", "-u", "git://127.0.0.1/topcomp"])
            .output()
            .expect("failed to upload component using sliderule-cli");

        // Get rid of the git daemon
        git_cmd.kill().expect("command wasn't running");

        assert!(&output.stderr.is_empty());
        assert!(String::from_utf8_lossy(&output.stdout).contains("Done uploading component."));
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
