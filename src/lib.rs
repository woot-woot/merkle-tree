// Cargo.toml dependencies will include "blake2" and "hex" for hashing

use blake2::{Blake2b512, Digest};
use hex; // Ensure the hex crate is imported

#[derive(Debug)]
pub struct MerkleProof<T> {
    pub hashes: Vec<String>,
    pub num_of_leaves: usize,
    pub leaf_index: usize,
    pub leaf_content: T,
}

pub struct MerkleTree;

impl MerkleTree {
    pub fn merkle_root<I>(leaves: I) -> String
    where
        I: Iterator<Item = String>,
    {
        let mut hashed_leaves: Vec<String> = leaves
            .map(|leaf| {
                let mut hasher = Blake2b512::new();
                hasher.update(leaf);
                let hash = hasher.finalize();
                hex::encode(hash)
            })
            .collect();

        while hashed_leaves.len() > 1 {
            let mut next_level = Vec::new();

            for chunk in hashed_leaves.chunks(2) {
                let concatenated = match chunk {
                    [a, b] => a.clone() + b,
                    [a] => a.clone() + a,
                    _ => unreachable!(),
                };

                let mut hasher = Blake2b512::new();
                hasher.update(concatenated);
                next_level.push(hex::encode(hasher.finalize()));
            }

            hashed_leaves = next_level;
        }

        hashed_leaves.pop().unwrap()
    }

    pub fn merkle_proof<I>(leaves: I, leaf_index: usize) -> MerkleProof<String>
    where
        I: Iterator<Item = String> + Clone,
    {
        let leaves: Vec<String> = leaves.collect();
        let mut proof = MerkleProof {
            hashes: Vec::new(),
            num_of_leaves: leaves.len(),
            leaf_index,
            leaf_content: leaves[leaf_index].clone(),
        };

        let mut hashed_leaves: Vec<String> = leaves
            .into_iter()
            .map(|leaf| {
                let mut hasher = Blake2b512::new();
                hasher.update(leaf);
                hex::encode(hasher.finalize())
            })
            .collect();

        let mut index = leaf_index;

        while hashed_leaves.len() > 1 {
            let mut next_level = Vec::new();

            for (i, chunk) in hashed_leaves.chunks(2).enumerate() {
                if i == index / 2 {
                    proof.hashes.push(match chunk {
                        [_, b] if index % 2 == 0 => b.clone(),
                        [a, _] if index % 2 == 1 => a.clone(),
                        [a] => a.clone(),
                        _ => unreachable!(),
                    });
                }

                let concatenated = match chunk {
                    [a, b] => a.clone() + b,
                    [a] => a.clone() + a,
                    _ => unreachable!(),
                };

                let mut hasher = Blake2b512::new();
                hasher.update(concatenated);
                next_level.push(hex::encode(hasher.finalize()));
            }

            index /= 2;
            hashed_leaves = next_level;
        }

        proof
    }

    pub fn verify_proof(root: &String, proof: &MerkleProof<String>) -> bool {
        let mut hash = {
            let mut hasher = Blake2b512::new();
            hasher.update(&proof.leaf_content);
            hex::encode(hasher.finalize())
        };

        let mut index = proof.leaf_index;

        for sibling_hash in &proof.hashes {
            let concatenated = if index % 2 == 0 {
                hash.clone() + sibling_hash
            } else {
                sibling_hash.clone() + &hash
            };

            let mut hasher = Blake2b512::new();
            hasher.update(concatenated);
            hash = hex::encode(hasher.finalize());

            index /= 2;
        }

        *root == hash
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merkle_root() {
        let data = vec!["a", "b", "c", "d", "e"];
        let root = MerkleTree::merkle_root(data.iter().cloned().map(String::from));
        assert!(!root.is_empty(), "Root hash should not be empty");
    }

    #[test]
    fn test_merkle_proof() {
        let data = vec!["a", "b", "c", "d", "e"];
        let proof = MerkleTree::merkle_proof(data.iter().cloned().map(String::from), 1);
        assert_eq!(proof.leaf_index, 1);
        assert_eq!(proof.leaf_content, "b");
    }

    #[test]
    fn test_verify_proof() {
        let data = vec!["a", "b", "c", "d", "e"];
        let root = MerkleTree::merkle_root(data.iter().cloned().map(String::from));
        let proof = MerkleTree::merkle_proof(data.iter().cloned().map(String::from), 1);
        assert!(
            MerkleTree::verify_proof(&root, &proof),
            "Proof should be valid"
        );
    }

    #[test]
    fn test_odd_number_of_leaves() {
        let data = vec!["a", "b", "c", "d", "e", "f", "g"];
        let root = MerkleTree::merkle_root(data.iter().cloned().map(String::from));
        let proof = MerkleTree::merkle_proof(data.iter().cloned().map(String::from), 4);
        assert!(
            MerkleTree::verify_proof(&root, &proof),
            "Proof should be valid"
        );
    }
}
