use anchor_lang::prelude::*;

declare_id!("daYasVmbocKeRKcwZM7FMeSwPTfsKCdR1fsWnZzy4De");

#[program]
pub mod day_1 {
    use super::*;

    pub fn initialize2(ctx: Context<Initialize>) -> Result<()> {
        msg!("Hello, world!");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
