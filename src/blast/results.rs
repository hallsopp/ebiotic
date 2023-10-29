use serde::Deserialize;

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
