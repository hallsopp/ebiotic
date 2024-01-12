mod blast;
mod msa;

pub use blast::{Blast, BlastResult};
pub use msa::{Clustalo, ClustaloResult};

pub use crate::core::Service;
pub use bio::io::fasta::{Reader, Record};

pub const EBI_TOOLS_ENDPOINT: &str = "https://www.ebi.ac.uk/Tools/services/rest/";
pub const BLAST_ENDPOINT: &str = "https://blast.ncbi.nlm.nih.gov/Blast.cgi";
