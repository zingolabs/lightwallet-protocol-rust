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
                pub mod rpc {
                    include!(concat!(env!("OUT_DIR"), "/cash.z.wallet.sdk.rpc.rs"));
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
            proto_version: 1,
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
}