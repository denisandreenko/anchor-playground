// use this import to gain access to common anchor features
use anchor_lang::prelude::*;
use anchor_spl::token::TokenAccount;

// declare an id for your program
declare_id!("ENWwGmKmN5quw638BYpeJhU7rhoVFpfDwCgRFwjv8ACD");

// write your business logic here
#[program]
pub mod anchor_playground {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, data: Data) -> Result<()> {
        if ctx.accounts.token_account.amount > 0 {
            ctx.accounts.new_account.data = data.data;
            ctx.accounts.new_account.age = data.age;
        }
        msg!("Changed data to: {}!", data.data); // Message will show up in the tx logs
        Ok(())
    }
}

// validate incoming accounts here
#[derive(Accounts)]
pub struct Initialize<'info> {
    // We must specify the space in order to initialize an account. First 8 bytes are default account discriminator, next 8 bytes come from NewAccount.data being type u64. (u64 = 64 bits unsigned integer = 8 bytes)
    // #[account(init, payer = signer, space = 8 + 8)]
    // pub signer: Signer<'info>,
    #[account(mut)] // indicate that should be mutable
    pub new_account: Account<'info, NewAccount>, // The Account type is used when an instruction is interested in the deserialized data of the account
    #[account(
        constraint = new_account.mint == token_account.mint, // whether the incoming TokenAccount belongs to the admin mint.
        has_one = owner // token_account.owner == owner.key()
    )]
    pub token_account: Account<'info, TokenAccount>,
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

// The #[account] attribute sets the owner of that data to the ID (declare_id) of the crate #[account] is used in
#[account]
pub struct NewAccount {
    data: u64,
    pub age: u8,
    mint: Pubkey
}

// Custom type used as an instruction data arg
#[derive(AnchorSerialize, AnchorDeserialize, Eq, PartialEq, Clone, Copy, Debug)]
pub struct Data {
    pub data: u64,
    pub age: u8
}