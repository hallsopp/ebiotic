use reqwest::{Client, Result};
use tokio::time::{self, Duration};

pub(crate) enum PollStatus {
    Finished,
    Running(u64),
    Error,
}

pub(crate) trait Pollable {
    fn poll_status(&self, response: &str) -> PollStatus;
}

pub(crate) async fn post_form(
    endpoint: &str,
    client: Client,
    body: &[(&str, &str)],
) -> Result<String> {
    client.post(endpoint).form(body).send().await?.text().await
}

// Use this a feedback loop to check if the job is done
pub(crate) async fn poll<F>(
    endpoint: &str,
    client: Client,
    post_body: Option<&[(&str, &str)]>,
    method_caller: &F,
) -> Result<String>
where
    F: Pollable,
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
            PollStatus::Error => panic!("Something went wrong with the job"),
        }
    }
}
