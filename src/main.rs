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

fn main() {
    let args = Args::parse();

    let exit_code = if args.values.contains(&args.input) {
        if !args.quiet {
            eprintln!("The same value with the `{}` input exists.", args.input);
        };
        1
    } else if let Some(sugg) = args.values.suggest_with_dist(&args.input, args.distance) {
        if !args.quiet {
            println!("The `{}` input is similar to `{}`.", args.input, sugg);
        };
        0
    } else {
        if !args.quiet {
            println!("No similar value for the `{}` input was found.", args.input);
        };
        1
    };
    std::process::exit(exit_code);
}
