// TODO
use anchor_lang::prelude::*;

use mpl_core::{instructions::UpdateV1CpiBuilder, ID as CORE_PROGRAM_ID};

use crate::{state::CollectionAuthority, error::MPLXCoreError};

#[derive(Accounts)]
pub struct UpdateNft<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(mut)]
    /// CHECK: this will be checked by the CORE PROGRAM
    pub asset: UncheckedAccount<'info>,
    #[account(
        mut,
        constraint = collection.owner == &CORE_PROGRAM_ID @ MPLXCoreError::InvalidCollection,
        constraint = !collection.data_is_empty() @ MPLXCoreError::CollectionNotInitialized
    )]
    /// CHECK: this will be checked by the CORE PROGRAM
    pub collection: UncheckedAccount<'info>,
    #[account(
        seeds = [b"collection_authority", collection.key().as_ref()],
        bump = collection_authority.bump,
        constraint = collection_authority.creator == authority.key() @ MPLXCoreError::NotAuthorized
    )]
    pub collection_authority: Account<'info, CollectionAuthority>,
    /// CHECK: this will be checked by the CPI into the CORE PROGRAM
    pub core_program: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>
}

impl<'info> UpdateNft<'info> {

    pub fn update_nft(&mut self, new_name: String) -> Result<()> {

        //let pubkey = self.collection.key();

        let signer_seeds: &[&[&[u8]]] = &[&[b"collection_authority", &self.collection.key().to_bytes(), &[self.collection_authority.bump]]];

        UpdateV1CpiBuilder::new(&self.core_program.to_account_info())
        .asset(&self.asset.to_account_info())
        .collection(Some(&self.collection.to_account_info()))
        .authority(Some(&self.collection_authority.to_account_info()))
        .payer(&self.authority.to_account_info())
        .new_name(new_name.clone())
        .system_program(&self.system_program.to_account_info())
        .invoke_signed(signer_seeds)?;

        self.collection_authority.nft_name = new_name;

        Ok(())
    }
}