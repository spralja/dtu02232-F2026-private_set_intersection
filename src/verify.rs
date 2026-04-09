#![allow(non_snake_case)]
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

  if true_intersection != I_set {
    println!("Expected intersection: {:?}", true_intersection);
    println!("Returned intersection: {:?}", I_set);
  }

  true_intersection == I_set
}
