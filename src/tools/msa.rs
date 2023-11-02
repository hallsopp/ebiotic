use serde::Deserialize;

use super::core::TOOLS_ENDPOINT;

pub struct Clustalo {
    endpoint: String,
    email: String,
    sequence: String,
}

impl Clustalo {
    pub fn new(email: String, sequence: String) -> Clustalo {
        Clustalo {
            endpoint: TOOLS_ENDPOINT.to_string(),
            email,
            sequence,
        }
    }

    pub async fn run(&self) -> () {
        todo!()
    }
}
