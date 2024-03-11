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

    pub fn set_client(&mut self, client: EbioticClient) {
        self.client = client;
    }

    pub fn get_domain(&self) -> &ebisearchdomains::EbiSearchDomains {
        &self.domain
    }

    pub fn get_return_format(&self) -> &DataReturnFormats {
        &self.return_format
    }

    // TODO - do we need to pass ownership of the client here? This goes for every service RN
    pub fn get_client(&self) -> &EbioticClient {
        &self.client
    }
}

impl Service for EbiSearch {
    type ResultType = Result<EbiSearchResult, EbioticError>;
    type InputType = String;

    async fn run(&self, query: Self::input) -> Self::output {
        let url = self.build_url(query);
        let response = self.client.get(&url).await?;
        Ok(EbiSearchResult { data: response })
    }
}

impl EbiSearch {
    fn build_url(&self, query: String) -> String {
        let mut url = format!("{}", EBI_SEARCH_ENDPOINT);

        match self.domain {
            ebisearchdomains::EbiSearchDomains::All => {
                url.push_str(self.domain.as_str());
            }
            _ => {
                url.push_str(format!("{}/", self.domain.as_str()));
            }
        }

        url.push_str(format!("?query={}", query));
        url.push_str(format!("&format={}", self.return_format));
        return url;
    }
}
