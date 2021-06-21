use clap::Clap;
use commands::Opts;
use simplelog::{Config, LevelFilter, TermLogger, TerminalMode};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let opts = Opts::parse();

    let log_level = if opts.debug {
        LevelFilter::Debug
    } else if opts.quiet {
        LevelFilter::Error
    } else {
        LevelFilter::Info
    };

    TermLogger::init(
        log_level,
        Config::default(),
        TerminalMode::Mixed,
        simplelog::ColorChoice::Always,
    )?;

    Ok(())
}

mod commands {
    use clap::Clap;
    #[derive(Clap)]
    pub struct Opts {
        /// Supresses normal output
        #[clap(short, long)]
        pub quiet: bool,

        /// Prints out debug messages
        #[clap(short, long)]
        pub debug: bool,

        #[clap(subcommand)]
        pub subcmd: SubCommand,
    }

    #[derive(Clap)]
    pub enum SubCommand {
        Connect,
    }
}
