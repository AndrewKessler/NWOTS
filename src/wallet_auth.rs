use std::fs;
use std::path::Path;

use bitcoin::bip32::{ChildNumber, Xpriv};
use bitcoin::secp256k1::Secp256k1;
use bitcoin::Network;
use bip39::{Language, Mnemonic};
use ed25519_dalek::{Signer, Verifier, Signature, SigningKey, VerifyingKey};
use rand::{rngs::OsRng, RngCore};
use serde::Deserialize;
use sha3::{Digest, Sha3_256};

#[derive(Debug, Deserialize)]
struct WalletAddress {
    index: u32,
    address: String,
    public_key_hex: String,
}

#[derive(Debug, Deserialize)]
struct WalletFile {
    network: String,
    seed_phrase: String,
    bip39_passphrase: String,
    addresses: Vec<WalletAddress>,
}

#[derive(Debug)]
pub struct WalletAuthentication {
    pub address: String,
    pub public_key_hex: String,
    pub challenge_hex: String,
    pub signature_hex: String,
}

const CHALLENGE_PREFIX: u8 = 0x6a;
const CHALLENGE_SUFFIX: [u8; 4] = [0xF9, 0xBE, 0xB4, 0xD9];

fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter()
        .map(|b| format!("{:02x}", b))
        .collect()
}

fn read_wallet(wallet_file: &str) -> Result<WalletFile, String> {
    let path = Path::new(wallet_file);

    let contents = fs::read_to_string(path)
        .map_err(|e| {
            format!(
                "failed to read wallet file {}: {}",
                path.display(),
                e
            )
        })?;

    serde_json::from_str::<WalletFile>(&contents)
        .map_err(|e| {
            format!(
                "failed to parse wallet file {}: {}",
                path.display(),
                e
            )
        })
}

fn derive_key(
    seed_phrase: &str,
    bip39_passphrase: &str,
    index: u32,
) -> Result<SigningKey, String> {
    let mnemonic = Mnemonic::parse_in_normalized(
        Language::English,
        seed_phrase,
    )
    .map_err(|e| format!("invalid BIP39 mnemonic: {}", e))?;

    let seed = mnemonic.to_seed(bip39_passphrase);

    let secp = Secp256k1::new();

    let master = Xpriv::new_master(
        Network::Bitcoin,
        &seed,
    )
    .map_err(|e| format!("failed to create master key: {}", e))?;

    let child_number = ChildNumber::from_hardened_idx(index)
        .map_err(|e| format!("invalid address index {}: {}", index, e))?;

    let child = master
        .derive_priv(&secp, &[child_number])
        .map_err(|e| format!("failed to derive child key: {}", e))?;

    let xpriv_string = child.to_string();
    let xpriv_bytes = xpriv_string.as_bytes();

    if xpriv_bytes.len() < 32 {
        return Err(
            "derived extended private key is unexpectedly short".to_string()
        );
    }

    let mut ed_seed = [0u8; 32];
    ed_seed.copy_from_slice(&xpriv_bytes[..32]);

    Ok(SigningKey::from_bytes(&ed_seed))
}

fn address_from_public_key(public_key: &[u8; 32]) -> String {
    let mut hasher = Sha3_256::new();
    hasher.update(public_key);

    let hash = hasher.finalize();

    hex_encode(&hash)
}

fn generate_challenge() -> [u8; 13] {
    let mut challenge = [0u8; 13];

    challenge[0] = CHALLENGE_PREFIX;

    let mut nonce_bytes = [0u8; 8];
    OsRng.fill_bytes(&mut nonce_bytes);

    challenge[1..9].copy_from_slice(&nonce_bytes);
    challenge[9..13].copy_from_slice(&CHALLENGE_SUFFIX);

    challenge
}

fn validate_challenge(challenge: &[u8]) -> Result<(), String> {
    if challenge.len() != 13 {
        return Err(format!(
            "invalid challenge length: expected 13 bytes, got {}",
            challenge.len()
        ));
    }

    if challenge[0] != CHALLENGE_PREFIX {
        return Err("invalid challenge prefix".to_string());
    }

    if challenge[9..13] != CHALLENGE_SUFFIX {
        return Err("invalid challenge suffix".to_string());
    }

    Ok(())
}

fn sign_and_verify(
    signing_key: &SigningKey,
    challenge: &[u8],
) -> Result<Vec<u8>, String> {
    validate_challenge(challenge)?;

    // Sign the exact raw challenge bytes.
    let signature = signing_key.sign(challenge);

    // Verify the signature using the corresponding public key.
    let verifying_key = signing_key.verifying_key();

    verifying_key
        .verify(challenge, &signature)
        .map_err(|e| format!("signature verification failed: {}", e))?;

    Ok(signature.to_bytes().to_vec())
}

pub fn authenticate_wallet(
    wallet_file: &str,
    address_index: usize,
    expected_owner_address: &str,
) -> Result<WalletAuthentication, String> {
    let wallet = read_wallet(wallet_file)?;

    let wallet_address = wallet
        .addresses
        .iter()
        .find(|address| address.index as usize == address_index)
        .ok_or_else(|| {
            format!(
                "wallet address index {} not found in {}",
                address_index,
                wallet_file
            )
        })?;

    let signing_key = derive_key(
        &wallet.seed_phrase,
        &wallet.bip39_passphrase,
        address_index as u32,
    )?;

    let public_key = signing_key.verifying_key().to_bytes();
    let public_key_hex = hex_encode(&public_key);

    let derived_address = address_from_public_key(&public_key);

    println!(
        "Wallet authentication: address index {}",
        address_index
    );

    println!(
        "Wallet registered address: {}",
        wallet_address.address
    );

    println!(
        "Wallet derived address:    {}",
        derived_address
    );

    // First prove that wallet.json has not been substituted
    // with a mismatching registered address.
    if derived_address != wallet_address.address {
        return Err(format!(
            "wallet address mismatch: derived {}, registered {}",
            derived_address,
            wallet_address.address
        ));
    }

    // Then prove that the wallet actually controls the address
    // that currently owns the authenticated Item.
    if derived_address != expected_owner_address {
        return Err(format!(
            "wallet does not control current Item owner address: \
             derived {}, current owner {}",
            derived_address,
            expected_owner_address
        ));
    }

    // Generate a fresh challenge for this authentication attempt.
    let challenge = generate_challenge();

    let signature = sign_and_verify(
        &signing_key,
        &challenge,
    )?;

    let challenge_hex = hex_encode(&challenge);
    let signature_hex = hex_encode(&signature);

    println!(
        "Challenge: {}",
        challenge_hex
    );

    println!(
        "Signature verified: yes"
    );

    println!(
        "Wallet proof-of-possession: verified"
    );

    Ok(WalletAuthentication {
        address: derived_address,
        public_key_hex,
        challenge_hex,
        signature_hex,
    })
}