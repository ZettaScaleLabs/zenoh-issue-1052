# Experiments

## Vanilla

Here a slow queryable (takes 1 second to respond) is connected to 80 peers that query it 5 times;
once every 20 seconds.

### Instructions

1. Create a directory called `vanilla`
2. Build the binaries using `cargo build --release`
3. Lunch the slow queryable using `./target/release/vanilla_slow_queryable vanilla.config.json5`
4. Open a python3 REPl and run `import vanilla; procs = vanilla.spawn_procs(80)`
5. Observe peer execution using `tail -f vanilla/<number>.stdout`
6. Observe slow queryable execution in (3)

### Results

The slow queryable gradually has its channel filled up, until it can no longer reply to any queries.
But the Zenoh runtime does not enter a deadlock state; the queryable continues to progress until the
channel size goes back to 0.

#### System information

Ubuntu 22.04.4 LTS (Linux 5.15.0-25-generic), Intel(R) Xeon(R) CPU E5-2630 v4 @ 2.20GHz, 16G RAM
