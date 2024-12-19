use anchor_lang::prelude::*;

declare_id!("ENWwGmKmN5quw638BYpeJhU7rhoVFpfDwCgRFwjv8ACD");

#[program]
pub mod anchor_playground {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
