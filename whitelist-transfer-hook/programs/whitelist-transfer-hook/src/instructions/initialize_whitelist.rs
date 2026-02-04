use anchor_lang::prelude::*;

use crate::state::WhitelistUser;

#[derive(Accounts)]
pub struct InitializeWhitelist<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
        init,
        payer = admin,
        space = 8 + 1,
        seeds = [b"whitelist", admin.key().as_ref()],
        bump
    )]
    pub whitelist: Account<'info, WhitelistUser>,
    pub system_program: Program<'info, System>,
}

impl<'info> InitializeWhitelist<'info> {
    pub fn initialize_whitelist(&mut self) -> Result<()> {
        // Initialize the whitelist with an empty address vector
        Ok(())
    }
}
