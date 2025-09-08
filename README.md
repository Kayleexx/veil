# Veil: Secure Multi-Party Computation & Encryption Framework

<img width="368" height="177" alt="image" src="https://github.com/user-attachments/assets/34e74614-d969-45b7-aa52-4aad5f532dde" />


Veil is a Rust-based project that provides a **multi-purpose encryption and secret-sharing framework** with support for:

* AES encryption
* Hash-based encryption
* String reversal
* Shamir’s Secret Sharing for secure split/combine of secrets
* Multi-party computation (MPC) with secret aggregation over a network

It includes both **server** and **client** components with an interactive CLI.

---

## Table of Contents

* [Features](#features)
* [Installation](#installation)
* [Project Structure](#project-structure)
* [Usage](#usage)

  * [Start Server](#start-server)
  * [Start Client](#start-client)
  * [Commands](#commands)
* [Examples](#examples)
* [Shamir’s Secret Sharing](#shamirs-secret-sharing)
* [MPC Aggregation](#mpc-aggregation)
* [License](#license)

---

## Features

* **Dynamic encryption selection:** `aes`, `hash`, `reverse`
* **Secret splitting & combining** using Shamir’s Secret Sharing
* **Multi-party secret aggregation** with XOR-based combination
* **Secure network communication** over TCP with random salts
* **Interactive CLI** for easy testing and experimentation

---

## Installation

1. Clone the repository:

```bash
git clone https://github.com/yourusername/veil.git
cd veil
```

2. Build the project:

```bash
cargo build --release
```

3. Run the project:

```bash
cargo run --release
```

> Requires Rust 1.70+ and the `tokio` async runtime.

---

## Project Structure

```
veil/
├── Cargo.toml
├── src/
│   ├── main.rs          # CLI entry point
│   ├── network.rs       # Server & Client networking logic
│   ├── mpc.rs           # Multi-party computation & aggregation
│   ├── secret_sharing.rs# Shamir's Secret Sharing implementation
│   ├── aes.rs           # AES encryption logic
│   ├── hash_and_reverse.rs # Hashing & string reversal encryptors
│   └── lib.rs
└── README.md
```

---

## Usage

### Start Server

```bash
cargo run --release
```

Server will ask:

```
Do you want to run as server/node or client?
```

Type `server` and provide an address:

```
127.0.0.1:7878
```

---

### Start Client

```bash
cargo run --release
```

Type `client` and provide server address:

```
127.0.0.1:7878
```

Then you can enter commands interactively.

---

### Commands

```
aes|text             # AES encrypt 'text'
hash|text            # Hash 'text'
reverse|text         # Reverse 'text'
split|secret|k|n     # Split 'secret' into n shares with threshold k
combine|share1,...   # Combine shares into original secret
mpc|party1,party2|party1,party2 # Aggregate multiple parties' shares
```

---

## Examples

**AES Encryption:**

```
> aes|hello
Server response: <hex encoded encrypted output>
```

**Reverse Encryption:**

```
> reverse|hello
Server response: olleh
```


**Hash Encryption:**

```
> hash|hello
Server response: <sha256 hash in hex>
```

**Secret Sharing (Split):**

```
> split|mysecret|3|5
Server response: 5 hex-encoded shares separated by commas
```

**Secret Sharing (Combine):**

```
> combine|share1,share2,share3
Server response: mysecret
```

**MPC Aggregation:**

```
> mpc|party1share1,party1share2|party2share1,party2share2
Server response: <hex aggregated secret>
```

---

## Shamir’s Secret Sharing

Veil uses Shamir’s Secret Sharing scheme to securely **split secrets into shares**:

* `k` = minimum threshold to reconstruct secret
* `n` = total number of shares

All shares are **hex encoded** for network transmission. Combining any `k` valid shares will reconstruct the original secret.

---

## MPC Aggregation

* Accepts multiple parties, each providing multiple shares
* Aggregates secrets using XOR across parties
* Returns a single **hex-encoded aggregated secret**
* Supports secure collaborative computation without revealing individual secrets

---

## Output

<img width="1349" height="517" alt="image" src="https://github.com/user-attachments/assets/36693847-104a-409f-993e-8276bf1c8894" />
<img width="1103" height="921" alt="image" src="https://github.com/user-attachments/assets/21c02c42-fc89-456a-bce5-deab85bb2ed2" />
<img width="1509" height="815" alt="image" src="https://github.com/user-attachments/assets/5f7e5c7c-6ca8-4e6c-ad24-3982c000519b" />
<img width="1502" height="141" alt="image" src="https://github.com/user-attachments/assets/42b23e4f-f75d-4ecd-bf2e-5b3b87e73a17" />

---

## License

MIT License.
