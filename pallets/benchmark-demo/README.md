# Benchmarking Demo

## Run

```shell
cd node

cargo build --release --features runtime-benchmarks

# benchmark our demo
cd ..
./target/release/node-template benchmark --chain dev --execution=wasm --wasm-execution=compiled --pallet pallet_benchmark_demo --extrinsic do_something --steps 20 --repeat 50
```

## Resources

[Substrate Benchmarking Documentation by Shawn](https://www.shawntabrizi.com/substrate-graph-benchmarks/docs/#/)