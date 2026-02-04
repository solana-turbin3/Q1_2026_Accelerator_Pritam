use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenInterface};

use crate::state::WhitelistUser;

#[derive(Accounts)]
pub struct TokenFactory<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        init,
        payer = user,
        mint::decimals = 9,
        mint::authority = user,
        extensions::transfer_hook::program_id = crate::ID,
    )]
    pub mint: InterfaceAccount<'info, Mint>,
    /// CHECK: ExtraAccountMetaList Account, will be checked by the transfer hook
    #[account(mut)]
    pub extra_account_meta_list: UncheckedAccount<'info>,
    #[account(
        seeds = [b"whitelist", user.key().as_ref()], 
        bump
    )]
    pub blocklist: Account<'info, WhitelistUser>,
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
}

impl<'info> TokenFactory<'info> {
    pub fn mint_token(&mut self) -> Result<()> {
        Ok(())
    }
}
