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
const RECIPIENT: &str = "recipient";
const MESSAGE: &str = "message";
const EMOJI: &str = "emoji";

fn store_message(sender: String, message: String) {
    
    // Store `message` under a new unforgeable reference.
    let message_ref: URef = storage::new_uref(message);

    // Wrap the unforgeable reference in a value of type `Key`.
    let message_key: Key = message_ref.into();

    runtime::put_key(&sender, message_key);
}

// All session code must have a `call` entrypoint.
#[no_mangle]
pub extern "C" fn call() {

    // Get arguments passed in during contract call
    let sender: String = runtime::get_named_arg(SENDER);
    let recipient: String = runtime::get_named_arg(RECIPIENT);
    let message: String = runtime::get_named_arg(MESSAGE);
    let emoji: String = runtime::get_named_arg(EMOJI);

    // Format to JSON to make parsing easier
    let json_msg = json!({
        "from": sender,
        "to": recipient,
        "message": message,
        "emoji": emoji
    });

    store_message(sender, json_msg.to_string());
}
