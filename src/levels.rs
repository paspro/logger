// -------------------------------------------------------------------------------------------------
//
//  Implementation of a general purpose logger.
//
//  Copyright (c) 2025 by Dr. Panos Asproulis (p.asproulis@icloud.com).
//  All Rights Reserved.
//
// -------------------------------------------------------------------------------------------------

//! Implementation of the logging levels.

///
/// Logging levels.
///
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LogLevel {
    ///
    /// Information level.
    ///
    Info,
    ///
    /// Debug level.
    ///
    Debug,
    ///
    /// Warning level.
    ///
    Warning,
    ///
    /// Error level.
    ///
    Error,
}

//
// Implementation of the `LogLevel` enum.
//
impl LogLevel {
    ///
    /// Convert the `LogLevel` to a string representation.
    ///
    /// - Returns:
    ///   - The string representation of the `LogLevel`.
    ///
    pub fn to_level_string(&self) -> String {
        match self {
            LogLevel::Info => "INFO".to_string(),
            LogLevel::Debug => "DEBUG".to_string(),
            LogLevel::Warning => "WARNING".to_string(),
            LogLevel::Error => "ERROR".to_string(),
        }
    }
}

//
// Implementation of the `std::fmt::Display` trait for `LogLevel`.
//
impl std::fmt::Display for LogLevel {
    ///
    /// Format the `LogLevel` as a string.
    ///
    /// - Arguments:
    ///  - `f`: The formatter to use for formatting.
    ///
    /// - Returns:
    ///   - A result indicating success or failure.
    ///
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_level_string())
    }
}
