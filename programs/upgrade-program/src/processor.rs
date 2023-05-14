use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint::ProgramResult, msg,
    program::{invoke_signed}, pubkey::Pubkey, system_instruction,
    sysvar::{rent::Rent, Sysvar},
};
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::secp256k1_recover::{SECP256K1_PUBLIC_KEY_LENGTH, SECP256K1_SIGNATURE_LENGTH};
use crate::state::{MAX_ADMIN_SIZE, UpgradeAdmin};
use crate::instructions::UpgradeInstruction;
use crate::ecdsa::verify_ecdsa_signature;
use crate::{HASH_CONSTANT, PDA_ADMIN_SEED};
use crate::error::UpgradeError;

pub fn process_instruction<'a>(
    program_id: &'a Pubkey,
    accounts: &'a [AccountInfo<'a>],
    input: &[u8],
) -> ProgramResult {
    let instruction = UpgradeInstruction::try_from_slice(input)?;
    match instruction {
        UpgradeInstruction::InitializeAdmin(args) => {
            msg!("Instruction: Create upgrade admin");
            process_init_admin(program_id, accounts, args.public_key, args.contract)
        }
        UpgradeInstruction::ChangePublicKey(args) => {
            msg!("Instruction: Change public key");
            process_change_public_key(program_id, accounts, args.new_public_key, args.signature, args.recovery_id)
        }
        UpgradeInstruction::ChangeAuthority(args) => {
            msg!("Instruction: Transfer upgrade authority");
            process_change_authority(program_id, accounts, args.signature, args.recovery_id)
        }
        UpgradeInstruction::Upgrade(args) => {
            msg!("Instruction: Upgrade");
            process_upgrade(program_id, accounts, args.signature, args.recovery_id)
        }
    }
}


pub fn process_init_admin<'a>(
    program_id: &'a Pubkey,
    accounts: &'a [AccountInfo<'a>],
    public_key: [u8; SECP256K1_PUBLIC_KEY_LENGTH],
    upgrade_program: Pubkey,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();

    let upgrade_admin_info = next_account_info(account_info_iter)?;
    let fee_payer_info = next_account_info(account_info_iter)?;
    let system_program = next_account_info(account_info_iter)?;
    let rent_info = next_account_info(account_info_iter)?;

    let (upgrade_key, bump) = Pubkey::find_program_address(&[PDA_ADMIN_SEED.as_bytes(), upgrade_program.as_ref()], &program_id);
    if upgrade_key != *upgrade_admin_info.key {
        return Err(UpgradeError::WrongAdmin.into());
    }

    let rent = Rent::from_account_info(rent_info)?;

    let instruction = system_instruction::create_account(
        fee_payer_info.key,
        upgrade_admin_info.key,
        rent.minimum_balance(MAX_ADMIN_SIZE),
        MAX_ADMIN_SIZE as u64,
        program_id,
    );

    invoke_signed(
        &instruction,
        &[
            fee_payer_info.clone(),
            upgrade_admin_info.clone(),
            system_program.clone(),
        ],
        &[&[PDA_ADMIN_SEED.as_bytes(), upgrade_program.as_ref(), &[bump]]],
    )?;

    let mut upgrade_admin: UpgradeAdmin = BorshDeserialize::deserialize(&mut upgrade_admin_info.data.borrow_mut().as_ref())?;
    if upgrade_admin.is_initialized {
        return Err(UpgradeError::AlreadyInUse.into());
    }

    upgrade_admin.contract = upgrade_program;
    upgrade_admin.public_key = public_key;
    upgrade_admin.is_initialized = true;
    upgrade_admin.nonce = 0;
    upgrade_admin.serialize(&mut *upgrade_admin_info.data.borrow_mut())?;
    Ok(())
}


pub fn process_change_public_key<'a>(
    program_id: &'a Pubkey,
    accounts: &'a [AccountInfo<'a>],
    new_public_key: [u8; SECP256K1_PUBLIC_KEY_LENGTH],
    signature: [u8; SECP256K1_SIGNATURE_LENGTH],
    recovery_id: u8,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let upgrade_admin_info = next_account_info(account_info_iter)?;

    let mut upgrade_admin: UpgradeAdmin = BorshDeserialize::deserialize(&mut upgrade_admin_info.data.borrow_mut().as_ref())?;
    if !upgrade_admin.is_initialized {
        return Err(UpgradeError::NotInitialized.into());
    }

    let (upgrade_admin_key, _) = Pubkey::find_program_address(&[PDA_ADMIN_SEED.as_bytes(),  upgrade_admin.contract.as_ref()], &program_id);
    if upgrade_admin_key != *upgrade_admin_info.key {
        return Err(UpgradeError::WrongSeeds.into());
    }

    verify_ecdsa_signature(
        solana_program::keccak::hash(
            &[
                upgrade_admin.contract.as_ref(),
                upgrade_admin.nonce.to_be_bytes().as_ref(),
                HASH_CONSTANT.as_bytes(),
                new_public_key.as_ref(),
            ].concat()
        ).as_ref(),
        signature.as_slice(),
        recovery_id,
        upgrade_admin.public_key,
    )?;

    upgrade_admin.public_key = new_public_key;
    upgrade_admin.nonce = upgrade_admin.nonce + 1;
    upgrade_admin.serialize(&mut *upgrade_admin_info.data.borrow_mut())?;
    Ok(())
}


