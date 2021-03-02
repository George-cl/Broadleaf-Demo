#![no_std]
#![no_main]

extern crate alloc;

use alloc::{string::ToString, vec::Vec};

use casper_contract::contract_api::{runtime, storage};
use casper_types::{
    contracts::{EntryPoint, EntryPoints},
    CLType, EntryPointAccess, EntryPointType,
};

// EP = Entry Point
const EP_SEND_MSG: &str = "sendMessage";
const EP_GET_MSG: &str = "getMessage";
const HASH_KEY_NAME: &str = "bl_messenger_hash";
const PACKAGE_HASH_KEY_NAME: &str = "bl_messenger_package_hash";
const ACCESS_KEY_NAME: &str = "bl_messenger_access";
const CONTRACT_VERSION: &str = "bl_messenger_version";

#[no_mangle]
pub extern "C" fn sendMessage() {
    // Method for storing a message on the chain

}

#[no_mangle]
pub extern "C" fn getMessage() {
    // Method for retrieving a message from the chain

}

#[no_mangle]
pub extern "C" fn call() {
    // Set entry points for contract
    let entry_points = {
        let mut entry_points = EntryPoints::new();
        let ep_send_msg = EntryPoint::new(
            EP_SEND_MSG.to_string(),
            Vec::new(),
            CLType::Unit,
            EntryPointAccess::Public,
            EntryPointType::Contract,
        );
        entry_points.add_entry_point(ep_send_msg);

        let ep_get_msg = EntryPoint::new(
            EP_GET_MSG.to_string(),
            Vec::new(),
            CLType::Unit,
            EntryPointAccess::Public,
            EntryPointType::Contract,
        );
        entry_points.add_entry_point(ep_get_msg);

        entry_points
    };

    // create the new contract with above entry points
    let (contract_hash, contract_version) = storage::new_contract(
        entry_points,
        None,
        Some(PACKAGE_HASH_KEY_NAME.to_string()),
        Some(ACCESS_KEY_NAME.to_string()),
    );

    // save contract version as a uref under the key named CONTRACT_VERSION
    runtime::put_key(CONTRACT_VERSION, storage::new_uref(contract_version).into());
    // save contract hash under the key named HASH_KEY_NAME 
    runtime::put_key(HASH_KEY_NAME, contract_hash.into());
}
