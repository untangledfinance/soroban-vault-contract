# Untangled Vault

This Rust crate implements a contract for managing token offers and facilitating trades between tokens. This README provides an overview of the contract's main functions.

## Overview

This Rust-based Soroban smart contract implements a token vault system that allows users to securely store tokens, create offers for token trading, and manage token redemption requests. The contract ensures secure and transparent operations, supporting features like deposits, withdrawals, price updates, and epoch-based settlements.

## Project Structure

```text
.
├── contracts
│   └── vault
│       ├── src
│       │   ├── vault.rs       # Main implementation of the Vault contract
│       │   ├── offer.rs       # Module for managing offers
│       │   ├── redeem_request.rs # Module for handling redemption requests
│       │   ├── storage_types.rs  # Definitions for storage keys and data structures
│       │   └── test.rs        # Unit tests for the Vault contract
│       └── Cargo.toml         # Rust package configuration for the Vault contract
├── Cargo.toml                 # Root Rust package configuration
└── README.md                  # Project documentation
```

## Overall Architecture

![overall architecture](./img/StellarVault.drawio.png)

## Overview

The Vault contract provides the following key functionalities:

- **Token Vault Management:** Securely store and manage tokens.
- **Token Trading Offers:** Create, update, and manage token trading offers.
- **Redemption Requests:** Handle token redemption requests and settlements.
- **Epoch Settlements:** Manage epochs for redeeming tokens and transferring assets.

## Functions

### 1. `initialize`

**Purpose:** Initialize the vault with an offer.

**Parameters:**

- `e: Env` - The contract environment.
- `seller: Address` - The address of the seller creating the offer.
- `treasury: Address` - The address where funds will be sent.
- `sell_token: Address` - The token being offered for sale.
- `buy_token: Address` - The token to be received in exchange.
- `initial_price: u32` - The price of one unit of `sell_token` in terms of `buy_token`, with 6 decimals precision.

**Restrictions:**

- The offer must not already exist.
- Prices cannot be zero.
- The seller must authorize the initialization.

**Event Emitted:**

- **`vault_initialized`**
  - Parameters: `(seller, treasury, sell_token, buy_token, initial_price)`

---

### 2. `deposit`

**Purpose:** Deposit the `buy_token` and receive the `sell_token`.

**Parameters:**

- `e: Env` - The contract environment.
- `buyer: Address` - The address of the buyer depositing tokens.
- `buy_token_amount: i128` - The amount of `buy_token` to deposit.
- `min_sell_token_amount: i128` - The minimum amount of `sell_token` the buyer expects to receive.

**Restrictions:**

- The buyer must authorize the deposit.
- The calculated `sell_token` amount must meet the minimum specified.

**Event Emitted:**

- **`vault_deposit`**
  - Parameters: `(buyer, treasury, buy_token_amount, sell_token_amount)`

---

### 3. `claim_leftover`

**Purpose:** Claim leftover tokens remaining in the vault.

**Parameters:**

- `e: Env` - The contract environment.
- `token: Address` - The token to claim.
- `amount: i128` - The amount of tokens to claim.

**Restrictions:**

- Only the seller can claim leftovers.

**Event Emitted:**

- **`vault_claim_leftover`**
  - Parameters: `(seller, token, amount)`

---

### 4. `updt_price`

**Purpose:** Update the price of the `sell_token`.

**Parameters:**

- `e: Env` - The contract environment.
- `new_price: u32` - The new price of the `sell_token` in terms of `buy_token`.

**Restrictions:**

- The seller must authorize the price update.
- The new price cannot be zero.

**Event Emitted:**

- **`vault_update_price`**
  - Parameters: `(seller, new_price)`

---

### 5. `redeem_request`

**Purpose:** Submit a redemption request to redeem shares.

**Parameters:**

- `e: Env` - The contract environment.
- `sender: Address` - The address of the user submitting the request.
- `amount: i128` - The amount of shares to redeem.

**Restrictions:**

