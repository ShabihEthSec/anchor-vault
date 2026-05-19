# Anchor Vault

A secure SOL vault program built with Anchor on Solana.

This project demonstrates PDA-based vault architecture for securely storing and managing SOL using Solana smart contracts.

Repository: [anchor-vault](https://github.com/ShabihEthSec/anchor-vault?utm_source=chatgpt.com)

---

## Overview

The vault program allows users to:

- Initialize a personal vault
- Deposit SOL
- Withdraw SOL
- Close the vault account

The implementation uses:

- Program Derived Addresses (PDAs)
- Native SOL transfers
- Anchor account constraints
- Secure signer validation

---

## Features

### Initialize Vault

Creates:

- Vault state PDA
- Vault PDA

The vault PDA becomes the secure storage account for SOL deposits.

### Deposit SOL

Users can deposit SOL into the vault securely through CPI calls to the System Program.

### Withdraw SOL

Authorized users can withdraw SOL from the vault.

### Close Vault

Closes:

- Vault account
- Vault state account

Returns remaining lamports and rent exemption balance to the owner.

---

## Architecture

### PDA Structure

#### Vault State PDA

Stores:

- authority
- bump seeds

Derived using:

```rust id="qqgcrj"
[b"state", user.key().as_ref()]
```

#### Vault PDA

Stores deposited SOL.

Derived using:

```rust id="obnlvf"
[b"vault", vault_state.key().as_ref()]
```

---

## Program Flow

```txt id="0nq5f0"
User
 ├── Initialize Vault
 │     ├── Create Vault State PDA
 │     └── Create Vault PDA
 │
 ├── Deposit SOL
 │     └── Transfer SOL → Vault PDA
 │
 ├── Withdraw SOL
 │     └── Transfer SOL ← Vault PDA
 │
 └── Close Vault
       ├── Return remaining SOL
       └── Close accounts
```

---

## Tech Stack

- Rust
- Solana
- Anchor Framework
- TypeScript
- Mocha Tests

---

## Project Structure

```txt id="0brhmd"
.
├── programs/
│   └── anchor-vault/
│       └── src/
│           └── lib.rs
├── tests/
│   └── anchor-vault.ts
├── migrations/
├── Anchor.toml
├── Cargo.toml
├── package.json
└── tsconfig.json
```

---

## Local Development

### Prerequisites

Install:

- Rust
- Solana CLI
- Anchor
- Node.js
- Yarn

Verify versions:

```bash id="ykgj7u"
solana --version
anchor --version
rustc --version
node -v
```

---

## Installation

Clone repository:

```bash id="gk8j7u"
git clone https://github.com/ShabihEthSec/anchor-vault.git
cd anchor-vault
```

Install dependencies:

```bash id="yvh0fe"
yarn install
```

---

## Configure Program ID

Generate keypair:

```bash id="prg4yo"
anchor keys list
```

Update the generated program ID in:

### `Anchor.toml`

```toml id="70i1sl"
[programs.localnet]
anchor_vault = "YOUR_PROGRAM_ID"
```

### `lib.rs`

```rust id="kr36mw"
declare_id!("YOUR_PROGRAM_ID");
```

---

## Build

```bash id="z1fpwd"
anchor build
```

---

## Test

Run tests locally:

```bash id="4v3okh"
anchor test
```

The tests verify:

- vault initialization
- SOL deposits
- SOL withdrawals
- vault closure
- PDA validation

---

## Deployment

Start local validator:

```bash id="0y3e0q"
solana-test-validator
```

Deploy program:

```bash id="v3oh7m"
anchor deploy
```

---

## Security Considerations

### PDA Ownership

Vault accounts are owned by PDAs, preventing unauthorized withdrawals.

### Signer Validation

Only the vault authority can withdraw or close vault accounts.

### Rent Recovery

Closing accounts returns rent-exempt lamports to users.

### Deterministic Addressing

PDAs ensure deterministic and collision-resistant vault generation.

---

## Example Use Case

A user wants secure program-controlled custody for SOL.

Flow:

1. Initialize vault
2. Deposit SOL
3. Program stores funds securely
4. Withdraw later when needed
5. Close vault and reclaim rent

---

## Author

**Mohd Shabihul Hasan Khan**
GitHub: [ShabihEthSec](https://github.com/ShabihEthSec?utm_source=chatgpt.com)

Solana • Rust • Anchor • Smart Contract Security

---

## References

- [Anchor Framework](https://www.anchor-lang.com/?utm_source=chatgpt.com)
- [Solana Docs](https://solana.com/docs?utm_source=chatgpt.com)
- [SPL Documentation](https://spl.solana.com/?utm_source=chatgpt.com)

Vault architecture concepts inspired by standard Anchor PDA vault implementations. ([github.com][1])

---

## License

MIT
