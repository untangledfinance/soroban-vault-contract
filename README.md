# Untangled Vault

This Rust crate implements a contract for managing token offers and facilitating trades between tokens. This README provides an overview of the contract's main functions.

## Overview

The contract allows sellers to create an offer to sell one type of token (e.g., XLM) in exchange for another type of token (e.g., USDC). Buyers can deposit funds to buy the offered tokens, and the seller can withdraw any remaining balance. The price is defined by a fixed ratio between the two tokens.

## Project Structure

```text
.
├── contracts
│   └── vault
│       ├── src
│       │   ├── lib.rs
│       │   └── test.rs
│       └── Cargo.toml
├── Cargo.toml
└── README.md
```

## Overall Architecture

![overall architecture](./img/StellarVault.drawio.png)

## Functions

### 1. `create(seller: Address, treasury: Address, sell_token: Address, buy_token: Address, sell_price: u32, buy_price: u32)`

- **Purpose:** Create an offer for selling one token for another.
- **Parameters:**
  - `seller`: The address of the seller who creates the offer.
  - `treasury`: The address where funds will be sent (typically a wallet or smart contract).
  - `sell_token`: The token being offered for sale.
  - `buy_token`: The token to be received in exchange for the sell token.
  - `sell_price`: The price of one unit of `sell_token` in terms of `buy_token`.
  - `buy_price`: The price of one unit of `buy_token` in terms of `sell_token`.
- **Restrictions:**
  - The offer is not allowed to already exist.
  - Neither the `sell_price` nor the `buy_price` can be zero.

### 2. `deposit(buyer: Address, buy_token_amount: i128, min_sell_token_amount: i128)`

- **Purpose:** Allow buyers to deposit `buy_token` and receive corresponding `sell_token`.
- **Parameters:**
  - `buyer`: The address of the buyer who will perform the deposit.
  - `buy_token_amount`: The amount of tokens that the buyer is willing to provide.
  - `min_sell_token_amount`: The minimum amount of `sell_token` they expect in return.
- **Restrictions:**
  - The buyer must authorize this call and the internal token transfer to the contract.

### 3. `claim(e: Env, token: Address, amount: i128)`

- **Purpose:** Allow sellers to withdraw any remaining balance of tokens from the contract.
- **Parameters:**
  - `token`: The address of the token being withdrawn (either `sell_token` or `buy_token`).
  - `amount`: The amount of tokens to be withdrawn.
- **Restrictions:**
  - This function must be authorized by the seller.

### 4. `updt_price(e: Env, sell_price: u32, buy_price: u32)`

- **Purpose:** Update the price of the offer.
- **Parameters:**
  - `sell_price`: The new selling price (in terms of `buy_token`).
  - `buy_price`: The new buying price (in terms of `sell_token`).
- **Restrictions:**
  - Neither the `sell_price` nor the `buy_price` can be zero.
  - This function must be authorized by the seller.

### 5. `get_offer(e: Env) -> Offer`

- **Purpose:** Retrieve the current state of the offer.
- **Returns:** The current `Offer` structure containing all relevant details about the offer.

### 6. `redeem(e: Env, receiver: Address, redeem_amount: i128, min_buy_token_amount: i128)`

- **Purpose:** Allow buyers to redeem `sell_token` for `buy_token`.
- **Parameters:**
  - `receiver`: The address that will receive the redeemed tokens.
  - `redeem_amount`: The amount of `sell_token` to be redeemed.
  - `min_buy_token_amount`: The minimum amount of `buy_token` they expect in return.
- **Restrictions:**
  - The receiver must authorize this call.

## Dependencies

The contract depends on the following crates:

- `soroban_sdk`: Provides the necessary environment and utilities for Soroban smart contracts.

## Usage

1. **Create Offer:** Use the `create` function to initiate a sale.
2. **Deposit Tokens:** Buyers can deposit tokens by calling the `deposit` function.
3. **Update Price:** Sellers may adjust the price using the `updt_price` function if needed.
4. **Withdraw Funds:** Sellers can withdraw any remaining balance with the `claim` function.
5. **Redeem Tokens:** Buyers can redeem their tokens for the agreed-upon amount via the `redeem` function.

## Testing

For testing, you should refer to the `test.rs` file located in the `mod test;` section of the codebase.

---

This contract is designed to be secure and transparent. It ensures that all operations are authorized and that no party can perform actions without proper authorization. Proper use of this contract requires understanding of both Soroban SDK and token transfers.
