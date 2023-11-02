use reqwest::Client;
use serde::Deserialize;
use serde_json::Value;
use tokio::time::{self, Duration};

use super::core::{self, PollResult, BLAST_ENDPOINT};

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
        let response = self.submit(query, client.clone()).await.unwrap();
        let (rid, rtoe) = self.fetch_ridrtoe(&response);

        if self.poll(&rid, &rtoe, client.clone()).await {
            let raw_results = core::post_form(
                &self.endpoint,
                client.clone(),
                &[("CMD", "Get"), ("RID", &rid)],
            )
            .await
            .unwrap();

            return Ok(self.parse_raw_results(&raw_results).unwrap());
        } else {
            panic!("Something went wrong with the BLAST job");
        }
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

    fn check_status(&self, response: &str) -> bool {
        let mut status = false;
        for line in response.lines() {
            let trimmed_line = line.trim_start();
            if trimmed_line.starts_with("Status=") {
                status = trimmed_line["Status=".len()..].to_string() == "READY";
            }
        }
        status
    }

    // TODO: add error handling
    fn fetch_ridrtoe(&self, response: &str) -> (String, String) {
        let mut rid = String::new();
        let mut rtoe = String::new();
        for line in response.lines() {
            let trimmed_line = line.trim_start();
            if trimmed_line.starts_with("RID = ") {
                rid = trimmed_line["RID = ".len()..].to_string();
            } else if trimmed_line.starts_with("RTOE = ") {
                rtoe = trimmed_line["RTOE = ".len()..].to_string();
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
        let test_json = include_str!("../../../tests/example_blast_response.json");
        let blast = Blast::default();
        Blast::parse_raw_results(&blast, test_json);
    }
}
