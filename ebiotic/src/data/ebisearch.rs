use super::{AvailableReturnFormats, DataReturnFormats, EBI_SEARCH_ENDPOINT};
use crate::core::{self, EbioticClient, EbioticHttpClient, Service};
use crate::errors::EbioticError;

pub mod ebisearchdomains;

#[derive(Debug, Clone)]
pub struct EbiSearch {
    pub(crate) client: EbioticClient,
    domain: ebisearchdomains::EbiSearchDomains,
    return_format: DataReturnFormats,
}

#[derive(Debug, Clone)]
pub struct EbiSearchResult {
    data: String,
}

impl Default for EbiSearch {
    fn default() -> Self {
        EbiSearch {
            client: EbioticClient::default(),
            domain: ebisearchdomains::EbiSearchDomains::Uniprot,
            return_format: DataReturnFormats::Json,
        }
    }
}
