use super::{AvailableReturnFormats, DataReturnFormats, EBI_SEARCH_ENDPOINT};
use crate::core::{self, EbioticClient, EbioticHttpClient, Service};
use crate::data::ebisearch::ebisearchquery::EbiSearchQuery;
use crate::errors::EbioticError;
use bio::io::fasta::Record;

pub mod ebisearchdomains;
pub mod ebisearchquery;

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

impl EbiSearchResult {
    pub fn data(&self) -> &self {
        &self.data
    }

    pub fn into_records(self) -> Result<Vec<Record>, EbioticError> {
        core::parse_fa_from_bufread(&self.data)
    }
}

impl EbiSearch {
    pub fn new(
        client: EbioticClient,
        domain: ebisearchdomains::EbiSearchDomains,
        return_format: DataReturnFormats,
    ) -> Self {
        EbiSearch {
            client,
            domain,
            return_format,
        }
    }

    pub fn set_domain(&mut self, domain: ebisearchdomains::EbiSearchDomains) {
        self.domain = domain;
    }

    pub fn set_return_format(&mut self, return_format: DataReturnFormats) {
        self.return_format = return_format;
    }

    pub fn client(&mut self, client: EbioticClient) {
        self.client = client;
    }

    pub fn domain(&self) -> &ebisearchdomains::EbiSearchDomains {
        &self.domain
    }

    pub fn return_format(&self) -> &DataReturnFormats {
        &self.return_format
    }

    // TODO - do we need to pass ownership of the client here? This goes for every service RN
    pub fn get_client(&self) -> &EbioticClient {
        &self.client
    }
}

impl Service for EbiSearch {
    type ResultType = Result<EbiSearchResult, EbioticError>;
    type InputType = EbiSearchQuery;

    async fn run(&self, query: Self::input) -> Self::output {
        let query_url = query.build()?;
        let url = self.concat_url(query_url);
        let response = self.client.get(&url).await?;
        Ok(EbiSearchResult { data: response })
    }
}

impl EbiSearch {
    fn concat_url(&self, query: &String) -> String {
        let mut url = format!("{}", EBI_SEARCH_ENDPOINT);

        match self.domain {
            ebisearchdomains::EbiSearchDomains::All => {}
            _ => {
                url.push_str(format!("{}/", self.domain.as_str()));
            }
        }

        url.push_str(format!("{}", query.query()));
        url.push_str(format!("&format={}", self.return_format));
        return url;
    }
}
