use anchor_lang::prelude::*;

declare_id!("6QS7RtjSAW4Df26qB3nRqAbnr3BzyUehbiWZ1epkMVjF");

#[program]
pub mod tryrust {
    use super::*;
    use std::collections::HashMap;

    pub fn initialize(_ctx: Context<Initialize>, name: String, age: u64) -> Result<()> {
        // Defining a struct in Solana
        struct Person {
            my_name: String,
            my_age: u64,
        }

        // Creating an instance of the struct
        let mut person1: Person = Person {
            my_name: name,
            my_age: age,
        };

        msg!("{} is {} years old", person1.my_name, person1.my_age);

        // Accessing and modifying struct fields
        person1.my_name = "Bob".to_string();
        person1.my_age = 18;

        msg!("{} is {} years old", person1.my_name, person1.my_age);

        Ok(())
    }

    pub fn even_numbers(vec: Vec<u64>) -> Result<()> {
        let mut new_vec: Vec<u64> = Vec::new();

        for i in vec {
            if i % 2 == 0 {
                new_vec.push(i as u64);
            }
        }
        
        msg!("Even numbers: {:?}", new_vec);

        Ok(())
    }

    pub fn age_checker(ctx: Context<Initialize>,
                    age: u64) -> Result<()> {
        if age >= 18 {
            msg!("You are 18 years old or above");
        } else {
            msg!("You are below 18 years old");
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
