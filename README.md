# Private Set Intersection

A Rust implementation of a basic **Private Set Intersection (PSI)** protocol over the **Ristretto group** using `curve25519-dalek`.

## How the protocol works

### Server side

The server:

1. samples a random scalar `alpha`
2. computes `omega_i = H'(alpha * H(x_i))` for all `x_i in X`
3. shuffles the resulting token list `L`
4. later computes `T_j = alpha * theta_j` for each blinded client point

### Client side

The client:

1. receives `L`
2. samples random blinding factors `beta_j`
3. computes `theta_j = beta_j * H(y_j)`
4. receives `T_j`
5. unblinds each value with `beta_j^{-1}`
6. hashes the unblinded point and checks whether it is contained in `L`

## Local setup

### Requirements

- Rust toolchain with Cargo
- A recent stable compiler that supports **edition 2024**

## Run the existing benchmark examples

```bash
scripts/run_locally.sh
```

This benchmark script will automatically switch git branches, compile the correct
binary, and run the binary over its first 100 test cases before switching to the next
branch and repeating the process. For task 4, it only runs the binary once to 
demonstrate that the intersection fails.

For task 6, to see that the verify algorithm outputs 0 if alpha is changed,
uncomment the code that generates `alpha_prime` in `server_t6.rs` and change 
all references to `self.alpha` to `alpha_prime` in that function. Re-running
the benchmark will show that the verify algorithm outputs 0 and the protocol stops.
