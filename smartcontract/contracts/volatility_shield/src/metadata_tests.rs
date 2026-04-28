use soroban_sdk::{contracttype, Env, String};

use crate::{DataKey, Error};

#[contracttype]
#[derive(Clone, Debug)]
pub struct VaultMetadata {
    pub name: String,
    pub description: String,
    pub risk_rating: u8,
    pub docs_url: String,
}

pub fn set_vault_metadata(env: &Env, metadata: VaultMetadata) -> Result<(), Error> {
    // Validate name <= 64 chars
    if metadata.name.len() > 64 {
        return Err(Error::MetadataNameTooLong);
    }

    // Validate description <= 256 chars
    if metadata.description.len() > 256 {
        return Err(Error::MetadataDescriptionTooLong);
    }

    // Validate risk_rating in 1–5
    if metadata.risk_rating < 1 || metadata.risk_rating > 5 {
        return Err(Error::InvalidRiskRating);
    }

    env.storage()
        .persistent()
        .set(&DataKey::VaultMetadata, &metadata);

    env.events().publish(
        (symbol_short!("meta"), symbol_short!("updated")),
        (metadata.name.clone(), metadata.risk_rating),
    );

    Ok(())
}

pub fn get_vault_metadata(env: &Env) -> Option<VaultMetadata> {
    env.storage()
        .persistent()
        .get(&DataKey::VaultMetadata)
}