// Copyright (c) The Libra Core Contributors
// SPDX-License-Identifier: Apache-2.0

// This file was generated. Do not modify!
//
// To re-generate this code, run: `(cd language/stdlib && cargo run --release)`

use libra_types::{
    account_address::AccountAddress,
    transaction::{Script, TransactionArgument},
};
use move_core_types::language_storage::TypeTag;

/// Add the currency identified by the type `currency` to the sending accounts. Aborts if
/// the account already holds a balance fo `currency` type.
pub fn encode_add_currency_to_account_script(currency: TypeTag) -> Script {
    Script::new(
        vec![
            161, 28, 235, 11, 1, 0, 6, 1, 0, 2, 3, 2, 6, 4, 8, 2, 5, 10, 7, 7, 17, 26, 8, 43, 16,
            0, 0, 0, 1, 0, 1, 1, 1, 0, 2, 1, 6, 12, 0, 1, 9, 0, 12, 76, 105, 98, 114, 97, 65, 99,
            99, 111, 117, 110, 116, 12, 97, 100, 100, 95, 99, 117, 114, 114, 101, 110, 99, 121, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 1, 3, 11, 0, 56, 0, 2,
        ],
        vec![currency],
        vec![],
    )
}

/// Add the `KeyRotationCapability` for `to_recover_account` to the `RecoveryAddress`
/// resource under `recovery_address`. Aborts if `to_recovery_account` and
/// `to_recovery_address belong to different VASPs, if `recovery_address` does not have a
/// `RecoveryAddress` resource, or if `to_recover_account` has already extracted its
/// `KeyRotationCapability`.
pub fn encode_add_recovery_rotation_capability_script(recovery_address: AccountAddress) -> Script {
    Script::new(
        vec![
            161, 28, 235, 11, 1, 0, 5, 1, 0, 2, 3, 2, 5, 5, 7, 5, 7, 12, 40, 8, 52, 16, 0, 0, 0, 1,
            0, 1, 0, 2, 6, 12, 5, 0, 15, 82, 101, 99, 111, 118, 101, 114, 121, 65, 100, 100, 114,
            101, 115, 115, 23, 97, 100, 100, 95, 114, 111, 116, 97, 116, 105, 111, 110, 95, 99, 97,
            112, 97, 98, 105, 108, 105, 116, 121, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
            0, 0, 1, 4, 11, 0, 10, 1, 17, 0, 2,
        ],
        vec![],
        vec![TransactionArgument::Address(recovery_address)],
    )
}

/// Add `new_validator` to the pending validator set. Fails if the `new_validator` address
/// is already in the validator set or does not have a `ValidatorConfig` resource stored
/// at the address.
pub fn encode_add_validator_script(validator_address: AccountAddress) -> Script {
    Script::new(
        vec![
            161, 28, 235, 11, 1, 0, 7, 1, 0, 4, 2, 4, 9, 3, 13, 17, 4, 30, 4, 5, 34, 39, 7, 73,
            121, 8, 194, 1, 16, 0, 0, 0, 1, 1, 2, 1, 1, 3, 1, 3, 1, 0, 1, 4, 0, 1, 1, 3, 1, 5, 2,
            3, 1, 3, 0, 6, 4, 3, 0, 0, 7, 1, 7, 1, 6, 12, 1, 11, 0, 1, 9, 0, 2, 6, 12, 11, 0, 1, 9,
            0, 0, 2, 6, 11, 0, 1, 8, 1, 5, 2, 6, 12, 5, 1, 11, 0, 1, 8, 1, 1, 8, 1, 11, 76, 105,
            98, 114, 97, 83, 121, 115, 116, 101, 109, 5, 82, 111, 108, 101, 115, 10, 67, 97, 112,
            97, 98, 105, 108, 105, 116, 121, 13, 76, 105, 98, 114, 97, 82, 111, 111, 116, 82, 111,
            108, 101, 31, 101, 120, 116, 114, 97, 99, 116, 95, 112, 114, 105, 118, 105, 108, 101,
            103, 101, 95, 116, 111, 95, 99, 97, 112, 97, 98, 105, 108, 105, 116, 121, 31, 114, 101,
            115, 116, 111, 114, 101, 95, 99, 97, 112, 97, 98, 105, 108, 105, 116, 121, 95, 116,
            111, 95, 112, 114, 105, 118, 105, 108, 101, 103, 101, 13, 97, 100, 100, 95, 118, 97,
            108, 105, 100, 97, 116, 111, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 5,
            6, 10, 10, 0, 56, 0, 12, 2, 14, 2, 10, 1, 17, 2, 11, 0, 11, 2, 56, 1, 2,
        ],
        vec![],
        vec![TransactionArgument::Address(validator_address)],
    )
}

/// Permanently destroy the `Token`s stored in the oldest burn request under the `Preburn`
/// resource. This will only succeed if `account` has a `MintCapability<Token>`, a
/// `Preburn<Token>` resource exists under `preburn_address`, and there is a pending burn
/// request. sliding_nonce is a unique nonce for operation, see sliding_nonce.move for
/// details
pub fn encode_burn_script(
    token: TypeTag,
    sliding_nonce: u64,
    preburn_address: AccountAddress,
) -> Script {
    Script::new(
        vec![
            161, 28, 235, 11, 1, 0, 6, 1, 0, 4, 3, 4, 11, 4, 15, 2, 5, 17, 17, 7, 34, 46, 8, 80,
            16, 0, 0, 0, 1, 0, 2, 0, 1, 1, 1, 1, 3, 2, 1, 0, 0, 4, 2, 6, 12, 5, 0, 2, 6, 12, 3, 3,
            6, 12, 3, 5, 1, 9, 0, 5, 76, 105, 98, 114, 97, 12, 83, 108, 105, 100, 105, 110, 103,
            78, 111, 110, 99, 101, 4, 98, 117, 114, 110, 21, 114, 101, 99, 111, 114, 100, 95, 110,
            111, 110, 99, 101, 95, 111, 114, 95, 97, 98, 111, 114, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 1, 1, 1, 3, 1, 7, 10, 0, 10, 1, 17, 1, 11, 0, 10, 2, 56, 0, 2,
        ],
        vec![token],
        vec![
            TransactionArgument::U64(sliding_nonce),
            TransactionArgument::Address(preburn_address),
        ],
    )
}

/// Burn transaction fees that have been collected in the given `currency` and relinquish
/// to the association. The currency must be non-synthetic.
pub fn encode_burn_txn_fees_script(coin_type: TypeTag) -> Script {
    Script::new(
        vec![
            161, 28, 235, 11, 1, 0, 7, 1, 0, 4, 2, 4, 9, 3, 13, 18, 4, 31, 6, 5, 37, 39, 7, 76,
            129, 1, 8, 205, 1, 16, 0, 0, 0, 1, 0, 2, 1, 1, 3, 0, 3, 1, 0, 0, 4, 0, 1, 1, 3, 0, 5,
            2, 3, 1, 3, 1, 6, 4, 3, 1, 1, 0, 6, 2, 7, 1, 6, 1, 6, 12, 1, 11, 0, 1, 9, 0, 2, 6, 12,
            11, 0, 1, 9, 0, 0, 2, 6, 12, 6, 11, 0, 1, 8, 1, 1, 11, 0, 1, 8, 1, 1, 8, 1, 1, 9, 0, 5,
            82, 111, 108, 101, 115, 14, 84, 114, 97, 110, 115, 97, 99, 116, 105, 111, 110, 70, 101,
            101, 10, 67, 97, 112, 97, 98, 105, 108, 105, 116, 121, 22, 84, 114, 101, 97, 115, 117,
            114, 121, 67, 111, 109, 112, 108, 105, 97, 110, 99, 101, 82, 111, 108, 101, 31, 101,
            120, 116, 114, 97, 99, 116, 95, 112, 114, 105, 118, 105, 108, 101, 103, 101, 95, 116,
            111, 95, 99, 97, 112, 97, 98, 105, 108, 105, 116, 121, 31, 114, 101, 115, 116, 111,
            114, 101, 95, 99, 97, 112, 97, 98, 105, 108, 105, 116, 121, 95, 116, 111, 95, 112, 114,
            105, 118, 105, 108, 101, 103, 101, 9, 98, 117, 114, 110, 95, 102, 101, 101, 115, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 5, 10, 10, 0, 56, 0, 12, 1, 10, 0,
            14, 1, 56, 1, 11, 0, 11, 1, 56, 2, 2,
        ],
        vec![coin_type],
        vec![],
    )
}

/// Cancel the oldest burn request from `preburn_address` and return the funds. Fails if
/// the sender does not have a published `BurnCapability<Token>`.
pub fn encode_cancel_burn_script(token: TypeTag, preburn_address: AccountAddress) -> Script {
    Script::new(
        vec![
            161, 28, 235, 11, 1, 0, 6, 1, 0, 2, 3, 2, 6, 4, 8, 2, 5, 10, 8, 7, 18, 25, 8, 43, 16,
            0, 0, 0, 1, 0, 1, 1, 1, 0, 2, 2, 6, 12, 5, 0, 1, 9, 0, 12, 76, 105, 98, 114, 97, 65,
            99, 99, 111, 117, 110, 116, 11, 99, 97, 110, 99, 101, 108, 95, 98, 117, 114, 110, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 1, 4, 11, 0, 10, 1, 56, 0, 2,
        ],
        vec![token],
        vec![TransactionArgument::Address(preburn_address)],
    )
}

