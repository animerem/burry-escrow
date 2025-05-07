use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Escrow {
    pub unlock_price: u64,
    pub escrow_amount: u64,
    pub bump: u8,
}