use clap::Parser;

/// Main struct for clap to put the cli argument values
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub(crate) struct Cli {
    // Searches for the pattern regardless of uppercase or lowercase.
    #[arg(short = 'i', help = "Search ignoring uppercase and lowercase")]
    pub ignore_case: bool,

    // Displays only the lines that do not contain the pattern.
    #[arg(short = 'v', help = "Show lines that do not match")]
    pub invert_match: bool,

    // Instead of showing the lines, it just tells you how many matches were found.
    #[arg(
        short = 'c',
        help = "Instead of showing the lines, it just tells you how many matches were found"
    )]
    pub count: bool,

    // Only matches the exact word, not if it is part of a larger
    #[arg(
        short = 'w',
        help = "Only matches the exact word, not if it is part of a larger"
    )]
    pub whole_word: bool,

    #[arg(help = "The file in which the pattern will be searched")]
    pub file: String,

    #[arg(help = "Pattern to search")]
    pub pattern: String,
}
