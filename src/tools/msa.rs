use reqwest::Client;
use std::fmt::Write;
use std::io::BufReader;

use super::TOOLS_ENDPOINT;
use crate::core::{self, PollStatus, PollableService};
use crate::errors::EbioticError;

pub use bio::io::fasta::{Reader, Record, Records};

pub struct Clustalo {
    endpoint: String,
    email: String,
    sequences: Vec<Record>,
}

pub struct ClustaloResult<'a> {
    aln_clustal_num: String,
    pim: String,
    phylotree: String,
    fasta: Records<BufReader<&'a [u8]>>,
}

impl Default for Clustalo {
    fn default() -> Self {
        Clustalo {
            endpoint: format!("{}{}", TOOLS_ENDPOINT.to_string(), "clustalo/"),
            email: "".to_string(),
            sequences: Vec::new(),
        }
    }
}

impl Clustalo {
    pub fn new(endpoint: String, email: String, sequences: Vec<Record>) -> Clustalo {
        Clustalo {
            endpoint,
            email,
            sequences,
        }
    }

    pub fn set_endpoint(&mut self, endpoint: String) -> () {
        self.endpoint = endpoint;
    }

    pub fn set_email(&mut self, email: String) -> () {
        self.email = email;
    }

    pub fn set_sequences(&mut self, sequences: Vec<Record>) -> () {
        self.sequences = sequences;
    }

    pub fn endpoint(&self) -> &String {
        &self.endpoint
    }

    pub fn email(&self) -> &String {
        &self.email
    }

    pub fn sequences(&self) -> &Vec<Record> {
        &self.sequences
    }

    pub async fn run(&self) -> Result<String, EbioticError> {
        let client = Client::new();

        let run_endpoint = format!("{}{}", self.endpoint, "run/");

        let response = core::post_form(
            &run_endpoint,
            client.clone(),
            &[
                ("email", &self.email.as_str()),
                ("sequence", &self.pretty_format_records().as_str()),
            ],
        )
        .await?;

        let poll_endpoint = format!("{}{}{}", &self.endpoint, &"status/", &response);

        let search = core::poll(&poll_endpoint, client.clone(), None, &self).await;

        if search.is_ok() {
            let results = client
                .get(&format!(
                    "{}{}{}{}",
                    self.endpoint, "result/", &response, "/aln-clustal_num"
                ))
                .send()
                .await?
                .text()
                .await?;

            return Ok(results);
        } else {
            return Err(EbioticError::ServiceError(
                "Something went wrong with the job.".to_string(),
            ));
        }
    }
}

impl PollableService for &Clustalo {
    fn poll_status(&self, response: &str) -> PollStatus {
        match response {
            "FINISHED" => PollStatus::Finished,
            "RUNNING" => PollStatus::Running(3),
            "QUEUED" => PollStatus::Running(3),
            _ => PollStatus::Error,
        }
    }
}

impl Clustalo {
    fn pretty_format_records(&self) -> String {
        let mut records = String::new();
        for record in &self.sequences {
            write!(records, "{}", record).unwrap();
        }
        records
    }

    fn parse_fasta_result<'a>(
        &'a self,
        raw_results: &'a str,
    ) -> Result<Records<BufReader<&[u8]>>, EbioticError> {
        let reader = Reader::new(raw_results.as_bytes());
        Ok(reader.records())
    }
}
