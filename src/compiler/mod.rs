use std::vec;

use crate::parser;
use crate::codegen;
use crate::parser::BinOperator;
use crate::parser::Item;

use anyhow;
use regex_macro::{regex};

#[derive(Debug, PartialEq, Clone)]
pub enum Attribute {
  Inline,
  ExportAs(String),
}

#[derive(Debug, PartialEq, Clone)]
pub struct CompilerState {
  attributes: Vec<Attribute>,
}

impl CompilerState {
  pub fn new() -> Self {
    Self {
      attributes: Vec::new(),
    }
  }

  pub fn add_attribute(&mut self, attribute: &parser::Attribute) -> Result<(), anyhow::Error> {
    match attribute {
      parser::Attribute::Inline => {
        self.attributes.push(Attribute::Inline);
        return Ok(())
      },
      parser::Attribute::ExportAs(content_str) => {
        self.attributes.push(Attribute::ExportAs(content_str.clone()));
        return Ok(())
      },
      _ => {
        return Err(anyhow::anyhow!("Attribute \"export_as\" requires a content string"))
      }
    }
  }

  pub fn pop_all_attributes(&mut self) -> Vec<Attribute> {
    std::mem::take(&mut self.attributes)
  }
}

/// Compiler module: 
/// 
/// Transforms the parsed syntax tree into an intermediate representation (IR) suitable for code generation. 
/// 
/// This includes tasks such as:
/// - Expanding expressions and statements into CatWeb equivalent structures (calls/ control flow structures)
/// - Inlining functions marked with the `inline` attribute
#[derive(Debug, PartialEq, Clone)]
pub struct Compiler {
  syntax_tree: parser::Program,
  state: CompilerState
}

impl Compiler {
  pub fn new(syntax_tree: parser::Program) -> Self {
    let mut temp = Self { syntax_tree, state: CompilerState::new() };
    // FIXME: Hoisting is unnecessary in CatWeb
    // temp.hoist_items();
    temp
  }

  // TODO: See if the implementation is correct
  pub fn compile(self: &mut Compiler) -> codegen::Program {
    // Temporarily take the main_block out of self to avoid borrow conflict
    // This leaves an empty Vec inside self.syntax_tree.main_block temporarily
    let main_block = std::mem::take(&mut self.syntax_tree.main_block);

    // Compilation logic goes here
    let compiled_items: Vec<codegen::Item> = main_block.iter().filter_map(|item| {
      self.compile_item(item)
    }).collect();

    // Restore the main block
    self.syntax_tree.main_block = main_block;

    codegen::Program::new(compiled_items)
  }

  fn compile_item(self: &mut Compiler, item: &parser::Item) -> Option<codegen::Item> {
    match item {
      parser::Item::Attribute(attr) => {
        self.state.add_attribute(attr).unwrap();
        None
      },
      parser::Item::FunctionDeclaration(func) => {
        // Compile function declaration

        let attributes = self.state.pop_all_attributes();

        // FIXME: Implement function inlining
        let inlining: bool = attributes.iter().any(|attr| matches!(attr, Attribute::Inline));

        // FIXME: Implement function renaming on symbol table

        let export_as: Option<String> = attributes.iter().find_map(|attr| match attr {
          Attribute::ExportAs(name) => {
            Some(name.clone())
          },
          _ => None,
        });

        Some(codegen::Item::FunctionDeclaration {
          // TODO: Register function in symbol table
          name: export_as.unwrap_or_else(|| func.name.clone()),
          body: func.body.iter().map(|stmt| self.compile_statement(stmt)).collect::<Vec<codegen::Statement>>(),
          // TODO: SYMBOL TABLE?
          parameters: func.parameters.iter().map(|param| {
            match param {
              parser::Expression::Identifier(iden) => codegen::Variable { name: iden.clone() },
              _ => unimplemented!("Unsupported parameter type in function declaration: {:?}", param),
            }
          }).collect::<Vec<codegen::Variable>>(),
        })
      }
    }
  }

  fn compile_statement(self: &mut Compiler, stmt: &parser::Statement) -> codegen::Statement {
    match stmt {
      // TODO: Implement link statement compilation
      parser::Statement::Expression { expr } => {
        codegen::Statement {
          dependencies: self.compile_expression(expr).dependencies,
          content: Vec::new(),
        }
      },
      parser::Statement::Assignment { lhs, rhs } => todo!("Assignment statement compilation to be implemented: {:?} = {:?}", lhs, rhs),
      parser::Statement::Link { path } => todo!("Linking CatWeb JSON to object identifiers to be implemented: {}", path),
    }
  }

