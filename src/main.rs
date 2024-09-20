use dentritic_calculus::{parse_dc_source,parse_literal_source, Interpreter};

use clap::Parser;
#[derive(Parser)]
struct Cli{
    source : std::path::PathBuf,
    #[arg(default_value="0")]
    input : String,

    #[clap(long,short,action)]
    compiled : bool,

    #[clap(long,short,action)]
    string : bool,

    #[clap(long,short,action)]
    latex : bool,
}

fn main() {
    let args = Cli::parse();
    let source = std::fs::read_to_string(&args.source)
        .unwrap_or_else(|err|panic!("Failed to open file '{:?}': {}.",args.source,err));

    let compiled = parse_dc_source(&source)
        .unwrap_or_else(|err|panic!("Compilation error: {}",err));

    if args.compiled {
        if args.latex{
            println!("{}",compiled.as_tex());
        } else {
            println!("{}",compiled);
        }
    } else {
        let mut interp = Interpreter::new();

        let input = parse_literal_source(&args.input)
            .unwrap_or_else(|err|panic!("Could not parse input '{}': {}",args.input,err));
        interp.set_xi(input);

        interp.execute(&compiled)
            .unwrap_or_else(|err|panic!("Execution error: {}",err));

        let output = interp.get_xi();
        if args.string{
            println!("{}",output.as_c_str().unwrap_or("(Not a valid string)".to_string()));    
        } else {
            if args.latex{
                println!("{}",output.as_tex());
            } else {
                println!("{}",output);
            }
        }
    }

}
