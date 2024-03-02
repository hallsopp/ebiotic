use bio::io::fasta::Record;

use serde::de::Deserializer;
use serde::Deserialize;
use serde_json::Value;

use super::BLAST_ENDPOINT;
use crate::core::{EbioticClient, EbioticHttpClient, PollStatus, PollableService, Service};
use crate::errors::EbioticError;

/// The `Description` struct is used to specify the description of the hit.
#[derive(Deserialize, Debug, Clone)]
pub struct Description {
    id: String,
    accession: String,
    title: String,
    taxid: u32,
    sciname: String,
}

/// The `Hsp` struct is used to specify the High-scoring Segment Pair (HSP) of the hit.
#[derive(Deserialize, Debug, Clone)]
pub struct Hsp {
    num: u32,
    bit_score: f64,
    score: u32,
    evalue: f64,
    identity: u32,
    #[serde(deserialize_with = "deserialize_hseq")]
    hseq: Record,
}

/// The `Hit` struct is used to specify the hit from the BLAST search.
#[derive(Deserialize, Debug, Clone)]
pub struct Hit {
    num: u32,
    description: Vec<Description>,
    len: u32,
    hsps: Vec<Hsp>,
}

/// The `BlastResult` struct is used to specify the result of the BLAST search.
#[derive(Deserialize, Debug, Clone)]
pub struct BlastResult {
    query_id: String,
    query_title: String,
    query_len: u32,
    hits: Vec<Hit>,
}

/// The `Blast` struct is used to specify the parameters for the `Blast` service.
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
    // but this is not possible as the context is not available
    // in the deserializer
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

    pub fn set_endpoint(&mut self, endpoint: String) {
        self.endpoint = endpoint;
    }

    pub fn set_program(&mut self, program: String) {
        self.program = program;
    }

    pub fn set_database(&mut self, database: String) {
        self.database = database;
    }

    pub fn set_matrix(&mut self, matrix: String) {
        self.matrix = matrix;
    }

    pub fn set_hitlist_size(&mut self, hitlist_size: u32) {
        self.hitlist_size = hitlist_size;
    }

    pub fn set_email(&mut self, email: String) {
        self.email = email;
    }

    pub fn set_tool(&mut self, tool: String) {
        self.tool = tool;
    }
}

impl BlastResult {
    pub fn query_id(&self) -> &String {
        &self.query_id
    }

    pub fn query_title(&self) -> &String {
        &self.query_title
    }

    pub fn query_len(&self) -> &u32 {
        &self.query_len
    }

    pub fn hits(&self) -> &Vec<Hit> {
        &self.hits
    }
}

impl Hit {
    pub fn num(&self) -> &u32 {
        &self.num
    }

    pub fn description(&self) -> &Vec<Description> {
        &self.description
    }

    pub fn len(&self) -> &u32 {
        &self.len
    }

    pub fn hsps(&self) -> &Vec<Hsp> {
        &self.hsps
    }
}

impl Hsp {
    pub fn num(&self) -> &u32 {
        &self.num
    }

    pub fn bit_score(&self) -> &f64 {
        &self.bit_score
    }

    pub fn score(&self) -> &u32 {
        &self.score
    }

    pub fn evalue(&self) -> &f64 {
        &self.evalue
    }

    pub fn identity(&self) -> &u32 {
        &self.identity
    }

    pub fn hseq(&self) -> &Record {
        &self.hseq
    }
}

impl Description {
    pub fn id(&self) -> &String {
        &self.id
    }

    pub fn accession(&self) -> &String {
        &self.accession
    }

    pub fn title(&self) -> &String {
        &self.title
    }

    pub fn taxid(&self) -> &u32 {
        &self.taxid
    }

    pub fn sciname(&self) -> &String {
        &self.sciname
    }
}

impl Service for Blast {
    type ResultType = BlastResult;
    type InputType = String;

    /// Run the `Blast` service with a query.
    async fn run(
        &self,
        client: EbioticClient,
        input: Self::InputType,
    ) -> Result<Self::ResultType, EbioticError> {
        let response = client
            .post_form(
                &self.endpoint,
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

        let _search_info = client
            .poll(
                &self.endpoint,
                Some(&[
                    ("CMD", "Get"),
                    ("FORMAT_OBJECT", "SearchInfo"),
                    ("RID", &rid),
                ]),
                &self,
            )
            .await?;

        let search_results = client
            .post_form(
                &self.endpoint,
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

    #[test]
    fn test_update_functions() {
        let mut blast = Blast::new(
            "endpoint".to_string(),
            "program".to_string(),
            "database".to_string(),
            "matrix".to_string(),
            10,
            "email".to_string(),
            "tool".to_string(),
        );

        // Update the values
        blast.set_endpoint("new_endpoint".to_string());
        blast.set_program("new_program".to_string());
        blast.set_database("new_database".to_string());
        blast.set_matrix("new_matrix".to_string());
        blast.set_hitlist_size(20);
        blast.set_email("new_email".to_string());
        blast.set_tool("new_tool".to_string());

        // Check that the values have been updated correctly
        assert_eq!(blast.endpoint, "new_endpoint");
        assert_eq!(blast.program, "new_program");
        assert_eq!(blast.database, "new_database");
        assert_eq!(blast.matrix, "new_matrix");
        assert_eq!(blast.hitlist_size, 20);
        assert_eq!(blast.email, "new_email");
        assert_eq!(blast.tool, "new_tool");
    }
}
