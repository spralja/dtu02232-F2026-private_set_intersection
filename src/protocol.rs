#![allow(non_snake_case)]
use crate::types::{Bytes};

use curve25519_dalek::ristretto::RistrettoPoint;
use sha2::{Digest, Sha256, Sha512};

/// H
pub fn hash_to_group(element: &Bytes<64>) -> RistrettoPoint {
  RistrettoPoint::hash_from_bytes::<Sha512>(element)
}

/// H'
pub fn hash_from_group(point: &RistrettoPoint) -> Bytes<32> {
  Sha256::digest(point.compress().to_bytes()).into()
}