/// Create a `ChildVASP` account for sender `parent_vasp` at `child_address` with a
/// balance of `child_initial_balance` in `CoinType` and an initial authentication_key
/// `auth_key_prefix | child_address`. If `add_all_currencies` is true, the child address
/// will have a zero balance in all available currencies in the system. This account will
/// a child of the transaction sender, which must be a ParentVASP.
pub fn encode_create_child_vasp_account_script(
    coin_type: TypeTag,
    child_address: AccountAddress,
    auth_key_prefix: Vec<u8>,
    add_all_currencies: bool,
    child_initial_balance: u64,
) -> Script {
    Script::new(
        vec![
            161, 28, 235, 11, 1, 0, 7, 1, 0, 4, 2, 4, 13, 3, 17, 34, 4, 51, 8, 5, 59, 62, 7, 121,
            219, 1, 8, 212, 2, 16, 0, 0, 0, 1, 1, 2, 1, 1, 3, 1, 3, 1, 0, 0, 6, 1, 0, 1, 4, 0, 1,
            1, 3, 1, 5, 2, 3, 1, 3, 0, 7, 4, 3, 1, 1, 0, 8, 0, 5, 0, 0, 9, 6, 3, 1, 1, 0, 10, 5, 3,
            0, 0, 9, 2, 10, 4, 10, 1, 9, 1, 6, 12, 1, 11, 0, 1, 9, 0, 2, 6, 12, 11, 0, 1, 9, 0, 0,
            5, 6, 12, 6, 11, 0, 1, 8, 1, 5, 10, 2, 1, 1, 8, 2, 3, 6, 8, 2, 5, 3, 5, 6, 12, 5, 10,
            2, 1, 3, 2, 11, 0, 1, 8, 1, 8, 2, 1, 8, 1, 1, 9, 0, 12, 76, 105, 98, 114, 97, 65, 99,
            99, 111, 117, 110, 116, 5, 82, 111, 108, 101, 115, 10, 67, 97, 112, 97, 98, 105, 108,
            105, 116, 121, 14, 80, 97, 114, 101, 110, 116, 86, 65, 83, 80, 82, 111, 108, 101, 31,
            101, 120, 116, 114, 97, 99, 116, 95, 112, 114, 105, 118, 105, 108, 101, 103, 101, 95,
            116, 111, 95, 99, 97, 112, 97, 98, 105, 108, 105, 116, 121, 31, 114, 101, 115, 116,
            111, 114, 101, 95, 99, 97, 112, 97, 98, 105, 108, 105, 116, 121, 95, 116, 111, 95, 112,
            114, 105, 118, 105, 108, 101, 103, 101, 18, 87, 105, 116, 104, 100, 114, 97, 119, 67,
            97, 112, 97, 98, 105, 108, 105, 116, 121, 25, 99, 114, 101, 97, 116, 101, 95, 99, 104,
            105, 108, 100, 95, 118, 97, 115, 112, 95, 97, 99, 99, 111, 117, 110, 116, 27, 101, 120,
            116, 114, 97, 99, 116, 95, 119, 105, 116, 104, 100, 114, 97, 119, 95, 99, 97, 112, 97,
            98, 105, 108, 105, 116, 121, 8, 112, 97, 121, 95, 102, 114, 111, 109, 27, 114, 101,
            115, 116, 111, 114, 101, 95, 119, 105, 116, 104, 100, 114, 97, 119, 95, 99, 97, 112,
            97, 98, 105, 108, 105, 116, 121, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1,
            7, 8, 27, 10, 0, 56, 0, 12, 5, 10, 0, 14, 5, 10, 1, 11, 2, 10, 3, 56, 1, 10, 4, 6, 0,
            0, 0, 0, 0, 0, 0, 0, 36, 3, 14, 5, 23, 10, 0, 17, 3, 12, 6, 14, 6, 10, 1, 10, 4, 56, 2,
            11, 6, 17, 5, 11, 0, 11, 5, 56, 3, 2,
        ],
        vec![coin_type],
        vec![
            TransactionArgument::Address(child_address),
            TransactionArgument::U8Vector(auth_key_prefix),
            TransactionArgument::Bool(add_all_currencies),
            TransactionArgument::U64(child_initial_balance),
        ],
    )
}

/// Create designated dealer account at 'new_account_address' and 'auth_key_prefix' for
/// nonsynthetic CoinType. Create dealer and preburn resource.
pub fn encode_create_designated_dealer_script(
    coin_type: TypeTag,
    sliding_nonce: u64,
    new_account_address: AccountAddress,
    auth_key_prefix: Vec<u8>,
) -> Script {
    Script::new(
        vec![
            161, 28, 235, 11, 1, 0, 7, 1, 0, 8, 2, 8, 9, 3, 17, 28, 4, 45, 6, 5, 51, 62, 7, 113,
            203, 1, 8, 188, 2, 16, 0, 0, 0, 1, 0, 2, 0, 3, 2, 4, 1, 1, 3, 2, 5, 1, 0, 2, 6, 0, 1,
            1, 3, 2, 7, 2, 3, 1, 3, 0, 8, 4, 3, 0, 3, 9, 5, 3, 0, 1, 10, 6, 3, 1, 1, 0, 9, 4, 10,
            1, 9, 1, 6, 12, 1, 11, 0, 1, 9, 0, 2, 6, 12, 11, 0, 1, 9, 0, 0, 3, 6, 11, 0, 1, 8, 1,
            5, 3, 2, 6, 12, 3, 4, 6, 12, 6, 11, 0, 1, 8, 1, 5, 10, 2, 4, 6, 12, 3, 5, 10, 2, 1, 11,
            0, 1, 8, 1, 1, 8, 1, 1, 9, 0, 16, 68, 101, 115, 105, 103, 110, 97, 116, 101, 100, 68,
            101, 97, 108, 101, 114, 12, 76, 105, 98, 114, 97, 65, 99, 99, 111, 117, 110, 116, 5,
            82, 111, 108, 101, 115, 12, 83, 108, 105, 100, 105, 110, 103, 78, 111, 110, 99, 101,
            10, 67, 97, 112, 97, 98, 105, 108, 105, 116, 121, 22, 84, 114, 101, 97, 115, 117, 114,
            121, 67, 111, 109, 112, 108, 105, 97, 110, 99, 101, 82, 111, 108, 101, 31, 101, 120,
            116, 114, 97, 99, 116, 95, 112, 114, 105, 118, 105, 108, 101, 103, 101, 95, 116, 111,
            95, 99, 97, 112, 97, 98, 105, 108, 105, 116, 121, 31, 114, 101, 115, 116, 111, 114,
            101, 95, 99, 97, 112, 97, 98, 105, 108, 105, 116, 121, 95, 116, 111, 95, 112, 114, 105,
            118, 105, 108, 101, 103, 101, 8, 97, 100, 100, 95, 116, 105, 101, 114, 21, 114, 101,
            99, 111, 114, 100, 95, 110, 111, 110, 99, 101, 95, 111, 114, 95, 97, 98, 111, 114, 116,
            24, 99, 114, 101, 97, 116, 101, 95, 100, 101, 115, 105, 103, 110, 97, 116, 101, 100,
            95, 100, 101, 97, 108, 101, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1,
            7, 8, 31, 10, 0, 10, 1, 17, 3, 10, 0, 56, 0, 12, 4, 10, 0, 14, 4, 10, 2, 11, 3, 56, 1,
            14, 4, 10, 2, 6, 32, 161, 7, 0, 0, 0, 0, 0, 17, 2, 14, 4, 10, 2, 6, 64, 75, 76, 0, 0,
            0, 0, 0, 17, 2, 14, 4, 10, 2, 6, 128, 240, 250, 2, 0, 0, 0, 0, 17, 2, 14, 4, 10, 2, 6,
            0, 101, 205, 29, 0, 0, 0, 0, 17, 2, 11, 0, 11, 4, 56, 2, 2,
        ],
        vec![coin_type],
        vec![
            TransactionArgument::U64(sliding_nonce),
            TransactionArgument::Address(new_account_address),
            TransactionArgument::U8Vector(auth_key_prefix),
        ],
    )
}

/// Create an account with the ParentVASP role at `address` with authentication key
/// `auth_key_prefix` | `new_account_address` and a 0 balance of type `currency`. If
/// `add_all_currencies` is true, 0 balances for all available currencies in the system
/// will also be added. This can only be invoked by an Association account.
pub fn encode_create_parent_vasp_account_script(
    coin_type: TypeTag,
    new_account_address: AccountAddress,
    auth_key_prefix: Vec<u8>,
    human_name: Vec<u8>,
    base_url: Vec<u8>,
    compliance_public_key: Vec<u8>,
    add_all_currencies: bool,
) -> Script {
    Script::new(
        vec![
            161, 28, 235, 11, 1, 0, 7, 1, 0, 4, 2, 4, 9, 3, 13, 18, 4, 31, 6, 5, 37, 62, 7, 99,
            135, 1, 8, 234, 1, 16, 0, 0, 0, 1, 1, 2, 1, 1, 3, 1, 3, 1, 0, 1, 4, 0, 1, 1, 3, 1, 5,
            2, 3, 1, 3, 0, 6, 4, 3, 1, 1, 0, 7, 2, 8, 1, 7, 1, 6, 12, 1, 11, 0, 1, 9, 0, 2, 6, 12,
            11, 0, 1, 9, 0, 0, 8, 6, 12, 6, 11, 0, 1, 8, 1, 5, 10, 2, 10, 2, 10, 2, 10, 2, 1, 7, 6,
            12, 5, 10, 2, 10, 2, 10, 2, 10, 2, 1, 1, 11, 0, 1, 8, 1, 1, 8, 1, 1, 9, 0, 12, 76, 105,
            98, 114, 97, 65, 99, 99, 111, 117, 110, 116, 5, 82, 111, 108, 101, 115, 10, 67, 97,
            112, 97, 98, 105, 108, 105, 116, 121, 13, 76, 105, 98, 114, 97, 82, 111, 111, 116, 82,
            111, 108, 101, 31, 101, 120, 116, 114, 97, 99, 116, 95, 112, 114, 105, 118, 105, 108,
            101, 103, 101, 95, 116, 111, 95, 99, 97, 112, 97, 98, 105, 108, 105, 116, 121, 31, 114,
            101, 115, 116, 111, 114, 101, 95, 99, 97, 112, 97, 98, 105, 108, 105, 116, 121, 95,
            116, 111, 95, 112, 114, 105, 118, 105, 108, 101, 103, 101, 26, 99, 114, 101, 97, 116,
            101, 95, 112, 97, 114, 101, 110, 116, 95, 118, 97, 115, 112, 95, 97, 99, 99, 111, 117,
            110, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 5, 6, 16, 10, 0, 56, 0,
            12, 7, 10, 0, 14, 7, 10, 1, 11, 2, 11, 3, 11, 4, 11, 5, 10, 6, 56, 1, 11, 0, 11, 7, 56,
            2, 2,
        ],
        vec![coin_type],
        vec![
            TransactionArgument::Address(new_account_address),
            TransactionArgument::U8Vector(auth_key_prefix),
            TransactionArgument::U8Vector(human_name),
            TransactionArgument::U8Vector(base_url),
            TransactionArgument::U8Vector(compliance_public_key),
            TransactionArgument::Bool(add_all_currencies),
        ],
    )
}

