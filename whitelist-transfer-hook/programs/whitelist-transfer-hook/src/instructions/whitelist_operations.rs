use anchor_lang::prelude::*;

use crate::state::whitelist::WhitelistUser;

#[derive(Accounts)]
#[instruction(user: Pubkey)]
pub struct WhitelistOperations<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
        init,
        payer = admin,
        seeds = [b"whitelist", user.as_ref()],
        bump,
        space = 8 + 1, // discriminator + bump
    )]
    pub whitelist: Account<'info, WhitelistUser>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(user: Pubkey)]
pub struct RemoveFromWhitelist<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
        mut,
        seeds = [b"whitelist", user.as_ref()],
        bump,
        close = admin,
    )]
    pub whitelist: Account<'info, WhitelistUser>,
    pub system_program: Program<'info, System>,
}

impl<'info> WhitelistOperations<'info> {
    pub fn add_to_whitelist(&mut self, user: Pubkey) -> Result<()> {
        msg!("Adding user to whitelist: {}", user);
        self.whitelist.set_inner(WhitelistUser {
            bump: self.whitelist.bump,
        });
        Ok(())

        // let account_info = self.whitelist.to_account_info();
        // if account_info.lamports() == 0 {
        //     self.realloc_whitelist(true)?;

        //     let (pda, bump) =
        //         Pubkey::find_program_address(&[b"whitelist", address.as_ref()], &program_id);

        //     let space: usize = 8 + 1 + 4 + 32;
        //     let rent_lamports = Rent::get()?.minimum_balance(space);

        //     let seeds = &[b"whitelist", address.as_ref(), &[bump]];
        //     let signer_seeds = &[&seeds[..]];

        //     let cpi_program = self.system_program.to_account_info();
        //     let cpi_accounts = system_program::CreateAccount {
        //         from: self.admin.to_account_info(),
        //         to: self.whitelist.to_account_info(),
        //     };

        //     let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);
        //     system_program::create_account(cpi_ctx, rent_lamports, space as u64, &program_id)?;
        // }
    }
}

impl<'info> RemoveFromWhitelist<'info> {
    pub fn remove_from_whitelist(&mut self, user: Pubkey) -> Result<()> {
        msg!("Removing user from whitelist: {}", user);
        // The account is closed automatically by the `close = admin` constraint in the struct definition.
        // This transfers the lamports back to the admin and wipes the data.
        Ok(())
    }
}

// pub fn remove_from_whitelist(&mut self, address: Pubkey) -> Result<()> {
//         let account_info = self.whitelist.to_account_info();
//         if account_info.lamports() == 0 {
//             return Err(ErrorCode::AccountNotInitialized.into());
//         }
//         let lamports = **self.whitelist.to_account_info().try_borrow_mut_lamports()?;
//         **self.admin.lamports.borrow_mut() += lamports;
//         **self.whitelist.to_account_info().try_borrow_mut_lamports()? -= 0;

//         let mut data = account_info.try_borrow_mut_data()?;
//         for byte in data.iter_mut() {
//             *byte = 0;
//         }
//         Ok(())
//     }
