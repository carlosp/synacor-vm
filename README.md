# Synacor challenge

This project is an implementation of the [Synacor OSCON 2012 challenge](https://challenge.synacor.com) as a learning exercise for the Rust programming language.

The Synacor challenge is a programming puzzle that involves implementing a virtual machine capable of executing a set of instructions and solving a series of puzzles along the way. The challenge domain is currently offline so a copy of the material such as the one found on [Aneurysm9/vm_challenge](https://github.com/Aneurysm9/vm_challenge) is needed.

## Contents

The project contains the following binaries:
* `debug`: runs the VM and provides simple debugging commands to play around and automatize the solution.
* `disassemble`: translates the binary into a readable assembly representation.
* `generate-graph`: generates the Graphviz DOT representation of the different locations, their connections and items on each.
* `solve-teleporter-puzzle`: solver for the setting needed for the teleporter puzzle.
* `solve-vault-puzzle`: solver for the last puzzle to find the way to enter the vault.

It also contains the inputs needed to get each of the 8 codes.

## Usage

Compile and run the binary using:

```
cargo run --release --bin $bin_name
```

Alternatively, if it's already been compiled, just run it with:

```
./target/release/$bin_name
```
