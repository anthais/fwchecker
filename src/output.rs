use crate::checker::CheckResult;
use colored::*;

#[derive(Debug, PartialEq)]
pub enum OutputFormat {
    Pretty,
    Csv,
    Tsv,
    Table,
}

pub struct ResultPrinter {
    format: OutputFormat,
}

impl ResultPrinter {
    pub fn new(format: OutputFormat) -> Self {
        Self { format }
    }

    pub fn print(&self, results: &[CheckResult]) {
        match self.format {
            OutputFormat::Pretty => self.print_pretty(results),
            OutputFormat::Csv => self.print_csv(results),
            OutputFormat::Tsv => self.print_tsv(results),
            OutputFormat::Table => self.print_table(results),
        }
    }

    fn print_pretty(&self, results: &[CheckResult]) {
        println!("\n{}", "============================================================".blue().bold());
        println!("{}", "  SUMMARY".blue().bold());
        println!("{}", "============================================================".blue().bold());
        println!();

        for result in results {
            if result.passed {
                println!("{} - {} -> {} ({})", 
                         "✓ PASS".green(),
                         result.source,
                         result.target,
                         result.method);
            } else {
                println!("{} - {} -> {} ({}: {})", 
                         "✗ FAIL".red(),
                         result.source,
                         result.target,
                         result.method,
                         result.detail);
            }
        }

        let total = results.len();
        let passed = results.iter().filter(|r| r.passed).count();
        let failed = total - passed;

        println!();
        println!("{} {} | {} {} | {} {}",
                 "Total:".bold(),
                 total,
                 "Passed:".green(),
                 passed,
                 "Failed:".red(),
                 failed);
        println!("{}", "============================================================".blue().bold());
        println!();
    }

    fn print_csv(&self, results: &[CheckResult]) {
        println!("Source,Target,Method,Detail,Result");
        for result in results {
            println!("{},{},{},\"{}\",{}",
                     result.source,
                     result.target,
                     result.method,
                     result.detail,
                     if result.passed { "PASS" } else { "FAIL" });
        }
    }

    fn print_tsv(&self, results: &[CheckResult]) {
        println!("Source\tTarget\tMethod\tDetail\tResult");
        for result in results {
            println!("{}\t{}\t{}\t{}\t{}",
                     result.source,
                     result.target,
                     result.method,
                     result.detail,
                     if result.passed { "PASS" } else { "FAIL" });
        }
    }

    fn print_table(&self, results: &[CheckResult]) {
        println!("{:<15} {:<30} {:<10} {:<30} {:<8}",
                 "Source", "Target", "Method", "Detail", "Result");
        println!("{:<15} {:<30} {:<10} {:<30} {:<8}",
                 "---------------", "------------------------------", 
                 "----------", "------------------------------", "--------");

        for result in results {
            println!("{:<15} {:<30} {:<10} {:<30} {:<8}",
                     result.source,
                     result.target,
                     result.method,
                     result.detail,
                     if result.passed { "PASS" } else { "FAIL" });
        }

        let total = results.len();
        let passed = results.iter().filter(|r| r.passed).count();
        let failed = total - passed;

        println!();
        println!("Total: {} | Passed: {} | Failed: {}", total, passed, failed);
    }
}

