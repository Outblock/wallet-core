// Copyright © 2017-2023 Trust Wallet.
//
// This file is part of Trust. The full Trust copyright notice, including
// terms governing use, modification, and redistribution, is contained in the
// file LICENSE at the root of the source code distribution tree.

use tw_any_coin::ffi::tw_any_address::{
    tw_any_address_create_bech32_with_public_key, tw_any_address_create_with_public_key_derivation,
    tw_any_address_create_with_string, tw_any_address_data, tw_any_address_description,
    tw_any_address_is_valid, tw_any_address_is_valid_bech32,
};
use tw_any_coin::test_utils::TWAnyAddressHelper;
use tw_coin_entry::derivation::Derivation;
use tw_coin_registry::blockchain_type::BlockchainType;
use tw_coin_registry::registry::supported_coin_items;
use tw_encoding::hex::DecodeHex;
use tw_keypair::ffi::privkey::tw_private_key_get_public_key_by_type;
use tw_keypair::test_utils::tw_private_key_helper::TWPrivateKeyHelper;
use tw_keypair::test_utils::tw_public_key_helper::TWPublicKeyHelper;
use tw_keypair::tw::PublicKeyType;
use tw_memory::test_utils::tw_data_helper::TWDataHelper;
use tw_memory::test_utils::tw_string_helper::TWStringHelper;

const ETHEREUM_COIN_TYPE: u32 = 60;
const OSMOSIS_COIN_TYPE: u32 = 10000118;

#[test]
fn test_any_address_derive() {
    let private_key = TWPrivateKeyHelper::with_hex(
        "afeefca74d9a325cf1d6b6911d61a65c32afa8e02bd5e78e2e4ac2910bab45f5",
    );

    for coin in supported_coin_items() {
        let public_key = TWPublicKeyHelper::wrap(unsafe {
            tw_private_key_get_public_key_by_type(private_key.ptr(), coin.public_key_type as u32)
        });

        // TODO match `CoinType` when it's generated.
        let expected_address = match coin.blockchain {
            BlockchainType::Aptos => {
                "0x9006fa46f038224e8004bdda97f2e7a60c2c3d135bce7cb15541e5c0aae907a4"
            },
            // By default, Bitcoin will return a P2PKH address.
            BlockchainType::Bitcoin => "19cAJn4Ms8jodBBGtroBNNpCZiHAWGAq7X",
            BlockchainType::Cosmos if coin.id == "cosmos" => {
                "cosmos1ten42eesehw0ktddcp0fws7d3ycsqez3lynlqx"
            },
            // Skip other Cosmos chains as they have different addresses.
            // TODO fix this when `CoinType` is generated by a codegen tool.
            BlockchainType::Cosmos => continue,
            BlockchainType::Ethereum => "0xAc1ec44E4f0ca7D172B7803f6836De87Fb72b309",
            BlockchainType::InternetComputer => {
                "290cc7c359f44c8516fc169c5ed4f0f3ae2e24bf5de0d4c51f5e7545b5474faa"
            },
            BlockchainType::NativeEvmos => "evmos14s0vgnj0pjnazu4hsqlksdk7slah9vcfvt8ssm",
            BlockchainType::NativeInjective => "inj14s0vgnj0pjnazu4hsqlksdk7slah9vcfyrp6ct",
            BlockchainType::Ronin => "ronin:Ac1ec44E4f0ca7D172B7803f6836De87Fb72b309",
            BlockchainType::Thorchain => "thor1ten42eesehw0ktddcp0fws7d3ycsqez3er2y4e",
            BlockchainType::Unsupported => unreachable!(),
        };

        let any_address = TWAnyAddressHelper::wrap(unsafe {
            tw_any_address_create_with_public_key_derivation(
                public_key.ptr(),
                coin.coin_id,
                Derivation::Default as u32,
            )
        });

        let description =
            TWStringHelper::wrap(unsafe { tw_any_address_description(any_address.ptr()) });
        assert_eq!(description.to_string(), Some(expected_address.to_string()));
    }
}