/// Extract the `KeyRotationCapability` for `recovery_account` and publish it in a
/// `RecoveryAddress` resource under `recovery_account`. Aborts if `recovery_account` has
/// delegated its `KeyRotationCapability`, already has a `RecoveryAddress` resource, or is
/// not a VASP.
pub fn encode_create_recovery_address_script() -> Script {
    Script::new(
        vec![
            161, 28, 235, 11, 1, 0, 5, 1, 0, 2, 3, 2, 5, 5, 7, 4, 7, 11, 24, 8, 35, 16, 0, 0, 0, 1,
            0, 1, 0, 1, 6, 12, 0, 15, 82, 101, 99, 111, 118, 101, 114, 121, 65, 100, 100, 114, 101,
            115, 115, 7, 112, 117, 98, 108, 105, 115, 104, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 1, 0, 0, 1, 3, 11, 0, 17, 0, 2,
        ],
        vec![],
        vec![],
    )
}

/// Create an account with the ParentVASP role at `address` with authentication key
/// `auth_key_prefix` | `new_account_address` and a 0 balance of type `currency`. If
/// `add_all_currencies` is true, 0 balances for all available currencies in the system
/// will also be added. This can only be invoked by an Association account. The
/// `human_name`, `base_url`, and compliance_public_key` fields of the ParentVASP are
/// filled in with dummy information.
pub fn encode_create_testing_account_script(
    coin_type: TypeTag,
    new_account_address: AccountAddress,
    auth_key_prefix: Vec<u8>,
    add_all_currencies: bool,
) -> Script {
    Script::new(
        vec![
            161, 28, 235, 11, 1, 0, 8, 1, 0, 4, 2, 4, 9, 3, 13, 18, 4, 31, 6, 5, 37, 56, 7, 93,
            135, 1, 8, 228, 1, 16, 6, 244, 1, 68, 0, 0, 0, 1, 1, 2, 1, 1, 3, 1, 3, 1, 0, 1, 4, 0,
            1, 1, 3, 1, 5, 2, 3, 1, 3, 0, 6, 4, 3, 1, 1, 0, 7, 2, 8, 1, 7, 1, 6, 12, 1, 11, 0, 1,
            9, 0, 2, 6, 12, 11, 0, 1, 9, 0, 0, 8, 6, 12, 6, 11, 0, 1, 8, 1, 5, 10, 2, 10, 2, 10, 2,
            10, 2, 1, 4, 6, 12, 5, 10, 2, 1, 1, 11, 0, 1, 8, 1, 1, 8, 1, 1, 9, 0, 12, 76, 105, 98,
            114, 97, 65, 99, 99, 111, 117, 110, 116, 5, 82, 111, 108, 101, 115, 10, 67, 97, 112,
            97, 98, 105, 108, 105, 116, 121, 13, 76, 105, 98, 114, 97, 82, 111, 111, 116, 82, 111,
            108, 101, 31, 101, 120, 116, 114, 97, 99, 116, 95, 112, 114, 105, 118, 105, 108, 101,
            103, 101, 95, 116, 111, 95, 99, 97, 112, 97, 98, 105, 108, 105, 116, 121, 31, 114, 101,
            115, 116, 111, 114, 101, 95, 99, 97, 112, 97, 98, 105, 108, 105, 116, 121, 95, 116,
            111, 95, 112, 114, 105, 118, 105, 108, 101, 103, 101, 26, 99, 114, 101, 97, 116, 101,
            95, 112, 97, 114, 101, 110, 116, 95, 118, 97, 115, 112, 95, 97, 99, 99, 111, 117, 110,
            116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 10, 2, 8, 7, 116, 101, 115, 116,
            110, 101, 116, 10, 2, 18, 17, 104, 116, 116, 112, 115, 58, 47, 47, 108, 105, 98, 114,
            97, 46, 111, 114, 103, 10, 2, 33, 32, 183, 163, 193, 45, 192, 200, 199, 72, 171, 7, 82,
            91, 112, 17, 34, 184, 139, 215, 143, 96, 12, 118, 52, 45, 39, 242, 94, 95, 146, 68, 76,
            222, 1, 1, 5, 6, 16, 10, 0, 56, 0, 12, 4, 10, 0, 14, 4, 10, 1, 11, 2, 7, 0, 7, 1, 7, 2,
            10, 3, 56, 1, 11, 0, 11, 4, 56, 2, 2,
        ],
        vec![coin_type],
        vec![
            TransactionArgument::Address(new_account_address),
            TransactionArgument::U8Vector(auth_key_prefix),
            TransactionArgument::Bool(add_all_currencies),
        ],
    )
}

/// Create a validator account at `new_validator_address` with `auth_key_prefix`.
pub fn encode_create_validator_account_script(
    new_account_address: AccountAddress,
    auth_key_prefix: Vec<u8>,
) -> Script {
    Script::new(
        vec![
            161, 28, 235, 11, 1, 0, 7, 1, 0, 4, 2, 4, 9, 3, 13, 17, 4, 30, 4, 5, 34, 45, 7, 79,
            133, 1, 8, 212, 1, 16, 0, 0, 0, 1, 1, 2, 1, 1, 3, 1, 3, 1, 0, 1, 4, 0, 1, 1, 3, 1, 5,
            2, 3, 1, 3, 0, 6, 4, 3, 0, 0, 7, 1, 7, 1, 6, 12, 1, 11, 0, 1, 9, 0, 2, 6, 12, 11, 0, 1,
            9, 0, 0, 4, 6, 12, 6, 11, 0, 1, 8, 1, 5, 10, 2, 3, 6, 12, 5, 10, 2, 1, 11, 0, 1, 8, 1,
            1, 8, 1, 12, 76, 105, 98, 114, 97, 65, 99, 99, 111, 117, 110, 116, 5, 82, 111, 108,
            101, 115, 10, 67, 97, 112, 97, 98, 105, 108, 105, 116, 121, 13, 76, 105, 98, 114, 97,
            82, 111, 111, 116, 82, 111, 108, 101, 31, 101, 120, 116, 114, 97, 99, 116, 95, 112,
            114, 105, 118, 105, 108, 101, 103, 101, 95, 116, 111, 95, 99, 97, 112, 97, 98, 105,
            108, 105, 116, 121, 31, 114, 101, 115, 116, 111, 114, 101, 95, 99, 97, 112, 97, 98,
            105, 108, 105, 116, 121, 95, 116, 111, 95, 112, 114, 105, 118, 105, 108, 101, 103, 101,
            24, 99, 114, 101, 97, 116, 101, 95, 118, 97, 108, 105, 100, 97, 116, 111, 114, 95, 97,
            99, 99, 111, 117, 110, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 5, 6,
            12, 10, 0, 56, 0, 12, 3, 10, 0, 14, 3, 10, 1, 11, 2, 17, 2, 11, 0, 11, 3, 56, 1, 2,
        ],
        vec![],
        vec![
            TransactionArgument::Address(new_account_address),
            TransactionArgument::U8Vector(auth_key_prefix),
        ],
    )
}

/// Create a validator operator account at `new_validator_address` with `auth_key_prefix`.
pub fn encode_create_validator_operator_account_script(
    token: TypeTag,
    new_account_address: AccountAddress,
    auth_key_prefix: Vec<u8>,
) -> Script {
    Script::new(
        vec![
            161, 28, 235, 11, 1, 0, 7, 1, 0, 4, 2, 4, 9, 3, 13, 17, 4, 30, 4, 5, 34, 45, 7, 79,
            142, 1, 8, 221, 1, 16, 0, 0, 0, 1, 1, 2, 1, 1, 3, 1, 3, 1, 0, 1, 4, 0, 1, 1, 3, 1, 5,
            2, 3, 1, 3, 0, 6, 4, 3, 0, 0, 7, 1, 7, 1, 6, 12, 1, 11, 0, 1, 9, 0, 2, 6, 12, 11, 0, 1,
            9, 0, 0, 4, 6, 12, 6, 11, 0, 1, 8, 1, 5, 10, 2, 3, 6, 12, 5, 10, 2, 1, 11, 0, 1, 8, 1,
            1, 8, 1, 12, 76, 105, 98, 114, 97, 65, 99, 99, 111, 117, 110, 116, 5, 82, 111, 108,
            101, 115, 10, 67, 97, 112, 97, 98, 105, 108, 105, 116, 121, 13, 76, 105, 98, 114, 97,
            82, 111, 111, 116, 82, 111, 108, 101, 31, 101, 120, 116, 114, 97, 99, 116, 95, 112,
            114, 105, 118, 105, 108, 101, 103, 101, 95, 116, 111, 95, 99, 97, 112, 97, 98, 105,
            108, 105, 116, 121, 31, 114, 101, 115, 116, 111, 114, 101, 95, 99, 97, 112, 97, 98,
            105, 108, 105, 116, 121, 95, 116, 111, 95, 112, 114, 105, 118, 105, 108, 101, 103, 101,
            33, 99, 114, 101, 97, 116, 101, 95, 118, 97, 108, 105, 100, 97, 116, 111, 114, 95, 111,
            112, 101, 114, 97, 116, 111, 114, 95, 97, 99, 99, 111, 117, 110, 116, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 5, 6, 12, 10, 0, 56, 0, 12, 3, 10, 0, 14, 3, 10, 1,
            11, 2, 17, 2, 11, 0, 11, 3, 56, 1, 2,
        ],
        vec![token],
        vec![
            TransactionArgument::Address(new_account_address),
            TransactionArgument::U8Vector(auth_key_prefix),
        ],
    )
}

/// Used for testing.
pub fn encode_empty_script_script() -> Script {
    Script::new(
        vec![161, 28, 235, 11, 1, 0, 1, 5, 0, 1, 0, 0, 0, 0, 1, 2],
        vec![],
        vec![],
    )
}

