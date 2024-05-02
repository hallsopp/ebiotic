//! This module contains the APIs for bioinformatics data and knowledge resources.
//!
//! The module (currently) provides access to the [EMBL-EBI](https://www.ebi.ac.uk/) data retrieval and query services. The module is divided into two submodules:
//!
//! - `dbfetch`: This submodule provides access to the [Dbfetch](https://www.ebi.ac.uk/Tools/dbfetch/) service. The `Dbfetch` struct is used to query the Dbfetch service.
//! - `ebisearch`: This submodule provides access to the [EBI Search](https://www.ebi.ac.uk/ebisearch/) service. The `EbiSearch` struct is used to query the EBI Search service.
//!
//! You can view a complete list of supported databases [here](https://www.ebi.ac.uk/Tools/dbfetch/dbfetch/dbfetch.databases). Please feel free to open an issue or a pull request if you would like to see support for more databases.

// TODO - change the docs here to reflect the current state of the module

use std::fmt::{Display, Formatter};

pub mod dbfetch;
pub mod ebisearch;

pub use dbfetch::{dbfetchdbs::DbfetchDbs, Dbfetch, DbfetchStyle};
pub use ebisearch::{
    ebisearchdomains::EbiSearchDomains, ebisearchquery::EbiSearchQuery, EbiSearch,
};

pub use crate::core::EbioticClient;
pub use crate::core::Service;

pub const EBI_DBFETCH_ENDPOINT: &str = "https://www.ebi.ac.uk/Tools/dbfetch/";
pub const EBI_SEARCH_ENDPOINT: &str = "https://www.ebi.ac.uk/ebisearch/ws/rest/";

/// The `DataReturnFormats` enum is used to specify the return format of the various data retrieval services. This is dependent on the type of data available from the database.
#[derive(Eq, PartialEq, Debug, Clone)]
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

    /// Add an ID to the `AccessionIds` object.
    pub fn add_id(&mut self, id: String) {
        self.ids.push(id);
    }

    /// Override the IDs for the `AccessionIds` object.
    pub fn set_ids(&mut self, ids: Vec<String>) {
        self.ids = ids;
    }

    /// Get the IDs from the `AccessionIds` object.
    pub fn ids(&self) -> &Vec<String> {
        &self.ids
    }
}

impl Display for AccessionIds {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let ids = self.ids.join(",");
        write!(f, "{}", ids)
    }
}

impl From<Vec<String>> for AccessionIds {
    fn from(ids: Vec<String>) -> Self {
        AccessionIds { ids }
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