#[test]
fn test_any_address_normalize_eth() {
    for coin in supported_coin_items() {
        // TODO match `CoinType` when it's generated.
        let (denormalized, expected_normalized) = match coin.blockchain {
            BlockchainType::Aptos => (
                "0xf3d7f364dd7705824a5ebda9c7aab6cb3fc7bb5b58718249f12defec240b36cc",
                "0xf3d7f364dd7705824a5ebda9c7aab6cb3fc7bb5b58718249f12defec240b36cc",
            ),
            BlockchainType::Bitcoin => (
                "19cAJn4Ms8jodBBGtroBNNpCZiHAWGAq7X",
                "19cAJn4Ms8jodBBGtroBNNpCZiHAWGAq7X",
            ),
            BlockchainType::Cosmos if coin.id == "cosmos" => (
                "cosmosvaloper1sxx9mszve0gaedz5ld7qdkjkfv8z992ax69k08",
                "cosmosvaloper1sxx9mszve0gaedz5ld7qdkjkfv8z992ax69k08",
            ),
            // Skip other Cosmos chains until `CoinType` is not generated by a codegen tool.
            BlockchainType::Cosmos => continue,
            BlockchainType::Ethereum => (
                "0xb16db98b365b1f89191996942612b14f1da4bd5f",
                "0xb16Db98B365B1f89191996942612B14F1Da4Bd5f",
            ),
            BlockchainType::InternetComputer => (
                "290CC7C359F44C8516FC169C5ED4F0F3AE2E24BF5DE0D4C51F5E7545B5474FAA",
                "290cc7c359f44c8516fc169c5ed4f0f3ae2e24bf5de0d4c51f5e7545b5474faa",
            ),
            BlockchainType::NativeEvmos => (
                "evmos17xpfvakm2amg962yls6f84z3kell8c5ljcjw34",
                "evmos17xpfvakm2amg962yls6f84z3kell8c5ljcjw34",
            ),
            BlockchainType::NativeInjective => (
                "inj14py36sx57ud82t9yrks9z6hdsrpn5x6k8tf7m3",
                "inj14py36sx57ud82t9yrks9z6hdsrpn5x6k8tf7m3",
            ),
            BlockchainType::Ronin => (
                "0xb16db98b365b1f89191996942612b14f1da4bd5f",
                "ronin:b16Db98B365B1f89191996942612B14F1Da4Bd5f",
            ),
            BlockchainType::Thorchain => (
                "thor1z53wwe7md6cewz9sqwqzn0aavpaun0gw0exn2r",
                "thor1z53wwe7md6cewz9sqwqzn0aavpaun0gw0exn2r",
            ),
            BlockchainType::Unsupported => unreachable!(),
        };

        let denormalized = TWStringHelper::create(denormalized);

        let any_address = TWAnyAddressHelper::wrap(unsafe {
            tw_any_address_create_with_string(denormalized.ptr(), coin.coin_id)
        });

        let normalized =
            TWStringHelper::wrap(unsafe { tw_any_address_description(any_address.ptr()) });

        assert_eq!(
            normalized.to_string(),
            Some(expected_normalized.to_string())
        );
    }
}

