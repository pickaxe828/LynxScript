
use serde::{Deserialize, Serialize};
use serde_json;

/// Data structures of CatWeb JSONs, used for code generation.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Script {
  // This isn't needed when generating single scripts?
  // pub globalid: String,
  pub alias: String,
  pub class: String,
  pub content: Vec<CodeCard>
  // globalid: String,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Wrapper {
  Script(Vec<Script>)
}


/// In CatWeb, a "code card" is a rectangle block that you can write code in.
/// 
/// Which are either function declarations or event handlers.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CodeCard {
  FunctionDeclaration(FunctionDeclaration),
  Event(Event)
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Default)]
pub struct FunctionDeclaration {
    pub globalid: String,
    pub variable_overrides: Vec<FunctionParameter>,
    pub id: String,
    pub text: Vec<TextFieldValue>,
    pub actions: Vec<Action>,
  }

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Default)]
pub struct Event {
  pub globalid: String,
  pub id: String,
  pub text: Vec<TextFieldValue>,
  pub actions: Vec<Action>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct FunctionParameter {
  pub value: String,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Default)]
pub struct Action {
  pub globalid: String,
  pub id: String,
  pub text: Vec<TextFieldValue>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TextFieldValue {
  PlainText(String),
  Parameter(Parameter),
  Tuple(Tuple),
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Tuple {
  pub value: Vec<TextFieldValue>,
  pub t: String, // "tuple"
}

impl Default for Tuple {
  fn default() -> Self {
    Self {
      value: vec![],
      t: "tuple".to_string(),
    }
  }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Default)]
pub struct Parameter {
  pub value: String,
  pub l: String,
  pub t: String,
}