use regex::Regex;
use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
    io::{self},
    path::Path,
    sync::{LazyLock, Mutex},
};

use clap::Parser;

/// Main struct for clap to put the cli argument values
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct GrepCli {
    // Searches for the pattern regardless of uppercase or lowercase.
    #[arg(short = 'i', help = "Search ignoring uppercase and lowercase")]
    ignore_case: bool,

    // Displays only the lines that do not contain the pattern.
    #[arg(short = 'v', help = "Show lines that do not match")]
    invert_match: bool,

    // Searches through all files in a directory and its subdirectories.
    //#[arg(
    //    short = 'r',
    //    help = "Searches through all files in a directory and its subdirectories"
    //)]
    //recursive: bool,

    // Shows the line number in the file where the match was found.
    #[arg(
        short = 'n',
        help = "Shows the line number in the file where the match was found"
    )]
    line_numbers: bool,

    // Instead of showing the lines, it just tells you how many matches were found.
    #[arg(
        short = 'c',
        help = "Instead of showing the lines, it just tells you how many matches were found"
    )]
    count: bool,

    // Only matches the exact word, not if it is part of a larger
    #[arg(
        short = 'w',
        help = "Only matches the exact word, not if it is part of a larger"
    )]
    whole_word: bool,

    #[arg(help = "Pattern to search")]
    pattern: String,

    #[arg(help = "The file in which the pattern will be searched")]
    file: String,
}

struct LineSearchOptions {
    ignore_case: bool,
    invert_match: bool,
    whole_word: bool,
}

struct ShowOptions {
    line_numbers: bool,
    count: bool,
    show_filename: bool,
}

#[allow(dead_code)]
struct PatternResult {
    line_number: usize,
    line_content: String,

    start: usize,
    end: usize,
}

#[derive(Clone)]
struct Regexes {
    normal: Regex,
    whole_word: Regex,
    ignore_case: Regex,
    ignore_case_whole_word: Regex,
}

/// Static cache where the regexes for each pattern are stored
static CACHE: LazyLock<Mutex<HashMap<String, Regexes>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

/// It retrieves the regexes from the cache, and if it doesn't have them, it creates them
fn cache_get_or_create(key: &str) -> Regexes {
    CACHE
        .lock()
        .unwrap()
        .entry(key.to_string())
        .or_insert_with(|| {
            let escaped = regex::escape(key);

            let normal = Regex::new(&escaped).unwrap();

            let whole_word = Regex::new(&format!(r"\b{}\b", escaped)).unwrap();

            let ignore_case = Regex::new(&format!(r"(?i){escaped}")).unwrap();

            let ignore_case_whole_word = Regex::new(&format!(r"(?i)\b{}\b", escaped)).unwrap();

            Regexes {
                normal,
                whole_word,
                ignore_case,
                ignore_case_whole_word,
            }
        })
        .clone()
}

/// Look for the pattern in the file lines.
///
/// The search can be customized by passing through options.
/// - options.ignore_case uses the regex specialized in ignoring whether it is in uppercase or lowercase.
/// - options.whole_word uses the regex specialized in checking if the entire line matches the pattern.
/// - options.invert_match Invert the results; what will be returned are the lines where the term was NOT found.
///
/// Return Ok(results) if successful, otherwise return [io::Error](io::Error)
fn search_pattern_in_file(
    filename: &str,
    pattern: &str,
    options: &LineSearchOptions,
) -> Result<Vec<PatternResult>, io::Error> {
    // Obtener o crear patrón regex de la caché
    let regexes = cache_get_or_create(&pattern);
    let regex = match (options.whole_word, options.ignore_case) {
        (true, true) => &regexes.ignore_case_whole_word,
        (true, false) => &regexes.whole_word,
        (false, true) => &regexes.ignore_case,
        (false, false) => &regexes.normal,
    };

    let content = read_to_string(filename)?;

    let mut result: Vec<PatternResult> = Vec::new();

    for (n, line) in content.lines().enumerate() {
        let matches: Vec<_> = regex.find_iter(line).collect();

        if options.invert_match {
            if matches.is_empty() {
                result.push(PatternResult {
                    line_number: n + 1,
                    line_content: line.to_string(),
                    start: 0,
                    end: line.len(),
                });
            }

            continue;
        }

        for m in matches {
            result.push(PatternResult {
                line_number: n + 1,
                line_content: line.to_string(),
                start: m.start(),
                end: m.end(),
            });
        }
    }

    Ok(result)
}

/// Prints on the output the results of the line search
/// 
/// It formats the result according to the options
fn show_results(filename: &str, results: &Vec<PatternResult>, options: &ShowOptions) {
    if options.show_filename {
        println!("{}", filename);
    }

    if options.count {
        let count = results
            .iter()
            .map(|i| i.line_number)
            .collect::<HashSet<_>>()
            .len();

        println!("{}", count);
        return;
    }

    for result in results {
        let message = match options.line_numbers {
            true => format!("{}. {}", result.line_number, result.line_content),
            false => format!("{}", result.line_content),
        };

        if options.show_filename {
            println!("\t{}", message);
        } else {
            println!("{}", message);
        }
    }
}

fn main() {
    let cli = GrepCli::parse();

    let filename = &cli.file;
    let pattern = &cli.pattern;

    let path = Path::new(filename);

    if !path.exists() {
        println!("The file doesn't exist");
        return;
    }

    if path.is_dir() {
        println!("The file is a folder. Recursive search is not currently supported.");
        return;
    }

    let search_options = LineSearchOptions {
        ignore_case: cli.ignore_case,
        invert_match: cli.invert_match,
        whole_word: cli.whole_word,
    };

    let show_options = ShowOptions {
        line_numbers: cli.line_numbers,
        count: cli.count,
        show_filename: false,
    };

    match search_pattern_in_file(filename, pattern, &search_options) {
        Ok(results) => {
            show_results(filename, &results, &show_options);
        }
        Err(err) => {
            eprintln!("Error: {}", err);
        }
    }
}
