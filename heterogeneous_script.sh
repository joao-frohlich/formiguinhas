#!/bin/bash

for i in {01..99}
do
    for j in {01..99}
    do
        echo 0.$i 0.$j
        cargo run --example formiguinhas_heterogeneas --release 0.$i 0.$j
    done
    echo 0.$i 1.0
    cargo run --example formiguinhas_heterogeneas --release 0.$i 1.0
done
echo 1.0 1.0
cargo run --example formiguinhas_heterogeneas --release 1.0 1.0