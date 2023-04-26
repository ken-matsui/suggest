use std::process::exit;

use clap::Parser;
use suggest::Suggest;

#[derive(Parser, Debug)]
#[clap(version, about, long_about = None)]
struct Args {
    /// Input to check if similar name exists
    input: String,

    /// Values of similar names
    values: Vec<String>,

    /// Disable console outputs
    #[clap(short, long)]
    quiet: bool,

    /// Levenshtein Distance
    #[clap(short, long)]
    distance: Option<usize>,
}

const SUCCESS: i32 = 0;
const FAILURE: i32 = 1;

fn main() {
    let args = Args::parse();

    if args.values.contains(&args.input) {
        if !args.quiet {
            eprintln!("The same value with the `{}` input exists.", args.input);
        };
        exit(FAILURE);
    } else if let Some(dist) = args.distance {
        if let Some(sugg) = args.values.suggest_by(&args.input, dist) {
            if !args.quiet {
                println!("The `{}` input is similar to `{}`.", args.input, sugg);
            };
            exit(SUCCESS);
        }
    } else if let Some(sugg) = args.values.suggest(&args.input) {
        if !args.quiet {
            println!("The `{}` input is similar to `{}`.", args.input, sugg);
        };
        exit(SUCCESS);
    };

    if !args.quiet {
        println!("No similar value for the `{}` input was found.", args.input);
    };
    exit(FAILURE);
}
