# Stellar Soroban Real Estate Registry

## Project Description

The Stellar Soroban Real Estate Registry is a decentralized application built on the Stellar network using the Soroban smart contract platform. It aims to modernize property ownership by tokenizing real-world assets. By bridging physical real estate with blockchain technology, this project provides a transparent, immutable, and highly secure ledger for tracking property rights and facilitating peer-to-peer transfers.

## What it does

This smart contract functions as a digital land registry. It allows a user to mint (register) a unique property identifier to a specific Stellar wallet address. Once a property is registered, the current owner can cryptographically authorize the transfer of that property to a new buyer. Anyone can query the blockchain to verify the true, current owner of a specific piece of real estate, completely eliminating the need for traditional, paper-based escrow or title search delays.

## Features

* **Property Registration:** Securely map a unique property ID (e.g., a digitized parcel number) to a Stellar `Address`.
* **Cryptographic Authorization:** Utilizes Soroban's native `require_auth()` to ensure that only the verified owner of a property can initiate a transfer.
* **Immutable Title Tracking:** All property states are saved to persistent contract storage, preventing unauthorized tampering or accidental loss of title records.
* **Public Verification:** Exposes a simple read-only function allowing third parties (auditors, buyers, banks) to instantly verify ownership status.

## Deployed Smart Contract Link

**Link:** XXX
*(Note: Replace XXX with your Stellar Expert explorer link or Contract ID once deployed to Testnet/Futurenet/Mainnet)*