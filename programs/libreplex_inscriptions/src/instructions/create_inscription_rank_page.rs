use crate::InscriptionRankPage;
use anchor_lang::prelude::*;

#[derive(Clone, AnchorDeserialize, AnchorSerialize)]
pub struct CreateInscriptionRankInput {
    pub page_index: u32, // 0,1,2,3,4, .... 
}

#[derive(Accounts)]
#[instruction(input: CreateInscriptionRankInput)]
pub struct CreateInscriptionRank<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(init_if_needed,
        // always leave 32 bytes spare at the end. new additions write to the last 32 bytes and add extra 32 bytes
        space = InscriptionRankPage::BASE_SIZE + 32, 
        payer = payer,
        seeds = ["inscription_rank".as_bytes(), &input.page_index.to_le_bytes()],
        bump)]
    pub page: Box<Account<'info, InscriptionRankPage>>,

    pub system_program: Program<'info, System>,
}

pub fn handler(
    _ctx: Context<CreateInscriptionRank>,
    _input: CreateInscriptionRankInput,
) -> Result<()> {
    Ok(())
}
