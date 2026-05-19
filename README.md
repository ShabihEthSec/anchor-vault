# Anchor Vault Q2 2026

An Anchor vault program that lets each user create a SOL vault, deposit lamports, withdraw lamports, and close the vault to recover the remaining funds.

## Program

Program ID:

```text
24UpHQhbwrBNJCdVaSJ7HSBta3aAM2R9yn8yA4DUiRqu
```

Accounts:

- `vault_state`: PDA derived from `[b"state", user]`; stores the vault and state bumps.
- `vault`: System account PDA derived from `[b"vault", vault_state]`; holds deposited lamports.

Instructions:

- `initialize`: creates the user vault state and records PDA bumps.
- `deposit(amount)`: transfers lamports from the user into the vault. `amount` must be greater than zero.
- `withdraw(amount)`: transfers lamports from the vault back to the user. `amount` must be greater than zero and cannot exceed the vault balance.
- `close`: transfers all remaining vault lamports to the user and closes the vault state account.

## Testing

The test suite uses LiteSVM directly, so it runs the program in-process without a local RPC validator. `Anchor.toml` is configured so `anchor test` builds the SBF program first and then runs:

```bash
cargo test --test test_initialize -- --nocapture
```

Run the full workflow:

```bash
NO_DNA=1 anchor test
```

Run only the Rust LiteSVM test after rebuilding the SBF artifact:

```bash
NO_DNA=1 anchor build
cargo test --test test_initialize -- --nocapture
```

The LiteSVM test covers:

- `initialize`
- `deposit`
- rejected zero-amount deposit
- `withdraw`
- rejected over-withdraw
- `close`

## Dependencies

The workspace test crate and program crate use Anchor `1.0.2`, with LiteSVM `0.10.0` and Solana 3.x test-side crates. Keeping these versions aligned avoids duplicate Anchor/Solana type versions in the Rust test harness.

If your installed Anchor CLI is `1.0.0`, Anchor may print a warning because the crate is `1.0.2`. The program and tests still pass with the current setup; install or select Anchor CLI `1.0.2` if you want the warning removed.
