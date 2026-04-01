use crate::client_t6::Message2;
use crate::protocol::{hash_from_group, hash_to_group, prove};
use crate::types::{Bytes, Element};

use curve25519_dalek::constants::RISTRETTO_BASEPOINT_POINT;
use curve25519_dalek::ristretto::RistrettoPoint;
use curve25519_dalek::scalar::Scalar;
use rand::{rngs::OsRng, seq::SliceRandom};

pub struct ServerStateInit {
  pub X: Vec<Element>,
}

#[derive(Clone)]
pub struct ServerState1 {
  pub X: Vec<Element>,
  pub alpha: Scalar,
  pub omega: Vec<Bytes<32>>,
  pub L: Vec<Bytes<32>>,
  pub R: RistrettoPoint,
}

pub struct Message1 {
  pub L: Vec<Bytes<32>>,
  pub R: RistrettoPoint,
}

impl ServerStateInit {
  pub fn start(self) -> (ServerState1, Message1) {
    let mut rng = OsRng;

    let alpha = Scalar::random(&mut rng);
    let R: RistrettoPoint = RISTRETTO_BASEPOINT_POINT * alpha;
    let omega: Vec<Bytes<32>> = self
      .X
      .iter()
      .map(|x| hash_from_group(&(hash_to_group(x) * alpha)))
      .collect();

    let L: Vec<Bytes<32>> = {
      let mut tmp = omega.clone();
      tmp.shuffle(&mut rng);
      tmp
    };

    let new_message = Message1 {
      L: L.clone(),
      R: R.clone(),
    };
    let new_state = ServerState1 {
      X: self.X,
      alpha,
      omega,
      L,
      R,
    };

    (new_state, new_message)
  }
}

pub struct ServerState3 {
  pub X: Vec<Element>,
  pub alpha: Scalar,
  pub omega: Vec<Bytes<32>>,
  pub L: Vec<Bytes<32>>,
  pub theta: Vec<RistrettoPoint>,
  pub T: Vec<RistrettoPoint>,
}

pub struct Message3 {
  pub T: Vec<RistrettoPoint>,
  pub pi: (RistrettoPoint, Vec<RistrettoPoint>, Scalar),
}

impl ServerState1 {
  pub fn respond(self, message: Message2) -> (ServerState3, Message3) {
    let T: Vec<RistrettoPoint> =
      message.theta.iter().map(|t| t * self.alpha).collect();
    let pi: (RistrettoPoint, Vec<RistrettoPoint>, Scalar) =
      prove(self.clone(), message.clone());

    let new_message = Message3 {
      T: T.clone(),
      pi: pi.clone(),
    };
    let new_server_state = ServerState3 {
      X: self.X,
      alpha: self.alpha,
      omega: self.omega,
      L: self.L,
      theta: message.theta,
      T,
    };

    (new_server_state, new_message)
  }
}