#[test]
fn test_any_address_is_valid_coin() {
    for coin in supported_coin_items() {
        let valid = match coin.blockchain {
            BlockchainType::Aptos => vec![
                "0x1",
                "0xeeff357ea5c1a4e7bc11b2b17ff2dc2dcca69750bfef1e1ebcaccf8c8018175b",
                "eeff357ea5c1a4e7bc11b2b17ff2dc2dcca69750bfef1e1ebcaccf8c8018175b",
                "19aadeca9388e009d136245b9a67423f3eee242b03142849eb4f81a4a409e59c",
                "0x777821c78442e17d82c3d7a371f42de7189e4248e529fe6eee6bca40ddbb",
                "0xeeff357ea5c1a4e7bc11b2b17ff2dc2dcca69750bfef1e1ebcaccf8c8018175",
            ],
            BlockchainType::Bitcoin => vec![
                "1MrZNGN7mfWZiZNQttrzHjfw72jnJC2JNx",
                "bc1qunq74p3h8425hr6wllevlvqqr6sezfxj262rff",
                "bc1pwse34zfpvt344rvlt7tw0ngjtfh9xasc4q03avf0lk74jzjpzjuqaz7ks5",
            ],
            BlockchainType::Cosmos if coin.id == "cosmos" => vec![
                "cosmos1hsk6jryyqjfhp5dhc55tc9jtckygx0eph6dd02",
                "cosmosvaloper1sxx9mszve0gaedz5ld7qdkjkfv8z992ax69k08",
                "cosmosvalconspub1zcjduepqjnnwe2jsywv0kfc97pz04zkm7tc9k2437cde2my3y5js9t7cw9mstfg3sa",
            ],
            // Skip other Cosmos chains until `CoinType` is not generated by a codegen tool.
            BlockchainType::Cosmos => continue,
            BlockchainType::Ethereum => vec![
                "0xb16db98b365b1f89191996942612b14f1da4bd5f",
                "0xb16Db98B365B1f89191996942612B14F1Da4Bd5f",
            ],
            BlockchainType::InternetComputer => vec![
                "fb257577279ecac604d4780214af95aa6adc3a814f6f3d6d7ac844d1deca500a",
                "e90c48d54847f4758f1d6b589a1db2500757a49a6722d4f775e050107b4b752d",
                "a7c5baf393aed527ef6fb3869fbf84dd4e562edf9b04bd8f9bfbd6b8e6a22776",
                "4cb2ca5cfcfa1d952f8cd7f0ec46c96e1023ab057b83a2c7ce236b9e71ccca0b",
            ],
            BlockchainType::NativeEvmos => vec![
                "evmos14py36sx57ud82t9yrks9z6hdsrpn5x6k0r05np",
                "evmos17xpfvakm2amg962yls6f84z3kell8c5ljcjw34"
            ],
            BlockchainType::NativeInjective => vec![
                "inj13u6g7vqgw074mgmf2ze2cadzvkz9snlwcrtq8a",
                "inj1xmpkmxr4as00em23tc2zgmuyy2gr4h3wgcl6vd"
            ],
            BlockchainType::Ronin => vec![
                "0xb16db98b365b1f89191996942612b14f1da4bd5f",
                "0xb16Db98B365B1f89191996942612B14F1Da4Bd5f",
                "ronin:b16db98b365b1f89191996942612b14f1da4bd5f",
                "ronin:b16Db98B365B1f89191996942612B14F1Da4Bd5f",
            ],
            BlockchainType::Thorchain => vec![
                "thor1z53wwe7md6cewz9sqwqzn0aavpaun0gw0exn2r",
                "thor1c8jd7ad9pcw4k3wkuqlkz4auv95mldr2kyhc65",
            ],
            _ => unreachable!(),
        };

        for valid_addr in valid {
            let valid = TWStringHelper::create(valid_addr);
            assert!(unsafe { tw_any_address_is_valid(valid.ptr(), coin.coin_id) });
        }
    }
}

