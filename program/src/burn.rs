use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, msg, pubkey::Pubkey};

use crate::generated::state::{Account, AccountPDA, GemMetadata};

pub fn burn(
    _program_id: &Pubkey,
    for_burn: &[&AccountInfo],
    _mint: &Account<spl_token::state::Mint>,
    gem: &mut AccountPDA<GemMetadata>,
    _account: &AccountPDA<spl_token::state::Account>,
    _owner: &AccountInfo,
    _wallet: &AccountInfo,
) -> ProgramResult {
    // Implement your business logic here...

    gem.data.assoc_account = None;
    csl_spl_token::src::cpi::burn(for_burn, 1)?;
    msg!("NFT burn successful!🔥");

    Ok(())
}
