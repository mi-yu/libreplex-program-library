use crate::state::{Metadata};
use crate::{ CreateMetadataInput, MetadataEvent, MetadataEventType, assert_pda_derivation::assert_pda_derivation};
use anchor_lang::prelude::*;
use solana_program::program_option::COption;
use spl_token_2022::extension::metadata_pointer::MetadataPointer;
use spl_token_2022::extension::{StateWithExtensions, BaseStateWithExtensions};
use crate::{errors::ErrorCode};
use spl_token_2022::ID as TOKEN_2022_PROGRAM_ID;

// whitelisted signer programs

pub mod migrator_lite {
    use super::*;
    declare_id!("migr1m1An7f3X75nKuuUn9mm3844miK62ZohpqRfQHp");
}


#[derive(Accounts)]
#[instruction(metadata_input: CreateMetadataInput)]
pub struct CreateMetadata<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(init, seeds = [b"metadata", mint.key().as_ref()],
              bump, payer = payer, space = Metadata::BASE_SIZE + metadata_input.get_size())]
    pub metadata: Box<Account<'info, Metadata>>,

    /// CHECK: Checked against ID constraint
    #[account(
        constraint = mint.owner.eq(&TOKEN_2022_PROGRAM_ID)
    )]
    pub mint: UncheckedAccount<'info>,

    /*
        Authority needs to be a mint or a PDA generated by a whitelisted migrator program
    */
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,

    /*
     only to be supplied if the migration is invoked by a whitelisted 
     migrator program.

     if a migrator program is invoked, then the signer account must be
     a PDA derived by the migrator program from seed [mint].
    */
    pub invoked_migrator_program: Option<UncheckedAccount<'info>> 
}

pub fn handler(ctx: Context<CreateMetadata>, metadata_input: CreateMetadataInput) -> Result<()> {
    let metadata = &mut ctx.accounts.metadata;
    let mint_info = &mut ctx.accounts.mint;
    let authority = &ctx.accounts.authority;
    let invoked_migrator_program = &ctx.accounts.invoked_migrator_program;

    let mint_data = mint_info.try_borrow_data()?;
    let mint = StateWithExtensions::<spl_token_2022::state::Mint>::unpack(&mint_data)?;

    assert_is_valid_signer(&authority.key(), mint_info.key, &mint.base, invoked_migrator_program)?;
    
    let metadata_pointer = mint.get_extension::<MetadataPointer>().unwrap();

    let metadata_pointer_authority: Option<Pubkey> = metadata_pointer.authority.into();
    let metadata_pointer_address: Option<Pubkey> = metadata_pointer.metadata_address.into();

    match (metadata_pointer_authority, metadata_pointer_address) {
        (None, Some(address)) => {
            if address != metadata.key() {
                return err!(ErrorCode::InvalidMetadataPointer)
            }
        },
        (_, _) => {
            return err!(ErrorCode::InvalidMetadataPointer)
        }
    };

    // Update the metadata state account
    metadata.mint = mint_info.key();
    metadata.is_mutable = true;
    metadata.symbol = metadata_input.symbol.clone();
    metadata.name = metadata_input.name.clone();
    metadata.creator = authority.key();
    metadata.asset = metadata_input.asset;
    metadata.update_authority = metadata_input.update_authority;
    metadata.extension = metadata_input.extension;

    metadata.group = None;

    msg!(
        "metadata created for mint with pubkey {}",
        mint_info.key()
    );

    emit!(MetadataEvent {
        id: metadata.key(),
        mint: mint_info.key(),
        event_type: MetadataEventType::Create
    });

    Ok(())
}

fn assert_is_valid_signer (signer: &Pubkey, mint_key: &Pubkey, mint: &spl_token_2022::state::Mint, invoked_migrator_program: &Option<UncheckedAccount<'_>>) -> Result<()> {
    match invoked_migrator_program {
        Some(x) => {

            // currently migrator is the only whitelisted signer program 
            if x.key() != migrator_lite::ID  {
                return err!(ErrorCode::InvalidSignedProgram);
            }

            let seeds = [
                b"metadata_signer",
                mint_key.as_ref()
            ];

            msg!("{} {}", x.key(), signer.key());
            assert_pda_derivation(&x.key(), signer, &seeds)?;

        },
        None => {
            // no migrator invoked. Hence mint must be the signer
                   

            if let COption::Some(mint_authority) = mint.mint_authority.as_ref() {
                if mint_authority != signer {
                    return err!(ErrorCode::BadAuthority);
                }
            }
            else {
                return err!(ErrorCode::BadAuthority);
            };
        }
    }

    Ok(())
}