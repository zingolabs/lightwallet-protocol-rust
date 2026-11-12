//! Rust bindings for Zcash lightwallet protocol buffers.
//!
//! This crate provides auto-generated Rust bindings for the Zcash lightwallet
//! protocol buffer definitions, including both the gRPC service definitions
//! and the compact block format messages.

// Include the generated protobuf code
pub mod cash {
    pub mod z {
        pub mod wallet {
            pub mod sdk {
                #[allow(clippy::double_must_use)]
                pub mod rpc {
                    include!("generated/cash.z.wallet.sdk.rpc.rs");
                }
            }
        }
    }
}

// Re-export commonly used types at the crate root for convenience
pub use cash::z::wallet::sdk::rpc::*;

// Re-export service traits and types for the gRPC service
pub use cash::z::wallet::sdk::rpc::compact_tx_streamer_client::CompactTxStreamerClient;
pub use cash::z::wallet::sdk::rpc::compact_tx_streamer_server::{
    CompactTxStreamer, CompactTxStreamerServer,
};

#[cfg(test)]
mod tests {
    use super::*;
    use prost::Message;

    #[test]
    fn test_block_id_creation() {
        let block_id = BlockId {
            height: 1000,
            hash: vec![0x00, 0x01, 0x02],
        };
        assert_eq!(block_id.height, 1000);
        assert_eq!(block_id.hash, vec![0x00, 0x01, 0x02]);
    }

    #[test]
    fn test_compact_block_creation() {
        let block = CompactBlock {
            height: 500000,
            hash: vec![0xaa, 0xbb],
            prev_hash: vec![0xcc, 0xdd],
            time: 1234567890,
            header: vec![],
            vtx: vec![],
            chain_metadata: None,
        };
        assert_eq!(block.height, 500000);
        assert_eq!(block.time, 1234567890);
    }

    #[test]
    fn test_raw_transaction() {
        let tx = RawTransaction {
            data: vec![0x01, 0x02, 0x03],
            height: 42,
        };
        assert_eq!(tx.data, vec![0x01, 0x02, 0x03]);
        assert_eq!(tx.height, 42);
    }

    #[test]
    fn standard_fee_keeps_its_wire_encoding() -> Result<(), prost::DecodeError> {
        const MARGINAL_FEE_ZAT: u64 = 5_000;
        const ESTIMATOR_VERSION: u32 = 1;
        const WIRE_ENCODING: [u8; 5] = [0x08, 0x88, 0x27, 0x10, 0x01];

        let standard_fee = StandardFee {
            fee_zat: MARGINAL_FEE_ZAT,
            estimator_version: ESTIMATOR_VERSION,
        };

        let encoded = standard_fee.encode_to_vec();
        assert_eq!(encoded, WIRE_ENCODING);

        let decoded = StandardFee::decode(encoded.as_slice())?;
        assert_eq!(decoded, standard_fee);
        Ok(())
    }
}
