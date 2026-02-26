use clap::Parser as ClapParser;

mod codegen;
mod parser;
mod compiler;

use serde::Serialize;
use serde_json;

#[derive(ClapParser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
  /// Input file
  #[arg(short, long)]
  compile: String,

  /// Output file
  #[arg(short, long, default_value = None)]
  output: Option<String>,
}

// FIXME: Implement proper CLI
fn main() {
  let args = Args::parse();
  // Read file ./test.lxs

  let input = std::fs::read_to_string(&args.compile).expect("File read error");
  let mut parser = parser::Parser::new();
  let syntax_tree = parser.parse_program_from_str(&input).unwrap();
  let mut compiler = compiler::Compiler::new(syntax_tree.clone());
  let program = compiler.compile();
  let generator = codegen::CWBlockScriptGenerator::new();
  let script = generator.generate(program);
  match &args.output {
    Some(output_path) => {
      std::fs::write(output_path, script).expect("Failed to write output file");
    },
    None => {
      println!("{}", script);
    }
  };
}