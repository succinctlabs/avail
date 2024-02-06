pub mod chains;

pub mod da_block_import;
pub use da_block_import::BlockImport;

pub mod cli;
pub mod rpc;
pub mod service;

pub const NODE_VERSION: &'static str = "2.0.0";