#[test]
fn test_any_address_is_valid_coin_invalid() {
    for coin in supported_coin_items() {
        let invalid = match coin.blockchain {
            BlockchainType::Aptos => {
                vec![
                    "",                                                                  // Empty
                    "Seff357ea5c1a4e7bc11b2b17ff2dc2dcca69750bfef1e1ebcaccf8c8018175b", // Invalid Hex
                    "eeff357ea5c1a4e7bc11b2b17ff2dc2dcca69750bfef1e1ebcaccf8c8018175bb", // Too long
                    "0xSeff357ea5c1a4e7bc11b2b17ff2dc2dcca69750bfef1e1ebcaccf8c8018175b",
                ]
            },
            BlockchainType::Bitcoin => {
                vec!["0xb16db98b365b1f89191996942612b14f1da4bd5f"]
            },
            BlockchainType::Cosmos => vec![
                "cosmosvaloper1sxx9mszve0gaedz5ld7qdkjkfv8z992ax6",
                "one1a50tun737ulcvwy0yvve0pvu5skq0kjargvhwe",
                "bnb1grpf0955h0ykzq3ar5nmum7y6gdfl6lxfn46h2",
            ],
            BlockchainType::Ethereum | BlockchainType::Ronin => {
                vec!["b16Db98B365B1f89191996942612B14F1Da4Bd5f"]
            },
            BlockchainType::InternetComputer => vec![
                "3357cba483f268d044d4bbd4639f82c16028a76eebdf62c51bc11fc918d278b",
                "3357cba483f268d044d4bbd4639f82c16028a76eebdf62c51bc11fc918d278bce",
                "553357cba483f268d044d4bbd4639f82c16028a76eebdf62c51bc11fc918d278",
            ],
            BlockchainType::NativeEvmos => vec!["evmos17xpfvakm2amg962yls6f84z3kell8c5ljcjw"],
            BlockchainType::NativeInjective => vec!["ini13u6g7vqgw074mgmf2ze2cadzvkz9snlwcrtq8a"],
            BlockchainType::Thorchain => vec![
                "cosmos1hsk6jryyqjfhp5dhc55tc9jtckygx0eph6dd02",
                "thor1z53wwe7md6cewz9sqwqzn0aavpaun0gw0e",
            ],
            BlockchainType::Unsupported => unreachable!(),
        };

        for invalid_addr in invalid {
            let valid = TWStringHelper::create(invalid_addr);
            assert!(!unsafe { tw_any_address_is_valid(valid.ptr(), coin.coin_id) });
        }
    }
}

#[test]
fn test_any_address_get_data_eth() {
    let addr = "0xb16Db98B365B1f89191996942612B14F1Da4Bd5f";

    let address_str = TWStringHelper::create(addr);
    let any_address = TWAnyAddressHelper::wrap(unsafe {
        tw_any_address_create_with_string(address_str.ptr(), ETHEREUM_COIN_TYPE)
    });
    let data = TWDataHelper::wrap(unsafe { tw_any_address_data(any_address.ptr()) });
    assert_eq!(data.to_vec(), Some(addr.decode_hex().unwrap()));
}

#[test]
fn test_any_address_is_valid_bech32() {
    let addr = "juno1mry47pkga5tdswtluy0m8teslpalkdq0gnn4mf";

    let address_str = TWStringHelper::create(addr);
    let hrp = TWStringHelper::create("juno");
    // Should be valid even though Osmosis chain has `osmo` default hrp.
    let result =
        unsafe { tw_any_address_is_valid_bech32(address_str.ptr(), OSMOSIS_COIN_TYPE, hrp.ptr()) };
    assert!(result);
}

#[test]
fn test_any_address_create_bech32_with_public_key() {
    let private_key = TWPrivateKeyHelper::with_hex(
        "afeefca74d9a325cf1d6b6911d61a65c32afa8e02bd5e78e2e4ac2910bab45f5",
    );
    let public_key = TWPublicKeyHelper::wrap(unsafe {
        tw_private_key_get_public_key_by_type(private_key.ptr(), PublicKeyType::Secp256k1 as u32)
    });
    let hrp = TWStringHelper::create("juno");

    // Should be valid even though Osmosis chain has `osmo` default hrp.
    let any_address = TWAnyAddressHelper::wrap(unsafe {
        tw_any_address_create_bech32_with_public_key(public_key.ptr(), OSMOSIS_COIN_TYPE, hrp.ptr())
    });

    let description =
        TWStringHelper::wrap(unsafe { tw_any_address_description(any_address.ptr()) });
    let expected = "juno1ten42eesehw0ktddcp0fws7d3ycsqez3fksy86";
    assert_eq!(description.to_string(), Some(expected.to_string()));
}
