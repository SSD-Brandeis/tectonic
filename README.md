<div align="center">
  <img width="77" height="64" alt="tectonic logo: three database cylinders shifting like tectonic plates" src="https://github.com/user-attachments/assets/47851a1a-0b6a-4ad7-b7e0-b0e1fe08c52b" />
  <h1>Tectonic</h1>
</div>

## Building

<https://www.rust-lang.org/tools/install>

```bash
cargo build --release
./target/release/tectonic
# or
cargo run --release
```

## Usage

```bash
./tectonic-cli schema > workload_schema.json

./tectonic-cli generate -w workload_spec.json
# or
./tectonic-cli generate -w workload_spec.json -o workload_outputs/
# or
./tectonic-cli generate -w workload_specs/ -o workload_outputs/
```

````bash
Usage: tectonic-cli <COMMAND>

Commands:
  generate  Generate workload(s) from a file or folder of workload specifications
  schema    Prints the JSON schema for IDE integration
  help      Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version`
```


```bash
Usage: tectonic-cli generate [OPTIONS] --workload <WORKLOAD_PATH>

Options:
  -w, --workload <WORKLOAD_PATH>  File or folder of workload spec files
  -o, --output <OUTPUT>           Output file or folder for workload(s). Defaults to the same directory as the workload spec
  -h, --help                      Print help
```


## Profiling

```bash
cargo flamegraph --unit-test workload_gen -- workload_1m_i
```
````
