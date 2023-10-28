use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, msg, pubkey::Pubkey};

use crate::generated::state::{Account, AccountPDA, GemMetadata};

pub fn transfer(
    _program_id: &Pubkey,
    for_create: &[&AccountInfo],
    for_transfer_checked: &[&AccountInfo],
    _mint: &Account<spl_token::state::Mint>,
    gem: &mut AccountPDA<GemMetadata>,
    _funding: &AccountInfo,
    assoc_token_account: &AccountInfo,
    _wallet: &AccountInfo,
    _source: &AccountInfo,
    destination: &AccountInfo,
    _authority: &AccountInfo,
) -> ProgramResult {
    // Implement your business logic here...

    gem.data.assoc_account = Some(*destination.key);
    // Create the ATA account for new owner if it hasn't been created
    if assoc_token_account.lamports() == 0 {
        csl_spl_assoc_token::src::cpi::create(for_create)?;
    }
    csl_spl_token::src::cpi::transfer_checked(for_transfer_checked, 1, 0)?;

    msg!("Transfer Successful!.");
    Ok(())
}
