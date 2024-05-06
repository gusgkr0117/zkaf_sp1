## Installation

### Docker

#### Build the Image

```shell
docker build . -t zkaf_sp1:latest
```

#### SSH into Image

```shell
docker run -v "$PWD:/home/appuser/program" -it zkaf_sp1:latest
```

### Build a program

Reference : [https://succinctlabs.github.io/sp1/writing-programs/setup.html](https://succinctlabs.github.io/sp1/writing-programs/setup.html)

```bash
cd program
cargo prove build 
```

### Verification

```bash
cd ../script
RUST_LOG=info cargo run --release
```

## Notes(Issues)

When you try to develop large programs(> 1 million cycles), you do not generate proofs each time. Instead, you should have your script only execute the program with RISC-V runtime and read `public_values`.

Currently there is a problem with generating a portable proof since the proof size is too big(>1 GB). If you try to generate the proof, you will see the program terminates with the sole message "killed".

Reference : [https://succinctlabs.github.io/sp1/generating-proofs/advanced.html](https://succinctlabs.github.io/sp1/generating-proofs/advanced.html)