use std::process::Command;

#[cfg(test)]
mod management {
    use Command;

    #[test]
    fn calling_with_no_commands() {
        let output = Command::new("./target/debug/sliderule-cli")
            .output()
            .expect("failed to execute process");
    
        assert_eq!(String::from_utf8_lossy(&output.stderr), "ERROR: Please supply an command to sliderule-cli.\n");
    }
}