- The sender must authorize the request.
- The shares will be transferred to the vault.

**Event Emitted:**

- **`vault_redeem_request`**
  - Parameters: `(sender, amount, new_redeem_amount, total_redeem_amount)`

---

### 6. `cancel_request`

**Purpose:** Cancel a redemption request and return the shares to the sender.

**Parameters:**

- `e: Env` - The contract environment.
- `sender: Address` - The address of the user canceling the request.

**Restrictions:**

- The sender must authorize the cancellation.
- The request must exist and have a positive share amount.

**Event Emitted:**

- **`vault_cancel_request`**
  - Parameters: `(sender, shares_amount, new_total_redeem)`

---

### 7. `claim_request`

**Purpose:** Claim the redeemed tokens after the epoch is settled.

**Parameters:**

- `e: Env` - The contract environment.
- `sender: Address` - The address of the user claiming the request.

**Restrictions:**

- The sender must authorize the claim.
- The epoch must be settled before the claim can be processed.

**Event Emitted:**

- **`vault_claim_request`**
  - Parameters: `(sender, shares_amount, buy_token_amount)`

---

### 8. `setle_epoch`

**Purpose:** Settle the current epoch and transfer the `buy_token` to the treasury.

**Parameters:**

- `e: Env` - The contract environment.

**Restrictions:**

- Only the treasury can authorize the settlement.

**Event Emitted:**

- **`vault_setle_epoch`**
  - Parameters: `(treasury, epoch_id, total_redeem, total_asset, price)`

---

### 9. `get_offer`

**Purpose:** Retrieve the current offer details.

**Parameters:**

- `e: Env` - The contract environment.

**Returns:** The `Offer` structure containing all relevant details.

**Event Emitted:**  
No events emitted.

---

### 10. `get_request`

**Purpose:** Retrieve the redemption request for a specific user.

**Parameters:**

- `e: Env` - The contract environment.
- `sender: Address` - The address of the user.

**Returns:** The `RedeemRequest` structure for the user.

**Event Emitted:**  
No events emitted.

---

### 11. `get_epoch_id`

**Purpose:** Retrieve the current epoch ID.

**Parameters:**

- `e: Env` - The contract environment.

**Returns:** The current epoch ID as a `u32`.

**Event Emitted:**  
No events emitted.

---

### 12. `get_total_redeem`

**Purpose:** Retrieve the total redeem amount for the current epoch.

**Parameters:**

- `e: Env` - The contract environment.

**Returns:** The total redeem amount as an `i128`.

**Event Emitted:**  
No events emitted.

---

### 13. `get_redeem_rate`

**Purpose:** Retrieve the redeem rate for a specific epoch.

**Parameters:**

- `e: Env` - The contract environment.
- `epoch_id: u32` - The ID of the epoch.

**Returns:** The redeem rate as a `u32`.

**Event Emitted:**  
No events emitted.

---

## Usage

1. **Initialize Vault:** Use the `initialize` function to set up the vault with an offer.
2. **Deposit Tokens:** Buyers can deposit tokens using the `deposit` function.
3. **Claim Leftovers:** Sellers can claim leftover tokens with the `claim_leftover` function.
4. **Update Price:** Sellers can update the offer price using the `updt_price` function.
5. **Redeem Shares:** Users can submit redemption requests with the `redeem_request` function.
6. **Cancel Requests:** Users can cancel redemption requests using the `cancel_request` function.
7. **Claim Requests:** Users can claim redeemed tokens after the epoch is settled using the `claim_request` function.
8. **Settle Epoch:** The treasury can settle the epoch using the `setle_epoch` function.

## Testing

The contract includes comprehensive tests in the `test.rs` file, covering all major functionalities such as initialization, deposits, withdrawals, price updates, and epoch settlements.

---

This contract is designed to provide secure and transparent token management. It ensures that all operations are authorized and that no party can perform actions without proper permissions. Proper use of this contract requires familiarity with the Soroban SDK and token transfer mechanisms.
