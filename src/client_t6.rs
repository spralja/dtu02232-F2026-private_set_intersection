use crate::protocol::{hash_from_group, hash_to_group, verify};
use crate::server_t6::{Message1, Message3};
use crate::types::{Bytes, Element};

use curve25519_dalek::ristretto::RistrettoPoint;
use curve25519_dalek::scalar::Scalar;
use rand::rngs::OsRng;

pub struct ClientStateInit {
  pub Y: Vec<Element>,
}

#[derive(Clone)]
pub struct ClientState2 {
  pub Y: Vec<Element>,
  pub L: Vec<Bytes<32>>,
  pub R: RistrettoPoint,
  pub beta: Vec<Scalar>,
  pub theta: Vec<RistrettoPoint>,
}

#[derive(Clone)]
pub struct Message2 {
  pub theta: Vec<RistrettoPoint>,
}

impl ClientStateInit {
  pub fn respond(self, message: Message1) -> (ClientState2, Message2) {
    let mut rng = OsRng;

    let m = self.Y.len();

    let beta: Vec<Scalar> = (0..m).map(|_| Scalar::random(&mut rng)).collect();

    let theta: Vec<RistrettoPoint> = self
      .Y
      .iter()
      .zip(beta.iter())
      .map(|(y, b)| hash_to_group(y) * b)
      .collect();

    let new_message = Message2 {
      theta: theta.clone(),
    };
    let new_state = ClientState2 {
      Y: self.Y,
      L: message.L,
      R: message.R,
      beta,
      theta,
    };

    (new_state, new_message)
  }
}

impl ClientState2 {
  pub fn complete(self, message: Message3) -> Vec<Element> {
    assert!(
      verify(self.clone(), message.clone()),
      "The verify algorithm failed, aborting"
    );
    let I: Vec<Element> = message
      .T
      .iter()
      .zip(self.beta.iter())
      .map(|(t, b)| self.L.contains(&hash_from_group(&(t * b.invert()))))
      .zip(self.Y.iter())
      .filter_map(|(i, y)| if i { Some(y.clone()) } else { None })
      .collect();

    I
  }
}