/// Freeze account `address`. Initiator must be authorized. `sliding_nonce` is a unique
/// nonce for operation, see sliding_nonce.move for details.
pub fn encode_freeze_account_script(
    sliding_nonce: u64,
    to_freeze_account: AccountAddress,
) -> Script {
    Script::new(
        vec![
            161, 28, 235, 11, 1, 0, 7, 1, 0, 6, 2, 6, 9, 3, 15, 22, 4, 37, 4, 5, 41, 46, 7, 87,
            160, 1, 8, 247, 1, 16, 0, 0, 0, 1, 0, 2, 1, 3, 1, 1, 3, 0, 7, 1, 0, 1, 4, 0, 1, 1, 3,
            1, 5, 2, 3, 1, 3, 2, 6, 4, 3, 0, 0, 8, 5, 3, 0, 0, 8, 1, 8, 1, 6, 12, 1, 11, 0, 1, 9,
            0, 2, 6, 12, 11, 0, 1, 9, 0, 0, 2, 6, 12, 3, 3, 6, 12, 6, 11, 0, 1, 8, 1, 5, 3, 6, 12,
            3, 5, 1, 11, 0, 1, 8, 1, 1, 8, 1, 12, 76, 105, 98, 114, 97, 65, 99, 99, 111, 117, 110,
            116, 5, 82, 111, 108, 101, 115, 12, 83, 108, 105, 100, 105, 110, 103, 78, 111, 110, 99,
            101, 10, 67, 97, 112, 97, 98, 105, 108, 105, 116, 121, 31, 101, 120, 116, 114, 97, 99,
            116, 95, 112, 114, 105, 118, 105, 108, 101, 103, 101, 95, 116, 111, 95, 99, 97, 112,
            97, 98, 105, 108, 105, 116, 121, 31, 114, 101, 115, 116, 111, 114, 101, 95, 99, 97,
            112, 97, 98, 105, 108, 105, 116, 121, 95, 116, 111, 95, 112, 114, 105, 118, 105, 108,
            101, 103, 101, 21, 114, 101, 99, 111, 114, 100, 95, 110, 111, 110, 99, 101, 95, 111,
            114, 95, 97, 98, 111, 114, 116, 15, 65, 99, 99, 111, 117, 110, 116, 70, 114, 101, 101,
            122, 105, 110, 103, 14, 102, 114, 101, 101, 122, 101, 95, 97, 99, 99, 111, 117, 110,
            116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 6, 7, 14, 10, 0, 10, 1, 17, 2,
            10, 0, 56, 0, 12, 3, 10, 0, 14, 3, 10, 2, 17, 3, 11, 0, 11, 3, 56, 1, 2,
        ],
        vec![],
        vec![
            TransactionArgument::U64(sliding_nonce),
            TransactionArgument::Address(to_freeze_account),
        ],
    )
}

/// Mint `amount_lbr` LBR from the sending account's constituent coins and deposits the
/// resulting LBR into the sending account.
pub fn encode_mint_lbr_script(amount_lbr: u64) -> Script {
    Script::new(
        vec![
            161, 28, 235, 11, 1, 0, 7, 1, 0, 12, 2, 12, 21, 3, 33, 38, 4, 71, 14, 5, 85, 102, 7,
            187, 1, 168, 1, 8, 227, 2, 16, 0, 0, 0, 1, 0, 2, 0, 3, 0, 4, 0, 5, 3, 3, 1, 1, 1, 0, 0,
            2, 0, 1, 1, 2, 0, 2, 2, 1, 0, 4, 8, 1, 0, 5, 6, 0, 1, 0, 2, 7, 2, 3, 0, 4, 9, 1, 4, 1,
            1, 4, 10, 5, 6, 1, 1, 4, 11, 0, 7, 0, 4, 12, 7, 6, 0, 4, 13, 8, 9, 1, 1, 2, 12, 2, 13,
            6, 12, 6, 13, 3, 14, 3, 12, 3, 13, 1, 6, 12, 1, 5, 3, 3, 11, 0, 1, 8, 1, 11, 0, 1, 8,
            2, 3, 11, 0, 1, 8, 3, 11, 0, 1, 8, 1, 11, 0, 1, 8, 2, 1, 3, 2, 6, 12, 11, 0, 1, 9, 0,
            0, 1, 8, 4, 2, 6, 8, 4, 3, 1, 11, 0, 1, 9, 0, 2, 6, 12, 3, 9, 11, 0, 1, 8, 1, 11, 0, 1,
            8, 1, 3, 11, 0, 1, 8, 2, 11, 0, 1, 8, 2, 3, 11, 0, 1, 8, 3, 5, 8, 4, 1, 8, 1, 1, 8, 2,
            1, 8, 3, 5, 67, 111, 105, 110, 49, 5, 67, 111, 105, 110, 50, 3, 76, 66, 82, 5, 76, 105,
            98, 114, 97, 12, 76, 105, 98, 114, 97, 65, 99, 99, 111, 117, 110, 116, 6, 83, 105, 103,
            110, 101, 114, 10, 97, 100, 100, 114, 101, 115, 115, 95, 111, 102, 6, 99, 114, 101, 97,
            116, 101, 18, 87, 105, 116, 104, 100, 114, 97, 119, 67, 97, 112, 97, 98, 105, 108, 105,
            116, 121, 7, 98, 97, 108, 97, 110, 99, 101, 10, 100, 101, 112, 111, 115, 105, 116, 95,
            116, 111, 27, 101, 120, 116, 114, 97, 99, 116, 95, 119, 105, 116, 104, 100, 114, 97,
            119, 95, 99, 97, 112, 97, 98, 105, 108, 105, 116, 121, 27, 114, 101, 115, 116, 111,
            114, 101, 95, 119, 105, 116, 104, 100, 114, 97, 119, 95, 99, 97, 112, 97, 98, 105, 108,
            105, 116, 121, 13, 119, 105, 116, 104, 100, 114, 97, 119, 95, 102, 114, 111, 109, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 10, 11, 39, 10, 0, 17, 0, 12, 9, 10, 9,
            56, 0, 12, 4, 10, 9, 56, 1, 12, 7, 10, 0, 17, 4, 12, 10, 14, 10, 10, 4, 56, 2, 12, 2,
            14, 10, 10, 7, 56, 3, 12, 5, 11, 10, 17, 5, 10, 1, 11, 2, 11, 5, 17, 1, 12, 6, 12, 3,
            12, 8, 10, 0, 11, 8, 56, 4, 10, 0, 11, 3, 56, 5, 11, 0, 11, 6, 56, 6, 2,
        ],
        vec![],
        vec![TransactionArgument::U64(amount_lbr)],
    )
}

/// Modify publishing options. Takes the LCS bytes of a `VMPublishingOption` object as
/// input.
pub fn encode_modify_publishing_option_script(args: Vec<u8>) -> Script {
    Script::new(
        vec![
            161, 28, 235, 11, 1, 0, 5, 1, 0, 2, 3, 2, 5, 5, 7, 6, 7, 13, 36, 8, 49, 16, 0, 0, 0, 1,
            0, 1, 0, 2, 6, 12, 10, 2, 0, 13, 76, 105, 98, 114, 97, 86, 77, 67, 111, 110, 102, 105,
            103, 21, 115, 101, 116, 95, 112, 117, 98, 108, 105, 115, 104, 105, 110, 103, 95, 111,
            112, 116, 105, 111, 110, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 4,
            11, 0, 11, 1, 17, 0, 2,
        ],
        vec![],
        vec![TransactionArgument::U8Vector(args)],
    )
}

/// Transfer `amount` coins to `recipient_address` with (optional) associated metadata
/// `metadata` and (optional) `signature` on the metadata, amount, and sender address. The
/// `metadata` and `signature` parameters are only required if `amount` >= 1_000_000 micro
/// LBR and the sender and recipient of the funds are two distinct VASPs. Fails if there
/// is no account at the recipient address or if the sender's balance is lower than
/// `amount`.
pub fn encode_peer_to_peer_with_metadata_script(
    token: TypeTag,
    payee: AccountAddress,
    amount: u64,
    metadata: Vec<u8>,
    metadata_signature: Vec<u8>,
) -> Script {
    Script::new(
        vec![
            161, 28, 235, 11, 1, 0, 7, 1, 0, 2, 2, 2, 4, 3, 6, 16, 4, 22, 2, 5, 24, 29, 7, 53, 111,
            8, 164, 1, 16, 0, 0, 0, 1, 1, 0, 0, 2, 0, 1, 0, 0, 3, 2, 3, 1, 1, 0, 4, 1, 3, 0, 1, 5,
            1, 6, 12, 1, 8, 0, 5, 6, 8, 0, 5, 3, 10, 2, 10, 2, 0, 5, 6, 12, 5, 3, 10, 2, 10, 2, 1,
            9, 0, 12, 76, 105, 98, 114, 97, 65, 99, 99, 111, 117, 110, 116, 18, 87, 105, 116, 104,
            100, 114, 97, 119, 67, 97, 112, 97, 98, 105, 108, 105, 116, 121, 27, 101, 120, 116,
            114, 97, 99, 116, 95, 119, 105, 116, 104, 100, 114, 97, 119, 95, 99, 97, 112, 97, 98,
            105, 108, 105, 116, 121, 22, 112, 97, 121, 95, 102, 114, 111, 109, 95, 119, 105, 116,
            104, 95, 109, 101, 116, 97, 100, 97, 116, 97, 27, 114, 101, 115, 116, 111, 114, 101,
            95, 119, 105, 116, 104, 100, 114, 97, 119, 95, 99, 97, 112, 97, 98, 105, 108, 105, 116,
            121, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 4, 1, 12, 11, 0, 17, 0, 12,
            5, 14, 5, 10, 1, 10, 2, 11, 3, 11, 4, 56, 0, 11, 5, 17, 2, 2,
        ],
        vec![token],
        vec![
            TransactionArgument::Address(payee),
            TransactionArgument::U64(amount),
            TransactionArgument::U8Vector(metadata),
            TransactionArgument::U8Vector(metadata_signature),
        ],
    )
}

/// Preburn `amount` `Token`s from `account`. This will only succeed if `account` already
/// has a published `Preburn<Token>` resource.
pub fn encode_preburn_script(token: TypeTag, amount: u64) -> Script {
    Script::new(
        vec![
            161, 28, 235, 11, 1, 0, 7, 1, 0, 4, 2, 4, 9, 3, 13, 22, 4, 35, 4, 5, 39, 33, 7, 72,
            119, 8, 191, 1, 16, 0, 0, 0, 1, 0, 0, 1, 1, 1, 1, 3, 1, 0, 0, 2, 0, 1, 1, 1, 1, 4, 2,
            3, 0, 1, 5, 3, 1, 0, 1, 6, 4, 5, 1, 1, 3, 7, 0, 7, 2, 6, 12, 11, 0, 1, 9, 0, 0, 1, 6,
            12, 1, 8, 1, 2, 6, 8, 1, 3, 1, 11, 0, 1, 9, 0, 2, 6, 12, 3, 1, 9, 0, 5, 76, 105, 98,
            114, 97, 12, 76, 105, 98, 114, 97, 65, 99, 99, 111, 117, 110, 116, 10, 112, 114, 101,
            98, 117, 114, 110, 95, 116, 111, 18, 87, 105, 116, 104, 100, 114, 97, 119, 67, 97, 112,
            97, 98, 105, 108, 105, 116, 121, 27, 101, 120, 116, 114, 97, 99, 116, 95, 119, 105,
            116, 104, 100, 114, 97, 119, 95, 99, 97, 112, 97, 98, 105, 108, 105, 116, 121, 27, 114,
            101, 115, 116, 111, 114, 101, 95, 119, 105, 116, 104, 100, 114, 97, 119, 95, 99, 97,
            112, 97, 98, 105, 108, 105, 116, 121, 13, 119, 105, 116, 104, 100, 114, 97, 119, 95,
            102, 114, 111, 109, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 6, 3, 11, 10,
            0, 17, 1, 12, 2, 11, 0, 14, 2, 10, 1, 56, 0, 56, 1, 11, 2, 17, 2, 2,
        ],
        vec![token],
        vec![TransactionArgument::U64(amount)],
    )
}

