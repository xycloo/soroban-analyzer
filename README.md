# Soroban Analyzer
### CLI tool to detect gas inefficiencies in soroban Rust code.

#### Unstable, still under development.

`soroban-analyzer` detects gas inefficiencies in Soroban Rust code. Since soroban hasn't reached a stable version yet there aren't much guidelines on budget and metering, this tool currently works on detecting common mistakes that are gas inefficient also in Solidity.

Currently detects:
- loops that indirectly change or access contract storage.
- functions that access indirectly contract storage that are used multiple times within a block.

By indirectly, we mean that `soroban-analyzer` will only throw alerts when the storage is accessed through a function that accesses it directly thorugh `environment.storage()` (directly or indireclty). This is since the tool assumes that when there is a direct statement the developer already konws what they are doing. When `soroban-analyzer` only checks for indirect storage access it is because the developer may not be aware that they are unnededly modifying/getting storage.

Mainly, the tool targets inefficiencies that are related to storage, which is generally more expensive than updating memory. The tool is contract-centered, meaning that it only suggests changes to improve gas efficiencies, not general programming concepts.

# Installation
Clone the repo, then `cd` into the repo's directory and build the executable:

```bash
git clone https://github.com/xycloo/soroban-analyzer ; cd soroban-analyzer ; cargo build -p soroban-analyzer --release
```

You can also create an alias to try out the tool before moving it to your PATH:

```bash
$ alias soroban-analyzer="/path/to/soroban-analyzer/target/release/soroban-analyzer"
```

# Usage

### Single file

You can try out the tool on an example contract (`test_input/foo.rs`):

```bash
$ soroban-analyzer --p ./test_input/foo.rs
```

### Project

You can also check for gas inefficiencies project-wide by running the following command inside the project's directory:

```
$ soroban-analyzer --all
```

## Contributing
If you encounter an unexpected bug or would like to suggest other checks, open an issue on this repo.

## Credits
The package relies on a slightly modified version of [rust-code-analysis](https://github.com/mozilla/rust-code-analysis/).

## What's next
- Add more checks.
- Start detecting security vulnerabilities.
