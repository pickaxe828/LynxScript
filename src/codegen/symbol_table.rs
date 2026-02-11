use std::{collections::{HashMap, HashSet}, hash::Hash};

#[derive(Debug, PartialEq, Clone)]
pub struct SymbolTable {
  pub upper: Option<Box<SymbolTable>>,
  pub symbols: HashSet<SymbolRecord>,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct SymbolRecord {
  pub name: String,
  pub symbol_type: SymbolType,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum SymbolType {
  Variable,
  Function,
  Event,
  UIObject,
}

impl SymbolTable {
  pub fn new(upper: Option<Box<SymbolTable>>, symbols: HashSet<SymbolRecord>) -> Self {
    Self { upper: upper, symbols: symbols }
  }

  pub fn add_symbol(&mut self, name: String, symbol_type: SymbolType) {
    let record = SymbolRecord { name, symbol_type };
    self.symbols.insert(record);
  }
  
  pub fn remove_symbol(&mut self, name: String, symbol_type: SymbolType) {
    let record = SymbolRecord { name, symbol_type };
    self.symbols.remove(&record);
  }

  pub fn lookup(&self, name: String, symbol_type: SymbolType) -> Option<&SymbolRecord> {
    self.symbols.get(&SymbolRecord {
      name: name,
      symbol_type: symbol_type,
    })
  }
}