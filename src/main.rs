mod codegen;
mod parser;
mod compiler;

use serde_json;

// FIXME: Implement proper CLI
fn main() {
  // Read file ./test.lxs
  let input = std::fs::read_to_string("./src/stdlib.lxs").expect("File read error");
  let mut parser = parser::Parser::new();
  let syntax_tree = parser.parse_program_from_str(&input).unwrap();
  dbg!(syntax_tree.clone());
  let mut compiler = compiler::Compiler::new(syntax_tree.clone());
  let program = compiler.compile();
  dbg!(program.clone());
  let generator = codegen::CWBlockScriptGenerator::new();
  let script = generator.generate_program(program);
  dbg!(script.clone());
}