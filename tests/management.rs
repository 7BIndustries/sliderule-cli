extern crate os_info;

use std::process::Command;

#[cfg(test)]
mod management {
    use Command;
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
    fn test_create_top_level_component() {
        let _my_setup = Noisy;
        let orig_path = env::current_dir().unwrap().join("target").join("debug").join("sliderule-cli");

        // The test framework doesn't support Windows at this time
        let info = os_info::get();
        if info.os_type() == os_info::Type::Windows {
            eprintln!("ERROR: This testing framework only supports Linux and MacOS at this time.");
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

        // TODO: Check the content of the files and directories as appropriate here
    }
}