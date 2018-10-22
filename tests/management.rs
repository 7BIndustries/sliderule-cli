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

    impl Drop for Noisy {
        fn drop(&mut self) {
            // Clean up after ourselves
            fs::remove_dir_all(Path::new("/tmp").join("test_top"))
                .expect("ERROR: not able to delete top levelcomponent directory.");
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
    
        assert_eq!(String::from_utf8_lossy(&output.stderr), "ERROR: Please supply an command to sliderule-cli.\n");
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
            .args(&["create", "test_top"])
            .output()
            .expect("failed to execute process");

        assert_eq!(String::from_utf8_lossy(&output.stdout), "Finished setting up component.\n");

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
        file_contains_content(&bom_file, 0, "# Bill of Materials for test_top");
        file_contains_content(&bom_file, 12, "  -component_1");
        file_contains_content(&package_file, 1, "  \"name\": \"test_top\",");
        file_contains_content(&package_file, 4, "  \"dependencies\": {");
        file_contains_content(&readme_file, 0, "# test_top");
        file_contains_content(&readme_file, 1, "New Sliderule DOF component.");
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
        let contents: Vec<&str> = contents.split("\r\n").collect();

        assert_eq!(contents[line], text);
    }
}
