# Micro-Eth 🦀⛓️

**Micro-Eth** is a lightweight, thread safe implementation of a blockchain state machine written in Rust. It serves as a study of the low-level primitives used in production Ethereum clients(ie Reth and Geth), focusing on safe concurrency, shared mutable state, and cryptographic primitives.

## How to Run

### Prereqs

```bash
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

### Installation & Execution

```bash
git clone https://github.com/kmarulab/micro-eth.git
cd micro-eth
cargo run
```