/// (1) Rotate the authentication key of the sender to `public_key` (2) Publish a resource
/// containing a 32-byte ed25519 public key and the rotation capability of the sender
/// under the sender's address. Aborts if the sender already has a
/// `SharedEd25519PublicKey` resource. Aborts if the length of `new_public_key` is not 32.
pub fn encode_publish_shared_ed25519_public_key_script(public_key: Vec<u8>) -> Script {
    Script::new(
        vec![
            161, 28, 235, 11, 1, 0, 5, 1, 0, 2, 3, 2, 5, 5, 7, 6, 7, 13, 31, 8, 44, 16, 0, 0, 0, 1,
            0, 1, 0, 2, 6, 12, 10, 2, 0, 22, 83, 104, 97, 114, 101, 100, 69, 100, 50, 53, 53, 49,
            57, 80, 117, 98, 108, 105, 99, 75, 101, 121, 7, 112, 117, 98, 108, 105, 115, 104, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 4, 11, 0, 11, 1, 17, 0, 2,
        ],
        vec![],
        vec![TransactionArgument::U8Vector(public_key)],
    )
}

/// Update configs of all the validators and emit reconfiguration event.
pub fn encode_reconfigure_script() -> Script {
    Script::new(
        vec![
            161, 28, 235, 11, 1, 0, 7, 1, 0, 4, 2, 4, 9, 3, 13, 17, 4, 30, 4, 5, 34, 34, 7, 68,
            130, 1, 8, 198, 1, 16, 0, 0, 0, 1, 1, 2, 1, 1, 3, 1, 3, 1, 0, 1, 4, 0, 1, 1, 3, 1, 5,
            2, 3, 1, 3, 0, 6, 4, 3, 0, 0, 6, 1, 6, 1, 6, 12, 1, 11, 0, 1, 9, 0, 2, 6, 12, 11, 0, 1,
            9, 0, 0, 1, 6, 11, 0, 1, 8, 1, 1, 11, 0, 1, 8, 1, 1, 8, 1, 11, 76, 105, 98, 114, 97,
            83, 121, 115, 116, 101, 109, 5, 82, 111, 108, 101, 115, 10, 67, 97, 112, 97, 98, 105,
            108, 105, 116, 121, 13, 76, 105, 98, 114, 97, 82, 111, 111, 116, 82, 111, 108, 101, 31,
            101, 120, 116, 114, 97, 99, 116, 95, 112, 114, 105, 118, 105, 108, 101, 103, 101, 95,
            116, 111, 95, 99, 97, 112, 97, 98, 105, 108, 105, 116, 121, 31, 114, 101, 115, 116,
            111, 114, 101, 95, 99, 97, 112, 97, 98, 105, 108, 105, 116, 121, 95, 116, 111, 95, 112,
            114, 105, 118, 105, 108, 101, 103, 101, 22, 117, 112, 100, 97, 116, 101, 95, 97, 110,
            100, 95, 114, 101, 99, 111, 110, 102, 105, 103, 117, 114, 101, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 5, 9, 10, 0, 56, 0, 12, 1, 14, 1, 17, 2, 11, 0, 11, 1,
            56, 1, 2,
        ],
        vec![],
        vec![],
    )
}

/// Adding `to_remove` to the set of pending validator removals. Fails if the `to_remove`
/// address is already in the validator set or already in the pending removals. Callable
/// by Validator's operator.
pub fn encode_remove_validator_script(validator_address: AccountAddress) -> Script {
    Script::new(
        vec![
            161, 28, 235, 11, 1, 0, 7, 1, 0, 4, 2, 4, 9, 3, 13, 17, 4, 30, 4, 5, 34, 39, 7, 73,
            124, 8, 197, 1, 16, 0, 0, 0, 1, 1, 2, 1, 1, 3, 1, 3, 1, 0, 1, 4, 0, 1, 1, 3, 1, 5, 2,
            3, 1, 3, 0, 6, 4, 3, 0, 0, 7, 1, 7, 1, 6, 12, 1, 11, 0, 1, 9, 0, 2, 6, 12, 11, 0, 1, 9,
            0, 0, 2, 6, 11, 0, 1, 8, 1, 5, 2, 6, 12, 5, 1, 11, 0, 1, 8, 1, 1, 8, 1, 11, 76, 105,
            98, 114, 97, 83, 121, 115, 116, 101, 109, 5, 82, 111, 108, 101, 115, 10, 67, 97, 112,
            97, 98, 105, 108, 105, 116, 121, 13, 76, 105, 98, 114, 97, 82, 111, 111, 116, 82, 111,
            108, 101, 31, 101, 120, 116, 114, 97, 99, 116, 95, 112, 114, 105, 118, 105, 108, 101,
            103, 101, 95, 116, 111, 95, 99, 97, 112, 97, 98, 105, 108, 105, 116, 121, 31, 114, 101,
            115, 116, 111, 114, 101, 95, 99, 97, 112, 97, 98, 105, 108, 105, 116, 121, 95, 116,
            111, 95, 112, 114, 105, 118, 105, 108, 101, 103, 101, 16, 114, 101, 109, 111, 118, 101,
            95, 118, 97, 108, 105, 100, 97, 116, 111, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 1, 0, 5, 6, 10, 10, 0, 56, 0, 12, 2, 14, 2, 10, 1, 17, 2, 11, 0, 11, 2, 56, 1, 2,
        ],
        vec![],
        vec![TransactionArgument::Address(validator_address)],
    )
}

/// Rotate the sender's authentication key to `new_key`. `new_key` should be a 256 bit
/// sha3 hash of an ed25519 public key.
pub fn encode_rotate_authentication_key_script(new_key: Vec<u8>) -> Script {
    Script::new(
        vec![
            161, 28, 235, 11, 1, 0, 6, 1, 0, 2, 2, 2, 4, 3, 6, 15, 5, 21, 18, 7, 39, 125, 8, 164,
            1, 16, 0, 0, 0, 1, 1, 0, 0, 2, 0, 1, 0, 0, 3, 1, 2, 0, 0, 4, 3, 2, 0, 1, 6, 12, 1, 8,
            0, 0, 2, 6, 8, 0, 10, 2, 2, 6, 12, 10, 2, 12, 76, 105, 98, 114, 97, 65, 99, 99, 111,
            117, 110, 116, 21, 75, 101, 121, 82, 111, 116, 97, 116, 105, 111, 110, 67, 97, 112, 97,
            98, 105, 108, 105, 116, 121, 31, 101, 120, 116, 114, 97, 99, 116, 95, 107, 101, 121,
            95, 114, 111, 116, 97, 116, 105, 111, 110, 95, 99, 97, 112, 97, 98, 105, 108, 105, 116,
            121, 31, 114, 101, 115, 116, 111, 114, 101, 95, 107, 101, 121, 95, 114, 111, 116, 97,
            116, 105, 111, 110, 95, 99, 97, 112, 97, 98, 105, 108, 105, 116, 121, 25, 114, 111,
            116, 97, 116, 101, 95, 97, 117, 116, 104, 101, 110, 116, 105, 99, 97, 116, 105, 111,
            110, 95, 107, 101, 121, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 4, 1, 9, 11,
            0, 17, 0, 12, 2, 14, 2, 11, 1, 17, 2, 11, 2, 17, 1, 2,
        ],
        vec![],
        vec![TransactionArgument::U8Vector(new_key)],
    )
}

/// Rotate the sender's authentication key to `new_key`. `new_key` should be a 256 bit
/// sha3 hash of an ed25519 public key. This script also takes `sliding_nonce`, as a
/// unique nonce for this operation. See sliding_nonce.move for details.
pub fn encode_rotate_authentication_key_with_nonce_script(
    sliding_nonce: u64,
    new_key: Vec<u8>,
) -> Script {
    Script::new(
        vec![
            161, 28, 235, 11, 1, 0, 6, 1, 0, 4, 2, 4, 4, 3, 8, 20, 5, 28, 23, 7, 51, 160, 1, 8,
            211, 1, 16, 0, 0, 0, 1, 0, 3, 1, 0, 1, 2, 0, 1, 0, 0, 4, 2, 3, 0, 0, 5, 3, 1, 0, 0, 6,
            4, 1, 0, 2, 6, 12, 3, 0, 1, 6, 12, 1, 8, 0, 2, 6, 8, 0, 10, 2, 3, 6, 12, 3, 10, 2, 12,
            76, 105, 98, 114, 97, 65, 99, 99, 111, 117, 110, 116, 12, 83, 108, 105, 100, 105, 110,
            103, 78, 111, 110, 99, 101, 21, 114, 101, 99, 111, 114, 100, 95, 110, 111, 110, 99,
            101, 95, 111, 114, 95, 97, 98, 111, 114, 116, 21, 75, 101, 121, 82, 111, 116, 97, 116,
            105, 111, 110, 67, 97, 112, 97, 98, 105, 108, 105, 116, 121, 31, 101, 120, 116, 114,
            97, 99, 116, 95, 107, 101, 121, 95, 114, 111, 116, 97, 116, 105, 111, 110, 95, 99, 97,
            112, 97, 98, 105, 108, 105, 116, 121, 31, 114, 101, 115, 116, 111, 114, 101, 95, 107,
            101, 121, 95, 114, 111, 116, 97, 116, 105, 111, 110, 95, 99, 97, 112, 97, 98, 105, 108,
            105, 116, 121, 25, 114, 111, 116, 97, 116, 101, 95, 97, 117, 116, 104, 101, 110, 116,
            105, 99, 97, 116, 105, 111, 110, 95, 107, 101, 121, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 1, 0, 5, 3, 12, 10, 0, 10, 1, 17, 0, 11, 0, 17, 1, 12, 3, 14, 3, 11, 2, 17, 3,
            11, 3, 17, 2, 2,
        ],
        vec![],
        vec![
            TransactionArgument::U64(sliding_nonce),
            TransactionArgument::U8Vector(new_key),
        ],
    )
}

