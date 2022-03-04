use crate::ast::Type::{self, *};
use std::option::Option;

pub trait Unifiable {
  fn unify(self, t: Type) -> Option<Type>;
}

impl Unifiable for Type {
  fn unify(self, t: Type) -> Option<Type> {
    match (self, t) {
      (Any, t) | (t, Any) => Some(t),
      (t1, t2) if t1 == t2 => Some(t1),
      (Pair(x1, x2), Pair(y1, y2)) => Some(Pair(
        Box::new((*x1).unify(*y1)?),
        Box::new((*x2).unify(*y2)?),
      )),
      (Array(t1), Array(t2)) => Some(Array(Box::new((*t1).unify(*t2)?))),
      _ => None,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn types_unify_themselves() {
    assert_eq!(Any.unify(Int), Some(Int));
    assert_eq!(Int.unify(Any), Some(Int));
    assert_eq!(Int.unify(Int), Some(Int));
    assert_eq!(Bool.unify(Bool), Some(Bool));
    assert_eq!(Bool.unify(Int), None);
    assert_eq!(
      Pair(Box::new(Int), Box::new(Int)).unify(Pair(Box::new(Any), Box::new(Int))),
      Some(Pair(Box::new(Int), Box::new(Int)))
    );
    assert_eq!(
      Pair(Box::new(Int), Box::new(Int)).unify(Pair(Box::new(Int), Box::new(Any))),
      Some(Pair(Box::new(Int), Box::new(Int)))
    );
    assert_eq!(
      Pair(Box::new(Int), Box::new(Int)).unify(Pair(Box::new(Any), Box::new(Any))),
      Some(Pair(Box::new(Int), Box::new(Int)))
    );
    assert_eq!(
      Pair(Box::new(Any), Box::new(Any)).unify(Pair(Box::new(Any), Box::new(Any))),
      Some(Pair(Box::new(Any), Box::new(Any)))
    );
    assert_eq!(
      Array(Box::new(Int)).unify(Array(Box::new(Int))),
      Some(Array(Box::new(Int)))
    );
    assert_eq!(
      Array(Box::new(Int)).unify(Array(Box::new(Any))),
      Some(Array(Box::new(Int)))
    );
    assert_eq!(
      Array(Box::new(Any)).unify(Array(Box::new(Any))),
      Some(Array(Box::new(Any)))
    );
    assert_eq!(
      Array(Box::new(Pair(Box::new(Any), Box::new(Int)))).unify(Array(Box::new(Any))),
      Some(Array(Box::new(Pair(Box::new(Any), Box::new(Int)))))
    );
    assert_eq!(Int.unify(Int), Some(Int)); // Some means ==
    assert_eq!(Int.unify(Char), None); // None means !=
    assert_eq!(Int.unify(Any), Some(Int)); // Any unifies with anything
    assert_ne!(Any.unify(Int), Some(Any)); // Its commutativitiytyty
    assert_eq!(
      Pair(Box::new(Any), Box::new(Int)).unify(Pair(Box::new(Int), Box::new(Int))),
      Some(Pair(Box::new(Int), Box::new(Int)))
    ); // Anys get overridden by other things recursive
    assert_eq!(
      Array(Box::new(Int)).unify(Array(Box::new(Any))),
      Some(Array(Box::new(Int)))
    )
  }
}
