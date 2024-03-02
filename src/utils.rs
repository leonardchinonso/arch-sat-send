use crate::constants::{SENDER_PRIVATE_KEY, SENDER_PUBLIC_KEY};
use bitcoin::secp256k1::{rand, Secp256k1, SecretKey, Signing};
use bitcoin::{
    Amount, Network, PrivateKey as BTCPrivateKey, PrivateKey, PublicKey as BTCPublicKey, PublicKey,
    WPubkeyHash,
};
use std::str::FromStr;

pub fn generate_keys<T: Signing>(secp: &Secp256k1<T>) -> (BTCPrivateKey, BTCPublicKey) {
    let private_key = BTCPrivateKey::generate(Network::Testnet);
    let public_key = BTCPublicKey::from_private_key(&secp, &private_key);

    println!("{:?}", private_key);
    println!("{:?}", public_key);

    (private_key, public_key)
}

pub fn generate_keys_wpkh<T: Signing>(secp: &Secp256k1<T>) -> (SecretKey, WPubkeyHash) {
    let sk = SecretKey::new(&mut rand::thread_rng());
    let pk = PublicKey::new(sk.public_key(secp));
    let wpkh = pk.wpubkey_hash().expect("key is compressed");

    println!("SK: {:?}", sk);
    println!("WPKH: {:?}", wpkh);

    (sk, wpkh)
}

pub fn get_wpkh_keys() -> (SecretKey, bitcoin::secp256k1::PublicKey, WPubkeyHash) {
    let pk = PrivateKey::from_wif(SENDER_PRIVATE_KEY).expect("invalid private key");
    let sk = pk.inner;
    let pub_k = PublicKey::from_str(SENDER_PUBLIC_KEY).unwrap();
    let wpkh = pub_k.wpubkey_hash().expect("key is compressed");
    let pub_k = pub_k.inner;

    println!("SK: {:?}", sk);
    println!("PK: {:?}", pk);
    println!("PUB_K: {:?}", pub_k);
    println!("WPKH: {:?}", wpkh);

    (sk, pub_k, wpkh)
}

pub fn get_p2sh_keys() -> (SecretKey, bitcoin::secp256k1::PublicKey, WPubkeyHash) {
    let pk = PrivateKey::from_wif(SENDER_PRIVATE_KEY).expect("invalid private key");
    let sk = pk.inner;
    let pub_k = PublicKey::from_str(SENDER_PUBLIC_KEY).unwrap();
    let wpkh = pub_k.wpubkey_hash().expect("valid pubkey");
    let pub_k = pub_k.inner;

    println!("SK: {:?}", sk);
    println!("PK: {:?}", pk);
    println!("PUB_K: {:?}", pub_k);
    println!("WPKH: {:?}", wpkh);

    (sk, pub_k, wpkh)
}

pub fn get_bip32_at_node_index(index: u32) -> PublicKey {
    // TODO: change to Bip32 public key using the index and derivation path
    PublicKey::from_str(SENDER_PUBLIC_KEY).unwrap()
}

pub fn calculate_change(utxo_amount: Amount, spend: Amount, fee: Amount) -> Amount {
    utxo_amount
        .checked_sub(spend.checked_add(fee).unwrap())
        .unwrap()
}
