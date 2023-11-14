use reqwest::Client;
use std::fmt::Write;
use std::io::{BufRead, Cursor};

use super::TOOLS_ENDPOINT;
use crate::core::{self, PollStatus, PollableService};
use crate::errors::EbioticError;

pub use bio::io::fasta::{Reader, Record, Records};

pub struct Clustalo {
    endpoint: String,
    email: String,
    sequences: Vec<Record>,
}

pub struct ClustaloResult {
    aln_clustal_num: String,
    pim: Vec<Vec<f64>>,
    phylotree: String,
    fasta: Vec<Record>,
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

    pub async fn run(&self) -> Result<ClustaloResult, EbioticError> {
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
            let acn = client
                .get(&format!(
                    "{}{}{}{}",
                    self.endpoint, "result/", &response, "/aln-clustal_num"
                ))
                .send()
                .await?
                .text()
                .await?;

            let pim = client
                .get(&format!(
                    "{}{}{}{}",
                    self.endpoint, "result/", &response, "/pim"
                ))
                .send()
                .await?
                .text()
                .await?;

            let phylotree = client
                .get(&format!(
                    "{}{}{}{}",
                    self.endpoint, "result/", &response, "/phylotree"
                ))
                .send()
                .await?
                .text()
                .await?;

            let fasta = client
                .get(&format!(
                    "{}{}{}{}",
                    self.endpoint, "result/", &response, "/clustal_num"
                ))
                .send()
                .await?
                .text()
                .await?;

            let results = ClustaloResult {
                aln_clustal_num: acn,
                pim: self.parse_pim_result(&pim)?,
                phylotree: phylotree,
                fasta: self.parse_fasta_result(&fasta)?,
            };

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

    fn parse_fasta_result(&self, raw_results: &str) -> Result<Vec<Record>, EbioticError> {
        let cursor = Cursor::new(raw_results.as_bytes());
        let reader = Reader::from_bufread(cursor);

        let records = reader
            .records()
            .collect::<Result<Vec<_>, std::io::Error>>()
            .map_err(EbioticError::from)?;

        Ok(records)
    }

    fn parse_pim_result(&self, raw_results: &str) -> Result<Vec<Vec<f64>>, EbioticError> {
        let mut pim = Vec::new();
        for line in raw_results.lines() {
            if line.trim().starts_with("#") || line.trim().is_empty() {
                continue;
            }
            let mut row = Vec::new();
            for value in line.split_whitespace() {
                row.push(value.parse::<f64>()?);
            }
            pim.push(row);
        }
        Ok(pim)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_pim() {
        let clustalo = Clustalo::default();

        let pim_string = "
        0.000000 0.000000 0.000000 0.000000
        0.000000 0.000000 0.000000 0.000000
        0.000000 0.000000 0.000000 0.000000
        0.000000 0.000000 0.000000 0.000000
        ";

        let pim = clustalo.parse_pim_result(&pim_string);

        assert_eq!(pim.is_ok(), true);
        assert_eq!(
            pim.unwrap(),
            vec![
                vec![0.0, 0.0, 0.0, 0.0],
                vec![0.0, 0.0, 0.0, 0.0],
                vec![0.0, 0.0, 0.0, 0.0],
                vec![0.0, 0.0, 0.0, 0.0]
            ]
        );
    }

    #[test]
    fn test_parse_fasta() {
        let clustalo = Clustalo::default();
        let fasta_string = ">seq1
AGCTTGAACGTTAGCGGAACGTAAGCGAGATCCGTAGGCTAACTCGTACGTA
>seq2
TACGATGCAAATCGTGCACGGTCCAGTACGATCCGATGCTAAGTCCGATCGA
>seq3
GCTAGTCCGATGCGTACGATCGTACGATGCTAGCTAGCTAGCTAGCTAGCTA
>seq4
CGTAGCTAGCTAGCTAGCTAGCTAGCTAGCTAGCTAGCTAGCTAGCTAGCTA";

        let fasta = clustalo.parse_fasta_result(&fasta_string);

        assert_eq!(fasta.is_ok(), true);
        assert_eq!(fasta.unwrap().len(), 4);
    }
}
