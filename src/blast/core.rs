use reqwest::Client;
use serde_json::Value;
use tokio::time::{self, Duration};

use super::results::Search;

const BLAST_ENDPOINT: &str = "https://blast.ncbi.nlm.nih.gov/Blast.cgi";

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
            let raw_results = self
                .fetch_results(&rid, &rtoe, client.clone())
                .await
                .unwrap();

            return Ok(self.parse_raw_results(&raw_results).unwrap());
        } else {
            panic!("Something went wrong with the BLAST job");
        }
    }
}

impl Blast {
    async fn submit(&self, query: &str, client: Client) -> reqwest::Result<String> {
        client
            .post(&self.endpoint)
            .form(&[
                ("CMD", "Put"),
                ("PROGRAM", &self.program),
                ("DATABASE", &self.database),
                ("MATRIX_NAME", &self.matrix),
                ("HITLIST_SIZE", &self.hitlist_size.to_string()),
                ("EMAIL", &self.email),
                ("TOOL", &self.tool),
                ("QUERY", query),
            ])
            .send()
            .await?
            .text()
            .await
    }

    // Use this a feedback loop to check if the job is done
    async fn poll(&self, rid: &str, rtoe: &str, client: Client) -> bool {
        loop {
            let response = client
                .post(&self.endpoint)
                .form(&[
                    ("CMD", "Get"),
                    ("FORMAT_OBJECT", "SearchInfo"),
                    ("RID", rid),
                    ("RTOE", rtoe),
                ])
                .send()
                .await
                .unwrap() // TODO: add error handling
                .text()
                .await
                .unwrap();
            if self.check_status(&response) {
                return true;
            } else {
                println!("BLAST job not ready yet, sleeping for 60 seconds");
                time::sleep(Duration::from_secs(60)).await;
            }
        }
    }

    async fn fetch_results(
        &self,
        rid: &str,
        rtoe: &str,
        client: Client,
    ) -> reqwest::Result<String> {
        client
            .post(&self.endpoint)
            .form(&[
                ("CMD", "Get"),
                ("FORMAT_TYPE", "JSON2_S"),
                ("RID", rid),
                ("RTOE", rtoe),
            ])
            .send()
            .await?
            .text()
            .await
    }

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
        let test_json = include_str!("../../tests/example_blast_response.json");
        let blast = Blast::default();
        Blast::parse_raw_results(&blast, test_json);
    }
}
