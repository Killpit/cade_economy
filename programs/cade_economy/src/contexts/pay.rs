use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::{Mint, TokenAccount, transfer_checked, TransferChecked};
use anchor_spl::token_interface::TokenInterface;
use crate::{Config, Lp_Config};

#[derive(Accounts)]
pub struct Pay<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    /// CHECK:
    pub gamer: AccountInfo<'info>,
    #[account(
        mut,
        seeds = [b"lp", config.key().as_ref()],
        bump = lp_config.lp_bump
    )]
    pub mint_lp: Box<InterfaceAccount<'info, Mint>>,
    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = mint_lp,
        associated_token::authority = user
    )]
    pub user_vault_lp: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = mint_lp,
        associated_token::authority = gamer
    )]

    pub gamer_vault_lp: Box<InterfaceAccount<'info, TokenAccount>>,
    ///CHECKED: This is not dangerous. It's just for signing.
    #[account(
        seeds = [b"auth"],
        bump = config.auth_bump
    )]
    pub auth: UncheckedAccount<'info>,
    #[account(
        seeds = [b"config", config.seed.to_le_bytes().as_ref()],
        bump = config.config_bump
    )]
    pub config: Box<Account<'info, Config>>,
    #[account(
        seeds = [
        b"lp_config",
        config.seed.to_le_bytes().as_ref()
        ],
        bump = lp_config.lp_config_bump,
    )]
    pub lp_config: Box<Account<'info, Lp_Config>>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> Pay<'info> {
    pub fn pay(
        &mut self,
        amount: u64,
    ) -> Result<()> {
        let cpi_accounts = TransferChecked {
            from: self.user_vault_lp.to_account_info(),
            mint: self.mint_lp.to_account_info(),
            to: self.gamer_vault_lp.to_account_info(),
            authority: self.user.to_account_info(),
        };

        let ctx = CpiContext::new(self.token_program.to_account_info(), cpi_accounts);

        transfer_checked(ctx, amount, self.mint_lp.decimals)
    }
}