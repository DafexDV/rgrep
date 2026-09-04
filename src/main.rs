pub(crate) mod cli;
mod regex;
pub(crate) mod search;

fn main() {
    cli::init_cli();
}
