use clap::Parser;
use colored::*;
use std::process;

mod config;
mod checker;
mod executor;
mod output;

use config::Config;
use checker::CheckRunner;
use output::{OutputFormat, ResultPrinter};

#[derive(Parser, Debug)]
#[command(name = "fwchecker")]
#[command(about = "Firewall Checker Tool v2.0 - Check firewall connectivity", long_about = None)]
struct Args {
    /// Path to config file
    #[arg(short, long, value_name = "FILE")]
    config: String,

    /// Output format: pretty, csv, tsv, table
    #[arg(short, long, default_value = "pretty")]
    format: String,

    /// Enable verbose mode
    #[arg(short, long)]
    verbose: bool,

    /// Maximum number of concurrent tasks (overrides config file value, default: 1)
    #[arg(long)]
    max_concurrent: Option<usize>,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    // Parse output format
    let format = match args.format.as_str() {
        "pretty" => OutputFormat::Pretty,
        "csv" => OutputFormat::Csv,
        "tsv" => OutputFormat::Tsv,
        "table" => OutputFormat::Table,
        _ => {
            eprintln!("{} Invalid format '{}'. Use: pretty, csv, tsv, or table", 
                     "Error:".red().bold(), args.format);
            process::exit(1);
        }
    };

    // Parse config
    let config = match Config::from_file(&args.config) {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("{} Failed to parse config: {}", "Error:".red().bold(), e);
            process::exit(1);
        }
    };

    if format == OutputFormat::Pretty {
        println!("{} Loaded {} host(s) and {} check section(s) from config\n", 
                 "✓".green(),
                 config.hosts.len(),
                 config.checks.len());
    }

    // Use max_concurrent from CLI if provided, otherwise use config value
    let max_concurrent = args.max_concurrent.unwrap_or(config.max_concurrent);
    
    // Run checks
    let mut runner = CheckRunner::new(config, args.verbose, max_concurrent);
    let results = runner.run_all_checks().await;

    // Print results
    let printer = ResultPrinter::new(format);
    printer.print(&results);

    // Exit with appropriate code
    let failed = results.iter().filter(|r| !r.passed).count();
    if failed > 0 {
        process::exit(1);
    }
}

