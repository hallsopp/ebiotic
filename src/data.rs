//! This module contains the APIs for bioinformatics data and knowledge resources.
//!
//! Currently, the supported databases are:
//!
//! * `DBFetch`: The EBI's database fetch service provides a simple and consistent web-based interface to the data retrieval systems for the major sequence databases. Supported databases include:
//!   * `ENA_Sequence`: The European Nucleotide Archive (ENA) is a comprehensive resource for the collection, storage and presentation of nucleotide sequence data.
//!
//! You can view a complete list of supported databases [here](https://www.ebi.ac.uk/Tools/dbfetch/dbfetch/dbfetch.databases). Please feel free to open an issue or a pull request if you would like to see support for more databases.

pub mod dbfetch;

pub use dbfetch::{Dbfetch, DbfetchDbs, DbfetchIds, DbfetchReturnFormat, DbfetchStyle};

pub use crate::core::Service;

pub const EBI_DBFETCH_ENDPOINT: &str = "https://www.ebi.ac.uk/Tools/dbfetch/";
