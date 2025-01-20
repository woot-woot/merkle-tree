fn main() {
    use merkle::*;

    let data = vec![
        "abc".to_string(),
        "bcd".to_string(),
        "cde".to_string(),
        "def".to_string(),
        "efg".to_string(),
    ];

    let root = MerkleTree::merkle_root(data.clone().into_iter());
    println!("Merkle Root: {:?}", root);

    let proof = MerkleTree::merkle_proof(data.clone().into_iter(), 0);
    println!("Merkle Proof: {:?}", proof);

    let is_valid = MerkleTree::verify_proof(&root, &proof);
    println!("Is proof valid? {}", is_valid);
}
