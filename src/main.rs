use std::path::Path;

use clap::Parser;
use owo_colors::OwoColorize;

use crate::{
    cli::Cli,
    search::{PatternSearchOptions, PatternSearchResult, search_patterns},
};

pub(crate) mod cli;
mod regex;
pub(crate) mod search;

struct ShowOptions {
    count: bool,
}

/// Prints on the output the results of the line search
///
/// It formats the result according to the options
fn show_results(results: &Vec<PatternSearchResult>, options: ShowOptions) {
    if options.count {
        let count = results.iter().map(|i| i.matches.len()).len();
        println!("{}", count);
        return;
    }

    for result in results {
        let file_path = &result.file_path;
        println!("{:?}", file_path.cyan());

        for m in &result.matches {
            let before = &m.line_content[..m.start];
            let matched = &m.line_content[m.start..m.end];
            let after = &m.line_content[m.end..];

            print!("\t");
            print!("{}", format!("{}.", m.line_number).yellow());
            print!("\t");
            print!("{}", before.bright_black());
            print!("{}", matched.green());
            print!("{}", after.bright_black());
            print!("\n");
        }
    }
}

fn main() {
    let cli = Cli::parse();

    let file = &cli.file;
    let pattern = &cli.pattern;

    let path = Path::new(file);

    let search_options = PatternSearchOptions {
        ignore_case: cli.ignore_case,
        invert_match: cli.invert_match,
        whole_word: cli.whole_word,
    };

    let show_options = ShowOptions { count: cli.count };

    match search_patterns(path, pattern, search_options) {
        Ok(results) => {
            show_results(&results, show_options);
        }
        Err(err) => eprintln!("Error: {}", err),
    }
}
