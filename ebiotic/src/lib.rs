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
//! ebiotic = "0.0.21"
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
//!    let blast = Blast::default();
//!    let query = "MAKQVQKARKLAEQAERYDDMAAAMKAVTEQGHELSNEERNLLSVAYKNVVGARRSSWRVISSIEQKTERNEKKQQMGKEYREKIEAELQDICNDVLELLDKYLIPNATQPESKVFYLKMKGDYFRYLSEVASGDNKQTTVSNSQQAYQEAFEISKKEMQPTHPIRLGLALNFSVFYYEILNSPDRACRLAKAAFDDASLAKDAESEKNPEEIAWYQSITQ";
//!    let result = blast.run(query.to_string()).await;
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
//!     let dbfetch = Dbfetch::new(DbfetchDbs::EnaSequence, DbfetchReturnFormat::Fasta, DbfetchStyle::Raw);
//!     let ids = DbfetchIds::new(vec!["M10051".to_string(), "M10052".to_string()]);
//!
//!     // The `Dbfetch` service returns a `DbfetchResult` which can be converted into a `Vec<Record>` using the `into_records` method.
//!     // I hope to provide a more ergonomic way of doing this in the future.
//!     let result = dbfetch.run(ids).await.unwrap().into_records();
//! }
//!
//! // This is also the default configuration for the `Dbfetch` service, so the above can be written as:
//!
//! #[tokio::main]
//! async fn main_fasta() {
//!    let dbfetch = Dbfetch::default();
//!    let ids = DbfetchIds::new(vec!["M10051".to_string(), "M10052".to_string()]);
//!    let result = dbfetch.run(ids).await.unwrap().into_records();
//! }
//!```
//!
//! Despite being designed with asynchronicity in mind, the services can also be run synchronously by blocking on the thread. In the future I hope to provide an API for this.
//!
//! ```rust
//! use ebiotic::tools::*;
//! use tokio;
//!
//! #[tokio::main]
//! async fn main() {
//!    let blast = Blast::default();
//!    let query = "MAKQVQKARKLAEQAERYDDMAAAMKAVTEQGHELSNEERNLLSVAYKNVVGARRSSWRVISSIEQKTERNEKKQQMGKEYREKIEAELQDICNDVLELLDKYLIPNATQPESKVFYLKMKGDYFRYLSEVASGDNKQTTVSNSQQAYQEAFEISKKEMQPTHPIRLGLALNFSVFYYEILNSPDRACRLAKAAFDDASLAKDAESEKNPEEIAWYQSITQ";
//!    let result = tokio::task::block_in_place(|| blast.run(query.to_string()));
//! }
//!

mod core;
pub mod data;
mod errors;
pub mod tools;