pub fn process_change_authority<'a>(
    program_id: &'a Pubkey,
    accounts: &'a [AccountInfo<'a>],
    signature: [u8; SECP256K1_SIGNATURE_LENGTH],
    recovery_id: u8,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let upgrade_admin_info = next_account_info(account_info_iter)?;
    let upgrade_program_data = next_account_info(account_info_iter)?;
    let authority = next_account_info(account_info_iter)?;


    let mut upgrade_admin: UpgradeAdmin = BorshDeserialize::deserialize(&mut upgrade_admin_info.data.borrow_mut().as_ref())?;
    if !upgrade_admin.is_initialized {
        return Err(UpgradeError::NotInitialized.into());
    }

    let (upgrade_admin_key, bump) = Pubkey::find_program_address(&[PDA_ADMIN_SEED.as_bytes(), upgrade_admin.contract.as_ref()], &program_id);
    if upgrade_admin_key != *upgrade_admin_info.key {
        return Err(UpgradeError::WrongSeeds.into());
    }

    verify_ecdsa_signature(
        solana_program::keccak::hash(
            &[
                upgrade_admin.contract.as_ref(),
                upgrade_admin.nonce.to_be_bytes().as_ref(),
                HASH_CONSTANT.as_bytes(),
                authority.key.as_ref(),
            ].concat()
        ).as_ref(),
        signature.as_slice(),
        recovery_id,
        upgrade_admin.public_key,
    )?;


    let instruction = solana_program::bpf_loader_upgradeable::set_upgrade_authority(
        &upgrade_admin.contract,
        upgrade_admin_info.key,
        Some(authority.key),
    );

    invoke_signed(
        &instruction,
        &[
            upgrade_program_data.clone(),
            upgrade_admin_info.clone(),
            authority.clone(),
        ],
        &[&[PDA_ADMIN_SEED.as_bytes(),  upgrade_admin.contract.as_ref(), &[bump]]],
    )?;


    upgrade_admin.nonce = upgrade_admin.nonce + 1;
    upgrade_admin.serialize(&mut *upgrade_admin_info.data.borrow_mut())?;
    Ok(())
}


pub fn process_upgrade<'a>(
    program_id: &'a Pubkey,
    accounts: &'a [AccountInfo<'a>],
    signature: [u8; SECP256K1_SIGNATURE_LENGTH],
    recovery_id: u8,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let upgrade_admin_info = next_account_info(account_info_iter)?;
    let upgrade_program_data = next_account_info(account_info_iter)?;
    let upgrade_program = next_account_info(account_info_iter)?;
    let upgrade_buffer = next_account_info(account_info_iter)?;
    let upgrade_spill = next_account_info(account_info_iter)?;
    let rent_info = next_account_info(account_info_iter)?;
    let clock_info = next_account_info(account_info_iter)?;

    let (upgrade_admin_key, bump) = Pubkey::find_program_address(&[PDA_ADMIN_SEED.as_bytes(), upgrade_program.key.as_ref()], &program_id);
    if upgrade_admin_key != *upgrade_admin_info.key {
        return Err(UpgradeError::WrongSeeds.into());
    }

    let mut upgrade_admin: UpgradeAdmin = BorshDeserialize::deserialize(&mut upgrade_admin_info.data.borrow_mut().as_ref())?;
    if !upgrade_admin.is_initialized {
        return Err(UpgradeError::NotInitialized.into());
    }

    verify_ecdsa_signature(
        solana_program::keccak::hash(
            &[
                upgrade_admin.contract.as_ref(),
                upgrade_admin.nonce.to_be_bytes().as_ref(),
                HASH_CONSTANT.as_bytes(),
                upgrade_buffer.key.as_ref(),
            ].concat()
        ).as_ref(),
        signature.as_slice(),
        recovery_id,
        upgrade_admin.public_key,
    )?;

    let instruction = solana_program::bpf_loader_upgradeable::upgrade(
        upgrade_program.key,
        upgrade_buffer.key,
        &upgrade_admin_key,
        upgrade_spill.key,
    );

    invoke_signed(
        &instruction,
        &[
            upgrade_program_data.clone(),
            upgrade_program.clone(),
            upgrade_buffer.clone(),
            upgrade_spill.clone(),
            rent_info.clone(),
            clock_info.clone(),
            upgrade_admin_info.clone(),
        ],
        &[&[PDA_ADMIN_SEED.as_bytes(), upgrade_program.key.as_ref(), &[bump]]],
    )?;

    upgrade_admin.nonce = upgrade_admin.nonce + 1;
    upgrade_admin.serialize(&mut *upgrade_admin_info.data.borrow_mut())?;
    Ok(())
}