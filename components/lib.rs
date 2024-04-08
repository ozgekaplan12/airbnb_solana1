use anchor_lang::prelude::*;

declare_id!("2XBvaK1GX1HG29EWzqBSWe8mnbfR4ULcuqwN7mtwA1nV");

#[program]
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
    program_error::ProgramError,
    sysvar::{rent::Rent, Sysvar},
};

entrypoint!(process_instruction);

#[derive(Debug)]
enum HouseState {
    Uninitialized,
    Listed { owner: Pubkey, price: u64 },
    Reserved { owner: Pubkey, price: u64 },
}

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction_size = instruction_data.len();
    if instruction_size == 0 {
        return Err(ProgramError::InvalidInstructionData);
    }

    let instruction_type = instruction_data[0];
    match instruction_type {
        0 => {
            msg!("Listing a house");

            let accounts_iter = &mut accounts.iter();
            let owner_account = next_account_info(accounts_iter)?;
            let house_account = next_account_info(accounts_iter)?;

            if !owner_account.is_signer {
                return Err(ProgramError::MissingRequiredSignature);
            }

            let mut house_state = HouseState::Uninitialized;
            match house_state {
                HouseState::Uninitialized => {
                    house_state = HouseState::Listed {
                        owner: *owner_account.key,
                        price: 0,
                    };
                }
                _ => return Err(ProgramError::AccountAlreadyInitialized),
            }

            // Serialize and save house_state into house_account.data.borrow_mut()

            Ok(())
        }
        1 => {
            msg!("Reserving a house");

            let accounts_iter = &mut accounts.iter();
            let _guest_account = next_account_info(accounts_iter)?;
            let house_account = next_account_info(accounts_iter)?;
            let rent_sysvar_account = next_account_info(accounts_iter)?;

            let mut house_state = HouseState::Uninitialized; // Deserialize house_state from house_account.data.borrow()

            match house_state {
                HouseState::Listed { owner, price } => {
                    // Perform reservation logic
                    house_state = HouseState::Reserved { owner, price };
                    // Serialize and save house_state back into house_account.data.borrow_mut()
                    Ok(())
                }
                _ => Err(ProgramError::InvalidAccountData),
            }
        }
        _ => Err(ProgramError::InvalidInstructionData),
    }
}
#[derive(Accounts)]
pub struct Initialize {}
