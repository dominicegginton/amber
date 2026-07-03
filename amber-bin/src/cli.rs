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

        #[arg(long, num_args = 2, action = clap::ArgAction::Append)]
        field: Vec<String>,
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

pub fn parse_fields_from_pairs(field: Vec<String>) -> Vec<(&'static str, &'static str)> {
    let mut result = Vec::new();
    let mut i = 0;
    while i + 1 < field.len() {
        let key_str: &'static str = Box::leak(field[i].clone().into_boxed_str());
        let value_str: &'static str = Box::leak(field[i + 1].clone().into_boxed_str());
        result.push((key_str, value_str));
        i += 2;
    }
    result
}
