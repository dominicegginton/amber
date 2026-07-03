use amber_lib::Level;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "amber")]
#[command(about = "Beautiful terminals")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Log {
        message: String,

        #[arg(long, default_value = "info")]
        level: String,

        #[arg(long, default_value_t = false)]
        structured: bool,

        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        fields: Vec<String>,
    },
}

pub fn parse_level(s: &str) -> Level {
    match s.to_lowercase().as_str() {
        "debug" => Level::Debug,
        "info" => Level::Info,
        "warn" => Level::Warn,
        "error" => Level::Error,
        _ => Level::Info,
    }
}

pub fn parse_fields(fields: Vec<String>) -> Vec<(&'static str, &'static str)> {
    let mut result = Vec::new();
    for field in fields {
        if let Some(eq_pos) = field.find('=') {
            let (key, value) = field.split_at(eq_pos);
            let value = &value[1..];
            let key_str: &'static str = Box::leak(key.to_string().into_boxed_str());
            let value_str: &'static str = Box::leak(value.to_string().into_boxed_str());
            result.push((key_str, value_str));
        }
    }
    result
}