/// Rotate the authentication key of `to_recover` to `new_key`. Can be invoked by either
/// `recovery_address` or `to_recover`. Aborts if `recovery_address` does not have the
/// `KeyRotationCapability` for `to_recover`.
pub fn encode_rotate_authentication_key_with_recovery_address_script(
    recovery_address: AccountAddress,
    to_recover: AccountAddress,
    new_key: Vec<u8>,
) -> Script {
    Script::new(
        vec![
            161, 28, 235, 11, 1, 0, 5, 1, 0, 2, 3, 2, 5, 5, 7, 8, 7, 15, 42, 8, 57, 16, 0, 0, 0, 1,
            0, 1, 0, 4, 6, 12, 5, 5, 10, 2, 0, 15, 82, 101, 99, 111, 118, 101, 114, 121, 65, 100,
            100, 114, 101, 115, 115, 25, 114, 111, 116, 97, 116, 101, 95, 97, 117, 116, 104, 101,
            110, 116, 105, 99, 97, 116, 105, 111, 110, 95, 107, 101, 121, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 6, 11, 0, 10, 1, 10, 2, 11, 3, 17, 0, 2,
        ],
        vec![],
        vec![
            TransactionArgument::Address(recovery_address),
            TransactionArgument::Address(to_recover),
            TransactionArgument::U8Vector(new_key),
        ],
    )
}

/// Rotate `vasp_root_addr`'s base URL to `new_url`.
pub fn encode_rotate_base_url_script(new_url: Vec<u8>) -> Script {
    Script::new(
        vec![
            161, 28, 235, 11, 1, 0, 5, 1, 0, 2, 3, 2, 5, 5, 7, 6, 7, 13, 21, 8, 34, 16, 0, 0, 0, 1,
            0, 1, 0, 2, 6, 12, 10, 2, 0, 4, 86, 65, 83, 80, 15, 114, 111, 116, 97, 116, 101, 95,
            98, 97, 115, 101, 95, 117, 114, 108, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0,
            0, 1, 4, 11, 0, 11, 1, 17, 0, 2,
        ],
        vec![],
        vec![TransactionArgument::U8Vector(new_url)],
    )
}

/// Encode a program that rotates `vasp_root_addr`'s compliance public key to `new_key`.
pub fn encode_rotate_compliance_public_key_script(new_key: Vec<u8>) -> Script {
    Script::new(
        vec![
            161, 28, 235, 11, 1, 0, 5, 1, 0, 2, 3, 2, 5, 5, 7, 6, 7, 13, 34, 8, 47, 16, 0, 0, 0, 1,
            0, 1, 0, 2, 6, 12, 10, 2, 0, 4, 86, 65, 83, 80, 28, 114, 111, 116, 97, 116, 101, 95,
            99, 111, 109, 112, 108, 105, 97, 110, 99, 101, 95, 112, 117, 98, 108, 105, 99, 95, 107,
            101, 121, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 4, 11, 0, 11, 1, 17,
            0, 2,
        ],
        vec![],
        vec![TransactionArgument::U8Vector(new_key)],
    )
}

/// (1) Rotate the public key stored in `account`'s `SharedEd25519PublicKey` resource to
/// `new_public_key` (2) Rotate the authentication key using the capability stored in
/// `account`'s `SharedEd25519PublicKey` to a new value derived from `new_public_key`
/// Aborts if `account` does not have a `SharedEd25519PublicKey` resource. Aborts if the
/// length of `new_public_key` is not 32.
pub fn encode_rotate_shared_ed25519_public_key_script(public_key: Vec<u8>) -> Script {
    Script::new(
        vec![
            161, 28, 235, 11, 1, 0, 5, 1, 0, 2, 3, 2, 5, 5, 7, 6, 7, 13, 34, 8, 47, 16, 0, 0, 0, 1,
            0, 1, 0, 2, 6, 12, 10, 2, 0, 22, 83, 104, 97, 114, 101, 100, 69, 100, 50, 53, 53, 49,
            57, 80, 117, 98, 108, 105, 99, 75, 101, 121, 10, 114, 111, 116, 97, 116, 101, 95, 107,
            101, 121, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 4, 11, 0, 11, 1, 17,
            0, 2,
        ],
        vec![],
        vec![TransactionArgument::U8Vector(public_key)],
    )
}

/// Set validator's config.
pub fn encode_set_validator_config_script(
    validator_account: AccountAddress,
    consensus_pubkey: Vec<u8>,
    validator_network_identity_pubkey: Vec<u8>,
    validator_network_address: Vec<u8>,
    fullnodes_network_identity_pubkey: Vec<u8>,
    fullnodes_network_address: Vec<u8>,
) -> Script {
    Script::new(
        vec![
            161, 28, 235, 11, 1, 0, 5, 1, 0, 2, 3, 2, 5, 5, 7, 15, 7, 22, 27, 8, 49, 16, 0, 0, 0,
            1, 0, 1, 0, 7, 6, 12, 5, 10, 2, 10, 2, 10, 2, 10, 2, 10, 2, 0, 15, 86, 97, 108, 105,
            100, 97, 116, 111, 114, 67, 111, 110, 102, 105, 103, 10, 115, 101, 116, 95, 99, 111,
            110, 102, 105, 103, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 9, 11, 0,
            10, 1, 11, 2, 11, 3, 11, 4, 11, 5, 11, 6, 17, 0, 2,
        ],
        vec![],
        vec![
            TransactionArgument::Address(validator_account),
            TransactionArgument::U8Vector(consensus_pubkey),
            TransactionArgument::U8Vector(validator_network_identity_pubkey),
            TransactionArgument::U8Vector(validator_network_address),
            TransactionArgument::U8Vector(fullnodes_network_identity_pubkey),
            TransactionArgument::U8Vector(fullnodes_network_address),
        ],
    )
}

/// Set validator's operator
pub fn encode_set_validator_operator_script(operator_account: AccountAddress) -> Script {
    Script::new(
        vec![
            161, 28, 235, 11, 1, 0, 5, 1, 0, 2, 3, 2, 5, 5, 7, 5, 7, 12, 29, 8, 41, 16, 0, 0, 0, 1,
            0, 1, 0, 2, 6, 12, 5, 0, 15, 86, 97, 108, 105, 100, 97, 116, 111, 114, 67, 111, 110,
            102, 105, 103, 12, 115, 101, 116, 95, 111, 112, 101, 114, 97, 116, 111, 114, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 4, 11, 0, 10, 1, 17, 0, 2,
        ],
        vec![],
        vec![TransactionArgument::Address(operator_account)],
    )
}

/// Send `amount` coins of type `Token` to `payee`.
pub fn encode_testnet_mint_script(token: TypeTag, payee: AccountAddress, amount: u64) -> Script {
    Script::new(
        vec![
            161, 28, 235, 11, 1, 0, 8, 1, 0, 4, 2, 4, 4, 3, 8, 26, 4, 34, 2, 5, 36, 38, 7, 74, 139,
            1, 8, 213, 1, 16, 6, 229, 1, 22, 0, 0, 0, 1, 0, 3, 1, 0, 1, 2, 0, 1, 0, 0, 4, 1, 2, 0,
            0, 5, 0, 3, 0, 0, 6, 4, 5, 1, 1, 0, 7, 3, 5, 0, 3, 8, 1, 6, 12, 1, 5, 1, 1, 1, 8, 0, 5,
            6, 8, 0, 5, 3, 10, 2, 10, 2, 0, 3, 6, 12, 5, 3, 7, 8, 0, 1, 3, 1, 3, 1, 3, 1, 9, 0, 12,
            76, 105, 98, 114, 97, 65, 99, 99, 111, 117, 110, 116, 6, 83, 105, 103, 110, 101, 114,
            10, 97, 100, 100, 114, 101, 115, 115, 95, 111, 102, 18, 87, 105, 116, 104, 100, 114,
            97, 119, 67, 97, 112, 97, 98, 105, 108, 105, 116, 121, 9, 101, 120, 105, 115, 116, 115,
            95, 97, 116, 27, 101, 120, 116, 114, 97, 99, 116, 95, 119, 105, 116, 104, 100, 114, 97,
            119, 95, 99, 97, 112, 97, 98, 105, 108, 105, 116, 121, 22, 112, 97, 121, 95, 102, 114,
            111, 109, 95, 119, 105, 116, 104, 95, 109, 101, 116, 97, 100, 97, 116, 97, 27, 114,
            101, 115, 116, 111, 114, 101, 95, 119, 105, 116, 104, 100, 114, 97, 119, 95, 99, 97,
            112, 97, 98, 105, 108, 105, 116, 121, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
            5, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 221, 10, 2, 1, 0, 1, 1, 6, 7, 45,
            10, 1, 17, 1, 12, 4, 11, 4, 3, 6, 5, 14, 10, 0, 17, 0, 7, 0, 33, 12, 6, 11, 6, 3, 18,
            5, 25, 11, 0, 1, 6, 203, 21, 122, 0, 0, 0, 0, 0, 39, 10, 2, 6, 0, 128, 198, 164, 126,
            141, 3, 0, 37, 12, 8, 11, 8, 3, 29, 5, 41, 11, 0, 1, 6, 204, 21, 122, 0, 0, 0, 0, 0,
            39, 11, 0, 17, 2, 12, 3, 14, 3, 10, 1, 10, 2, 7, 1, 7, 1, 56, 0, 11, 3, 17, 4, 2, 11,
            0, 1, 6, 205, 21, 122, 0, 0, 0, 0, 0, 39,
        ],
        vec![token],
        vec![
            TransactionArgument::Address(payee),
            TransactionArgument::U64(amount),
        ],
    )
}

