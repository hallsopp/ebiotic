use reqwest::Client;
use std::fmt::Write;

pub use bio::io::fasta::{Record, Records};

use super::TOOLS_ENDPOINT;
use crate::core::{self, PollStatus, Pollable};

pub struct Clustalo {
    endpoint: String,
    email: String,
    sequences: Vec<Record>,
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

    pub async fn run(&self) -> String {
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
        .await
        .unwrap();

        let poll_endpoint = format!("{}{}{}", &self.endpoint, &"status/", &response);

        let search = core::poll(&poll_endpoint, client.clone(), None, &self).await;

        if search.is_ok() {
            let results = client
                .get(&format!(
                    "{}{}{}{}",
                    self.endpoint, "result/", &response, "/aln-clustal_num"
                ))
                .send()
                .await
                .unwrap()
                .text()
                .await
                .unwrap();

            return results;
        } else {
            todo!();
        }
    }
}

impl Pollable for &Clustalo {
    fn poll_status(&self, response: &str) -> PollStatus {
        println!("{}", response);
        match response {
            "FINISHED" => PollStatus::Finished,
            "RUNNING" => PollStatus::Running(60),
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
}
