use serde::{Deserialize, Serialize};
use serde_json::Result;

mod json;
mod structures;
mod symbol_table;

pub use structures::*;
pub use symbol_table::*;