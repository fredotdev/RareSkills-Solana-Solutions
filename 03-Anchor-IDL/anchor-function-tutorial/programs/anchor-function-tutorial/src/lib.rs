use anchor_lang::prelude::*;

declare_id!("3dhSxQ3hkEfJivFGNbKmpNYrPdVm2QEtUmHVhtqfN47B");

#[program]
pub mod anchor_function_tutorial {
    use super::*;

    // exercise 1
    pub fn boaty_mc_boatface(ctx: Context<BoatyMcBoatface>, num: u64) -> Result<()> {
        Ok(())
    }

    pub fn add(ctx: Context<BoatyMcBoatface>, a: u64, b: u64) -> Result<()> {
        let sum = a + b;
        msg!("Sum is {}", sum);  
            Ok(())
    }

    pub fn sub(ctx: Context<BoatyMcBoatface>, a: u64, b: u64) -> Result<()> {
        let difference = a - b;
        msg!("Difference is {}", difference);  
            Ok(())
    }

    // exercise 2
    pub fn mul(ctx: Context<BoatyMcBoatface>, a: u64, b: u64) -> Result<()> {
        let multiplication = a * b;
        msg!("Multiplication is {}", multiplication);  
            Ok(())
    }

    pub fn div(ctx: Context<BoatyMcBoatface>, a: u64, b: u64) -> Result<()> {
        let division = a / b;
        msg!("Division is {}", division);  
            Ok(())
    }

    pub fn modulo(ctx: Context<BoatyMcBoatface>, a: u64, b: u64) -> Result<()> {
        let modulo = a % b;
        msg!("Mod is {}", modulo);  
            Ok(())
    }
}

#[derive(Accounts)]
pub struct BoatyMcBoatface {}
