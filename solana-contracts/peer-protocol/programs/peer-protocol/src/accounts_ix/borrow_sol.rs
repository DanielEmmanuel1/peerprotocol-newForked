use crate::{
    constants::{LOAN_OFFER_TAG, USER_PROFILE_TAG},
    errors::PeerProtocolError,
    state::{loan_sol::LoanSol, protocol::Protocol, user_profile::UserProfile},
};
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(loan_idx: u64)]
pub struct BorrowSol<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [
            LOAN_OFFER_TAG,
            loan.lender.as_ref(),
            loan_idx.to_le_bytes().as_ref()
        ],
        bump
    )]
    pub loan: Account<'info, LoanSol>,

    #[account(
        constraint = user_profile.is_init
        @ PeerProtocolError::UserProfileNotInitialized,
        mut,
        seeds = [USER_PROFILE_TAG, authority.key().as_ref()],
        bump
    )]
    pub user_profile: Account<'info, UserProfile>,

    #[account(
        constraint = protocol.is_init
        @ PeerProtocolError::ProtocolNotInitialized
)]
    pub protocol: Account<'info, Protocol>,
    pub system_program: Program<'info, System>,
}
