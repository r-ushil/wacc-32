/* This is code General's version of context.rs, cannot re-use context.rs
because that's mutable. */

use crate::analyser::context::{self, *};
use crate::ast::*;

pub use context::SymbolTable;

use super::asm::{Label, Offset, Reg, ARM_DSIZE_WORD};

#[derive(Debug)]
pub struct ScopeReader<'a> {
  /* Maps identifiers to types for each variable declared in this scope. */
  current: SymbolTable,
  /* The scope this scope is inside of,
  and where abouts within that scope it is. */
  /* context: None means this is the global scope. */
  parents: Option<&'a ScopeReader<'a>>,
}

impl ScopeReader<'_> {
  /* Makes new Symbol table with initial global scope. */
  pub fn new<'a>(st: &'a SymbolTable) -> ScopeReader<'a> {
    /* When symbol tables are used in the analyser, they're used by callers
    who only have the origional idents the programmer gave to them, now we're
    in code General, the global rename has been done to the whole AST.
    This means .generate(...) is being called on AST nodes which have the
    renamed identifiers, so the symbol table needs to be changed to be indexed
    by these new values. */

    /* Make new symbol table from fresh to copy the renamed values into. */
    let mut new_st = SymbolTable::default();
    new_st.prefix = st.prefix.clone();

    for (id, entry) in st.table.iter() {
      /* Only rename local variables. */
      if let IdentInfo::LocalVar(t, offset) = entry {
        /* Calculate what it got renamed to. */
        let new_id = format!("{}{}", st.prefix, offset);

        new_st
          .table
          .insert(new_id, IdentInfo::LocalVar(t.clone(), *offset));
      } else {
        new_st.table.insert(id.clone(), entry.clone());
      }
    }

    ScopeReader {
      current: new_st,
      parents: None,
    }
  }

  /* Get the information about this ident,
  renames it if it's a local variable (global rename). */
  /* The offsets returned are distances from THE BOTTOM
  of this scope. (THE STACK POINTER) */
  /* ONLY CALL THIS ONCE PER AST IDENT, OTHERWISE THE RENAME WILL HAPPEN TWICE. */
  pub fn get(&self, ident: &Ident) -> Option<IdentInfo> {
    use IdentInfo::*;
    match self.current.table.get(ident) {
      /* Identifier declared in this scope, return. */
      Some(info) => {
        if let LocalVar(type_, reg) = info {
          Some(LocalVar(type_.clone(), *reg))
        } else {
          Some(info.clone())
        }
      }
      /* Look for identifier in parent scope, recurse. */
      None => match self.parents?.get(ident)? {
        LocalVar(t, reg) => Some(LocalVar(t, reg)),
        info => Some(info),
      },
    }
  }

  pub fn _get_var(&self, ident: &Ident) -> Option<(Type, Reg)> {
    match self.get(ident)? {
      IdentInfo::LocalVar(t, reg) => Some((t, reg)),
      _ => None,
    }
  }

  pub fn get_def(&self, ident: &Ident) -> Option<Struct> {
    match self.get(ident)? {
      IdentInfo::TypeDef(def) => Some(def),
      _ => None,
    }
  }

  pub fn _get_label(&self, ident: &Ident) -> Option<(Type, Label)> {
    match self.get(ident)? {
      IdentInfo::Label(t, label) => Some((t, label)),
      _ => None,
    }
  }

  /* Returns type of given ident */
  pub fn _get_type(&self, ident: &Ident) -> Option<&Type> {
    use IdentInfo::*;
    match self.current.table.get(ident) {
      Some(LocalVar(t, _) | Label(t, _)) => Some(t),
      None => self.parents?._get_type(ident),
      Some(TypeDef(_)) => None,
    }
  }

  // pub fn get_total_offset(&self) -> Offset {
  //   if self.current.table.is_empty() && self.current.size == ARM_DSIZE_WORD {
  //     /* When there are no symbols but the scope is 4 bytes long, we're at the
  //     scope used to reserve space for the lr register. */
  //     0
  //   } else {
  //     /* Otherwise, add the size of this scope and all the above scopes. */
  //     self.current.size + self.parents.unwrap().get_total_offset()
  //   }
  // }

  pub fn new_scope<'a>(
    &'a self,
    symbol_table: &'a SymbolTable,
  ) -> ScopeReader<'a> {
    let mut st = ScopeReader::new(symbol_table);

    /* The parent of the returned scope is the caller. */
    st.parents = Some(self);

    st
  }
}
