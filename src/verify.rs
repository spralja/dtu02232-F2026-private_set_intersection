use crate::types::Element;
use std::collections::HashSet;

pub fn verify_intersection(
  X: &Vec<Element>,
  Y: &Vec<Element>,
  I: &Vec<Element>,
) -> bool {
  let X_set: HashSet<Element> = X.iter().cloned().collect();
  let Y_set: HashSet<Element> = Y.iter().cloned().collect();

  let true_intersection: HashSet<Element> =
    X_set.intersection(&Y_set).cloned().collect();

  let I_set: HashSet<Element> = I.iter().cloned().collect();

  true_intersection == I_set
}
