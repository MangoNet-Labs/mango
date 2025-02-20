# Vesting Smart Contract

## Overview
This module implements a linear vesting strategy that allows users to claim coins over time in a controlled manner. The smart contract ensures that coins are gradually released according to a predefined schedule, preventing immediate full access and enabling vesting compliance.

## Features
- **Linear Vesting Schedule:** Coins are released proportionally over a specified duration.
- **Wallet Management:** Users can create a vesting wallet that holds their coins and claim the available portion over time.
- **Claiming Mechanism:** Users can claim the vested amount at any time, with the smart contract computing the claimable amount based on elapsed time.
- **Validation:** Ensures that vesting starts in the future to prevent instant claiming.
- **Deletion:** Allows removal of empty wallets to free up storage.

## Smart Contract Details

### Structs
#### `Wallet<T>`
Represents a wallet that holds coins subject to vesting.
- `id`: Unique identifier.
- `balance`: The total balance of the wallet.
- `start`: The timestamp when vesting starts.
- `claimed`: The amount of coins already claimed.
- `duration`: Total vesting duration.

### Errors
- `EInvalidStartTime`: Raised if vesting start time is not in the future.

### Functions
#### `new_wallet<T>(coins: Coin<T>, clock: &Clock, start: u64, duration: u64, ctx: &mut TxContext) -> Wallet<T>`
Creates a new vesting wallet.
- **Aborts with** `EInvalidStartTime` if the start time is in the past.

#### `claim<T>(self: &mut Wallet<T>, clock: &Clock, ctx: &mut TxContext) -> Coin<T>`
Allows the user to claim the portion of coins that have vested.

#### `claimable<T>(self: &Wallet<T>, clock: &Clock) -> u64`
Calculates the amount of coins that can be claimed at the current time.

#### `delete_wallet<T>(self: Wallet<T>)`
Deletes the wallet if it has no remaining balance.

#### Accessors
- `balance<T>(self: &Wallet<T>) -> u64`: Returns the remaining balance in the wallet.
- `start<T>(self: &Wallet<T>) -> u64`: Returns the start time of the vesting schedule.
- `duration<T>(self: &Wallet<T>) -> u64`: Returns the total duration of the vesting schedule.

## Usage Example
```move
let wallet = vesting::linear::new_wallet(coins, &clock, start_time, duration, &mut ctx);
let claimable_amount = wallet.claimable(&clock);
let claimed_coins = wallet.claim(&mut clock, &mut ctx);
```

## License
This project is licensed under the Apache 2.0 License. See the [LICENSE](LICENSE) file for details.
