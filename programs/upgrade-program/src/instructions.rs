use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;
use solana_program::secp256k1_recover::{SECP256K1_PUBLIC_KEY_LENGTH, SECP256K1_SIGNATURE_LENGTH};

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]
pub struct InitializeAdminArgs {
    // ECDSA public key (64 byte format)
    pub public_key: [u8; SECP256K1_PUBLIC_KEY_LENGTH],
    // Contract to manage
    pub contract: Pubkey,
}

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]
pub struct ChangePublicKeyArgs {
    // New ECDSA public key (64 byte format)
    pub new_public_key: [u8; SECP256K1_PUBLIC_KEY_LENGTH],
    // Signature of keccak_hash(nonce, "solana-upgrade-program".bytes, new_public_key) by old public key
    pub signature: [u8; SECP256K1_SIGNATURE_LENGTH],
    // Signature recovery id
    pub recovery_id: u8,
}

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]
pub struct ChangeAuthorityArgs {
    // Signature of keccak_hash(nonce, "solana-upgrade-program".bytes, new_authority)
    pub signature: [u8; SECP256K1_SIGNATURE_LENGTH],
    // Signature recovery id
    pub recovery_id: u8,
}

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]
pub struct UpgradeArgs {
    // Signature for keccak_hash(target_contract, nonce, "solana-upgrade-program".bytes, buffer_address)
    pub signature: [u8; SECP256K1_PUBLIC_KEY_LENGTH],
    // Corresponding seed to use in PDA for admin account
    pub recovery_id: u8,
}

#[derive(BorshSerialize, BorshDeserialize, Clone)]
pub enum UpgradeInstruction {
    /// Initialize new UpgradeAdmin that will be an authority for target upgradable program.
    /// Admin public key should be equal to PDA(seed, target_contract)
    ///
    /// Accounts expected by this instruction:
    ///
    ///   0. `[writable]` The UpgradeAdmin account to initialize
    ///   1. `[writable,signer]` The fee payer
    ///   2. `[]` System program
    ///   3. `[]` Rent sysvar
    InitializeAdmin(InitializeAdminArgs),

    /// Change pubkey in UpgradeAdmin. The Keccak Hash of `[target_contract, nonce, "solana-upgrade-program".bytes, new_public_key]`
    /// should be signed by old public key to perform that operation.
    /// Also, admin public key should be equal to PDA(seed, target_contract)
    ///
    /// Accounts expected by this instruction:
    ///
    ///   0. `[writable]` The UpgradeAdmin account
    ChangePublicKey(ChangePublicKeyArgs),

    /// Change contract upgrade authority. The Keccak Hash of `[target_contract, nonce, "solana-upgrade-program".bytes, new_authority]`
    /// should be signed by stored public key to perform that operation.
    /// Also, admin public key should be equal to PDA(seed, target_contract)
    ///
    /// Accounts expected by this instruction:
    ///
    ///   0. `[writable]` The UpgradeAdmin account
    ///   1. `[writable]` The ProgramData account.
    ///   2. `[]` The new authority account
    ///   3. `[]` BPFLoaderUpgradable program
    ChangeAuthority(ChangeAuthorityArgs),

    /// Upgrade contract. The Keccak Hash of `[target_contract, nonce, "solana-upgrade-program".bytes, buffer_address]`
    /// should be signed by stored public key to perform that operation.
    /// Also, admin public key should be equal to PDA(seed, target_contract)
    ///
    /// Accounts expected by this instruction:
    ///
    ///   0. `[writable]` The UpgradeAdmin account
    ///   1. `[writable]` The ProgramData account.
    ///   2. `[writable]` The Program account corresponding to stores address in UpgradeAdmin.
    ///   3. `[writable]` The Buffer account where the program data has been
    ///      written.  The buffer account's authority must match the program's
    ///      authority
    ///   4. `[writable]` The spill account.
    ///   5. `[]` Rent sysvar.
    ///   6. `[]` Clock sysvar.
    ///   7. `[]` BPFLoaderUpgradable program
    Upgrade(UpgradeArgs),
}