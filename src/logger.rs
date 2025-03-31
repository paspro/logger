// -------------------------------------------------------------------------------------------------
//
//  Implementation of a general purpose logger.
//
//  Copyright (c) 2025 by Dr. Panos Asproulis (p.asproulis@icloud.com).
//  All Rights Reserved.
//
// -------------------------------------------------------------------------------------------------

//! Implementation of a general purpose logger.

use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;

use crate::levels::LogLevel;

///
/// This struct is responsible for logging the messages produced
/// by the application which uses this library.
///
#[derive(Debug, Clone)]
pub struct Logger {
    ///
    /// The complete path of the file to use for logging.
    ///
    log_file: String,
    ///
    /// If true then terminate the application when an error
    /// message is logged.
    ///
    terminate_on_error: bool,
}

//
// Implementation of the `Logger` struct.
//
impl Logger {
    ///
    /// Create a new `Logger`.
    ///
    /// - Arguments:
    ///   - `log_file_path`: The complete path of the file to use for logging.
    ///
    /// - Returns:
    ///   - The newly constructed `Logger` object.
    ///
    pub fn new(log_file_path: &str, terminate_on_error: bool) -> Self {
        //
        // Create the logging file and verify the success of the operation.
        //
        let log_file = if log_file_path.is_empty() {
            "default.log".to_string()
        } else {
            log_file_path.to_string()
        };

        match File::create(&log_file) {
            Ok(_) => (),
            Err(error) => {
                panic!("Logger: I cannot create the log file: {:?}", error)
            }
        }
        //
        // Create and return the Logger.
        //
        Self {
            log_file,
            terminate_on_error,
        }
    }

    ///
    /// Log a message.
    ///
    /// - Arguments:
    ///   - `message`: The message to log.
    ///   - `level`: The level of the message.
    ///
    pub fn log(&self, level: LogLevel, message: &str) -> Result<(), std::io::Error> {
        //
        // Error message in case the log file cannot be used.
        //
        const WRITE_ERROR: &str = "Logger: I cannot write to the log file.";
        //
        // Log the message on the screen.
        //
        println!("[{}] {}", level, message);
        //
        // Log the message on the log file.
        //
        let mut log_file = OpenOptions::new()
            .append(true)
            .read(false)
            .create(false)
            .open(&self.log_file)
            .unwrap();

        let msg = format!("[{}] {}\n", level.to_level_string(), message);
        log_file.write_all(msg.as_bytes()).expect(WRITE_ERROR);
        log_file.flush().expect(WRITE_ERROR);
        //
        // If the level is `Error`, then we need to terminate the application.
        //
        if level == LogLevel::Error && self.terminate_on_error {
            panic!("Logger: Application terminated abnormally.");
        }

        Ok(())
    }
}

//
// Implementation of the `Default` trait for `Logger`.
//
impl Default for Logger {
    ///
    /// Create a default `Logger`.
    ///
    /// - Returns:
    ///   - The default `Logger` object.
    ///
    fn default() -> Self {
        Logger::new("default.log", true)
    }
}

