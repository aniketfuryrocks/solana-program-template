use lib::ix::Ix;
use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, msg, pubkey::Pubkey,
};

// Program entry point
entrypoint!(process_instruction);

fn process_instruction(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let ix: Ix = bincode::deserialize(instruction_data)
        .map_err(|_| solana_program::program_error::ProgramError::InvalidInstructionData)?;

    match ix {
        Ix::CreatePool => {
            msg!("CreatePool");
        }
    }

    msg!("Hello, world!");

    Ok(())
}
