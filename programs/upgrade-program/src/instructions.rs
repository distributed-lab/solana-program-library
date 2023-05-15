use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;
use solana_program::secp256k1_recover::{SECP256K1_PUBLIC_KEY_LENGTH, SECP256K1_SIGNATURE_LENGTH};
use solana_program::instruction::{Instruction, AccountMeta};
use crate::PDA_ADMIN_SEED;

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

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, Clone)]
pub enum UpgradeInstruction {
    /// Initialize new UpgradeAdmin that will be an authority for target upgradable program.
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
    ///
    /// Accounts expected by this instruction:
    ///
    ///   0. `[writable]` The UpgradeAdmin account
    ChangePublicKey(ChangePublicKeyArgs),

    /// Change contract upgrade authority. The Keccak Hash of `[target_contract, nonce, "solana-upgrade-program".bytes, new_authority]`
    /// should be signed by stored public key to perform that operation.
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

pub fn initialize_admin(
    program_id: Pubkey,
    contract: Pubkey,
    fee_payer: Pubkey,
    public_key: [u8; SECP256K1_PUBLIC_KEY_LENGTH],
) -> Instruction {
    let (admin, _) = Pubkey::find_program_address(&[PDA_ADMIN_SEED.as_bytes(), contract.as_ref()], &program_id);
    Instruction{
        program_id,
        data: UpgradeInstruction::InitializeAdmin(
            InitializeAdminArgs {
                public_key,
                contract: Default::default(),
            }
        ).try_to_vec().unwrap(),
        accounts: vec![
            AccountMeta::new(admin, false),
            AccountMeta::new(fee_payer, true),
            AccountMeta::new_readonly(solana_program::system_program::id(), false),
            AccountMeta::new_readonly(solana_program::sysvar::clock::id(), false),
        ],
    }
}


pub fn change_public_key(
    program_id: Pubkey,
    contract: Pubkey,
    new_public_key: [u8; SECP256K1_PUBLIC_KEY_LENGTH],
    signature: [u8; SECP256K1_SIGNATURE_LENGTH],
    recovery_id: u8,
) -> Instruction {
    let (admin, _) = Pubkey::find_program_address(&[PDA_ADMIN_SEED.as_bytes(), contract.as_ref()], &program_id);
    Instruction{
        program_id,
        data: UpgradeInstruction::ChangePublicKey(
            ChangePublicKeyArgs {
                new_public_key,
                signature,
                recovery_id,
            }
        ).try_to_vec().unwrap(),
        accounts: vec![
            AccountMeta::new(admin, false),
        ],
    }
}

pub fn change_authority(
    program_id: Pubkey,
    contract: Pubkey,
    new_authority: Pubkey,
    signature: [u8; SECP256K1_SIGNATURE_LENGTH],
    recovery_id: u8,
) -> Instruction {
    let (admin, _) = Pubkey::find_program_address(&[PDA_ADMIN_SEED.as_bytes(), contract.as_ref()], &program_id);
    let (program_data, _) = Pubkey::find_program_address(&[contract.as_ref()], &solana_program::bpf_loader_upgradeable::id());

    Instruction{
        program_id,
        data: UpgradeInstruction::ChangeAuthority(
            ChangeAuthorityArgs {
                signature,
                recovery_id,
            }
        ).try_to_vec().unwrap(),
        accounts: vec![
            AccountMeta::new(admin, false),
            AccountMeta::new(program_data, false),
            AccountMeta::new(new_authority, false),
            AccountMeta::new(solana_program::bpf_loader_upgradeable::id(), false),
        ],
    }
}

pub fn upgrade(
    program_id: Pubkey,
    contract: Pubkey,
    buffer: Pubkey,
    spill: Pubkey,
    signature: [u8; SECP256K1_SIGNATURE_LENGTH],
    recovery_id: u8,
) -> Instruction {
    let (admin, _) = Pubkey::find_program_address(&[PDA_ADMIN_SEED.as_bytes(), contract.as_ref()], &program_id);
    let (program_data, _) = Pubkey::find_program_address(&[contract.as_ref()], &solana_program::bpf_loader_upgradeable::id());

    Instruction {
        program_id,
        data: UpgradeInstruction::Upgrade(
            UpgradeArgs {
                signature,
                recovery_id,
            }
        ).try_to_vec().unwrap(),
        accounts: vec![
            AccountMeta::new(admin, false),
            AccountMeta::new(program_data, false),
            AccountMeta::new(contract, false),
            AccountMeta::new(buffer, false),
            AccountMeta::new(spill, false),
            AccountMeta::new(solana_program::sysvar::rent::id(), false),
            AccountMeta::new(solana_program::sysvar::clock::id(), false),
            AccountMeta::new(solana_program::bpf_loader_upgradeable::id(), false),
        ],
    }
}