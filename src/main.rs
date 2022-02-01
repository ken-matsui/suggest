use clap::Parser;
use suggestion::Suggest;

#[derive(Parser, Debug)]
#[clap(version, about, long_about = None)]
struct Args {
    /// Input to check if similar name exists
    input: String,

    /// Values of similar names
    values: Vec<String>,

    /// Levenshtein Distance
    #[clap(short, long)]
    distance: Option<usize>,
}

fn main() {
    let args = Args::parse();

    let exit_code = if args.values.contains(&args.input) {
        eprintln!("The same value with the `{}` input exists.", args.input);
        1
    } else if let Some(sugg) = args.values.suggest_with_dist(&args.input, args.distance) {
        println!("The `{}` input is similar to `{}`.", args.input, sugg);
        0
    } else {
        println!("No similar value for the `{}` input was found.", args.input);
        1
    };
    std::process::exit(exit_code);
}
