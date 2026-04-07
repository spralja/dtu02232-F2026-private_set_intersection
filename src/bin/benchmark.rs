use psi::threaded::run_threaded;
use psi::types::Element;
use psi::verify::verify_intersection;

use std::env;
use std::time::Instant;

use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use rand::{RngCore, SeedableRng};

const SALT: u64 = 0xb1bc77cc4ae2bd04;
const REPS: usize = 10;

const X_SIZES: [usize; 12] = [1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048];
const Y_SIZES: [usize; 12] = [1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048];
const I_SIZES: [usize; 12] = [1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048];


fn time_protocol(X: Vec<Element>, Y: Vec<Element>) -> f64 {
  let X_copy = X.clone();
  let Y_Copy = Y.clone();

  let start = Instant::now();

  let I = run_threaded(X, Y);

  let elapsed = start.elapsed();

  if !verify_intersection(&X_copy, &Y_Copy, &I) {
    println!("X: {:?}", X_copy);
    println!("Y: {:?}", Y_Copy);
    println!("Returned intersection: {:?}", I);
    panic!("PSI protocol returned incorrect intersection");
  }

  elapsed.as_secs_f64()
}

fn random_element(rng: &mut StdRng) -> Element {
  let mut element = [0u8; 64];
  rng.fill_bytes(&mut element);

  element
}

fn random_vector(size: usize, rng: &mut StdRng) -> Vec<Element> {
  (0..size).map(|_| random_element(rng)).collect()
}

fn shuffle_together(
  a: Vec<Element>,
  b: Vec<Element>,
  rng: &mut StdRng,
) -> Vec<Element> {
  let mut v: Vec<Element> = a.into_iter().chain(b.into_iter()).collect();

  v.shuffle(rng);

  v
}

fn config(index: usize) -> (usize, usize, usize) {
  let mut v: Vec<(usize, usize, usize)> = Vec::new();

  for &x_size in &X_SIZES {
    for &y_size in &Y_SIZES {
      for &i_size in &I_SIZES {
        if i_size <= x_size && i_size <= y_size && x_size >= y_size {
          v.push((x_size, y_size, i_size));
        }
      }
    }
  }

  v[index - 1]
}

fn main() {
  let NUM_CONFIGS: usize = {
    let mut n = 0;
    for &x_size in &X_SIZES {
      for &y_size in &Y_SIZES {
        for &i_size in &I_SIZES {
          if i_size <= x_size && i_size <= y_size && x_size <= y_size {
            n += 1;
          }
        }
      }
    }

    n
  };

  // the X, Y, I sets are based on seeded arguments, but the protocol is not!
  let args: Vec<String> = env::args().collect();

  if args.len() != 2 {
    eprintln!("Usage: benchmark <LSF_INDEX>");
    std::process::exit(1);
  }

  let index: usize = args[1].parse().expect("invalid LSF_INDEX");

  assert!(index >= 1 && index <= NUM_CONFIGS, "index must be in [1, {}]", NUM_CONFIGS);

  let (x_size, y_size, i_size) = config(index);

  let seed: u64 = SALT ^ index as u64;

  let mut rng = StdRng::seed_from_u64(seed);

  let I = random_vector(i_size, &mut rng);

  let X = shuffle_together(
    I.clone(),
    random_vector(x_size - i_size, &mut rng),
    &mut rng,
  );

  let Y = shuffle_together(
    I.clone(),
    random_vector(y_size - i_size, &mut rng),
    &mut rng,
  );

  let times: Vec<f64> = (0..REPS)
    .map(|_| time_protocol(X.clone(), Y.clone()))
    .collect();

  let mean = times.iter().sum::<f64>() / REPS as f64;

  let variance =
    times.iter().map(|t| (t - mean) * (t - mean)).sum::<f64>() / REPS as f64;

  let mean_ms = (mean * 1000.0).round() as u64;
  let stddev_ms = (variance.sqrt() * 1000.0).round() as u64;

  println!(
    "{},{},{},{},{},{}",
    index, x_size, y_size, i_size, mean_ms, stddev_ms
  );
}
