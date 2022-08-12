#![no_std]

#[cfg(feature = "testutils")]
extern crate std;

mod cryptography;
mod public_types;
mod test;

use public_types::{Identifier, KeyedAuthorization};
use soroban_sdk::{contractimpl, contracttype, BigInt, Env, IntoVal};

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Acc(Identifier),
    Nonce(Identifier),
}

pub struct AuthContract;

#[contractimpl(export_if = "export")]
impl AuthContract {
    pub fn save_data(e: Env, auth: KeyedAuthorization, num: BigInt) {
        cryptography::check_auth(
            &e,
            &auth,
            cryptography::Domain::SaveData,
            (vec![&e, num]).into_val(&e),
        );

        let auth_id = auth.get_identifier(&e);
        e.contract_data().set(DataKey::Acc(auth_id), num);
    }

    pub fn nonce(e: Env, to: Identifier) -> BigInt {
        cryptography::read_nonce(&e, to)
    }
}
