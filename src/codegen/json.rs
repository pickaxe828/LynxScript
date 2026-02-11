/// Data structures of CatWeb JSONs, used for code generation.

use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Script {
  globalid: String,
  alias: String,
  class: String,
  content: Vec<CodeCard>
  // globalid: String,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum CodeCard {
  FunctionDeclaration {
    globalid: String,
    variable_overrides: Vec<FunctionParameter>,
    id: String,
    text: Vec<Text>,
    actions: Vec<Action>,
  },
  Event {
    globalid: String,
    id: String,
    text: Vec<Text>,
    actions: Vec<Action>,
  }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Text {
  PlainText(String),
  Parameter {
    value: String,
    l: String,
    t: String,
  },
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct FunctionParameter {
  value: String,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Action {
  globalid: String,
  id: String,
  text: Vec<Text>,
}