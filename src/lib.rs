//! # Ebiotic
//!
//! `ebiotic` provides a light-weight asynchronous interface for some popular Bioinformatics web services. It is designed to
//! enable access to the rich data and tools provided by institutes like the European Bioinformatics
//! Institute's ([(EBI)](https://www.ebi.ac.uk/)) and the National Center for Biotechnology Information ([NCBI](https://www.ncbi.nlm.nih.gov/)). It's built to serialize
//! and deserialze data using common formats like JSON and specialised bioinformatics formats like FASTA from
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
//! ebiotic = "0.0.1"
//! ```
//!
//! ## Example
//!
//! ```rust
//! use ebiotic::tools::*;
//!
//! #[tokio::main]
//! async fn main() {
//!   let blast = Blast::default();
//!   let query = "MAKQVQKARKLAEQAERYDDMAAAMKAVTEQGHELSNEERNLLSVAYKNVVGARRSSWRVISSIEQKTERNEKKQQMGKEYREKIEAELQDICNDVLELLDKYLIPNATQPESKVFYLKMKGDYFRYLSEVASGDNKQTTVSNSQQAYQEAFEISKKEMQPTHPIRLGLALNFSVFYYEILNSPDRACRLAKAAFDDASLAKDAESEKNPEEIAWYQSITQ";
//!   let result = blast.run(query.to_string()).await;   
//! }
//! ```
//!

mod core;
pub mod data;
mod errors;
pub mod tools;
