mod cli;

use clap::Parser;

fn main() -> std::io::Result<()> {
    let cli = cli::Cli::parse();

    match cli.command {
        cli::Commands::Log {
            message,
            level,
            field,
            ..
        } => {
            let parsed_level = cli::parse_level(&level);
            let parsed_fields = cli::parse_fields_from_pairs(field);
            amber_lib::log(parsed_level, &message, &parsed_fields)?;
        }
    }

    Ok(())
}
