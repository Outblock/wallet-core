// Copyright © 2017-2023 Trust Wallet.
//
// This file is part of Trust. The full Trust copyright notice, including
// terms governing use, modification, and redistribution, is contained in the
// file LICENSE at the root of the source code distribution tree.

use quick_protobuf::MessageWrite;
use serde::Serialize;
use serde_json::Value as Json;
use tw_coin_entry::error::{SigningError, SigningErrorType, SigningResult};
use tw_encoding::base64::Base64Encoded;
use tw_proto::serialize;

pub enum BroadcastMode {
    Block,
    Async,
    Sync,
}

#[derive(Serialize)]
#[serde(untagged)]
pub enum BroadcastMsg {
    /// Binary representation of the transaction.
    Raw {
        mode: String,
        tx_bytes: Base64Encoded,
    },
    /// JSON encoded transaction.
    Json { mode: String, tx: Json },
}

impl BroadcastMsg {
    pub fn raw<Tx: MessageWrite>(mode: BroadcastMode, tx: &Tx) -> BroadcastMsg {
        let mode = match mode {
            BroadcastMode::Block => "BROADCAST_MODE_BLOCK",
            BroadcastMode::Async => "BROADCAST_MODE_ASYNC",
            BroadcastMode::Sync => "BROADCAST_MODE_SYNC",
        }
        .to_string();
        let tx_bytes = Base64Encoded(serialize(tx).expect("Error on serializing transaction"));
        BroadcastMsg::Raw { mode, tx_bytes }
    }

    pub fn json<Tx: Serialize>(mode: BroadcastMode, tx: Tx) -> SigningResult<BroadcastMsg> {
        let mode = match mode {
            BroadcastMode::Block => "block",
            BroadcastMode::Async => "async",
            BroadcastMode::Sync => "sync",
        }
        .to_string();
        let tx =
            serde_json::to_value(tx).map_err(|_| SigningError(SigningErrorType::Error_internal))?;
        Ok(BroadcastMsg::Json { mode, tx })
    }

    pub fn to_json_string(&self) -> String {
        // It's safe to unwrap here because `BroadcastMsg` consists of checked fields only.
        serde_json::to_string(self).expect("Unexpected error on serializing a BroadcastMsg")
    }
}
