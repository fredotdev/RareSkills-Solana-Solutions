use anchor_lang::prelude::*;

declare_id!("5R9eQnyNB5gRoLK1ceqE8cc74BzvzMYxTfbZUpN27psa");

#[program]
pub mod deploy_tutorial {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Hello, world!");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
