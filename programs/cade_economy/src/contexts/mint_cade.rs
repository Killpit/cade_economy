use crate::state::Config;
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{
        mint_to, Mint, MintTo, TokenAccount, TokenInterface
    },
};
use crate::Lp_Config;

#[derive(Accounts)]
pub struct MintCade<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    pub mint_x : Box<InterfaceAccount<'info,Mint>>,
    #[account(
    mut,
    seeds = [b"lp", config.key().as_ref()],
    bump = lp_config.lp_bump
    )]
    pub mint_lp: Box<InterfaceAccount<'info, Mint>>,
    #[account(
    mut,
    associated_token::mint = mint_lp,
    associated_token::authority = auth
    )]
    pub vault_lp: Box<InterfaceAccount<'info, TokenAccount>>,
    ///CHECKED: This is not dangerous. It's just used for signing.
    #[account(
    seeds = [b"auth"],
    bump = config.auth_bump
    )]
    pub auth: UncheckedAccount<'info>,
    #[account(
    seeds = [
    b"config",
    config.seed.to_le_bytes().as_ref()
    ],
    bump = config.config_bump,
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

impl<'info> MintCade<'info> {
    pub fn mint_lp(&mut self, amount: u64) -> Result<()> {
        self.mint_lp_tokens(amount);
        Ok(())
    }

    pub fn mint_lp_tokens(&mut self, amount: u64) -> Result<()> {
        let accounts = MintTo {
            mint: self.mint_lp.to_account_info(),
            to: self.vault_lp.to_account_info(),
            authority: self.auth.to_account_info(),
        };

        let seeds = &[&b"auth"[..], &[self.config.auth_bump]];

        let signer_seeds = &[&seeds[..]];

        let ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            accounts,
            signer_seeds
        );

        mint_to(ctx, amount)
    }
}
