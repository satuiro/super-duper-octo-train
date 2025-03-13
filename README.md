# Rust Blockchain Implementation

A simple blockchain implementation written in Rust that demonstrates the core concepts of blockchain technology including blocks, transactions, proof-of-work mining, and blockchain validation.

## Features

- Basic blockchain structure with blocks linked by cryptographic hashes
- Transaction system for recording transfers between parties
- Proof-of-work mining with adjustable difficulty
- Chain validation to ensure integrity
- Tamper detection demonstration

## Requirements

- Rust (stable)
- Dependencies:
  - `chrono` - For timestamp handling
  - `rust-crypto` - For SHA-256 hashing

## Installation

Clone the repository:

```bash
git clone https://github.com/satuiro/super-duper-octo-train.git
cd super-duper-octo-train
```

Add the required dependencies to your `Cargo.toml`:

```toml
[dependencies]
chrono = "0.4"
rust-crypto = "0.2"
```

Build the project:

```bash
cargo build
```

## Usage

Run the example provided in `main.rs`:

```bash
cargo run
```

The example:
1. Creates a new blockchain with a genesis block
2. Verifies the initial blockchain validity
3. Adds a new block with two transactions
4. Verifies the blockchain is still valid
5. Simulates tampering with transaction data
6. Shows how the blockchain validation detects the tampering

## How It Works

### Block Structure

Each block contains:
- Index - The position in the blockchain
- Timestamp - When the block was created
- Transactions - A list of transfers between parties
- Previous Hash - Hash of the previous block (creates the chain)
- Hash - Hash of the current block's contents
- Nonce - Value used in the mining process

### Mining

The implementation uses a proof-of-work algorithm that:
1. Takes block data and creates a hash
2. Continues incrementing a nonce value until finding a hash with a specified number of leading zeros
3. The difficulty parameter determines how many leading zeros are required

### Validation

The blockchain is validated by:
1. Ensuring each block references the previous block's hash correctly
2. Recalculating each block's hash to verify it hasn't been tampered with
