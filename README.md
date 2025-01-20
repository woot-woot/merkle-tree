# Merkle Tree Implementation in Rust

This project implements a Merkle Tree and its associated functionalities in Rust, using the `blake2` hashing library for cryptographic operations. It supports the creation of a Merkle root, the generation of Merkle proofs, and the verification of those proofs.

## Features

1. **Generate Merkle Root**: Calculate the root hash of a binary Merkle Tree given a set of leaves.
2. **Generate Merkle Proofs**: Create proofs for individual leaves to verify their inclusion in the tree.
3. **Verify Proofs**: Validate Merkle proofs against a given root hash.

## Data Structures

### `MerkleProof<T>`

A struct that stores the data needed to verify a proof:

- `hashes`: A vector of sibling hashes.
- `num_of_leaves`: Total number of leaves in the tree.
- `leaf_index`: Index of the leaf in the original data.
- `leaf_content`: The content of the leaf being proven.

## Usage

### Example

```rust
use merkle_tree::*;

fn main() {
    let data = vec!["abc", "bcd", "cde", "def", "efg"];
    let root = MerkleTree::merkle_root(data.iter().cloned().map(String::from));
    let proof = MerkleTree::merkle_proof(data.iter().cloned().map(String::from), 1);

    assert!(MerkleTree::verify_proof(&root, &proof));
    println!("Merkle root: {}", root);
}
```

### Test

Run the unit tests included in the project:

```sh
cargo test
```

## Installation

1. Clone the repository:

   ```sh
   git clone <repository-url>
   ```

2. Navigate to the project directory:

   ```sh
   cd merkle-tree
   ```

3. Build the project:
   ```sh
   cargo build
   ```

## Dependencies

- [blake2](https://docs.rs/blake2/latest/blake2/) - Cryptographic hashing library.

## Submission

This implementation fulfills the requirements of the assignment:

1. Implements the functions:
   - `merkle_root`
   - `merkle_proof`
   - `verify_proof`
2. Supports arbitrary numbers of leaves.
3. Includes a README for usage and inline comments for understanding the implementation.
4. Comprehensive test cases to validate functionality.

### References

- [Merkle Tree - Wikipedia](https://en.wikipedia.org/wiki/Merkle_tree)
- [Merkle Proofs - Ethereum](https://ethereum.org/en/developers/tutorials/merkle-proofs-for-offline-data-integrity/)
