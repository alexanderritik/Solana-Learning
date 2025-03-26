#![allow(unexpected_cfgs)]

use anchor_lang::prelude::*;

declare_id!("84MHWoTefv3wjACsGTNjHQNpbeNG5Pzb7MNeNaBSk7fQ");

pub const ANCHOR_DISCRIMINATOR_SIZE: usize = 8;

#[program]
pub mod favorite {
    use super::*;

    pub fn set_favorites(context: Context<SetFavorites>, number: u64, color: String, hobbies: Vec<String>) -> Result<()>{
        msg!("Greeting from set favorites {}", context.program_id);

        let user_public_key = context.accounts.user.key();

        msg!("User {} public key, number {} color {} hobbies {:?}", user_public_key, number, color, hobbies);

        context.accounts.favorites.set_inner(Favorite {
            number,
            color,
            hobbies
        });
        Ok(())
    }
}


#[account]
#[derive(InitSpace)]
pub struct Favorite {
    pub number : u64,

    #[max_len(50)]
    pub color: String,

    #[max_len(5, 50)]
    pub hobbies: Vec<String>,
}

#[derive(Accounts)]
pub struct SetFavorites<'info>{

    #[account(mut)]
    pub user: Signer<'info>,


    #[account(
        init_if_needed,
        payer = user,
        space = ANCHOR_DISCRIMINATOR_SIZE + Favorite::INIT_SPACE,
        seeds = [b"favorites", user.key().as_ref()],
        bump
    )]
    pub favorites: Account<'info, Favorite>,

    pub system_program: Program<'info, System>,
}