use reqwest::Client;
use serde::Deserialize;
use serde_json::Value;

use super::BLAST_ENDPOINT;

use crate::core::{self, PollStatus, Pollable};

#[derive(Deserialize, Debug)]
struct Description {
    id: String,
    accession: String,
    title: String,
    taxid: u32,
    sciname: String,
}

#[derive(Deserialize, Debug)]
struct Hsp {
    bit_score: f64,
    score: u32,
    evalue: f64,
    identity: u32,
    hseq: String,
}

#[derive(Deserialize, Debug)]
struct Hit {
    num: u32,
    description: Vec<Description>,
    len: u32,
    hsps: Vec<Hsp>,
}

#[derive(Deserialize, Debug)]
pub struct Search {
    query_id: String,
    query_title: String,
    query_len: u32,
    hits: Vec<Hit>,
}

pub struct Blast {
    endpoint: String,
    program: String,
    database: String,
    matrix: String,
    hitlist_size: u32,
    email: String,
    tool: String,
}

impl Default for Blast {
    fn default() -> Self {
        Blast {
            endpoint: BLAST_ENDPOINT.to_string(),
            program: "blastp".to_string(),
            database: "nr".to_string(),
            matrix: "BLOSUM62".to_string(),
            hitlist_size: 10,
            email: "".to_string(),
            tool: "".to_string(),
        }
    }
}

impl Blast {
    pub fn new(
        endpoint: String,
        program: String,
        database: String,
        matrix: String,
        hitlist_size: u32,
        email: String,
        tool: String,
    ) -> Blast {
        Blast {
            endpoint,
            program,
            matrix,
            database,
            hitlist_size,
            email,
            tool,
        }
    }

    pub async fn run(&self, query: &str) -> Result<Search, ()> {
        let client = Client::new();
        let response = core::post_form(
            &self.endpoint,
            client.clone(),
            &[
                ("CMD", "Put"),
                ("PROGRAM", &self.program),
                ("DATABASE", &self.database),
                ("MATRIX", &self.matrix),
                ("HITLIST_SIZE", &self.hitlist_size.to_string()),
                ("EMAIL", &self.email),
                ("TOOL", &self.tool),
                ("QUERY", query),
            ],
        )
        .await
        .unwrap();

        let (rid, rtoe) = self.fetch_ridrtoe(&response);

        let search_info = core::poll(
            &self.endpoint,
            client.clone(),
            Some(&[
                ("CMD", "Get"),
                ("FORMAT_OBJECT", "SearchInfo"),
                ("RID", &rid),
            ]),
            &self,
        )
        .await;

        if search_info.is_ok() {
            let search_results = core::post_form(
                &self.endpoint,
                client.clone(),
                &[
                    ("CMD", "Get"),
                    ("FORMAT_TYPE", "JSON2"),
                    ("RID", &rid),
                    ("FORMAT_OBJECT", "Search"),
                ],
            )
            .await
            .unwrap();
            self.parse_raw_results(&search_results)
        } else {
            panic!("Something went wrong with the job");
        }
    }
}

impl Pollable for &Blast {
    fn poll_status(&self, response: &str) -> PollStatus {
        for line in response.lines() {
            let trimmed_line = line.trim_start();
            if let Some(line) = trimmed_line.strip_prefix("Status=") {
                let status_string = line.to_string();
                if status_string == "READY" {
                    return PollStatus::Finished;
                } else if status_string == "WAITING" {
                    return PollStatus::Running(60);
                }
            }
        }
        PollStatus::Error
    }
}

impl Blast {
    // TODO: add error handling
    fn parse_raw_results(&self, raw_results: &str) -> Result<Search, ()> {
        let parsed: Value = serde_json::from_str(raw_results).unwrap();
        let flat = &parsed["BlastOutput2"][0]["report"]["results"]["search"];

        if flat != &Value::Null {
            let search: Search = serde_json::from_value(flat.clone()).unwrap();
            Ok(search)
        } else {
            panic!("No results found");
        }
    }

    // TODO: add error handling
    fn fetch_ridrtoe(&self, response: &str) -> (String, String) {
        let mut rid = String::new();
        let mut rtoe = String::new();
        for line in response.lines() {
            let trimmed_line = line.trim_start();
            if let Some(line) = trimmed_line.strip_prefix("RID = ") {
                rid = line.to_string();
            } else if let Some(line) = trimmed_line.strip_prefix("RTOE = ") {
                rtoe = line.to_string();
            }
        }
        (rid, rtoe)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_raw_results() {
        let test_json = include_str!("../../tests/example_blast_response.json");
        let blast = Blast::default();
        Blast::parse_raw_results(&blast, test_json);
    }
}
