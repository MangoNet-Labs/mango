# Flash Lender Module Example

This document provides an overview of a Mgo Move module named `flash_lender::example`. The module defines a flash lending mechanism for any type of `Coin`, allowing users to borrow and repay funds within the same transaction. It includes several key components and functions for managing the lending process.

## Module Overview

### 1. FlashLender Struct
The `FlashLender` struct is a shared object that allows users to borrow funds. It contains the following fields:
- **id**: A unique identifier for the flash lender.
- **to_lend**: The amount of funds available for lending.
- **fee**: The flat fee charged for borrowing funds.

### 2. Receipt Struct
The `Receipt` struct records the details of a loan transaction, including:
- **flash_lender_id**: The ID of the lender from which the funds were borrowed.
- **repay_amount**: The total amount that must be repaid, including the fee.

### 3. AdminCap Struct
The `AdminCap` struct grants administrative control over a `FlashLender`. The holder of an `AdminCap` can withdraw, deposit, and modify the lending parameters.

## Public Functions

### 1. `new`
**`new<T>(to_lend: Balance<T>, fee: u64, ctx: &mut TxContext): AdminCap`**  
Creates a new `FlashLender` instance, setting the initial funds and fee. It returns an `AdminCap` for managing the lender.

### 2. `loan`
**`loan<T>(self: &mut FlashLender<T>, amount: u64, ctx: &mut TxContext): (Coin<T>, Receipt<T>)`**  
Requests a loan of the specified `amount` from the lender. Returns the borrowed `Coin` and a `Receipt` to ensure the loan is repaid within the same transaction.

### 3. `repay`
**`repay<T>(self: &mut FlashLender<T>, payment: Coin<T>, receipt: Receipt<T>)`**  
Repays the loan recorded in the `Receipt` using the `payment`. Aborts if the repayment amount or lender is incorrect.

## Accessor Functions

### 1. `fee`
**`fee<T>(self: &FlashLender<T>): u64`**  
Returns the current fee for the flash loan.

### 2. `max_loan`
**`max_loan<T>(self: &FlashLender<T>): u64`**  
Returns the maximum amount available for borrowing from the lender.

### 3. `repay_amount`
**`repay_amount<T>(self: &Receipt<T>): u64`**  
Returns the total amount that must be repaid according to the `Receipt`.

### 4. `flash_lender_id`
**`flash_lender_id<T>(self: &Receipt<T>): ID`**  
Returns the ID of the `FlashLender` object that issued the loan.

## Admin Functions

### 1. `withdraw`
**`withdraw<T>(self: &mut FlashLender<T>, admin: &AdminCap, amount: u64, ctx: &mut TxContext): Coin<T>`**  
Allows the `AdminCap` holder to withdraw funds from the lender.

### 2. `deposit`
**`deposit<T>(self: &mut FlashLender<T>, admin: &AdminCap, coin: Coin<T>)`**  
Allows the `AdminCap` holder to deposit additional funds into the lender.

### 3. `update_fee`
**`update_fee<T>(self: &mut FlashLender<T>, admin: &AdminCap, new_fee: u64)`**  
Allows the `AdminCap` holder to update the lending fee.

## Error Codes

The module defines several error codes for various failure conditions:
- **`ELoanTooLarge`**: The loan amount exceeds the lender's available funds.
- **`EInvalidRepaymentAmount`**: The repayment amount does not match the required amount.
- **`ERepayToWrongLender`**: The repayment is attempted to the wrong lender.
- **`EAdminOnly`**: An admin-only operation was attempted without valid permissions.
- **`EWithdrawTooLarge`**: The withdrawal amount exceeds the lender's available funds.

## Tests

The module includes a test function `test_flash_loan` to verify the functionality of the flash lending process:
1. **Admin creates a flash lender** with initial funds and a fee.
2. **Alice requests and repays a loan**, ensuring the fee is added to the lender's balance.
3. **Admin withdraws the profit** made from the fee, verifying the lender's balance and the withdrawal process.

This module demonstrates a simple implementation of a flash lending system with comprehensive administrative control and error handling.