/// Mint 'mint_amount' to 'designated_dealer_address' for 'tier_index' tier. Max valid
/// tier index is 3 since there are max 4 tiers per DD. Sender should be treasury
/// compliance account and receiver authorized DD. `sliding_nonce` is a unique nonce for
/// operation, see sliding_nonce.move for details.
pub fn encode_tiered_mint_script(
    coin_type: TypeTag,
    sliding_nonce: u64,
    designated_dealer_address: AccountAddress,
    mint_amount: u64,
    tier_index: u64,
) -> Script {
    Script::new(
        vec![
            161, 28, 235, 11, 1, 0, 7, 1, 0, 10, 2, 10, 14, 3, 24, 29, 4, 53, 8, 5, 61, 73, 7, 134,
            1, 195, 1, 8, 201, 2, 16, 0, 0, 0, 1, 0, 2, 0, 3, 0, 4, 3, 5, 1, 1, 3, 3, 6, 1, 0, 1,
            1, 1, 1, 1, 3, 7, 0, 1, 1, 3, 3, 8, 2, 3, 1, 3, 0, 9, 4, 5, 1, 1, 4, 10, 6, 3, 0, 2,
            11, 7, 3, 1, 1, 0, 10, 2, 11, 1, 10, 4, 11, 1, 6, 12, 1, 11, 0, 1, 9, 0, 2, 6, 12, 11,
            0, 1, 9, 0, 0, 5, 6, 12, 6, 11, 0, 1, 8, 1, 3, 5, 3, 1, 11, 2, 1, 9, 0, 2, 6, 12, 3, 3,
            6, 12, 5, 11, 2, 1, 9, 0, 5, 6, 12, 3, 5, 3, 3, 2, 11, 2, 1, 9, 0, 11, 0, 1, 8, 1, 1,
            8, 1, 1, 9, 0, 16, 68, 101, 115, 105, 103, 110, 97, 116, 101, 100, 68, 101, 97, 108,
            101, 114, 5, 76, 105, 98, 114, 97, 12, 76, 105, 98, 114, 97, 65, 99, 99, 111, 117, 110,
            116, 5, 82, 111, 108, 101, 115, 12, 83, 108, 105, 100, 105, 110, 103, 78, 111, 110, 99,
            101, 10, 67, 97, 112, 97, 98, 105, 108, 105, 116, 121, 22, 84, 114, 101, 97, 115, 117,
            114, 121, 67, 111, 109, 112, 108, 105, 97, 110, 99, 101, 82, 111, 108, 101, 31, 101,
            120, 116, 114, 97, 99, 116, 95, 112, 114, 105, 118, 105, 108, 101, 103, 101, 95, 116,
            111, 95, 99, 97, 112, 97, 98, 105, 108, 105, 116, 121, 31, 114, 101, 115, 116, 111,
            114, 101, 95, 99, 97, 112, 97, 98, 105, 108, 105, 116, 121, 95, 116, 111, 95, 112, 114,
            105, 118, 105, 108, 101, 103, 101, 11, 116, 105, 101, 114, 101, 100, 95, 109, 105, 110,
            116, 21, 114, 101, 99, 111, 114, 100, 95, 110, 111, 110, 99, 101, 95, 111, 114, 95, 97,
            98, 111, 114, 116, 7, 100, 101, 112, 111, 115, 105, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 1, 1, 1, 8, 9, 21, 10, 0, 10, 1, 17, 3, 10, 0, 56, 0, 12, 6, 10, 0, 14,
            6, 10, 3, 10, 2, 10, 4, 56, 1, 12, 5, 10, 0, 11, 6, 56, 2, 11, 0, 10, 2, 11, 5, 56, 3,
            2,
        ],
        vec![coin_type],
        vec![
            TransactionArgument::U64(sliding_nonce),
            TransactionArgument::Address(designated_dealer_address),
            TransactionArgument::U64(mint_amount),
            TransactionArgument::U64(tier_index),
        ],
    )
}

/// Unfreeze account `address`. Initiator must be authorized. `sliding_nonce` is a unique
/// nonce for operation, see sliding_nonce.move for details.
pub fn encode_unfreeze_account_script(
    sliding_nonce: u64,
    to_unfreeze_account: AccountAddress,
) -> Script {
    Script::new(
        vec![
            161, 28, 235, 11, 1, 0, 7, 1, 0, 6, 2, 6, 9, 3, 15, 22, 4, 37, 4, 5, 41, 46, 7, 87,
            164, 1, 8, 251, 1, 16, 0, 0, 0, 1, 0, 2, 1, 3, 1, 1, 3, 0, 7, 1, 0, 1, 4, 0, 1, 1, 3,
            1, 5, 2, 3, 1, 3, 2, 6, 4, 3, 0, 0, 8, 5, 3, 0, 0, 8, 1, 8, 1, 6, 12, 1, 11, 0, 1, 9,
            0, 2, 6, 12, 11, 0, 1, 9, 0, 0, 2, 6, 12, 3, 3, 6, 12, 6, 11, 0, 1, 8, 1, 5, 3, 6, 12,
            3, 5, 1, 11, 0, 1, 8, 1, 1, 8, 1, 12, 76, 105, 98, 114, 97, 65, 99, 99, 111, 117, 110,
            116, 5, 82, 111, 108, 101, 115, 12, 83, 108, 105, 100, 105, 110, 103, 78, 111, 110, 99,
            101, 10, 67, 97, 112, 97, 98, 105, 108, 105, 116, 121, 31, 101, 120, 116, 114, 97, 99,
            116, 95, 112, 114, 105, 118, 105, 108, 101, 103, 101, 95, 116, 111, 95, 99, 97, 112,
            97, 98, 105, 108, 105, 116, 121, 31, 114, 101, 115, 116, 111, 114, 101, 95, 99, 97,
            112, 97, 98, 105, 108, 105, 116, 121, 95, 116, 111, 95, 112, 114, 105, 118, 105, 108,
            101, 103, 101, 21, 114, 101, 99, 111, 114, 100, 95, 110, 111, 110, 99, 101, 95, 111,
            114, 95, 97, 98, 111, 114, 116, 17, 65, 99, 99, 111, 117, 110, 116, 85, 110, 102, 114,
            101, 101, 122, 105, 110, 103, 16, 117, 110, 102, 114, 101, 101, 122, 101, 95, 97, 99,
            99, 111, 117, 110, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 6, 7, 14,
            10, 0, 10, 1, 17, 2, 10, 0, 56, 0, 12, 3, 10, 0, 14, 3, 10, 2, 17, 3, 11, 0, 11, 3, 56,
            1, 2,
        ],
        vec![],
        vec![
            TransactionArgument::U64(sliding_nonce),
            TransactionArgument::Address(to_unfreeze_account),
        ],
    )
}

/// Unmints `amount_lbr` LBR from the sending account into the constituent coins and
/// deposits the resulting coins into the sending account."
pub fn encode_unmint_lbr_script(amount_lbr: u64) -> Script {
    Script::new(
        vec![
            161, 28, 235, 11, 1, 0, 7, 1, 0, 10, 2, 10, 21, 3, 31, 27, 4, 58, 6, 5, 64, 76, 7, 140,
            1, 142, 1, 8, 154, 2, 16, 0, 0, 0, 1, 0, 2, 0, 3, 0, 4, 3, 3, 1, 1, 1, 0, 0, 2, 0, 1,
            1, 2, 0, 2, 2, 1, 0, 4, 6, 1, 0, 2, 5, 0, 1, 0, 4, 7, 2, 3, 1, 1, 4, 8, 4, 5, 0, 4, 9,
            5, 3, 0, 4, 10, 6, 7, 1, 1, 4, 10, 1, 11, 1, 12, 2, 6, 12, 11, 0, 1, 8, 3, 2, 11, 0, 1,
            8, 1, 11, 0, 1, 8, 2, 2, 6, 12, 11, 0, 1, 9, 0, 0, 1, 6, 12, 1, 8, 4, 2, 6, 8, 4, 3, 1,
            11, 0, 1, 9, 0, 2, 6, 12, 3, 4, 11, 0, 1, 8, 1, 11, 0, 1, 8, 2, 11, 0, 1, 8, 3, 8, 4,
            1, 8, 3, 1, 8, 1, 1, 8, 2, 5, 67, 111, 105, 110, 49, 5, 67, 111, 105, 110, 50, 3, 76,
            66, 82, 5, 76, 105, 98, 114, 97, 12, 76, 105, 98, 114, 97, 65, 99, 99, 111, 117, 110,
            116, 6, 117, 110, 112, 97, 99, 107, 18, 87, 105, 116, 104, 100, 114, 97, 119, 67, 97,
            112, 97, 98, 105, 108, 105, 116, 121, 10, 100, 101, 112, 111, 115, 105, 116, 95, 116,
            111, 27, 101, 120, 116, 114, 97, 99, 116, 95, 119, 105, 116, 104, 100, 114, 97, 119,
            95, 99, 97, 112, 97, 98, 105, 108, 105, 116, 121, 27, 114, 101, 115, 116, 111, 114,
            101, 95, 119, 105, 116, 104, 100, 114, 97, 119, 95, 99, 97, 112, 97, 98, 105, 108, 105,
            116, 121, 13, 119, 105, 116, 104, 100, 114, 97, 119, 95, 102, 114, 111, 109, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 8, 9, 21, 10, 0, 17, 2, 12, 5, 14, 5, 10, 1,
            56, 0, 12, 4, 11, 5, 17, 3, 10, 0, 11, 4, 17, 0, 12, 3, 12, 2, 10, 0, 11, 2, 56, 1, 11,
            0, 11, 3, 56, 2, 2,
        ],
        vec![],
        vec![TransactionArgument::U64(amount_lbr)],
    )
}

/// Update the on-chain exchange rate to LBR for the given `currency` to be given by
/// `new_exchange_rate_denominator/new_exchange_rate_numerator`.
pub fn encode_update_exchange_rate_script(
    currency: TypeTag,
    sliding_nonce: u64,
    new_exchange_rate_denominator: u64,
    new_exchange_rate_numerator: u64,
) -> Script {
    Script::new(
        vec![
            161, 28, 235, 11, 1, 0, 7, 1, 0, 8, 2, 8, 13, 3, 21, 28, 4, 49, 6, 5, 55, 57, 7, 112,
            204, 1, 8, 188, 2, 16, 0, 0, 0, 1, 0, 2, 0, 3, 2, 4, 1, 1, 3, 2, 5, 1, 0, 0, 0, 2, 0,
            2, 6, 0, 1, 1, 3, 2, 7, 2, 3, 1, 3, 0, 8, 4, 5, 0, 1, 9, 6, 3, 1, 1, 3, 10, 7, 3, 0, 0,
            10, 3, 11, 1, 10, 1, 6, 12, 1, 11, 0, 1, 9, 0, 2, 6, 12, 11, 0, 1, 9, 0, 0, 2, 3, 3, 1,
            8, 2, 2, 6, 11, 0, 1, 8, 1, 8, 2, 2, 6, 12, 3, 4, 6, 12, 3, 3, 3, 2, 8, 2, 11, 0, 1, 8,
            1, 1, 8, 1, 1, 9, 0, 12, 70, 105, 120, 101, 100, 80, 111, 105, 110, 116, 51, 50, 5, 76,
            105, 98, 114, 97, 5, 82, 111, 108, 101, 115, 12, 83, 108, 105, 100, 105, 110, 103, 78,
            111, 110, 99, 101, 10, 67, 97, 112, 97, 98, 105, 108, 105, 116, 121, 22, 84, 114, 101,
            97, 115, 117, 114, 121, 67, 111, 109, 112, 108, 105, 97, 110, 99, 101, 82, 111, 108,
            101, 31, 101, 120, 116, 114, 97, 99, 116, 95, 112, 114, 105, 118, 105, 108, 101, 103,
            101, 95, 116, 111, 95, 99, 97, 112, 97, 98, 105, 108, 105, 116, 121, 31, 114, 101, 115,
            116, 111, 114, 101, 95, 99, 97, 112, 97, 98, 105, 108, 105, 116, 121, 95, 116, 111, 95,
            112, 114, 105, 118, 105, 108, 101, 103, 101, 20, 99, 114, 101, 97, 116, 101, 95, 102,
            114, 111, 109, 95, 114, 97, 116, 105, 111, 110, 97, 108, 24, 117, 112, 100, 97, 116,
            101, 95, 108, 98, 114, 95, 101, 120, 99, 104, 97, 110, 103, 101, 95, 114, 97, 116, 101,
            21, 114, 101, 99, 111, 114, 100, 95, 110, 111, 110, 99, 101, 95, 111, 114, 95, 97, 98,
            111, 114, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 8, 9, 17, 10, 0,
            10, 1, 17, 4, 10, 0, 56, 0, 12, 5, 10, 2, 10, 3, 17, 2, 12, 4, 14, 5, 11, 4, 56, 1, 11,
            0, 11, 5, 56, 2, 2,
        ],
        vec![currency],
        vec![
            TransactionArgument::U64(sliding_nonce),
            TransactionArgument::U64(new_exchange_rate_denominator),
            TransactionArgument::U64(new_exchange_rate_numerator),
        ],
    )
}

