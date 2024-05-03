use crate::errors::EbioticError;
use bio::io::fasta::{Reader, Record};
use std::future::Future;
use std::io::Cursor;

mod network;
mod reqwest;

pub use self::network::EbioticHttpClient;
// This will allow us in the future to optionally compile with different clients
// but maintain the same interface internally
pub use self::reqwest::EbioticReqwestClient as EbioticClient;

pub type EbioticResult<T> = Result<T, EbioticError>;

pub(crate) enum PollStatus {
    Finished,
    Running(u64),
    Error(EbioticError),
}

pub(crate) trait PollableService {
    fn poll_status(&self, response: &str) -> PollStatus;
}

pub trait Service {
    type ResultType;
    type InputType;

    fn run(
        &self,
        input: Self::InputType,
    ) -> impl Future<Output = EbioticResult<Self::ResultType>> + Send;
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
