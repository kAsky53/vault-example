pub mod error;
pub mod located;
pub mod processor;
pub mod state;

use crate::processor::*;
use anchor_lang::prelude::*;

declare_id!("BVtzz2pAELfipE4ZbotkTpBuMSW1g1u3BEy15hPUiiv8");

#[program]
pub mod vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        process_initialize(ctx)
    }

    #[access_control(Deposit::validate(&ctx))]
    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        process_deposit(ctx, amount)
    }

    #[access_control(Withdraw::validate(&ctx, amount))]
    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        process_withdraw(ctx, amount)
    }

    #[access_control(Compound::validate(&ctx))]
    pub fn compound(ctx: Context<Compound>, interest_rate_bps: u16) -> Result<()> {
        process_compound(ctx, interest_rate_bps)
    }
}
