use anchor_lang::prelude::*;
use integer_sqrt::IntegerSquareRoot;

declare_id!("2xFcBC6Bt2VVE7y3zELmFB3QxF6a8xLRDgrJWqK4ukBL");

#[program]
pub mod day_2 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>,
                    a: u64,
                    b: u64,
                    message: String) -> Result<()> {
        msg!("You said {}", message);
        msg!("You sent {} and {}", a, b);
        Ok(())
    }

    // added this function
    pub fn array(ctx: Context<Initialize>,
                arr: Vec<u64>) -> Result<()> {
        msg!("Your array {:?}", arr);
        Ok(())
    }

    // exercise 1 and 2, change overflow-checks to true and false in cargo.toml, fails for 1 and passes for 2
    pub fn sub(ctx: Context<Initialize>,
                    a: u64,
                    b: u64) -> Result<()> {
        let x: u64 = a - b;
        Ok(())
    }
    
    // exercise 3 (test fail when func name is snake case)
    pub fn checkedsub(ctx: Context<Initialize>,
                    a: u64,
                    b: u64) -> Result<()> {
        let x: u64 = a.checked_sub(b).unwrap();
        Ok(())
    }

    // exercise 4 from here on
    pub fn add(ctx: Context<Initialize>,
                    a: u64,
                    b: u64) -> Result<()> {
        let x: u64 = a.checked_add(b).unwrap();
        Ok(())
    }

    pub fn mul(ctx: Context<Initialize>,
                    a: u64,
                    b: u64) -> Result<()> {
        let x: u64 = a.checked_mul(b).unwrap();
        Ok(())
    }

    pub fn div(ctx: Context<Initialize>,
                    a: u64,
                    b: u64) -> Result<()> {
        let x: u64 = a.checked_div(b).unwrap();
        Ok(())
    }

    pub fn log(ctx: Context<Initialize>,
                    a: u64) -> Result<()> {
        let x: u64 = a.checked_ilog10().unwrap().into();
        Ok(())
    }

    pub fn sqrt(ctx: Context<Initialize>,
                    a: u64) -> Result<()> {
        let x: u64 = a.integer_sqrt();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
