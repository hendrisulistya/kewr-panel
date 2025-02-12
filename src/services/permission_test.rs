use std::fs::File;
use std::io::Write;
use crate::config::WorkingDirectoryConfig;

pub fn test_directory_permissions(working_dirs: &WorkingDirectoryConfig) -> Result<(), String> {
    // Ensure directories exist
    working_dirs.ensure_directories_exist()
        .map_err(|e| format!("Failed to create directories: {}", e))?;

    // Test www directory
    test_directory_write(&working_dirs.www_dir, "www_test.txt")
        .map_err(|e| format!("WWW directory test failed: {}", e))?;

    // Test logs directory
    test_directory_write(&working_dirs.logs_dir, "log_test.txt")
        .map_err(|e| format!("Logs directory test failed: {}", e))?;

    // Test config directory
    test_directory_write(&working_dirs.config_dir, "config_test.txt")
        .map_err(|e| format!("Config directory test failed: {}", e))?;

    // Test run directory
    test_directory_write(&working_dirs.run_dir, "run_test.txt")
        .map_err(|e| format!("Run directory test failed: {}", e))?;

    Ok(())
}

fn test_directory_write(dir: &std::path::Path, test_file: &str) -> std::io::Result<()> {
    let test_path = dir.join(test_file);
    let mut file = File::create(&test_path)?;
    file.write_all(b"Test content")?;
    std::fs::remove_file(test_path)?;
    Ok(())
}