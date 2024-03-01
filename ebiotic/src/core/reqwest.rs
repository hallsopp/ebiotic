use reqwest::Client;
use tokio::time::{self, Duration};

use crate::core::{EbioticHttpClient, PollStatus, PollableService};
use crate::errors::EbioticError;

#[derive(Clone)]
pub struct EbioticReqwestClient {
    pub(crate) client: Client,
}

impl Default for EbioticReqwestClient {
    fn default() -> Self {
        EbioticReqwestClient {
            client: Client::new(),
        }
    }
}

impl EbioticReqwestClient {
    pub fn new(client: Client) -> EbioticReqwestClient {
        EbioticReqwestClient { client }
    }
}

impl EbioticHttpClient for EbioticReqwestClient {
    async fn post_form(
        &self,
        endpoint: &str,
        body: &[(&str, &str)],
    ) -> Result<String, EbioticError> {
        let response = self.client.post(endpoint).form(body).send().await?;
        Ok(response.text().await?)
    }

    async fn get(&self, endpoint: &str) -> Result<String, EbioticError> {
        let response = self.client.get(endpoint).send().await?;
        Ok(response.text().await?)
    }

    async fn poll<F>(
        &self,
        endpoint: &str,
        post_body: Option<&[(&str, &str)]>,
        method_caller: &F,
    ) -> Result<String, EbioticError>
    where
        F: PollableService,
    {
        loop {
            let response;
            if let Some(body) = post_body {
                response = self.post_form(endpoint, body).await?;
            } else {
                response = self.get(endpoint).await?;
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
}
