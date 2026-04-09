#!/bin/sh

git switch main
cargo build --release --bin benchmark

for i in {1..100}
do
  target/release/benchmark $i
done

git switch task4
cargo build --release --bin benchmark

target/release/benchmark 200

git switch task6
cargo build --release --bin benchmark

for i in {1..10}
do
  target/release/benchmark $i
done

git switch task6_malicious
cargo build --release --bin benchmark
target/release/benchmark 200

git switch main
