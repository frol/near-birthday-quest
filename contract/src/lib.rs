use std::convert::TryFrom;

use ed25519_dalek::Verifier;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{near_bindgen, setup_alloc, Promise};

setup_alloc!();

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Default)]
pub struct BirthdayQuest {}

const ONE_NEAR: u128 = 1_000_000_000_000_000_000_000_000;

#[near_bindgen]
impl BirthdayQuest {
    pub fn gimme_my_present(&mut self, signature: Vec<u8>) -> Promise {
        let signature = ed25519_dalek::Signature::try_from(signature.as_ref())
            .expect("Signature should be a valid array of 64 bytes [13, 254, 123, ...]");

        let account_id = near_sdk::env::signer_account_id();

        // Someone will give you the corresponding private key...
        let public_key = ed25519_dalek::PublicKey::from_bytes(
            &near_sdk::json_types::Base58PublicKey::try_from(
                "ed25519:H5ANpdUoXVwhYBgAgEi1ieMQZKJbwxjPJtHX4vkVcSnF",
            )
            .unwrap()
            .0,
        )
        .unwrap();

        near_sdk::env::log(
            format!(
                "Verifiying validity of signature ('{:?}') for string '{}'...",
                signature, account_id
            )
            .as_bytes(),
        );

        if let Ok(_) = public_key.verify(account_id.as_bytes(), &signature) {
            return Promise::new(account_id).transfer(16 * 16 * ONE_NEAR);
        }

        panic!("Ima no gonna give-ya the present without a signature! :-P");
    }
}
