# tipbox

A small NEAR smart contract for tipping. Fans attach NEAR to `tip`, the creator's balance inside the contract grows, and the creator calls `withdraw` to move it to their account.

## Methods

| Method | Kind | What it does |
| --- | --- | --- |
| `new()` | init | creates an empty tip box |
| `tip(creator)` | payable | credits the attached NEAR to `creator` |
| `balance_of(account)` | view | yoctoNEAR the account can withdraw |
| `total_tips()` | view | all NEAR ever tipped |
| `withdraw(amount)` | call | pays `amount` yoctoNEAR to the caller |

## Build and test

```sh
cargo test
cargo near build non-reproducible-wasm
```

## Deploy to testnet

```sh
near deploy <your-account>.testnet target/near/tipbox.wasm --initFunction new --initArgs '{}'
near call <your-account>.testnet tip '{"creator":"alice.testnet"}' --accountId bob.testnet --deposit 1
near call <your-account>.testnet withdraw '{"amount":"1000000000000000000000000"}' --accountId alice.testnet
```
