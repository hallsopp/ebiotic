use reqwest::Client;
use std::collections::HashMap;
use std::fmt::Write;
use std::io::{BufRead, Cursor};

use super::TOOLS_ENDPOINT;
use crate::core::{self, PollStatus, PollableService, Service};
use crate::errors::EbioticError;

pub use bio::io::fasta::{Reader, Record, Records};
use serde::Deserialize;

pub struct Clustalo {
    endpoint: String,
    email: String,
}

#[derive(Debug)]
pub struct ClustaloResult {
    aln_clustal_num: String,
    pim: HashMap<String, Vec<f64>>,
    phylotree: String,
    fasta: Vec<Record>,
}

impl Default for Clustalo {
    fn default() -> Self {
        Clustalo {
            endpoint: format!("{}{}", TOOLS_ENDPOINT.to_string(), "clustalo/"),
            email: "".to_string(),
        }
    }
}

impl Clustalo {
    pub fn new(endpoint: String, email: String) -> Clustalo {
        Clustalo { endpoint, email }
    }

    pub fn set_endpoint(&mut self, endpoint: String) -> () {
        self.endpoint = endpoint;
    }

    pub fn set_email(&mut self, email: String) -> () {
        self.email = email;
    }

    pub fn endpoint(&self) -> &String {
        &self.endpoint
    }

    pub fn email(&self) -> &String {
        &self.email
    }
}

impl Service for Clustalo {
    type ResultType = ClustaloResult;
    type InputType = Vec<Record>;

    async fn run(&self, input: Self::InputType) -> Result<Self::ResultType, EbioticError> {
        let client = Client::new();

        let run_endpoint = format!("{}{}", &self.endpoint, "run/");

        let response = core::post_form(
            &run_endpoint,
            client.clone(),
            &[
                ("email", &self.email.as_str()),
                ("sequence", &self.pretty_format_records(input).as_str()),
            ],
        )
        .await?;

        let poll_endpoint = format!("{}{}{}", &self.endpoint, &"status/", &response);

        // Polling to wait for the result, however result is not directly returned
        let _ = core::poll(&poll_endpoint, client.clone(), None, &self).await?;

        // Assuming the polling does not error out, the earlier response number
        // can be used to fetch the results
        let acn = client
            .get(&format!(
                "{}{}{}{}",
                &self.endpoint, "result/", &response, "/aln-clustal_num"
            ))
            .send()
            .await?
            .text()
            .await?;

        let pim = client
            .get(&format!(
                "{}{}{}{}",
                &self.endpoint, "result/", &response, "/pim"
            ))
            .send()
            .await?
            .text()
            .await?;

        let phylotree = client
            .get(&format!(
                "{}{}{}{}",
                &self.endpoint, "result/", &response, "/phylotree"
            ))
            .send()
            .await?
            .text()
            .await?;

        let fasta = client
            .get(&format!(
                "{}{}{}{}",
                &self.endpoint, "result/", &response, "/clustal_num"
            ))
            .send()
            .await?
            .text()
            .await?;

        let results = ClustaloResult {
            aln_clustal_num: acn,
            pim: self.parse_pim_result(&pim)?,
            phylotree,
            fasta: self.parse_fasta_result(&fasta)?,
        };

        return Ok(results);
    }
}

impl PollableService for &Clustalo {
    fn poll_status(&self, response: &str) -> PollStatus {
        match response {
            "FINISHED" => PollStatus::Finished,
            "RUNNING" => PollStatus::Running(3),
            "QUEUED" => PollStatus::Running(3),
            _ => PollStatus::Error(EbioticError::ServiceError(
                "Something went wrong with the job.".to_string(),
            )),
        }
    }
}

