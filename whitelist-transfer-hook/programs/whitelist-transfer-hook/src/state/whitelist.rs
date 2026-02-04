use anchor_lang::prelude::*;

#[account]
pub struct WhitelistUser {
    pub bump: u8,
}
