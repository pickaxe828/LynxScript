use std::vec;

use serde::{Deserialize, Serialize};
use serde_json::Result;

pub mod json;
pub mod structures;
pub mod symbol_table;

pub use structures::*;
pub use symbol_table::*;

#[derive(Debug, PartialEq, Clone)]
pub struct CWBlockScriptGenerator {}

impl CWBlockScriptGenerator {
  pub fn new() -> Self {
    Self {}
  }

  pub fn generate(&self, program: Program) -> String {
    let json_program = json::Wrapper::Script(vec![self.generate_program(program)]);
    format!("{}", serde_json::to_string(&json_program).unwrap())
  }

  pub fn generate_program(&self, program: Program) -> json::Script {
    json::Script {
      class: String::from("script"),
      alias: String::from(""),
      content: program.main_block.into_iter().map(|item| self.generate_item(item)).collect(),
    }
  }

  pub fn generate_item(&self, item: Item) -> json::CodeCard {
    match item {
      Item::FunctionDeclaration { name, parameters, body } => {
        json::CodeCard::FunctionDeclaration(
          json::FunctionDeclaration{
            // ID: Function declaration
            id: "6".to_string(),
            text: vec![
              json::TextFieldValue::PlainText(Default::default()), // Argument padding
              json::TextFieldValue::Parameter(json::Parameter {    // Function name
                value: name, 
                t: "string".to_string(),
                ..Default::default() // TODO: Check if key "l" are crucial for import
              }),
            ],
            variable_overrides: parameters.into_iter()
              .map(|param| json::FunctionParameter { value: param.name }).collect(),
            actions: body.into_iter()
              .map(|statement| {
                let content = statement.content.into_iter();
                let depedencies = statement.dependencies.into_iter();
                // Put dependency calls before content, since dependencies should execute first
                depedencies.map(|call|
                  self.generate_script_block(call)
                ).chain(
                  content.map(|call|
                    self.generate_script_block(call)
                  )
                )
              }).flatten().collect(),
            globalid: Default::default(),
          }
        )
      },
      Item::Event { name, body } => {
        json::CodeCard::Event(
          json::Event {
            id: todo!(),
            text: vec![], // TODO
            actions: vec![], // TODO
            ..Default::default()
          }
        );
        todo!()
      }
    }
  }

  pub fn generate_script_block(&self, call: Call) -> json::Action {
    match call {
      Call::CWScriptBlockCall { block_id, arguments, return_var, dependencies } => {
        json::Action {
          id: block_id.id,
          text: arguments.into_iter().map(|arg| match arg {
            Argument::Literal(lit) => json::TextFieldValue::Parameter( json::Parameter { 
              value: lit.value, 
              t: "string".to_string(), // TODO: Check if "string" is the correct value for key "t"
              ..Default::default()     // TODO: Check if key "l" are crucial for import
            }),
            Argument::Identifier(var) => json::TextFieldValue::Parameter( json::Parameter { 
              value: var.name,
              t: "string".to_string(), // TODO: Check if "string" is the correct value for key "t"
              ..Default::default()     // TODO: Check if key "l" are crucial for import
            }),
            Argument::RawString(rstr) => json::TextFieldValue::PlainText(rstr), // FIXME: Implement RawString
          }).chain(
            // Return variable are provided to CatWeb as the last parameter of the script block
            // So we append it to the end of the arguments list if it's Some
            //
            // Chain return_var as the last element if it's Some
            (|| -> Vec<json::TextFieldValue> {
              if let Some(return_var) = return_var {
                // Return a vector with the return_var as the only element if return_var is Some
                vec![json::TextFieldValue::Parameter( json::Parameter {
                  value: return_var.name,
                  t: "string".to_string(), // TODO: Check if "string" is the correct value for key "t"
                  ..Default::default()     // TODO: Check if key "l" are crucial for import
                })]
              } else {
                // Else return_var is None, return empty vec
                vec![]
              }
            })()
          ).collect(),
          ..Default::default()
        }
      },
      Call::FunctionCall { dependencies, function_name, arguments, return_var } => {
        // TODO: Replace with in-language implementation of call with inlining and export-as and to be put in symbol table for compiler to find, instead of hardcoding the implementation
        
        json::Action {
          id: "87".to_string(),
          text: vec![
            json::TextFieldValue::PlainText("".to_string()), // Argument padding
            json::TextFieldValue::Tuple( json::Tuple {
              value: vec![
              ],
              t: "tuple".to_string(),
            }),
            json::TextFieldValue::PlainText("".to_string()), // Argument padding
            json::TextFieldValue::Parameter( json::Parameter {
              value: return_var.map(|var| var.name).unwrap_or_default(),
              t: "string".to_string(),
              ..Default::default() // TODO: Check if key "l" are crucial for import
            }),
          ],
          ..Default::default()
        }
      },
    }
  }
}