impl Clustalo {
    fn pretty_format_records(&self, sequences: Vec<Record>) -> String {
        let mut records = String::new();
        for record in &sequences {
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

    fn parse_pim_result(
        &self,
        raw_results: &str,
    ) -> Result<HashMap<String, Vec<f64>>, EbioticError> {
        let mut pim = HashMap::new();
        for line in raw_results.lines() {
            if line.trim().starts_with("#") || line.trim().is_empty() {
                continue;
            }
            let mut row = Vec::new();
            let split_line: Vec<&str> = line.split_whitespace().collect();
            let sequence_name = split_line[1].to_string();
            for value in &split_line[2..] {
                row.push(value.parse::<f64>()?);
            }
            if row.is_empty() {
                return Err(EbioticError::ServiceError(format!(
                    "No valid percentages found for sequence: {}",
                    sequence_name
                )));
            }
            pim.insert(sequence_name, row);
        }
        if pim.is_empty() {
            return Err(EbioticError::ServiceError(
                "No valid lines found in Percent Identity Matrix (PIM).".to_string(),
            ));
        }
        Ok(pim)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bio::io::fasta::Record;
    use std::io::Cursor;

    #[test]
    fn clustalo_new_creates_correct_instance() {
        let endpoint = "http://example.com".to_string();
        let email = "test@example.com".to_string();

        let clustalo = Clustalo::new(endpoint.clone(), email.clone());

        assert_eq!(clustalo.endpoint(), &endpoint);
        assert_eq!(clustalo.email(), &email);
    }

    #[test]
    fn clustalo_setters_update_fields_correctly() {
        let mut clustalo = Clustalo::default();
        let endpoint = "http://example.com".to_string();
        let email = "test@example.com".to_string();

        clustalo.set_endpoint(endpoint.clone());
        clustalo.set_email(email.clone());

        assert_eq!(clustalo.endpoint(), &endpoint);
        assert_eq!(clustalo.email(), &email);
    }

    #[test]
    fn pretty_format_records_formats_correctly() {
        let sequences = vec![
            Record::with_attrs(
                "seq1",
                None,
                b"AGCTTGAACGTTAGCGGAACGTAAGCGAGATCCGTAGGCTAACTCGTACGTA",
            ),
            Record::with_attrs(
                "seq2",
                None,
                b"TACGATGCAAATCGTGCACGGTCCAGTACGATCCGATGCTAAGTCCGATCGA",
            ),
        ];
        let clustalo = Clustalo::default();

        let formatted = clustalo.pretty_format_records(sequences);

        assert_eq!(formatted, ">seq1\nAGCTTGAACGTTAGCGGAACGTAAGCGAGATCCGTAGGCTAACTCGTACGTA\n>seq2\nTACGATGCAAATCGTGCACGGTCCAGTACGATCCGATGCTAAGTCCGATCGA\n");
    }

    #[test]
    fn parse_fasta_result_parses_correctly() {
        let clustalo = Clustalo::default();
        let fasta_string = ">seq1\nAGCTTGAACGTTAGCGGAACGTAAGCGAGATCCGTAGGCTAACTCGTACGTA\n>seq2\nTACGATGCAAATCGTGCACGGTCCAGTACGATCCGATGCTAAGTCCGATCGA";

        let fasta = clustalo.parse_fasta_result(&fasta_string).unwrap();

        assert_eq!(fasta.len(), 2);
        assert_eq!(fasta[0].id(), "seq1");
        assert_eq!(
            fasta[0].seq(),
            b"AGCTTGAACGTTAGCGGAACGTAAGCGAGATCCGTAGGCTAACTCGTACGTA"
        );
        assert_eq!(fasta[1].id(), "seq2");
        assert_eq!(
            fasta[1].seq(),
            b"TACGATGCAAATCGTGCACGGTCCAGTACGATCCGATGCTAAGTCCGATCGA"
        );
    }

    #[test]
    fn parse_pim_result_parses_correctly() {
        let clustalo = Clustalo::default();
        let pim_string = "\
        1: Sequence1   100.00   36.73   40.91   40.91   40.00
        2: Sequence2    36.73  100.00   44.44   31.71   33.33
        3: Sequence3    40.91   44.44  100.00   77.78   83.78
        4: Sequence4    40.91   31.71   77.78  100.00   96.00
        5: Sequence5    40.00   33.33   83.78   96.00  100.00";

        let pim = clustalo.parse_pim_result(&pim_string).unwrap();

        assert_eq!(pim.len(), 5);
        assert_eq!(pim["Sequence1"], vec![100.00, 36.73, 40.91, 40.91, 40.00]);
        assert_eq!(pim["Sequence2"], vec![36.73, 100.00, 44.44, 31.71, 33.33]);
        assert_eq!(pim["Sequence3"], vec![40.91, 44.44, 100.00, 77.78, 83.78]);
        assert_eq!(pim["Sequence4"], vec![40.91, 31.71, 77.78, 100.00, 96.00]);
        assert_eq!(pim["Sequence5"], vec![40.00, 33.33, 83.78, 96.00, 100.00]);
    }

    #[test]
    fn parse_pim_result_handles_invalid_input() {
        let clustalo = Clustalo::default();
        let pim_string = "invalid input";

        let pim = clustalo.parse_pim_result(&pim_string);

        assert!(pim.is_err());
    }

    #[test]
    fn parse_fasta_result_handles_invalid_input() {
        let clustalo = Clustalo::default();
        let fasta_string = "invalid input";

        let fasta = clustalo.parse_fasta_result(&fasta_string);

        assert!(fasta.is_err());
    }
}
