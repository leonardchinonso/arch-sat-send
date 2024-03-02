use crate::constants::{
    LAST_SENDER_TXID, RECIPIENT_ADDRESS, SAT_FEE, SENDER_PUBLIC_KEY, SEND_AMOUNT, UTXO_AMOUNT,
};
use crate::utils::{calculate_change, get_bip32_at_node_index, get_p2sh_keys};
use bitcoin::absolute::LockTime;
use bitcoin::address::NetworkUnchecked;
use bitcoin::ecdsa::Signature;
use bitcoin::hashes::Hash;
use bitcoin::opcodes::all::OP_CHECKMULTISIG;
use bitcoin::secp256k1::{Message, Secp256k1};
use bitcoin::sighash::SighashCache;
use bitcoin::transaction::Version;
use bitcoin::{
    Address, EcdsaSighashType, Network, OutPoint, PublicKey, Script, ScriptBuf, Sequence,
    Transaction, TxIn, TxOut, Txid, Witness,
};
use std::str::FromStr;

mod constants;
mod utils;

fn main() {
    let (sk, pub_key, wpkh) = get_p2sh_keys();

    let recp_address: Address<NetworkUnchecked> = RECIPIENT_ADDRESS.parse().unwrap();
    let recp_address = recp_address.require_network(Network::Testnet).unwrap();

    let txid = Txid::from_str(LAST_SENDER_TXID).unwrap();
    let out_point = OutPoint { txid, vout: 0 };

    let s_pubkey = ScriptBuf::new_p2wpkh(&wpkh);
    let utxo = TxOut {
        value: UTXO_AMOUNT,
        script_pubkey: s_pubkey,
    };

    // build redeem script
    let mut script_builder = Script::builder();
    script_builder = script_builder.push_int(2);
    script_builder = script_builder.push_key(&get_bip32_at_node_index(0));
    script_builder = script_builder.push_key(&PublicKey::from_str(SENDER_PUBLIC_KEY).unwrap());
    script_builder = script_builder.push_opcode(OP_CHECKMULTISIG);

    let script_hash = script_builder.into_script().script_hash();

    let txin = TxIn {
        previous_output: out_point,
        script_sig: ScriptBuf::new_p2sh(&script_hash),
        sequence: Sequence::ENABLE_RBF_NO_LOCKTIME,
        witness: Witness::default(),
    };

    let txout_spend = TxOut {
        value: SEND_AMOUNT,
        script_pubkey: recp_address.script_pubkey(),
    };

    let change = TxOut {
        value: calculate_change(UTXO_AMOUNT, SEND_AMOUNT, SAT_FEE),
        script_pubkey: ScriptBuf::new_p2sh(&script_hash),
    };

    let mut raw_tx = Transaction {
        version: Version::TWO,
        lock_time: LockTime::ZERO,
        input: vec![txin],
        output: vec![txout_spend, change],
    };

    let sighash_type = EcdsaSighashType::All;
    let mut sighasher = SighashCache::new(&mut raw_tx);
    let sighash = sighasher
        .p2wsh_signature_hash(0, &utxo.script_pubkey, SEND_AMOUNT, sighash_type)
        .expect("failed to create sighash");

    let msg = Message::from(sighash);
    let signature = Secp256k1::new().sign_ecdsa(&msg, &sk);

    let signature = Signature {
        sig: signature,
        hash_ty: sighash_type,
    };
    *sighasher.witness_mut(0).unwrap() = Witness::p2wpkh(&signature, &pub_key);

    let tx = sighasher.into_transaction();
    println!("Signed Tx: {:#?}", tx);

    let s_tx = bitcoin::consensus::encode::serialize_hex(tx);
    println!("Broadcast Tx: {:#?}", s_tx);
}
