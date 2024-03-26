//! This module contains the APIs for bioinformatics data and knowledge resources.
//!
//! Currently, the supported databases are:
//!
//! * `DBFetch`: The EBI's database fetch service provides a simple and consistent web-based interface to the data retrieval systems for the major sequence databases. Supported databases include:
//!   * `ENA_Sequence`: The European Nucleotide Archive (ENA) is a comprehensive resource for the collection, storage and presentation of nucleotide sequence data.
//!
//! You can view a complete list of supported databases [here](https://www.ebi.ac.uk/Tools/dbfetch/dbfetch/dbfetch.databases). Please feel free to open an issue or a pull request if you would like to see support for more databases.

// TODO - change the docs here to reflect the current state of the module

pub mod dbfetch;
mod ebisearch;

pub use dbfetch::{dbfetchdbs::DbfetchDbs, Dbfetch, DbfetchStyle};
use std::fmt::{Display, Formatter};

pub use crate::core::EbioticClient;
pub use crate::core::Service;

pub const EBI_DBFETCH_ENDPOINT: &str = "https://www.ebi.ac.uk/Tools/dbfetch/";
pub const EBI_SEARCH_ENDPOINT: &str = "https://www.ebi.ac.uk/ebisearch/ws/rest/";

/// The `DataReturnFormats` enum is used to specify the return format of the various data retrieval services. This is dependent on the type of data available from the database.
#[derive(PartialEq, Debug, Clone)]
pub enum DataReturnFormats {
    Fasta,
    Json,
    Pdb,
    Mmcif,
    Xml,
    Obo,
    Csv,
    Tsv,
    Gff3,
    Gff2,
    PatentEquivalents,
}

/// The `DbfetchIds` struct is used to specify the IDs to be fetched from the `Dbfetch` service.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AccessionIds {
    ids: Vec<String>,
}

trait AvailableReturnFormats {
    fn available_return_formats(&self) -> Vec<DataReturnFormats>;
}

impl AccessionIds {
    /// Create a new `AccessionIds` object with a list of IDs.
    pub fn new(ids: Vec<String>) -> AccessionIds {
        AccessionIds { ids }
    }

    pub fn set_ids(&mut self, ids: Vec<String>) {
        self.ids = ids;
    }

    pub fn ids(&self) -> &Vec<String> {
        &self.ids
    }
}

impl Display for AccessionIds {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut ids = String::new();
        for id in &self.ids {
            ids.push_str(id);
            ids.push(',');
        }
        write!(f, "{}", ids)
    }
}

impl Display for DataReturnFormats {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DataReturnFormats::Fasta => write!(f, "fasta"),
            DataReturnFormats::Json => write!(f, "json"),
            DataReturnFormats::Pdb => write!(f, "pdb"),
            DataReturnFormats::Mmcif => write!(f, "mmcif"),
            DataReturnFormats::Xml => write!(f, "xml"),
            DataReturnFormats::Obo => write!(f, "obo"),
            DataReturnFormats::Csv => write!(f, "csv"),
            DataReturnFormats::Gff3 => write!(f, "gff3"),
            DataReturnFormats::Tsv => write!(f, "tab"),
            DataReturnFormats::Gff2 => write!(f, "gff2"),
            DataReturnFormats::PatentEquivalents => write!(f, "patent_equivalents"),
        }
    }
}
