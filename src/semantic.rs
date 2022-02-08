use std::collections::HashMap;
use std::collections::VecDeque;

use super::ast::*;

fn type_from_unary_op(
  symbol_tables: &VecDeque<HashMap<String, Type>>,
  op: &UnaryOper,
  expr: &Expr,
) -> Result<Type, String> {
  match op {
    UnaryOper::Bang | &UnaryOper::Neg => {
      if type_from_expr(symbol_tables, expr) == Type::BaseType(BaseType::Bool) {
        return Ok(Type::BaseType(BaseType::Bool));
      }
    },
    UnaryOper::Len => match type_from_expr(symbol_tables, expr) {
      Type::Array(_) => return Ok(Type::BaseType(BaseType::Int)),
      _ => return Err("Invalid Unary Op".to_string()),
    },
    UnaryOper::Ord => {
      if type_from_expr(symbol_tables, expr) == Type::BaseType(BaseType::Char) {
        return Ok(Type::BaseType(BaseType::Int));
      }
    },
    UnaryOper::Chr => {
      if type_from_expr(symbol_tables, expr) == Type::BaseType(BaseType::Int) {
        return Ok(Type::BaseType(BaseType::Char));
      }
    },
  }

  return Err("Invalid type of unary op".to_string());
}

fn type_from_binary_op(
  symbol_tables: &VecDeque<HashMap<String, Type>>,
  op: &BinaryOper,
  exp1: &Expr,
  exp2: &Expr,
) -> Result<Type, String> {
  match op {
    BinaryOper::Mul | BinaryOper::Div | BinaryOper::Mod | BinaryOper::Add | BinaryOper::Sub => {
      if type_from_expr(symbol_tables, exp1) == Type::BaseType(BaseType::Int)
        && type_from_expr(symbol_tables, exp2) == Type::BaseType(BaseType::Int)
      {
        return Ok(Type::BaseType(BaseType::Int));
      }
    },

    BinaryOper::Gt | BinaryOper::Gte | BinaryOper::Lt | BinaryOper::Lte => {
      if (type_from_expr(symbol_tables, exp1) == Type::BaseType(BaseType::Int)
        && type_from_expr(symbol_tables, exp2) == Type::BaseType(BaseType::Int))
        || (type_from_expr(symbol_tables, exp1) == Type::BaseType(BaseType::Char)
          && type_from_expr(symbol_tables, exp2) == Type::BaseType(BaseType::Char))
      {
        return Ok(Type::BaseType(BaseType::Bool));
      }
    },

    BinaryOper::Eq | BinaryOper::Neq => {
      if type_from_expr(symbol_tables, exp1) == Type::BaseType(BaseType::Int)
        && type_from_expr(symbol_tables, exp2) == Type::BaseType(BaseType::Int)
      {
        return Ok(Type::BaseType(BaseType::Bool));
      }
    },

    BinaryOper::And | BinaryOper::Or => {
      if type_from_expr(symbol_tables, exp1) == Type::BaseType(BaseType::Bool)
        && type_from_expr(symbol_tables, exp2) == Type::BaseType(BaseType::Bool)
      {
        return Ok(Type::BaseType(BaseType::Bool));
      }
    },
  }

  return Err("Invalid binary op!".to_string());
}

fn type_from_expr(symbol_tables: &VecDeque<HashMap<String, Type>>, expr: &Expr) -> Type {
  match expr {
    Expr::IntLiter(_) => Type::BaseType(BaseType::Int),
    Expr::BoolLiter(_) => Type::BaseType(BaseType::Bool),
    Expr::CharLiter(_) => Type::BaseType(BaseType::Char),
    Expr::StrLiter(_) => Type::BaseType(BaseType::String),
    Expr::PairLiter => Type::BaseType(BaseType::Null),
    Expr::Ident(id) => match table_lookup(symbol_tables, &id.0) {
      Ok(t) => t,
      Err(s) => panic!("{}", s),
    },

    Expr::ArrayElem(elem) => {
      if !elem.1.is_empty() {
        let array_type = type_from_expr(symbol_tables, elem.1.first().unwrap());
        for exp in elem.1.clone() {
          if type_from_expr(symbol_tables, &exp) != array_type {
            panic!("Mixed array types")
          }
        }
        Type::Array(Box::new(array_type))
      } else {
        panic!("Array is empty.")
      }
    },

    Expr::UnaryApp(op, exp) => match type_from_unary_op(symbol_tables, op, exp) {
      Ok(t) => t,
      Err(s) => panic!("{}", s),
    },

    Expr::BinaryApp(exp1, op, exp2) => match type_from_binary_op(symbol_tables, op, &exp1, &exp2) {
      Ok(t) => t,
      Err(s) => panic!("{}", s),
    },
  }
}

