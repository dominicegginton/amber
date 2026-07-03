use std::fmt;
use std::io::{self, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Level {
    Debug,
    Info,
    Warn,
    Error,
}

impl Level {
    fn label(&self) -> &'static str {
        match self {
            Level::Debug => "DEBUG",
            Level::Info => "INFO",
            Level::Warn => "WARN",
            Level::Error => "ERROR",
        }
    }
}

impl fmt::Display for Level {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.label())
    }
}

pub trait Logger {
    fn log(&self, level: Level, message: &str, fields: &[(&str, &str)]) -> io::Result<()>;
}

pub struct StdoutLogger;

impl Logger for StdoutLogger {
    fn log(&self, level: Level, message: &str, fields: &[(&str, &str)]) -> io::Result<()> {
        let output = format_log_line(level, message, fields);
        let mut stdout = io::stdout();
        stdout.write_all(output.as_bytes())?;
        stdout.write_all(b"\n")?;
        stdout.flush()?;
        Ok(())
    }
}

fn format_field_value(value: &str) -> String {
    if value.contains(' ') {
        format!("\"{}\"", value.replace('"', "\\\""))
    } else {
        value.to_string()
    }
}

fn format_fields(fields: &[(&str, &str)]) -> String {
    if fields.is_empty() {
        String::new()
    } else {
        let mut output = String::new();
        for (name, value) in fields {
            output.push(' ');
            output.push_str(name);
            output.push('=');
            output.push_str(&format_field_value(value));
        }
        output
    }
}

fn format_log_line(level: Level, message: &str, fields: &[(&str, &str)]) -> String {
    format!("{} {}{}", level, message, format_fields(fields))
}

thread_local! {
    static LOGGER: StdoutLogger = StdoutLogger;
}

/// Log a structured message to stdout.
pub fn log(level: Level, message: &str, fields: &[(&str, &str)]) -> io::Result<()> {
    LOGGER.with(|logger| logger.log(level, message, fields))
}

/// Shortcut for debug-level logging.
pub fn debug(message: &str, fields: &[(&str, &str)]) -> io::Result<()> {
    log(Level::Debug, message, fields)
}

/// Shortcut for info-level logging.
pub fn info(message: &str, fields: &[(&str, &str)]) -> io::Result<()> {
    log(Level::Info, message, fields)
}

/// Shortcut for warn-level logging.
pub fn warn(message: &str, fields: &[(&str, &str)]) -> io::Result<()> {
    log(Level::Warn, message, fields)
}

/// Shortcut for error-level logging.
pub fn error(message: &str, fields: &[(&str, &str)]) -> io::Result<()> {
    log(Level::Error, message, fields)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quoted_fields_are_escaped() {
        let line = format_log_line(Level::Debug, "Test", &[("name", "file name.txt")]);
        assert_eq!(line, "DEBUG Test name=\"file name.txt\"");
    }

    #[test]
    fn no_fields_prints_plain_line() {
        let line = format_log_line(Level::Info, "Hello", &[]);
        assert_eq!(line, "INFO Hello");
    }
}
