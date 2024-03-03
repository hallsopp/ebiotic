//! # Ebiotic
//!
//! `ebiotic` provides a light-weight asynchronous interface for some popular bioinformatics web services. It is designed to
//! enable access to the rich data and tools provided by institutes like the European Bioinformatics
//! Institute's ([EBI](https://www.ebi.ac.uk/)) and the National Center for Biotechnology Information ([NCBI](https://www.ncbi.nlm.nih.gov/)). It's built to serialize
//! and deserialze data using common formats like JSON and specialised bioinformatics formats like FASTA using data structures from
//! the [rust-bio](https://rust-bio.github.io/) library.
//!
//! ## Modules
//!
//! * `data`: Includes interfaces to various bioinformatics databases and data retrieval services.
//! * `tools`: Various tools and utilities for bioinformatics analysis.
//!
//! ## Usage
//!
//! Add this to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! ebiotic = "0.0.23"
//! ```
//!
//! ## Examples
//!
//! The idea of this crate is that all the services, regardless of the type, use a similar interface. This is achieved (currently) by using the `core::Service` trait and accompanying utilities.
//! Therefore, this trait must be in scope to use any of the services. This can be done by importing it directly or by importing an entire module that contains it. Each utility currently initialises
//! its own `reqwest::Client` which is used to make the requests. This was done for fine-grained control, but I hope to provide a way to use a custom client in the future.
//!
//! Here's an example of how to use the `Blast` service:
//!
//! ```rust
//! use ebiotic::tools::*;
//!
//! #[tokio::main]
//! async fn main() {
//!    // The `EbioticClient` is an HTTP client wrapper. At the moment, it supports an async `reqwest::Client`, but is modular so more frameworks can be added in the future.
//!    // This means that the client is configurable, i.e. a proxy or a custom user agent can be set. The client is designed to be initialized once and then passed to the services.
//!    let client = EbioticClient::default();
//!
//!    let blast = Blast::default();
//!    let query = "MAKQVQKARKLAEQAERYDDMAAAMKAVTEQGHELSNEERNLLSVAYKNVVGARRSSWRVISSIEQKTERNEKKQQMGKEYREKIEAELQDICNDVLELLDKYLIPNATQPESKVFYLKMKGDYFRYLSEVASGDNKQTTVSNSQQAYQEAFEISKKEMQPTHPIRLGLALNFSVFYYEILNSPDRACRLAKAAFDDASLAKDAESEKNPEEIAWYQSITQ";
//!    // The client is passed to the `run` method along with the query. The `run` method returns a `Result` which can be unwrapped to get the result.
//!    let result = blast.run(client, query.to_string()).await;
//! }
//! ```
//!
//! And this is an example of how to use the `Dbfetch` service to query the European Nucleotide Archive (ENA) for a sequence in FASTA format:
//!
//! ```rust
//! use ebiotic::data::*;
//!
//! #[tokio::main]
//! async fn main() {
//!     let client = EbioticClient::default();
//!     let dbfetch = Dbfetch::new(DbfetchDbs::EnaSequence, DbfetchReturnFormat::Fasta, DbfetchStyle::Raw);
//!     let ids = DbfetchIds::new(vec!["M10051".to_string(), "M10052".to_string()]);
//!
//!     // The `Dbfetch` service returns a `DbfetchResult` which can be converted into a `Vec<Record>` using the `into_records` method.
//!     // I hope to provide a more ergonomic way of doing this in the future.
//!     let result = dbfetch.run(client, ids).await.unwrap().into_records();
//! }
//!
//! // This is also the default configuration for the `Dbfetch` service, so the above can be written as:
//!
//! #[tokio::main]
//! async fn main_fasta() {
//!    let client = EbioticClient::default();
//!    let dbfetch = Dbfetch::default();
//!    let ids = DbfetchIds::new(vec!["M10051".to_string(), "M10052".to_string()]);
//!    let result = dbfetch.run(client, ids).await.unwrap().into_records();
//! }
//!```
//!
//! ### Synchronous example
//!
//! Despite being designed with asynchronicity in mind, the services can also be run synchronously by blocking on the thread. In the future I hope to provide an API for this.
//!
//! ```rust
//! use ebiotic::tools::*;
//! use tokio;
//!
//! #[tokio::main]
//! async fn main() {
//!    let client = EbioticClient::default();
//!    let blast = Blast::default();
//!    let query = "MAKQVQKARKLAEQAERYDDMAAAMKAVTEQGHELSNEERNLLSVAYKNVVGARRSSWRVISSIEQKTERNEKKQQMGKEYREKIEAELQDICNDVLELLDKYLIPNATQPESKVFYLKMKGDYFRYLSEVASGDNKQTTVSNSQQAYQEAFEISKKEMQPTHPIRLGLALNFSVFYYEILNSPDRACRLAKAAFDDASLAKDAESEKNPEEIAWYQSITQ";
//!    let result = tokio::task::block_in_place(|| blast.run(client, query.to_string()));
//! }
//! ```
//!
//! ### Custom reqwest client
//!
//! The EbioicClient can be configured with a custom reqwest client. This can be useful for setting platform or application specific settings like a proxy or a timeout.
//!
//! ```rust
//! use ebiotic::tools::*;
//! use std::time::Duration;
//! use reqwest;
//!
//! #[tokio::main]
//! async fn main_blast() {
//!
//!    let client = EbioticClient::new(
//!        reqwest::Client::builder()
//!            .timeout(Duration::from_secs(10))
//!            .proxy(reqwest::Proxy::all("http://my-proxy:8080").unwrap())
//!            .build()
//!            .unwrap(),
//!    );
//!
//!    let blast = Blast::default();
//!    let query = "MAKQVQKARKLAEQAERYDDMAAAMKAVTEQGHELSNEERNLLSVAYKNVVGARRSSWRVISSIEQKTERNEKKQQMGKEYREKIEAELQDICNDVLELLDKYLIPNATQPESKVFYLKMKGDYFRYLSEVASGDNKQTTVSNSQQAYQEAFEISKKEMQPTHPIRLGLALNFSVFYYEILNSPDRACRLAKAAFDDASLAKDAESEKNPEEIAWYQSITQ";
//!    let result = blast.run(client, query.to_string()).await;
//! }
//! ```

mod core;
pub mod data;
mod errors;
pub mod tools;
