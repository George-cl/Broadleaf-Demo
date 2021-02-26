#![cfg_attr(
    not(target_arch = "wasm32"),
    crate_type = "target arch should be wasm32"
)]
#![no_main]
extern crate alloc;

use alloc::string::String;
use serde_json::json;

use casper_contract::{
    contract_api::{runtime, storage},
};
use casper_types::{Key, URef};

const SENDER: &str = "sender";
const MESSAGE: &str = "message";
const EMOJI: &str = "emoji";

fn store_kv_pair(key: String, value: String) {
    // Store `value` under a new unforgeable reference.
    let value_ref: URef = storage::new_uref(value);

    // Wrap the unforgeable reference in a value of type `Key`.
    let value_key: Key = value_ref.into();

    // Store this key under the name "special_value" in context-local storage.
    runtime::put_key(&key, value_key);
}

// All session code must have a `call` entrypoint.
#[no_mangle]
pub extern "C" fn call() {
    // Get the optional first argument supplied to the argument.
    let sender: String = runtime::get_named_arg(SENDER);
    let message: String = runtime::get_named_arg(MESSAGE);
    let emoji: String = runtime::get_named_arg(EMOJI);
    let json_msg = json!({
        "message": message,
        "emoji": emoji
    });

    store_kv_pair(sender, json_msg.to_string());
}
