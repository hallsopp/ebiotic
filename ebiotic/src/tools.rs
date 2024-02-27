//! This module contains the APIs for the various services provided by the EBI and the NCBI.
//!
//! The current services include:
//!
//! * `Blast`: The Basic Local Alignment Search Tool (BLAST) is used to find regions of local similarity between sequences.
//! * `Clustalo`: Clustal Omega is a multiple sequence alignment program for proteins.
//!
//! You can view a complete list of supported services [here](https://www.ebi.ac.uk/Tools/webservices/). Please feel free to open an issue or a pull request if you would like to see support for more services.

pub mod blast;
pub mod msa;

pub use blast::{Blast, BlastResult, Description, Hit, Hsp};
pub use msa::{Clustalo, ClustaloResult};

pub use crate::core::Service;
pub use bio::io::fasta::{Reader, Record};

pub const EBI_TOOLS_ENDPOINT: &str = "https://www.ebi.ac.uk/Tools/services/rest/";
pub const BLAST_ENDPOINT: &str = "https://blast.ncbi.nlm.nih.gov/Blast.cgi";
