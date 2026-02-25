// TODO: Implement stringification
/// This module defines the lowered data structures.
#[derive(Debug, PartialEq, Clone)]
pub struct Program {
  pub main_block: Vec<Item>,
}

impl Program {
  pub fn new(main_block: Vec<Item>) -> Self {
    Self { main_block }
  }
}

#[derive(Debug, PartialEq, Clone, Default)]
pub struct Expression {
  pub dependencies: Vec<Call>,
  pub content: Option<Argument>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Statement {
  pub dependencies: Vec<Call>,
  pub content: Vec<Call>,
}

#[derive(Debug, PartialEq, Clone, Default)]
pub struct Literal {
  pub value: String,
}

#[derive(Debug, PartialEq, Clone, Default)]
pub struct Variable {
  pub name: String
}

#[derive(Debug, PartialEq, Clone)]
pub struct CWScriptBlockID {
  pub id: String
}

#[derive(Debug, PartialEq, Clone)]
pub enum Identifier {
  Name(Variable),
  CWScriptBlockID(CWScriptBlockID),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Item {
  FunctionDeclaration {
    name: String,
    parameters: Vec<Variable>, // FIXME: Variables only
    body: Vec<Statement>,
  },
  Event {
    name: String,
    body: Vec<Statement>,
  }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Argument {
  RawString(String),
  Literal(Literal),
  Identifier(Variable),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Call {
  FunctionCall {
    dependencies: Vec<Call>,
    function_name: Variable,
    arguments: Vec<Argument>,
    return_var: Option<Variable>,
  },
  CWScriptBlockCall {
    dependencies: Vec<Call>,
    block_id: CWScriptBlockID,
    arguments: Vec<Argument>,
    return_var: Option<Variable>,
  }
}