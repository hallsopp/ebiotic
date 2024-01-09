use bio::io::fasta::Record;
use reqwest::Client;
use serde::de::{Deserializer, Visitor};
use serde::Deserialize;
use serde_json::Value;

use super::BLAST_ENDPOINT;

use crate::core::{self, PollStatus, PollableService, Service};
use crate::errors::EbioticError;

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
    num: u32,
    bit_score: f64,
    score: u32,
    evalue: f64,
    identity: u32,
    #[serde(deserialize_with = "deserialize_hseq")]
    hseq: Record,
}

#[derive(Deserialize, Debug)]
struct Hit {
    num: u32,
    description: Vec<Description>,
    len: u32,
    hsps: Vec<Hsp>,
}

#[derive(Deserialize, Debug)]
pub struct BlastResult {
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

fn deserialize_hseq<'de, D>(deserializer: D) -> Result<Record, D::Error>
where
    D: Deserializer<'de>,
{
    let hseq_str = String::deserialize(deserializer)?;
    // Potentially change the id to be linked to Hsp.num,
    // but this is not possible as
    Ok(Record::with_attrs(
        "consensus_hsp_seq",
        None,
        hseq_str.as_bytes(),
    ))
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
}

impl Service for Blast {
    type ResultType = BlastResult;
    type InputType = String;

    async fn run(&self, input: Self::InputType) -> Result<Self::ResultType, EbioticError> {
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
                ("QUERY", &input),
            ],
        )
        .await?;

        let (rid, _rtoe) = &self.fetch_ridrtoe(&response);

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
        .await?;

        let search_results = core::post_form(
            &self.endpoint,
            client.clone(),
            &[("CMD", "Get"), ("FORMAT_TYPE", "JSON2_S"), ("RID", &rid)],
        )
        .await?;
        self.parse_raw_results(&search_results)
    }
}

impl PollableService for &Blast {
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
        PollStatus::Error(EbioticError::ServiceError(
            "Something went wrong with the job".to_string(),
        ))
    }
}

impl Blast {
    fn parse_raw_results(&self, raw_results: &str) -> Result<BlastResult, EbioticError> {
        let parsed: Value = serde_json::from_str(raw_results)?;
        let flat = &parsed["BlastOutput2"][0]["report"]["results"]["search"];

        if flat != &Value::Null {
            let search: BlastResult = serde_json::from_value(flat.clone())?;
            Ok(search)
        } else {
            Err(EbioticError::ServiceError(
                "No results were found.".to_string(),
            ))
        }
    }

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
        blast.parse_raw_results(test_json).unwrap();
    }
}
