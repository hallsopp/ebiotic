mod dbfetch;

pub use dbfetch::{Dbfetch, DbfetchIds, DbfetchReturnFormat};

pub use crate::core::Service;

pub const EBI_DBFETCH_ENDPOINT: &str = "https://www.ebi.ac.uk/Tools/dbfetch/";