  fn compile_expression(self: &mut Compiler, expr: &parser::Expression) -> codegen::Expression {
    // Compile the expression based on it's type
    match expr {
      parser::Expression::Literal (literal) => {

        let content = match literal {
          parser::Literal::Bool(bool) 
          => codegen::Argument::Literal(
            codegen::Literal { value: bool.to_string() }
          ),
          parser::Literal::Float(inner_string)
          | parser::Literal::Integer(inner_string)
          | parser::Literal::String(inner_string)
          => codegen::Argument::Literal(
            codegen::Literal { value: inner_string.to_owned() }
          ),
          parser::Literal::RawString(inner_string) => codegen::Argument::RawString(inner_string.to_owned()),
        };
        codegen::Expression {
          dependencies: Vec::new(),
          content: Some(content),
        }
      },
      parser::Expression::Identifier (name) => {
        // TODO: Lookup identifier in symbol table
        codegen::Expression { 
          dependencies: Vec::new(),
          content: Some(codegen::Argument::Identifier(codegen::Variable { name: name.clone() })),
        }
      },
      parser::Expression::CWScriptBlockID (block_id) => {
        unimplemented!("CWScriptBlockID cannot be read as expressions or values.");
        // codegen::Expression {
        //   dependencies: Vec::new(),
        //   content: Some(codegen::CWScriptBlockID{ id: block_id.clone() }),
        // }
      },
      parser::Expression::Call { function, arguments } => self.compile_call(function, arguments),
      parser::Expression::BinOperation { lhs, op, rhs } => {
        // TODO: Operator overloading?
        let lhs_compiled = self.compile_expression(lhs);
        let rhs_compiled = self.compile_expression(rhs);

        let lhs_as_arg = lhs_compiled.content.expect("Binary operation expected LHS argument"); 
        let rhs_as_arg = rhs_compiled.content.expect("Binary operation expected RHS argument");
        // Generate the call
        let call = codegen::Call::FunctionCall {
          dependencies: vec![
            lhs_compiled.dependencies,
            rhs_compiled.dependencies,
          ].into_iter().flatten().collect(),
          function_name: codegen::Variable { name: Compiler::map_bin_op(op) },
          arguments: vec![
            // FIXME: Wrong implementation: Calculation of expressions should be standalone dependencies as well
            lhs_as_arg,
            rhs_as_arg,
          ],
          return_var: None, // FIXME: Match return variable
        };

        // Wrap it in an expression
        codegen::Expression {
          dependencies: vec![call],
          // FIXME: Binary operations compilation unimplemented, calculated contents are not passed as variable
          content: None, // FIXME: Match return variable
        }
      },
      
      parser::Expression::UnaryOperation { op, expr } => todo!()
    }
  }

  pub fn compile_call(&mut self, function: &parser::Expression, arguments: &Vec<parser::Expression>) -> codegen::Expression {
    // Compile arguments of the call first
    let (dependencies, arguments): (Vec<Vec<codegen::Call>>, Vec<codegen::Argument>) = arguments.iter()
      .map(|arg| {
        let dep_expr = self.compile_expression(arg);
        let dep_var = dep_expr.content.expect("Expected argument value");
        (
          dep_expr.dependencies, 
          dep_var
        )
      }
    ).unzip();
    
    // Match cases based on type of the function call
    match &*function {
      // Normal function calls
    // FIXME: Handle function inlining
      parser::Expression::Identifier(iden) => {
        codegen::Expression {
          dependencies: vec![
            codegen::Call::FunctionCall {
              dependencies: dependencies.into_iter().flatten().collect(),
              function_name: codegen::Variable { name: iden.clone() },
              arguments: arguments,
              // FIXME: Return variable handling
              return_var: None,
            }
          ],
          // FIXME: Return values from function calls is not implemented yet
          content: None
        }
      },

      // Function call by block id (raw calls)
      // FIXME: Argument compilation not implemented yet
      parser::Expression::CWScriptBlockID(action_id) => {
        codegen::Expression {
          dependencies: vec![
            codegen::Call::CWScriptBlockCall {
              dependencies: dependencies.into_iter().flatten().collect(),
              block_id: codegen::CWScriptBlockID { id: action_id.to_owned() },
              arguments: arguments,
              return_var: None,
            }
          ], 
          // FIXME: Return values from raw calls not implemented yet
          content: None
        }
      },
      parser::Expression::Call { .. } => unimplemented!("Function as return value and chained calls are not supported yet."),
      others => unimplemented!("Unsupported call target in call: {:?}", others),
    }
  }

  /// Returns call signature string for given binary operator
  pub fn map_bin_op(op: &BinOperator) -> String {
    match op {
      &BinOperator::Addition => "add".to_string(),
      &BinOperator::Subtraction => "sub".to_string(),
      &BinOperator::Multiplication => "mul".to_string(),
      &BinOperator::Division => "div".to_string(),
      &BinOperator::Power => "pow".to_string(),
      // &BinOperator::Dot => panic!("Dot cannot be mapped to call signature"),
      // &BinOperator::Comma => panic!("Comma cannot be mapped to call signature"),
      op_rest => panic!("Binary operator {:?} cannot be mapped to call signature", op_rest),
    }
  }
  
  /// Moves all the function declarations to the top of the main block
  pub fn hoist_items(&mut self) {
    let mut hoisted_items: Vec<parser::Item> = Vec::new();
    let mut other_items: Vec<parser::Item> = Vec::new();

    for item in self.syntax_tree.main_block.drain(..) {
      match item {
        parser::Item::FunctionDeclaration { .. } => hoisted_items.push(item),
        _ => other_items.push(item),
      }
    }

    // Reconstruct the main block with hoisted items first
    self.syntax_tree.main_block = hoisted_items;
    self.syntax_tree.main_block.extend(other_items);
  }
  
  /// Generate CatWeb calls
  pub fn generate_catweb_sync_call(name: codegen::Variable, argument: Vec<codegen::Argument>) -> codegen::Call {
    codegen::Call::CWScriptBlockCall { 
      dependencies: Vec::new(),
      block_id: codegen::CWScriptBlockID { id: "".to_string() }, // FIXME: Change ActionBlockID to the correct block ID
      arguments: argument,
      return_var: None, // FIXME: Handle return variable for sync calls
    }
  }
}