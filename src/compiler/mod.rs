use std::vec;

use crate::parser;
use crate::codegen;
use crate::parser::BinOperator;

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

  pub fn add_attribute(&mut self, attribute: parser::Item) -> Result<(), anyhow::Error> {
    match attribute {
      parser::Item::Attribute { name, content } => {
        match name.as_str() {
          "inline" => {self.attributes.push(Attribute::Inline); Ok(())},
          "export_as" => {
            if let Some(content_str) = content {
              self.attributes.push(Attribute::ExportAs(content_str));
              Ok(())
            } else {
              Err(anyhow::anyhow!("Attribute \"export_as\" requires a content string"))
            }
          },
          attr_str => {
            Err(anyhow::anyhow!("Unknown attribute: {}", attr_str))
          }
        }
      }
      attr => unimplemented!("Unsupported item type {:?} in add_attribute", attr)
    }
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct LynxScriptCompiler {
  syntax_tree: parser::Program,
  state: CompilerState
}

impl LynxScriptCompiler {
  pub fn new(syntax_tree: parser::Program) -> Self {
    let mut temp = Self { syntax_tree, state: CompilerState::new() };
    temp.hoist_items();
    temp
  }

  pub fn compile(self: &mut LynxScriptCompiler) -> codegen::Program {
    // Temporarily take the main_block out of self to avoid borrow conflict
    // This leaves an empty Vec inside self.syntax_tree.main_block temporarily
    let main_block = std::mem::take(&mut self.syntax_tree.main_block);

    // Compilation logic goes here
    let compiled_items: Vec<Option<codegen::Item>> = 
      main_block.iter().map(|item| -> Option<codegen::Item> {
        self.compile_item(item)
      }).collect::<Vec<Option<codegen::Item>>>();

    // Restore the main_block
    self.syntax_tree.main_block = main_block;

    codegen::Program::new(
      compiled_items.into_iter().filter_map(|item| item).collect::<Vec<codegen::Item>>()
    )
  }

  fn compile_item(self: &mut LynxScriptCompiler, item: &parser::Item) -> Option<codegen::Item> {
    match item {
      parser::Item::Attribute { name, content } => {
        self.state.add_attribute(item.clone()).unwrap();
        None
      },
      parser::Item::FunctionDeclaration { name, parameters, body } => {
        // Compile function declaration
        Some(codegen::Item::FunctionDeclaration {
          // TODO: Register function in symbol table
          name: name.clone(),
          body: body.iter().map(|stmt| self.compile_statement(stmt)).collect::<Vec<codegen::Statement>>(),
          // TODO: SYMBOL TABLE?
          parameters: parameters.into_iter().map(|param| {
            match param {
              parser::Expression::Identifier(iden) => codegen::Variable { name: iden.clone() },
              _ => unimplemented!("Unsupported parameter type in function declaration: {:?}", param),
            }
          }).collect::<Vec<codegen::Variable>>(),
        })
      }
    }
  }

  fn compile_statement(self: &mut LynxScriptCompiler, stmt: &parser::Statement) -> codegen::Statement {
    match stmt {
      // TODO: Implement link statement compilation
      parser::Statement::Expression { expr } => {
        codegen::Statement {
          dependencies: self.compile_expression(expr).dependencies,
          content: Vec::new(),
        }
      },
      parser::Statement::Assignment { lhs, rhs } => todo!(""),
      parser::Statement::Link { path } => todo!("Linking CatWeb JSON to object identifiers to be implemented: {}", path),
    }
  }

  fn compile_expression(self: &mut LynxScriptCompiler, expr: &parser::Expression) -> codegen::Expression {
    match expr {
      parser::Expression::Literal (literal) => {
        codegen::Expression {
          dependencies: Vec::new(),
          content: Some( codegen::Variable { name: match literal {
            parser::Literal::Bool(bool) => bool.to_string(),
            parser::Literal::Float(inner_string)
            | parser::Literal::Integer(inner_string)
            | parser::Literal::String(inner_string) 
            => inner_string.to_owned(),
          }}),
        }
      },
      parser::Expression::Identifier (name) => {
        // TODO: Lookup identifier in symbol table
        codegen::Expression { 
          dependencies: Vec::new(),
          content: Some(codegen::Variable { name: name.clone() }),
        }
      },
      parser::Expression::ActionScriptBlockID (block_id) => {
        unimplemented!("ActionScriptBlockID cannot be read as expressions or values.");
        // codegen::Expression {
        //   dependencies: Vec::new(),
        //   content: Some(codegen::ActionScriptBlockID{ id: block_id.clone() }),
        // }
      },
      parser::Expression::Call { function, arguments } => {
        // FIXME: Handle function inlining
        match &**function {
          // Function call by name
          parser::Expression::Identifier(iden) => {
            // Use fold to accumulate directly into single vectors
            let (arg_dependencies, arg_contents) = arguments.iter().fold(
                (Vec::new(), Vec::new()), 
                |(mut deps, mut conts), arg| -> (Vec<codegen::Call>, Vec<codegen::Argument>) {
                    let expr = self.compile_expression(arg);
                    deps.extend(expr.dependencies);
                    conts.push(codegen::Argument::Identifier(codegen::Variable { name: expr.content.unwrap_or_default().name }));
                    (deps, conts)
                }
            );
            
            codegen::Expression {
              dependencies: vec![
                codegen::Call::FunctionCall {
                  dependencies: arg_dependencies,
                  function_name: codegen::Variable { name: iden.clone() },
                  arguments: arg_contents,
                  // FIXME: Return variable handling
                  return_var: None,
                }
              ], 
              // FIXME: Return values from function calls is not implemented yet
              content: None
            }
          },
          // Function call by block id (raw calls)
          parser::Expression::ActionScriptBlockID(action_id) => 
            codegen::Expression { dependencies: vec![
              codegen::Call::ActionScriptBlockCall {
                dependencies: Vec::new(),
                block_id: codegen::ActionScriptBlockID { id: action_id.to_owned() },
                arguments: vec![],
                return_var: None,
              }
            ], 
            // No return from ActionScriptBlock
            content: None },
          parser::Expression::Call { .. } => unimplemented!("Function as returned value and nested calls are not supported yet."),
          others => unimplemented!("Unsupported call target in call: {:?}", others),
        }
      },
      parser::Expression::BinOperation { lhs, op, rhs } => {
        // TODO: Operator overloading?
        // Generate the call
        let call = codegen::Call::FunctionCall {
          dependencies: vec![
            self.compile_expression(lhs).dependencies,
            self.compile_expression(rhs).dependencies,
          ].into_iter().flatten().collect(),
          function_name: codegen::Variable { name: LynxScriptCompiler::map_bin_op(op) },
          arguments: vec![
            // FIXME: Wrong implementation: Calculation of expressions should be standalone dependencies as well
            codegen::Argument::Identifier(codegen::Variable { name: self.compile_expression(lhs).content.unwrap_or_default().name }),
            codegen::Argument::Identifier(codegen::Variable { name: self.compile_expression(rhs).content.unwrap_or_default().name }),
          ],
          return_var: None, // FIXME: Match return variable
        };

        // Wrap it in an expression
        codegen::Expression {
          dependencies: vec![call],
          content: None, // FIXME: Match return variable
        }
      },
      
      parser::Expression::UnaryOperation { op, expr } => todo!()
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
      &BinOperator::Dot => panic!("Dot cannot be mapped to call signature"),
      &BinOperator::Comma => panic!("Comma cannot be mapped to call signature"),
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
    codegen::Call::ActionScriptBlockCall { 
      dependencies: Vec::new(),
      block_id: codegen::ActionScriptBlockID { id: "".to_string() }, // FIXME: Change ActionBlockID to the correct block ID
      arguments: argument,
      return_var: None, // FIXME: Handle return variable for sync calls
    }
  }
}