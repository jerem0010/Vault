use anchor_lang::prelude::*;

declare_id!("5E5FJSGRonoQHfM2UpP6qbjvZZVKsXYVfPH5zsc8Lgar");

#[program]
pub mod earn_vault {
    use super::*;

    pub fn initialize_vault(ctx: Context<InitializeVault>) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        vault.total_deposited = 0;
        vault.yield_rate = 5; // 5% fixed yield for demo
        Ok(())
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        let user = &mut ctx.accounts.user_account;
        user.owner = ctx.accounts.signer.key();
        user.balance += amount;
        vault.total_deposited += amount;
        Ok(())
    }

    pub fn simulate_yield(ctx: Context<SimulateYield>) -> Result<()> {
        let user = &mut ctx.accounts.user_account;
        let interest = user.balance * 5 / 100; // +5%
        user.balance += interest;
        Ok(())
    }
}

#[account]
pub struct Vault {
    pub total_deposited: u64,
    pub yield_rate: u64,
}

#[account]
pub struct UserAccount {
    pub owner: Pubkey,
    pub balance: u64,
}

#[derive(Accounts)]
pub struct InitializeVault<'info> {
    #[account(init, payer = signer, space = 8 + 16)]
    pub vault: Account<'info, Vault>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub vault: Account<'info, Vault>,
    #[account(init_if_needed, payer = signer, space = 8 + 48, seeds = [b"user", signer.key().as_ref()], bump)]
    pub user_account: Account<'info, UserAccount>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SimulateYield<'info> {
    #[account(mut)]
    pub user_account: Account<'info, UserAccount>,
}
