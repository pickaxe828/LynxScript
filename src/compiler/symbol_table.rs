use anyhow::{anyhow, Result};

/// Stable identifier for a scope within the symbol table.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct ScopeId(pub usize);

/// Arena of scopes with explicit scope IDs for symbol insertion and lookup.
#[derive(Debug, PartialEq, Clone)]
pub struct SymbolTable {
  pub scopes: Vec<Scope>,
}

/// Single lexical scope with a parent link and owned symbols.
#[derive(Debug, PartialEq, Clone)]
pub struct Scope {
  pub parent: Option<ScopeId>,
  pub symbols: Vec<SymbolRecord>,
}

/// Represents a symbol in the symbol table, including its original name, unique name (for mangling), type, and scope.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SymbolRecord {
  pub original_name: String,
  pub unique_name: String,
  pub symbol_type: SymbolType,
  pub scope_id: ScopeId,
}

/// Kind of symbol used for namespace and semantic checks.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum SymbolType {
  Variable,
  Function,
  Event,
  UIObject,
}

impl SymbolTable {
  pub fn new() -> Self {
    let root_scope = Scope {
      parent: None,
      symbols: Vec::new(),
    };
    Self {
      scopes: vec![root_scope],
    }
  }

  /// Return the root (global) scope ID.
  pub fn root_scope(&self) -> ScopeId {
    ScopeId(0)
  }

  /// Create a new scope under the given parent and return its ID.
  pub fn enter_scope(&mut self, parent: ScopeId) -> ScopeId {
    let scope_id = ScopeId(self.scopes.len());
    let new_scope = Scope {
      parent: Some(parent),
      symbols: Vec::new(),
    };
    self.scopes.push(new_scope);
    scope_id
  }

  /// Return the parent scope ID for the given scope, or error if at root.
  pub fn exit_scope(&self, scope_id: ScopeId) -> Result<ScopeId> {
    self.scopes
      .get(scope_id.0)
      .and_then(|scope| scope.parent)
      .ok_or_else(|| anyhow!("Cannot exit root scope"))
  }

  /// Insert a symbol into the given scope, error on same-scope duplicates.
  pub fn add_symbol(&mut self, scope_id: ScopeId, name: &str, symbol_type: SymbolType) -> Result<SymbolRecord> {
    let is_shadowing = self.is_shadowing_name(scope_id, name);
    let unique_name = if is_shadowing {
      format!("{}__s{}", name, scope_id.0)
    } else {
      name.to_string()
    };

    if self.find_in_scope(scope_id, name).is_some() {
      return Err(anyhow!("Duplicate symbol in scope {:?}: {}", scope_id, name));
    }

    let scope = self
      .scopes
      .get_mut(scope_id.0)
      .ok_or_else(|| anyhow!("Invalid scope id: {:?}", scope_id))?;

    let record = SymbolRecord {
      original_name: name.to_string(),
      unique_name,
      symbol_type,
      scope_id,
    };
    scope.symbols.push(record.clone());
    Ok(record)
  }

  /// Find a symbol by name within a single scope only.
  pub fn find_in_scope(&self, scope_id: ScopeId, name: &str) -> Option<&SymbolRecord> {
    self.scopes
      .get(scope_id.0)
      .and_then(|scope| scope.symbols.iter().find(|symbol| symbol.original_name == name))
  }

  /// Resolve a name by walking parent scopes outward from the given scope.
  pub fn resolve(&self, scope_id: ScopeId, name: &str) -> Option<&SymbolRecord> {
    let mut scope_id = Some(scope_id);
    while let Some(current) = scope_id {
      if let Some(symbol) = self.find_in_scope(current, name) {
        return Some(symbol);
      }
      scope_id = self.scopes.get(current.0).and_then(|scope| scope.parent);
    }
    None
  }

  /// Check if a name exists in any ancestor scope.
  fn is_shadowing_name(&self, scope_id: ScopeId, name: &str) -> bool {
    let mut current = self.scopes.get(scope_id.0).and_then(|scope| scope.parent);
    while let Some(parent_id) = current {
      if self.find_in_scope(parent_id, name).is_some() {
        return true;
      }
      current = self.scopes.get(parent_id.0).and_then(|scope| scope.parent);
    }
    false
  }
}