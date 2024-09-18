use dentritic_calculus::{parse_dc_source,parse_literal_source, Interpreter};

use clap::Parser;
#[derive(Parser)]
struct Cli{
    source : std::path::PathBuf,
    #[arg(default_value="0")]
    input : String,
}

fn main() {
    let args = Cli::parse();
    let source = std::fs::read_to_string(&args.source)
        .unwrap_or_else(|err|panic!("Failed to open file '{:?}': {}.",args.source,err));

    let compiled = parse_dc_source(&source)
        .unwrap_or_else(|err|panic!("Compilation error: {}",err));

    let mut interp = Interpreter::new();

    let input = parse_literal_source(&args.input)
        .unwrap_or_else(|err|panic!("Could not parse input '{}': {}",args.input,err));
    interp.set_xi(input);

    interp.execute(&compiled)
        .unwrap_or_else(|err|panic!("Execution error: {:?}",err));

    println!("{}",interp.get_xi());

}
