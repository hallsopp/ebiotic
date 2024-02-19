use bio::io::fasta::{Reader, Record};
use reqwest::Client;
use tokio::time::{self, Duration};

use std::future::Future;
use std::io::Cursor;

use crate::errors::EbioticError;

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
    ) -> impl Future<Output = Result<Self::ResultType, EbioticError>> + Send;
}

pub(crate) async fn post_form(
    endpoint: &str,
    client: Client,
    body: &[(&str, &str)],
) -> Result<String, EbioticError> {
    let response = client.post(endpoint).form(body).send().await?;
    Ok(response.text().await?)
}

// Use this a feedback loop to check if the job is done
pub(crate) async fn poll<F>(
    endpoint: &str,
    client: Client,
    post_body: Option<&[(&str, &str)]>,
    method_caller: &F,
) -> Result<String, EbioticError>
where
    F: PollableService,
{
    loop {
        let response;
        if let Some(body) = post_body {
            response = post_form(endpoint, client.clone(), body).await?;
        } else {
            response = client.get(endpoint).send().await?.text().await?;
        }

        let status = method_caller.poll_status(&response);

        match status {
            PollStatus::Finished => return Ok(response),
            PollStatus::Running(sleep_time) => {
                println!("Job is still running, sleeping for {} seconds", sleep_time);
                time::sleep(Duration::from_secs(sleep_time)).await;
            }
            PollStatus::Error(err) => return Err(err),
        }
    }
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
