use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;
use solana_program::secp256k1_recover::SECP256K1_PUBLIC_KEY_LENGTH;

pub const MAX_ADMIN_SIZE: usize = SECP256K1_PUBLIC_KEY_LENGTH + (32 as usize) + (8 as usize) + (1 as usize);

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]
pub struct UpgradeAdmin {
    pub public_key: [u8; SECP256K1_PUBLIC_KEY_LENGTH],
    pub contract: Pubkey,
    pub nonce: u64,
    pub is_initialized: bool,
}