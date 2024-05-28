use anchor_lang::prelude::*;

declare_id!("Bs5QoCadKbunU2oMbS7YEGwEjfyPfPYN1k4Td9z5WWmP");

#[program]
pub mod anchor_project {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
