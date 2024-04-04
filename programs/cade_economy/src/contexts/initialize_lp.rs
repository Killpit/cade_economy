use anchor_lang::prelude::*;
use crate::state::Lp_Config;
use crate::state::Config;
use anchor_spl::{token_interface::{TokenAccount, Mint}, associated_token::AssociatedToken};
use anchor_spl::token_interface::TokenInterface;

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct InitializeLP<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
    init,
    payer = user,
    seeds = [b"lp", config.key().as_ref()],
    bump,
    mint::decimals = 6,
    mint::authority = auth
    )]
    pub mint_lp: Box<InterfaceAccount<'info, Mint>>,
    #[account(
    init,
    payer = user,
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
    init,
    payer = user,
    seeds = [b"lp_config",seed.to_le_bytes().as_ref()],
    bump,
    space = Config::INIT_SPACE
    )]
    pub lp_config : Box<Account<'info , Lp_Config>>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> InitializeLP<'info> {
    pub fn init(
        &mut self,
        bumps: &InitializeLPBumps,
        seed: u64,
        authority: Option<Pubkey>,
    ) -> Result<()> {
        self.lp_config.set_inner(
            Lp_Config {
                seed,
                authority,
                mint_lp : self.mint_lp.key(),
                lp_bump: bumps.mint_lp,
                lp_config_bump : bumps.lp_config
            });

        Ok(())
    }

    pub fn lp_initialize(
        ctx: Context<InitializeLP>,
        seed: u64,
        authority: Option<Pubkey>,
    ) -> Result<()> {
        ctx.accounts.init(&ctx.bumps, seed, authority)
    }

    pub fn mint_lp(ctx: Context<MintCade>, amount: u64) -> Result<()> {
        ctx.accounts.mint_lp(amount)
    }

    pub fn swap(ctx: Context<Swap>, amount: u64, expiration: i64) -> Result<()> {
        ctx.accounts.swap(amount, expiration)
    }

    pub fn pay(ctx: Context<Pay>, amount: u64) -> Result<()> {
        ctx.accounts.pay(amount)
    }

    pub fn claim_usdc_for_cade(ctx: Context<ClaimUsdc>) -> Result<()> {
        ctx.accounts.claim_usdc()
    }

}