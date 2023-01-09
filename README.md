# Soroban Analyzer
### CLI tool to detect gas inefficiencies in soroban Rust code.

#### Unstable, still under development.

`soroban-analyzer` detects gas inefficiencies in Soroban Rust code. Since soroban hasn't reached a stable version yet there aren't much guidelines on budget and metering, this tool currently works on detecting common mistakes that are gas inefficient also in Solidity.

Currently detects:
- loops that change or access contract storage.
- functions that access directly or indirectly contract storage that are used multiple times within a block.

Mainly, the tool targets inefficiencies that are related to storage, which is generally more expensive than updating memory. The tool is contract-centered, meaning that it only suggests changes to improve gas efficiencies, not general programming concepts.

## Usage
Currently only single-file contracts are supported, but we plan on adding support for structured projects soon. To install `soroban-analyzer`, clone the repo, then `cd` into the repo's directory and run:

```bash
cargo build -p soroban-analyzer --release
```

You can then try out the tool on the example contract in `test_input/foo.rs`:

```bash
$> ./target/release/soroban-analyzer --p ./test_input/foo.rs

[+] Soroban Analyzer started. Disclaimer: still under development, the tool's scope is currently very limited, expect bugs and breaking changes may occur. 
Report bugs or suggestions in the github issues page: https://github.com/xycloo/soroban-analyzer/issues.


 [DEBUG] Functions found directly or indirectly accessing contract state: 

> test_s at line 6
> get_test at line 10
> test_s_loop at line 14
> hello at line 26
> test at line 39


 [WARNING] Loops that access state: 

[-] Line 15: loop accesses contract state, it could lead to breaking the budget as state functions are more expensive. Make sure you trust the range and that accessing or modifying the state within the loop is necessary. 


[-] Line 31: loop accesses contract state, it could lead to breaking the budget as state functions are more expensive. Make sure you trust the range and that accessing or modifying the state within the loop is necessary. 


 [WARNING] Blocks that use state functions multiple times: 

[-] The function `test_s` defined at line 6 accessed contract state and is used multiple times inside the block between lines 26 and 37. It may be better to use `test_s` once and save it in memory. 


[-] The function `get_test` defined at line 10 accessed contract state and is used multiple times inside the block between lines 39 and 43. It may be better to use `get_test` once and save it in memory. 

```

## Contributing
If you encounter an unexpected bug or would like to suggest other checks, open an issue on this repo.

## What's next
- Add more checks.
- Start detecting security vulnerabilities.
