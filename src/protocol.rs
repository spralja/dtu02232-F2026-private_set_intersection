use crate::client_t6::{ClientState2, Message2};

use crate::types::{Bytes, Element};

use crate::server_t6::{Message3, ServerState1};
use curve25519_dalek::constants::RISTRETTO_BASEPOINT_POINT;
use curve25519_dalek::{Scalar, ristretto::RistrettoPoint};
use rand::rngs::OsRng;
use sha2::{Digest, Sha256, Sha512};

/// H
pub fn hash_to_group(element: &Bytes<64>) -> RistrettoPoint {
  RistrettoPoint::hash_from_bytes::<Sha512>(element)
}

/// H'
pub fn hash_from_group(point: &RistrettoPoint) -> Bytes<32> {
  Sha256::digest(point.compress().to_bytes()).into()
}

/// H''
fn arb_hash_to_scalar(element: &[u8]) -> Scalar {
  Scalar::hash_from_bytes::<Sha512>(element)
}

pub fn prove(
  s1: ServerState1,
  m2: Message2,
) -> (RistrettoPoint, Vec<RistrettoPoint>, Scalar) {
  let mut rng = OsRng;
  let a = Scalar::random(&mut rng);

  let A_list: Vec<RistrettoPoint> = m2.theta.iter().map(|t| t * a).collect();
  let A = RISTRETTO_BASEPOINT_POINT * a;
  let mut e_bytes: Vec<u8> = A.compress().to_bytes().into();
  let mut tmp: Vec<u8> = A_list
    .iter()
    .map(|t| t.compress().to_bytes())
    .flat_map(|t| t)
    .collect();
  e_bytes.append(&mut tmp);
  let mut tmp: Vec<u8> = m2
    .theta
    .iter()
    .map(|t| t.compress().to_bytes())
    .flat_map(|t| t)
    .collect();
  e_bytes.append(&mut tmp);
  let mut tmp: Vec<u8> = s1.R.compress().to_bytes().into();
  e_bytes.append(&mut tmp);
  let e = arb_hash_to_scalar(&e_bytes);

  let z = a + (e * s1.alpha);
  (A, A_list, z)
  // e.append(A_list.iter().map(|t| t.compress().to_bytes()).collect().into())
}

pub fn verify(c2: ClientState2, m3: Message3) -> bool {
  let A = m3.pi.0;
  let A_list = m3.pi.1;
  let z = m3.pi.2;

  let mut e_bytes: Vec<u8> = A.compress().to_bytes().into();
  let mut tmp: Vec<u8> = A_list
    .iter()
    .map(|t| t.compress().to_bytes())
    .flat_map(|t| t)
    .collect();
  e_bytes.append(&mut tmp);
  let mut tmp: Vec<u8> = c2
    .theta
    .iter()
    .map(|t| t.compress().to_bytes())
    .flat_map(|t| t)
    .collect();
  e_bytes.append(&mut tmp);
  let mut tmp: Vec<u8> = c2.R.compress().to_bytes().into();
  e_bytes.append(&mut tmp);
  let e = arb_hash_to_scalar(&e_bytes);

  let gz = RISTRETTO_BASEPOINT_POINT * z;

  let cond1 = gz == A + (c2.R * e);
  let cond2 = c2
    .theta
    .iter()
    .zip(A_list)
    .zip(m3.T)
    .all(|((theta, a), t)| {
      let thetaz = theta * z;
      let tmp = a + t * e;
      tmp == thetaz
    });

  cond1 && cond2
}