/// Update Libra version.
pub fn encode_update_libra_version_script(major: u64) -> Script {
    Script::new(
        vec![
            161, 28, 235, 11, 1, 0, 5, 1, 0, 2, 3, 2, 5, 5, 7, 5, 7, 12, 17, 8, 29, 16, 0, 0, 0, 1,
            0, 1, 0, 2, 6, 12, 3, 0, 12, 76, 105, 98, 114, 97, 86, 101, 114, 115, 105, 111, 110, 3,
            115, 101, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 4, 11, 0, 10,
            1, 17, 0, 2,
        ],
        vec![],
        vec![TransactionArgument::U64(major)],
    )
}

/// Allows--true--or disallows--false--minting of `currency` based upon `allow_minting`.
pub fn encode_update_minting_ability_script(currency: TypeTag, allow_minting: bool) -> Script {
    Script::new(
        vec![
            161, 28, 235, 11, 1, 0, 7, 1, 0, 4, 2, 4, 9, 3, 13, 18, 4, 31, 6, 5, 37, 42, 7, 79,
            133, 1, 8, 212, 1, 16, 0, 0, 0, 1, 1, 2, 1, 1, 3, 1, 3, 1, 0, 1, 4, 0, 1, 1, 3, 1, 5,
            2, 3, 1, 3, 0, 6, 4, 3, 1, 1, 0, 7, 2, 8, 1, 7, 1, 6, 12, 1, 11, 0, 1, 9, 0, 2, 6, 12,
            11, 0, 1, 9, 0, 0, 2, 6, 11, 0, 1, 8, 1, 1, 2, 6, 12, 1, 1, 11, 0, 1, 8, 1, 1, 8, 1, 1,
            9, 0, 5, 76, 105, 98, 114, 97, 5, 82, 111, 108, 101, 115, 10, 67, 97, 112, 97, 98, 105,
            108, 105, 116, 121, 22, 84, 114, 101, 97, 115, 117, 114, 121, 67, 111, 109, 112, 108,
            105, 97, 110, 99, 101, 82, 111, 108, 101, 31, 101, 120, 116, 114, 97, 99, 116, 95, 112,
            114, 105, 118, 105, 108, 101, 103, 101, 95, 116, 111, 95, 99, 97, 112, 97, 98, 105,
            108, 105, 116, 121, 31, 114, 101, 115, 116, 111, 114, 101, 95, 99, 97, 112, 97, 98,
            105, 108, 105, 116, 121, 95, 116, 111, 95, 112, 114, 105, 118, 105, 108, 101, 103, 101,
            22, 117, 112, 100, 97, 116, 101, 95, 109, 105, 110, 116, 105, 110, 103, 95, 97, 98,
            105, 108, 105, 116, 121, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 5, 6,
            10, 10, 0, 56, 0, 12, 2, 14, 2, 10, 1, 56, 1, 11, 0, 11, 2, 56, 2, 2,
        ],
        vec![currency],
        vec![TransactionArgument::Bool(allow_minting)],
    )
}

/// Update the travel rule limit to `new_micro_lbr_limit`.
pub fn encode_update_travel_rule_limit_script(
    sliding_nonce: u64,
    new_micro_lbr_limit: u64,
) -> Script {
    Script::new(
        vec![
            161, 28, 235, 11, 1, 0, 7, 1, 0, 8, 2, 8, 9, 3, 17, 27, 4, 44, 4, 5, 48, 47, 7, 95,
            207, 1, 8, 174, 2, 16, 0, 0, 0, 1, 0, 2, 0, 3, 1, 4, 1, 1, 3, 0, 8, 1, 0, 1, 5, 0, 1,
            1, 3, 1, 6, 2, 3, 1, 3, 2, 7, 0, 4, 0, 0, 9, 5, 3, 0, 3, 10, 6, 3, 0, 0, 9, 1, 9, 1, 6,
            12, 1, 11, 0, 1, 9, 0, 2, 6, 12, 11, 0, 1, 9, 0, 0, 1, 5, 3, 6, 11, 0, 1, 8, 1, 5, 3,
            2, 6, 12, 3, 3, 6, 12, 3, 3, 1, 11, 0, 1, 8, 1, 1, 8, 1, 20, 68, 117, 97, 108, 65, 116,
            116, 101, 115, 116, 97, 116, 105, 111, 110, 76, 105, 109, 105, 116, 5, 82, 111, 108,
            101, 115, 6, 83, 105, 103, 110, 101, 114, 12, 83, 108, 105, 100, 105, 110, 103, 78,
            111, 110, 99, 101, 10, 67, 97, 112, 97, 98, 105, 108, 105, 116, 121, 31, 101, 120, 116,
            114, 97, 99, 116, 95, 112, 114, 105, 118, 105, 108, 101, 103, 101, 95, 116, 111, 95,
            99, 97, 112, 97, 98, 105, 108, 105, 116, 121, 31, 114, 101, 115, 116, 111, 114, 101,
            95, 99, 97, 112, 97, 98, 105, 108, 105, 116, 121, 95, 116, 111, 95, 112, 114, 105, 118,
            105, 108, 101, 103, 101, 10, 97, 100, 100, 114, 101, 115, 115, 95, 111, 102, 30, 85,
            112, 100, 97, 116, 101, 68, 117, 97, 108, 65, 116, 116, 101, 115, 116, 97, 116, 105,
            111, 110, 84, 104, 114, 101, 115, 104, 111, 108, 100, 20, 115, 101, 116, 95, 109, 105,
            99, 114, 111, 108, 105, 98, 114, 97, 95, 108, 105, 109, 105, 116, 21, 114, 101, 99,
            111, 114, 100, 95, 110, 111, 110, 99, 101, 95, 111, 114, 95, 97, 98, 111, 114, 116, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 7, 8, 15, 10, 0, 10, 1, 17, 4, 10, 0,
            56, 0, 12, 3, 14, 3, 10, 0, 17, 2, 10, 2, 17, 3, 11, 0, 11, 3, 56, 1, 2,
        ],
        vec![],
        vec![
            TransactionArgument::U64(sliding_nonce),
            TransactionArgument::U64(new_micro_lbr_limit),
        ],
    )
}

/// Optionally update global thresholds of max balance, total flow (inflow + outflow)
/// (microLBR) for `LimitsDefinition` bound accounts. If a new threshold is 0, that
/// particular config does not get updated. `sliding_nonce` is a unique nonce for
/// operation, see sliding_nonce.move for details.
pub fn encode_update_unhosted_wallet_limits_script(
    coin_type: TypeTag,
    sliding_nonce: u64,
    new_max_total_flow: u64,
    new_max_holding_balance: u64,
) -> Script {
    Script::new(
        vec![
            161, 28, 235, 11, 1, 0, 7, 1, 0, 6, 2, 6, 9, 3, 15, 22, 4, 37, 4, 5, 41, 46, 7, 87,
            178, 1, 8, 137, 2, 16, 0, 0, 0, 1, 0, 2, 1, 3, 1, 1, 3, 1, 4, 1, 0, 1, 5, 0, 1, 1, 3,
            1, 6, 2, 3, 1, 3, 0, 7, 4, 3, 0, 2, 8, 5, 3, 0, 0, 8, 1, 8, 1, 6, 12, 1, 11, 0, 1, 9,
            0, 2, 6, 12, 11, 0, 1, 9, 0, 0, 3, 6, 11, 0, 1, 8, 1, 3, 3, 2, 6, 12, 3, 4, 6, 12, 3,
            3, 3, 1, 11, 0, 1, 8, 1, 1, 8, 1, 13, 65, 99, 99, 111, 117, 110, 116, 76, 105, 109,
            105, 116, 115, 5, 82, 111, 108, 101, 115, 12, 83, 108, 105, 100, 105, 110, 103, 78,
            111, 110, 99, 101, 10, 67, 97, 112, 97, 98, 105, 108, 105, 116, 121, 22, 84, 114, 101,
            97, 115, 117, 114, 121, 67, 111, 109, 112, 108, 105, 97, 110, 99, 101, 82, 111, 108,
            101, 31, 101, 120, 116, 114, 97, 99, 116, 95, 112, 114, 105, 118, 105, 108, 101, 103,
            101, 95, 116, 111, 95, 99, 97, 112, 97, 98, 105, 108, 105, 116, 121, 31, 114, 101, 115,
            116, 111, 114, 101, 95, 99, 97, 112, 97, 98, 105, 108, 105, 116, 121, 95, 116, 111, 95,
            112, 114, 105, 118, 105, 108, 101, 103, 101, 24, 117, 112, 100, 97, 116, 101, 95, 108,
            105, 109, 105, 116, 115, 95, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110, 21, 114,
            101, 99, 111, 114, 100, 95, 110, 111, 110, 99, 101, 95, 111, 114, 95, 97, 98, 111, 114,
            116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 6, 7, 14, 10, 0, 10, 1, 17,
            3, 10, 0, 56, 0, 12, 4, 14, 4, 10, 2, 10, 3, 17, 2, 11, 0, 11, 4, 56, 1, 2,
        ],
        vec![coin_type],
        vec![
            TransactionArgument::U64(sliding_nonce),
            TransactionArgument::U64(new_max_total_flow),
            TransactionArgument::U64(new_max_holding_balance),
        ],
    )
}
