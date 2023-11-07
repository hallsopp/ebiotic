mod blast;
// pub mod msa;

pub use blast::{Blast, Search};

pub const TOOLS_ENDPOINT: &str = "https://www.ebi.ac.uk/Tools/services/rest/";
pub const BLAST_ENDPOINT: &str = "https://blast.ncbi.nlm.nih.gov/Blast.cgi";
