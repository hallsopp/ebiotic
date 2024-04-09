use super::{AccessionIds, AvailableReturnFormats, DataReturnFormats, EBI_SEARCH_ENDPOINT};
use crate::core::{self, EbioticClient, EbioticHttpClient, EbioticResult, Service};
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
    pub fn data(self) -> String {
        self.data
    }

    pub fn into_records(&self) -> Result<Vec<Record>, EbioticError> {
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

    pub async fn xref(
        &self,
        target_domain: Option<ebisearchdomains::EbiSearchDomains>,
        filters: Option<ebisearchquery::EbiSearchFilters>,
    ) -> EbioticResult<EbiSearchResult> {
        let mut query = ebisearchquery::EbiSearchQuery::new(
            vec![ebisearchquery::QueryCommand::Xref(None)],
            filters,
        )?;
        if let Some(domain) = target_domain {
            query.add_command(ebisearchquery::QueryCommand::Xref(Some(domain)));
        }

        self.run(query).await
    }

    pub async fn autocomplete(
        &self,
        term: String,
        filters: Option<ebisearchquery::EbiSearchFilters>,
    ) -> EbioticResult<EbiSearchResult> {
        let query = ebisearchquery::EbiSearchQuery::new(
            vec![ebisearchquery::QueryCommand::AutoComplete(term)],
            filters,
        )?;
        self.run(query).await
    }

    pub async fn entries(
        &self,
        ids: AccessionIds,
        filters: Option<ebisearchquery::EbiSearchFilters>,
    ) -> EbioticResult<EbiSearchResult> {
        let query = ebisearchquery::EbiSearchQuery::new(
            vec![ebisearchquery::QueryCommand::Entry(Some(ids))],
            filters,
        )?;
        self.run(query).await
    }

    pub async fn more_like_this(
        &self,
        ids: AccessionIds,
        target_domain: Option<ebisearchdomains::EbiSearchDomains>,
        filters: Option<ebisearchquery::EbiSearchFilters>,
    ) -> EbioticResult<EbiSearchResult> {
        let mut query = ebisearchquery::EbiSearchQuery::new(
            vec![ebisearchquery::QueryCommand::Entry(Some(ids))],
            filters,
        )?;
        if let Some(domain) = target_domain {
            query.add_command(ebisearchquery::QueryCommand::MoreLikeThis(Some(domain)));
        }
        self.run(query).await
    }

    pub async fn seq_tool_results(
        &self,
        tool_id: String,
        job_id: String,
        filters: Option<ebisearchquery::EbiSearchFilters>,
    ) -> EbioticResult<EbiSearchResult> {
        let query = ebisearchquery::EbiSearchQuery::new(
            vec![ebisearchquery::QueryCommand::SeqToolResults(
                tool_id, job_id,
            )],
            filters,
        )?;
        self.run(query).await
    }

    pub async fn top_terms(
        &self,
        field_id: String,
        filters: Option<ebisearchquery::EbiSearchFilters>,
    ) -> EbioticResult<EbiSearchResult> {
        let query = ebisearchquery::EbiSearchQuery::new(
            vec![ebisearchquery::QueryCommand::TopTerms(field_id)],
            filters,
        )?;
        self.run(query).await
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

    pub fn domain(&self) -> &ebisearchdomains::EbiSearchDomains {
        &self.domain
    }

    pub fn return_format(&self) -> &DataReturnFormats {
        &self.return_format
    }

    pub fn client(self) -> EbioticClient {
        self.client
    }
}

impl Service for EbiSearch {
    type ResultType = EbiSearchResult;
    type InputType = ebisearchquery::EbiSearchQuery;

    async fn run(&self, query: Self::InputType) -> EbioticResult<Self::ResultType> {
        let query_url = query.build()?;
        let url = self.concat_url(&query_url);
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
                url.push_str(&format!("{}/", self.domain));
            }
        }

        url.push_str(&format!("{}", query));
        url.push_str(&format!("&format={}", self.return_format));
        return url;
    }
}
