extern crate alloc;
use alloc::{collections::BTreeSet, string::String};
use std::convert::TryInto;
use serde_json::json;

use casperlabs_contract_macro::{casperlabs_constructor, casperlabs_contract, casperlabs_method};

use contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};

use types::{
    bytesrepr::ToBytes,
    contracts::{EntryPoint, EntryPointAccess, EntryPointType, EntryPoints},
    runtime_args, CLType, CLTyped, CLValue, Group, Parameter, RuntimeArgs, URef,
};

#[casperlabs_contract]
mod kvstorage_contract {

    #[casperlabs_constructor]
    fn init() {}

    #[casperlabs_method]
    fn store_user_message(user_public_key: String, message_content: String, message_emoji: String) {
        
        let message = json!({
            "user_public_key": user_public_key,
            "message_content": message_content,
            "message_emoji": message_emoji
        });
        set_key(user_public_key.as_str(), message.to_string());
    }

    fn set_key<T: ToBytes + CLTyped>(name: &str, value: T) {
        match runtime::get_key(name) {
            Some(key) => {
                let key_ref = key.try_into().unwrap_or_revert();
                storage::write(key_ref, value);
            }
            None => {
                let key = storage::new_uref(value).into();
                runtime::put_key(name, key);
            }
        }
    }
}
