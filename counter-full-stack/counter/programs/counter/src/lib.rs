#![allow(unexpected_cfgs)]

use anchor_lang::prelude::*;

declare_id!("J7QfMXRDBQoPKKbcWMUyKKb1sgByzgQoXYxmRsj7gsJw");

#[program]
pub mod counter {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.bump = ctx.bumps.counter; // store bump seed in `Counter` account
        msg!("Counter account created! Current count: {}", counter.count);
        msg!("Counter bump: {}", counter.bump);
        Ok(())
    }

    pub fn increment(ctx: Context<Increment>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        msg!("Previous counter: {}", counter.count);
        counter.count = counter.count.checked_add(1).unwrap();
        msg!("Counter incremented! Current count: {}", counter.count);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    // Create and initialize `Counter` account using a PDA as the address
    #[account(
        init,
        seeds = [b"counter"], // optional seeds for pda
        bump,                 // bump seed for pda
        payer = user,
        space = 8 + Counter::INIT_SPACE
    )]
    pub counter: Account<'info, Counter>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Increment<'info> {
    // The address of the `Counter` account must be a PDA derived with the specified `seeds`
    #[account(
        mut,
        seeds = [b"counter"], // optional seeds for pda
        bump = counter.bump,  // bump seed for pda stored in `Counter` account
    )]
    pub counter: Account<'info, Counter>,
}

#[account]
#[derive(InitSpace)]
pub struct Counter {
    pub count: u64, // 8 bytes
    pub bump: u8,   // 1 byte
}

// #[program]
// pub mod counter {
//     use super::*;

//     pub fn initialize(ctx: Context<Initialize>, initial_balance: u64) -> Result<()> {
//         let account = &mut ctx.accounts.account;
//         account.balance = initial_balance;
//         account.bump = ctx.bumps.account; // Store bump seed
//         msg!("Bank account created! Initial balance: {}", account.balance);
//         Ok(())
//     }

//     // 2. Deposit money into the account
//     pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
//         let account = &mut ctx.accounts.account;
//         msg!("Previous balance: {}", account.balance);
//         account.balance = account.balance.checked_add(amount).unwrap();
//         msg!("New balance: {}", account.balance);
//         Ok(())
//     }

//     // 3. Withdraw money from the account
//     pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
//         let account = &mut ctx.accounts.account;

//         require!(account.balance >= amount, BankError::InsufficientFunds);

//         msg!("Previous balance: {}", account.balance);
//         account.balance = account.balance.checked_sub(amount).unwrap();
//         msg!("New balance: {}", account.balance);
//         Ok(())
//     }

// }



// // Define accounts needed for the program
// #[derive(Accounts)]
// pub struct Initialize<'info> {
//     #[account(mut)]
//     pub user: Signer<'info>,

//     #[account(
//         init,
//         seeds = [b"bank_account", user.key().as_ref()],
//         bump,
//         payer = user,
//         space = 8 + BankAccount::INIT_SPACE
//     )]
//     pub account: Account<'info, BankAccount>,

//     pub system_program: Program<'info, System>,
// }

// #[derive(Accounts)]
// pub struct Deposit<'info> {
//     #[account(
//         mut,
//         seeds = [b"bank_account", user.key().as_ref()],
//         bump = account.bump,
//     )]
//     pub account: Account<'info, BankAccount>,

//     pub user: Signer<'info>,
// }

// #[derive(Accounts)]
// pub struct Withdraw<'info> {
//     #[account(
//         mut,
//         seeds = [b"bank_account", user.key().as_ref()],
//         bump = account.bump,
//     )]
//     pub account: Account<'info, BankAccount>,

//     pub user: Signer<'info>,
// }

// // The Bank Account struct
// #[account]
// #[derive(InitSpace)]
// pub struct BankAccount {
//     pub balance: u64,  // 8 bytes
//     pub bump: u8,      // 1 byte
// }

// // Custom error handling
// #[error_code]
// pub enum BankError {
//     #[msg("Insufficient funds in the account")]
//     InsufficientFunds,
// }