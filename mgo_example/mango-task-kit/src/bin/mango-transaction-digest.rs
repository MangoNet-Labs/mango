//! Digest for TransactionData BCS encoded by the pinned local Mango client.
//! The pinned Mango transaction::Message implementation uses the Keccak256
//! hash of the type-tagged value, rendered as an unpadded base58 digest.
use std::io::{self, Read};

use anyhow::Result;
use base64::{engine::general_purpose::STANDARD, Engine};
use sha3::{Digest, Keccak256};

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().take(1024 * 1024).read_to_string(&mut input)?;
    let data = STANDARD.decode(input.trim())?;
    anyhow::ensure!(!data.is_empty(), "Missing local TransactionData bytes");
    let mut hasher = Keccak256::new();
    hasher.update(b"TransactionData::");
    hasher.update(data);
    println!("{}", bs58::encode(hasher.finalize()).into_string());
    Ok(())
}
