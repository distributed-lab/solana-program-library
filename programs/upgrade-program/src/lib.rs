pub mod entrypoint;
pub mod state;
pub mod processor;
pub mod instructions;
pub mod ecdsa;
pub mod error;

const HASH_CONSTANT: &str = "solana-upgrade-program";
const PDA_ADMIN_SEED: &str = "admin-upgrade-account";