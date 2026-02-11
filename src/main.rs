mod codegen;
mod parser;
mod compiler;

fn main() {
  // Read file ./test.lxs
  let input = std::fs::read_to_string("./src/stdlib.lxs").expect("File read error");
  let syntax_tree = parser::LynxScriptParser.parse_program_from_str(&input).unwrap();
  let mut compiler = compiler::LynxScriptCompiler::new(syntax_tree.clone());
  dbg!(compiler.compile());
}