// lookup: takes a string and symbol table and returns Ok(type) if found, Err if
// not
fn table_lookup(
  symbol_tables: &VecDeque<HashMap<String, Type>>,
  var: &str,
) -> Result<Type, String> {
  for table in symbol_tables {
    if let Some(var_type) = table.get(var) {
      return Result::Ok(var_type.clone());
    }
  }

  return Result::Err("not found".to_string());
}

// update: takes a string, type, exists and symbol table and returns
// Ok(symbolTable) if success, error otherwise

fn table_update(
  symbol_tables: &mut VecDeque<HashMap<String, Type>>,
  var: &str,
  val: Type,
  exists: bool,
) -> Result<HashMap<String, Type>, String> {
  if exists {
    for table in symbol_tables {
      if table.contains_key(var) {
        table.remove(var);
        table.insert(var.to_string(), val);
        return Ok(table.clone());
      }
    }
  } else {
    let curr = symbol_tables.front_mut().unwrap();
    curr.entry(var.to_string()).or_insert(val);
    return Ok(curr.clone());
  }

  unreachable!();
}

#[cfg(test)]

mod tests {

  use super::*;

  #[test]
  fn test_table_lookup() {
    let table: HashMap<String, Type> = HashMap::new();
    let mut vec: VecDeque<HashMap<String, Type>> = VecDeque::new();
    vec.push_front(table);

    for i in 0..4 {
      let mut curr: HashMap<String, Type> = HashMap::new();

      let var1 = format!("{}{}", "x", i);
      let var2 = format!("{}{}", "y", i);
      let var3 = format!("{}{}", "z", i);

      curr.insert(var1, Type::BaseType(BaseType::Bool));
      curr.insert(var2, Type::BaseType(BaseType::Int));
      curr.insert(var3, Type::BaseType(BaseType::String));

      vec.push_front(curr.clone());
    }

    assert_eq!(table_lookup(&vec, "x3"), Ok(Type::BaseType(BaseType::Bool)),);

    assert_eq!(
      table_lookup(&vec, "z3"),
      Ok(Type::BaseType(BaseType::String)),
    );

    assert_ne!(
      table_lookup(&vec, "v3"),
      Ok(Type::BaseType(BaseType::String)),
    );

    assert_eq!(table_lookup(&vec, "random"), Err("not found".to_string()),);
  }

  #[test]
  fn test_table_update() {
    let table: HashMap<String, Type> = HashMap::new();
    let mut vec: VecDeque<HashMap<String, Type>> = VecDeque::new();
    vec.push_front(table);

    for i in 0..4 {
      let mut curr: HashMap<String, Type> = HashMap::new();

      let var1 = format!("{}{}", "x", i);
      let var2 = format!("{}{}", "y", i);
      let var3 = format!("{}{}", "z", i);

      curr.insert(var1, Type::BaseType(BaseType::Bool));
      curr.insert(var2, Type::BaseType(BaseType::Int));
      curr.insert(var3, Type::BaseType(BaseType::String));

      vec.push_front(curr.clone());
    }

    // before tests = [x1: Bool, y1: Int, z1: String][x2: Bool, y2: Int, z2: String]
    // [x3: Bool, y3: Int, z3: String]

    assert_eq!(
      table_update(&mut vec, "x1", Type::BaseType(BaseType::Char), true),
      Ok(HashMap::from([
        ("x1".to_string(), Type::BaseType(BaseType::Char)),
        ("y1".to_string(), Type::BaseType(BaseType::Int)),
        ("z1".to_string(), Type::BaseType(BaseType::String)),
      ]))
    );

    assert_ne!(table_lookup(&vec, "x1"), Ok(Type::BaseType(BaseType::Bool)),)
  }
}