// -------------------------------------------------------------------------------------------------
//
// Unit Tests.
//
// -------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use rand::prelude::*;
    use std::fs;
    use std::path::Path;

    //
    // Helper function to generate unique log filenames for tests
    //
    fn get_unique_log_filename() -> String {
        let mut rng = rand::rng();
        format!("test_log_{}.log", rng.random::<u64>())
    }

    #[test]
    fn test_logger_new_creates_file() {
        let log_file = get_unique_log_filename();
        let _logger = Logger::new(&log_file, true);
        assert!(Path::new(&log_file).exists());
        //
        // Clean up.
        //
        fs::remove_file(&log_file).unwrap();
    }

    #[test]
    #[should_panic(expected = "Logger: I cannot create the log file")]
    fn test_logger_new_invalid_path() {
        let invalid_path = "/nonexistent/directory/test.log";
        let _logger = Logger::new(invalid_path, true);
    }

    #[test]
    fn test_logger_log_info_message() {
        let log_file = get_unique_log_filename();
        let logger = Logger::new(&log_file, true);

        logger.log(LogLevel::Info, "Test info message").unwrap();

        let mut file = File::open(&log_file).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        assert!(contents.contains("[INFO] Test info message"));
        //
        // Clean up.
        //
        fs::remove_file(&log_file).unwrap();
    }

    #[test]
    fn test_logger_log_warning_message() {
        let log_file = get_unique_log_filename();
        let logger = Logger::new(&log_file, true);

        logger
            .log(LogLevel::Warning, "Test warning message")
            .unwrap();

        let mut file = File::open(&log_file).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        assert!(contents.contains("[WARNING] Test warning message"));
        //
        // Clean up.
        //
        fs::remove_file(&log_file).unwrap();
    }

    #[test]
    fn test_logger_log_debug_message() {
        let log_file = get_unique_log_filename();
        let logger = Logger::new(&log_file, true);

        logger.log(LogLevel::Debug, "Test debug message").unwrap();

        let mut file = File::open(&log_file).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        assert!(contents.contains("[DEBUG] Test debug message"));
        //
        // Clean up.
        //
        fs::remove_file(&log_file).unwrap();
    }

    #[test]
    fn test_logger_log_error_message() {
        let log_file = get_unique_log_filename();
        let logger = Logger::new(&log_file, false);

        logger.log(LogLevel::Error, "Test error message").unwrap();

        let mut file = File::open(&log_file).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        assert!(contents.contains("[ERROR] Test error message"));
        //
        // Clean up.
        //
        fs::remove_file(&log_file).unwrap();
    }

    #[test]
    fn test_logger_multiple_messages() {
        let log_file = get_unique_log_filename();
        let logger = Logger::new(&log_file, true);

        logger.log(LogLevel::Info, "First message").unwrap();
        logger.log(LogLevel::Warning, "Second message").unwrap();
        logger.log(LogLevel::Debug, "Third message").unwrap();

        let mut file = File::open(&log_file).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        let lines: Vec<&str> = contents.lines().collect();
        assert_eq!(lines.len(), 3);
        assert!(contents.contains("[INFO] First message"));
        assert!(contents.contains("[WARNING] Second message"));
        assert!(contents.contains("[DEBUG] Third message"));
        //
        // Clean up.
        //
        fs::remove_file(&log_file).unwrap();
    }

    #[test]
    fn test_logger_multiline_message() {
        let log_file = get_unique_log_filename();
        let logger = Logger::new(&log_file, true);

        logger.log(LogLevel::Info, "Line 1\nLine 2").unwrap();

        let mut file = File::open(&log_file).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        assert!(contents.contains("[INFO] Line 1\nLine 2"));
        //
        // Clean up.
        //
        fs::remove_file(&log_file).unwrap();
    }

    #[test]
    fn test_logger_special_characters() {
        let log_file = get_unique_log_filename();
        let logger = Logger::new(&log_file, true);

        logger
            .log(LogLevel::Info, "Special chars: !@#$%^&*()")
            .unwrap();

        let mut file = File::open(&log_file).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        assert!(contents.contains("[INFO] Special chars: !@#$%^&*()"));
        //
        // Clean up.
        //
        fs::remove_file(&log_file).unwrap();
    }

    #[test]
    fn test_logger_clone() {
        let log_file = get_unique_log_filename();
        let logger = Logger::new(&log_file, true);
        let logger_clone = logger.clone();

        logger.log(LogLevel::Info, "Original logger").unwrap();
        logger_clone
            .log(LogLevel::Warning, "Cloned logger")
            .unwrap();

        let mut file = File::open(&log_file).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        assert!(contents.contains("[INFO] Original logger"));
        assert!(contents.contains("[WARNING] Cloned logger"));
        //
        // Clean up.
        //
        fs::remove_file(&log_file).unwrap();
    }

    #[test]
    fn test_logger_debug_format() {
        let log_file = get_unique_log_filename();
        let logger = Logger::new(&log_file, true);

        let debug_output = format!("{:?}", logger);
        assert!(debug_output.contains(&log_file));
        //
        // Clean up.
        //
        fs::remove_file(&log_file).unwrap();
    }

    #[test]
    fn test_default_logger() {
        let default_logger = Logger::default();
        assert_eq!(default_logger.log_file, "default.log");
    }
}
