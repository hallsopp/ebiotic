use bio::io::fasta::{Reader, Record};

use std::future::Future;
use std::io::Cursor;

use crate::errors::EbioticError;

mod reqwest;

pub use self::reqwest::EbioticReqwestClient as EbioticClient;

pub(crate) enum PollStatus {
    Finished,
    Running(u64),
    Error(EbioticError),
}

pub(crate) trait EbioticHttpClient: Default + Send + Clone {
    async fn post_form(
        &self,
        endpoint: &str,
        body: &[(&str, &str)],
    ) -> Result<String, EbioticError>;
    async fn get(&self, endpoint: &str) -> Result<String, EbioticError>;
    async fn poll<F>(
        &self,
        endpoint: &str,
        post_body: Option<&[(&str, &str)]>,
        method_caller: &F,
    ) -> Result<String, EbioticError>
    where
        F: PollableService;
}

pub(crate) trait PollableService {
    fn poll_status(&self, response: &str) -> PollStatus;
}

pub trait Service {
    type ResultType;
    type InputType;

    fn run(
        &self,
        client: EbioticClient,
        input: Self::InputType,
    ) -> impl Future<Output = Result<Self::ResultType, EbioticError>> + Send;
}

pub(crate) fn parse_fa_from_bufread(raw_results: &str) -> Result<Vec<Record>, EbioticError> {
    let cursor = Cursor::new(raw_results.as_bytes());
    let reader = Reader::from_bufread(cursor);

    let records = reader
        .records()
        .collect::<Result<Vec<_>, std::io::Error>>()
        .map_err(EbioticError::from)?;

    Ok(records)
}
