use reqwest::{Client, Result};
use serde::Serialize;
use tokio::time::{self, Duration};

pub const TOOLS_ENDPOINT: &str = "https://www.ebi.ac.uk/Tools/services/rest/";
pub const BLAST_ENDPOINT: &str = "https://blast.ncbi.nlm.nih.gov/Blast.cgi";

pub(crate) enum PollResult {
    Finished,
    Running,
    Error,
}

pub(crate) async fn post_form(
    endpoint: &str,
    client: Client,
    body: &[(&str, &dyn Serialize)],
) -> Result<String> {
    client.post(endpoint).form(body).send().await?.text().await
}

// Use this a feedback loop to check if the job is done
pub(crate) async fn poll<F>(
    endpoint: &str,
    client: Client,
    sleep_time: u64,
    post_body: Option<&[(&str, &dyn Serialize)]>,
    status_function: F,
) -> Result<String>
where
    F: Fn(&str) -> PollResult,
{
    loop {
        let mut response;
        if let Some(body) = post_body {
            response = post_form(endpoint, client.clone(), body).await?;
        } else {
            response = client.get(endpoint).send().await?.text().await?;
        }

        let status = status_function(&response);

        match status {
            PollResult::Finished => return Ok(response),
            PollResult::Running => {
                time::sleep(Duration::from_secs(sleep_time)).await;
            }
            PollResult::Error => panic!("Something went wrong with the job"),
        }
    }
}
